use core::{fmt, ops::Deref};

use nb::block;

use super::{
    config, Error, Event, Listen, Rx, RxISR, RxListen, Serial, SerialExt, Tx, TxISR, TxListen,
};
// use crate::dma::{
//     traits::{DMASet, PeriAddress},
//     MemoryToPeripheral, PeripheralToMemory,
// };
use crate::crm::{self, Clocks};
use crate::gpio::{alt::SerialAsync as CommonPins, NoPin, PushPull};

#[cfg(feature = "uart4")]
pub(crate) use crate::pac::uart4::RegisterBlock as RegisterBlockUart;
pub(crate) use crate::pac::usart1::RegisterBlock as RegisterBlockUsart;

impl crate::Sealed for RegisterBlockUsart {}

// Implemented by all USART/UART instances
pub trait Instance: crate::Sealed + crm::Enable + crm::Reset + crm::BusClock + CommonPins {
    type RegisterBlock;

    #[doc(hidden)]
    fn ptr() -> *const Self::RegisterBlock;
    #[doc(hidden)]
    fn set_stopbits(&self, bits: config::StopBits);
}

pub trait RegisterBlockImpl: crate::Sealed {
    #[allow(clippy::new_ret_no_self)]
    fn new<UART: Instance<RegisterBlock = Self>, WORD>(
        uart: UART,
        pins: (impl Into<UART::Tx<PushPull>>, impl Into<UART::Rx<PushPull>>),
        config: impl Into<config::Config>,
        clocks: &Clocks,
    ) -> Result<Serial<UART, WORD>, config::InvalidConfig>;

    fn read_u16(&self) -> nb::Result<u16, Error>;
    fn write_u16(&self, word: u16) -> nb::Result<(), Error>;

    fn read_u8(&self) -> nb::Result<u8, Error> {
        // Delegate to u16 version, then truncate to 8 bits
        self.read_u16().map(|word16| word16 as u8)
    }

    fn write_u8(&self, word: u8) -> nb::Result<(), Error> {
        // Delegate to u16 version
        self.write_u16(u16::from(word))
    }

    fn flush(&self) -> nb::Result<(), Error>;

    fn bwrite_all_u8(&self, buffer: &[u8]) -> Result<usize, Error> {
        for &b in buffer {
            nb::block!(self.write_u8(b))?;
        }
        Ok(buffer.len())
    }

    fn bwrite_all_u16(&self, buffer: &[u16]) -> Result<(), Error> {
        for &b in buffer {
            nb::block!(self.write_u16(b))?;
        }
        Ok(())
    }

    fn bflush(&self) -> Result<(), Error> {
        nb::block!(self.flush())
    }

    // RxISR
    fn is_idle(&self) -> bool;
    fn is_rx_not_empty(&self) -> bool;
    fn clear_idle_interrupt(&self);

    // TxISR
    fn is_tx_empty(&self) -> bool;

    // RxListen
    fn listen_rxne(&self);
    fn unlisten_rxne(&self);
    fn listen_idle(&self);
    fn unlisten_idle(&self);

    // TxListen
    fn listen_txe(&self);
    fn unlisten_txe(&self);

    // Listen
    fn listen(&self, event: Event);
    fn unlisten(&self, event: Event);

    // PeriAddress
    fn peri_address(&self) -> u32;
}

macro_rules! uartCommon {
    ($RegisterBlock:ty) => {
        impl RegisterBlockImpl for $RegisterBlock {
            fn new<UART: Instance<RegisterBlock = Self>, WORD>(
                uart: UART,
                pins: (impl Into<UART::Tx<PushPull>>, impl Into<UART::Rx<PushPull>>),
                config: impl Into<config::Config>,
                clocks: &Clocks,
            ) -> Result<Serial<UART, WORD>, config::InvalidConfig>
        where {
                use self::config::*;

                let config = config.into();
                unsafe {
                    // Enable clock.
                    UART::enable_unchecked();
                    UART::reset_unchecked();
                }

                let pclk_freq = UART::clock(clocks).raw();
                let baud = config.baudrate.0;

                // Calculate correct baudrate divisor on the fly
                let (_over8, div) = if (pclk_freq / 16) >= baud {
                    // We have the ability to oversample to 16 bits, take
                    // advantage of it.
                    //
                    // We also add `baud / 2` to the `pclk_freq` to ensure
                    // rounding of values to the closest scale, rather than the
                    // floored behavior of normal integer division.
                    let div = (pclk_freq + (baud / 2)) / baud;
                    (false, div)
                } else if (pclk_freq / 8) >= baud {
                    // We are close enough to pclk where we can only
                    // oversample 8.
                    //
                    // See note above regarding `baud` and rounding.
                    let div = ((pclk_freq * 2) + (baud / 2)) / baud;

                    // Ensure the the fractional bits (only 3) are
                    // right-aligned.
                    let frac = div & 0xF;
                    let div = (div & !0xF) | (frac >> 1);
                    (true, div)
                } else {
                    return Err(config::InvalidConfig);
                };

                let register_block = unsafe { &*UART::ptr() };
                register_block
                    .baudr
                    .write(|w| w.bits(div.try_into().unwrap()));

                // Reset other registers to disable advanced USART features
                register_block.ctrl2.reset();
                register_block.ctrl3.reset();

                // Enable transmission and receiving
                // and configure frame

                register_block.ctrl1.write(|w| {
                    w.uen().enable();
                    w.ten().enable();
                    w.ren().enable();
                    w.dbn().bit(config.wordlength == WordLength::DataBits9);
                    w.pen().bit(config.parity != Parity::ParityNone);
                    w.psel().bit(config.parity == Parity::ParityOdd)
                });

                match config.dma {
                    DmaConfig::Tx => register_block.ctrl3.write(|w| w.dmaten().enable()),
                    DmaConfig::Rx => register_block.ctrl3.write(|w| w.dmaren().enable()),
                    DmaConfig::TxRx => register_block
                        .ctrl3
                        .write(|w| w.dmaren().enable().dmaten().enable()),
                    DmaConfig::None => {}
                }

                let serial = Serial {
                    tx: Tx::new(uart, pins.0.into()),
                    rx: Rx::new(pins.1.into()),
                };
                serial.tx.usart.set_stopbits(config.stopbits);
                Ok(serial)
            }

            fn read_u16(&self) -> nb::Result<u16, Error> {
                // NOTE(unsafe) atomic read with no side effects
                let sts = self.sts.read();

                // Any error requires the dr to be read to clear
                if sts.perr().bit_is_set()
                    || sts.ferr().bit_is_set()
                    || sts.nerr().bit_is_set()
                    || sts.roerr().bit_is_set()
                {
                    self.dt.read();
                }

                Err(if sts.perr().is_error() {
                    Error::Parity.into()
                } else if sts.ferr().is_error() {
                    Error::FrameFormat.into()
                } else if sts.nerr().is_noise() {
                    Error::Noise.into()
                } else if sts.roerr().is_overflow() {
                    Error::Overrun.into()
                } else if sts.rdbf().is_full() {
                    // NOTE(unsafe) atomic read from stateless register
                    return Ok(self.dt.read().bits());
                } else {
                    nb::Error::WouldBlock
                })
            }

            fn write_u16(&self, word: u16) -> nb::Result<(), Error> {
                // NOTE(unsafe) atomic read with no side effects
                let sts = self.sts.read();

                if sts.tdbe().is_empty() {
                    // NOTE(unsafe) atomic write to stateless register
                    self.dt.write(|w| unsafe { w.bits(word) });
                    Ok(())
                } else {
                    Err(nb::Error::WouldBlock)
                }
            }

            fn flush(&self) -> nb::Result<(), Error> {
                // NOTE(unsafe) atomic read with no side effects
                let sts = self.sts.read();

                if sts.tdc().is_completed() {
                    Ok(())
                } else {
                    Err(nb::Error::WouldBlock)
                }
            }

            fn is_idle(&self) -> bool {
                self.sts.read().idlef().is_idle()
            }

            fn is_rx_not_empty(&self) -> bool {
                self.sts.read().rdbf().is_full()
            }

            fn clear_idle_interrupt(&self) {
                let _ = self.sts.read();
                let _ = self.dt.read();
            }

            fn is_tx_empty(&self) -> bool {
                self.sts.read().tdbe().is_empty()
            }

            fn listen_rxne(&self) {
                self.ctrl1.modify(|_, w| w.rdbfien().enable())
            }

            fn unlisten_rxne(&self) {
                self.ctrl1.modify(|_, w| w.rdbfien().disable())
            }

            fn listen_idle(&self) {
                self.ctrl1.modify(|_, w| w.idleien().enable())
            }

            fn unlisten_idle(&self) {
                self.ctrl1.modify(|_, w| w.idleien().disable())
            }

            fn listen_txe(&self) {
                self.ctrl1.modify(|_, w| w.tdbeien().enable())
            }

            fn unlisten_txe(&self) {
                self.ctrl1.modify(|_, w| w.tdbeien().disable())
            }

            fn listen(&self, event: Event) {
                match event {
                    Event::Rxne => self.ctrl1.modify(|_, w| w.rdbfien().enable()),
                    Event::Txe => self.ctrl1.modify(|_, w| w.tdbeien().enable()),
                    Event::Idle => self.ctrl1.modify(|_, w| w.idleien().enable()),
                }
            }

            fn unlisten(&self, event: Event) {
                match event {
                    Event::Rxne => self.ctrl1.modify(|_, w| w.rdbfien().disable()),
                    Event::Txe => self.ctrl1.modify(|_, w| w.tdbeien().disable()),
                    Event::Idle => self.ctrl1.modify(|_, w| w.idleien().disable()),
                }
            }

            fn peri_address(&self) -> u32 {
                &self.dt as *const _ as u32
            }
        }
    };
}

uartCommon! { RegisterBlockUsart }

impl<UART: Instance, WORD> RxISR for Serial<UART, WORD>
where
    Rx<UART, WORD>: RxISR,
{
    fn is_idle(&self) -> bool {
        self.rx.is_idle()
    }

    fn is_rx_not_empty(&self) -> bool {
        self.rx.is_rx_not_empty()
    }

    fn clear_idle_interrupt(&self) {
        self.rx.clear_idle_interrupt();
    }
}

impl<UART: Instance, WORD> RxISR for Rx<UART, WORD>
where
    <UART as Instance>::RegisterBlock: RegisterBlockImpl,
{
    fn is_idle(&self) -> bool {
        unsafe { (*UART::ptr()).is_idle() }
    }

    fn is_rx_not_empty(&self) -> bool {
        unsafe { (*UART::ptr()).is_rx_not_empty() }
    }

    fn clear_idle_interrupt(&self) {
        unsafe {
            (*UART::ptr()).clear_idle_interrupt();
        }
    }
}

impl<UART: Instance, WORD> TxISR for Serial<UART, WORD>
where
    Tx<UART, WORD>: TxISR,
{
    fn is_tx_empty(&self) -> bool {
        self.tx.is_tx_empty()
    }
}

impl<UART: Instance, WORD> TxISR for Tx<UART, WORD>
where
    <UART as Instance>::RegisterBlock: RegisterBlockImpl,
    UART: Deref<Target = <UART as Instance>::RegisterBlock>,
{
    fn is_tx_empty(&self) -> bool {
        self.usart.is_tx_empty()
    }
}

impl<UART: Instance, WORD> RxListen for Rx<UART, WORD>
where
    <UART as Instance>::RegisterBlock: RegisterBlockImpl,
{
    fn listen(&mut self) {
        unsafe { (*UART::ptr()).listen_rxne() }
    }

    fn unlisten(&mut self) {
        unsafe { (*UART::ptr()).unlisten_rxne() }
    }

    fn listen_idle(&mut self) {
        unsafe { (*UART::ptr()).listen_idle() }
    }

    fn unlisten_idle(&mut self) {
        unsafe { (*UART::ptr()).unlisten_idle() }
    }
}

impl<UART: Instance, WORD> TxListen for Tx<UART, WORD>
where
    <UART as Instance>::RegisterBlock: RegisterBlockImpl,
    UART: Deref<Target = <UART as Instance>::RegisterBlock>,
{
    fn listen(&mut self) {
        self.usart.listen_txe()
    }

    fn unlisten(&mut self) {
        self.usart.unlisten_txe()
    }
}

impl<UART: Instance, WORD> Listen for Serial<UART, WORD>
where
    <UART as Instance>::RegisterBlock: RegisterBlockImpl,
    UART: Deref<Target = <UART as Instance>::RegisterBlock>,
{
    fn listen(&mut self, event: Event) {
        self.tx.usart.listen(event)
    }

    fn unlisten(&mut self, event: Event) {
        self.tx.usart.unlisten(event)
    }
}

impl<UART: Instance> fmt::Write for Serial<UART>
where
    Tx<UART>: fmt::Write,
{
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.tx.write_str(s)
    }
}

impl<UART: Instance> fmt::Write for Tx<UART>
where
    <UART as Instance>::RegisterBlock: RegisterBlockImpl,
    UART: Deref<Target = <UART as Instance>::RegisterBlock>,
{
    fn write_str(&mut self, s: &str) -> fmt::Result {
        s.bytes()
            .try_for_each(|c| block!(self.usart.write_u8(c)))
            .map_err(|_| fmt::Error)
    }
}

impl<UART: Instance> SerialExt for UART
where
    <UART as Instance>::RegisterBlock: RegisterBlockImpl,
{
    fn serial<WORD>(
        self,
        pins: (impl Into<Self::Tx<PushPull>>, impl Into<Self::Rx<PushPull>>),
        config: impl Into<config::Config>,
        clocks: &Clocks,
    ) -> Result<Serial<Self, WORD>, config::InvalidConfig> {
        Serial::new(self, pins, config, clocks)
    }
    fn tx<WORD>(
        self,
        tx_pin: impl Into<Self::Tx<PushPull>>,
        config: impl Into<config::Config>,
        clocks: &Clocks,
    ) -> Result<Tx<Self, WORD>, config::InvalidConfig>
    where
        NoPin: Into<Self::Rx<PushPull>>,
    {
        Serial::tx(self, tx_pin, config, clocks)
    }
    fn rx<WORD>(
        self,
        rx_pin: impl Into<Self::Rx<PushPull>>,
        config: impl Into<config::Config>,
        clocks: &Clocks,
    ) -> Result<Rx<Self, WORD>, config::InvalidConfig>
    where
        NoPin: Into<Self::Tx<PushPull>>,
    {
        Serial::rx(self, rx_pin, config, clocks)
    }
}

impl<UART: Instance, WORD> Serial<UART, WORD>
where
    <UART as Instance>::RegisterBlock: RegisterBlockImpl,
{
    pub fn tx(
        usart: UART,
        tx_pin: impl Into<UART::Tx<PushPull>>,
        config: impl Into<config::Config>,
        clocks: &Clocks,
    ) -> Result<Tx<UART, WORD>, config::InvalidConfig>
    where
        NoPin: Into<UART::Rx<PushPull>>,
    {
        Self::new(usart, (tx_pin, NoPin::new()), config, clocks).map(|s| s.split().0)
    }
}

impl<UART: Instance, WORD> Serial<UART, WORD>
where
    <UART as Instance>::RegisterBlock: RegisterBlockImpl,
{
    pub fn rx(
        usart: UART,
        rx_pin: impl Into<UART::Rx<PushPull>>,
        config: impl Into<config::Config>,
        clocks: &Clocks,
    ) -> Result<Rx<UART, WORD>, config::InvalidConfig>
    where
        NoPin: Into<UART::Tx<PushPull>>,
    {
        Self::new(usart, (NoPin::new(), rx_pin), config, clocks).map(|s| s.split().1)
    }
}

// unsafe impl<UART: Instance> PeriAddress for Rx<UART, u8>
// where
//     <UART as Instance>::RegisterBlock: RegisterBlockImpl,
// {
//     #[inline(always)]
//     fn address(&self) -> u32 {
//         unsafe { (*UART::ptr()).peri_address() }
//     }

//     type MemSize = u8;
// }

// unsafe impl<UART: CommonPins, STREAM, const CHANNEL: u8> DMASet<STREAM, CHANNEL, PeripheralToMemory>
//     for Rx<UART>
// where
//     UART: DMASet<STREAM, CHANNEL, PeripheralToMemory>,
// {
// }

// unsafe impl<UART: Instance> PeriAddress for Tx<UART, u8>
// where
//     <UART as Instance>::RegisterBlock: RegisterBlockImpl,
//     UART: Deref<Target = <UART as Instance>::RegisterBlock>,
// {
//     #[inline(always)]
//     fn address(&self) -> u32 {
//         self.usart.peri_address()
//     }

//     type MemSize = u8;
// }

// unsafe impl<UART: CommonPins, STREAM, const CHANNEL: u8> DMASet<STREAM, CHANNEL, MemoryToPeripheral>
//     for Tx<UART>
// where
//     UART: DMASet<STREAM, CHANNEL, MemoryToPeripheral>,
// {
// }
