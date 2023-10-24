//!
//! Asynchronous serial communication using USART peripherals
//!
//! # Word length
//!
//! By default, the UART/USART uses 8 data bits. The `Serial`, `Rx`, and `Tx` structs implement
//! the embedded-hal read and write traits with `u8` as the word type.
//!
//! You can also configure the hardware to use 9 data bits with the `Config` `wordlength_9()`
//! function. After creating a `Serial` with this option, use the `with_u16_data()` function to
//! convert the `Serial<_, u8>` object into a `Serial<_, u16>` that can send and receive `u16`s.
//!
//! In this mode, the `Serial<_, u16>`, `Rx<_, u16>`, and `Tx<_, u16>` structs instead implement
//! the embedded-hal read and write traits with `u16` as the word type. You can use these
//! implementations for 9-bit words.

use core::marker::PhantomData;

mod hal;

pub(crate) mod uart_impls;
pub use uart_impls::Instance;
use uart_impls::RegisterBlockImpl;

use crate::gpio::{self, PushPull};

use crate::pac;

use crate::crm::Clocks;
use crate::gpio::NoPin;

/// Serial error
pub use embedded_hal_nb::serial::ErrorKind as Error;

/// Interrupt event
pub enum Event {
    /// New data has been received
    Rxne,
    /// New data can be sent
    Txe,
    /// Idle line state detected
    Idle,
}

pub mod config;

pub use config::Config;

/// A filler type for when the Tx pin is unnecessary
pub use gpio::NoPin as NoTx;
/// A filler type for when the Rx pin is unnecessary
pub use gpio::NoPin as NoRx;

pub use gpio::alt::SerialAsync as CommonPins;

/// Trait for [`Rx`] interrupt handling.
pub trait RxISR {
    /// Return true if the line idle status is set
    fn is_idle(&self) -> bool;

    /// Return true if the rx register is not empty (and can be read)
    fn is_rx_not_empty(&self) -> bool;

    /// Clear idle line interrupt flag
    fn clear_idle_interrupt(&self);
}

/// Trait for [`Tx`] interrupt handling.
pub trait TxISR {
    /// Return true if the tx register is empty (and can accept data)
    fn is_tx_empty(&self) -> bool;
}

/// Trait for listening [`Rx`] interrupt events.
pub trait RxListen {
    /// Start listening for an rx not empty interrupt event
    ///
    /// Note, you will also have to enable the corresponding interrupt
    /// in the NVIC to start receiving events.
    fn listen(&mut self);

    /// Stop listening for the rx not empty interrupt event
    fn unlisten(&mut self);

    /// Start listening for a line idle interrupt event
    ///
    /// Note, you will also have to enable the corresponding interrupt
    /// in the NVIC to start receiving events.
    fn listen_idle(&mut self);

    /// Stop listening for the line idle interrupt event
    fn unlisten_idle(&mut self);
}

/// Trait for listening [`Tx`] interrupt event.
pub trait TxListen {
    /// Start listening for a tx empty interrupt event
    ///
    /// Note, you will also have to enable the corresponding interrupt
    /// in the NVIC to start receiving events.
    fn listen(&mut self);

    /// Stop listening for the tx empty interrupt event
    fn unlisten(&mut self);
}

/// Trait for listening [`Serial`] interrupt events.
pub trait Listen {
    /// Starts listening for an interrupt event
    ///
    /// Note, you will also have to enable the corresponding interrupt
    /// in the NVIC to start receiving events.
    fn listen(&mut self, event: Event);

    /// Stop listening for an interrupt event
    fn unlisten(&mut self, event: Event);
}

/// Serial abstraction
pub struct Serial<USART: CommonPins, WORD = u8> {
    tx: Tx<USART, WORD>,
    rx: Rx<USART, WORD>,
}

/// Serial receiver containing RX pin
pub struct Rx<USART: CommonPins, WORD = u8> {
    _word: PhantomData<(USART, WORD)>,
    pin: USART::Rx<PushPull>,
}

/// Serial transmitter containing TX pin
pub struct Tx<USART: CommonPins, WORD = u8> {
    _word: PhantomData<WORD>,
    usart: USART,
    pin: USART::Tx<PushPull>,
}

pub trait SerialExt: Sized + Instance {
    fn serial<WORD>(
        self,
        pins: (impl Into<Self::Tx<PushPull>>, impl Into<Self::Rx<PushPull>>),
        config: impl Into<config::Config>,
        clocks: &Clocks,
    ) -> Result<Serial<Self, WORD>, config::InvalidConfig>;

    fn tx<WORD>(
        self,
        tx_pin: impl Into<Self::Tx<PushPull>>,
        config: impl Into<config::Config>,
        clocks: &Clocks,
    ) -> Result<Tx<Self, WORD>, config::InvalidConfig>
    where
        NoPin: Into<Self::Rx<PushPull>>;

    fn rx<WORD>(
        self,
        rx_pin: impl Into<Self::Rx<PushPull>>,
        config: impl Into<config::Config>,
        clocks: &Clocks,
    ) -> Result<Rx<Self, WORD>, config::InvalidConfig>
    where
        NoPin: Into<Self::Tx<PushPull>>;
}

impl<USART: Instance, WORD> Serial<USART, WORD> {
    pub fn new(
        usart: USART,
        pins: (
            impl Into<USART::Tx<PushPull>>,
            impl Into<USART::Rx<PushPull>>,
        ),
        config: impl Into<config::Config>,
        clocks: &Clocks,
    ) -> Result<Self, config::InvalidConfig>
    where
        <USART as Instance>::RegisterBlock: uart_impls::RegisterBlockImpl,
    {
        <USART as Instance>::RegisterBlock::new(usart, pins, config, clocks)
    }
}

impl<UART: CommonPins, WORD> Serial<UART, WORD> {
    pub fn split(self) -> (Tx<UART, WORD>, Rx<UART, WORD>) {
        (self.tx, self.rx)
    }

    #[allow(clippy::type_complexity)]
    pub fn release(self) -> (UART, (UART::Tx<PushPull>, UART::Rx<PushPull>)) {
        (self.tx.usart, (self.tx.pin, self.rx.pin))
    }
}

macro_rules! halUsart {
    ($USART:ty, $Serial:ident, $Rx:ident, $Tx:ident) => {
        pub type $Serial<WORD = u8> = Serial<$USART, WORD>;
        pub type $Rx<WORD = u8> = Rx<$USART, WORD>;
        pub type $Tx<WORD = u8> = Tx<$USART, WORD>;

        impl Instance for $USART {
            type RegisterBlock = crate::serial::uart_impls::RegisterBlockUsart;

            fn ptr() -> *const crate::serial::uart_impls::RegisterBlockUsart {
                <$USART>::ptr() as *const _
            }

            fn set_stopbits(&self, bits: config::StopBits) {
                use crate::pac::usart1::ctrl2::STOPBN_A;
                use config::StopBits;

                self.ctrl2.write(|w| {
                    w.stopbn().variant(match bits {
                        StopBits::STOP0P5 => STOPBN_A::Bit05,
                        StopBits::STOP1 => STOPBN_A::Bit1,
                        StopBits::STOP1P5 => STOPBN_A::Bit15,
                        StopBits::STOP2 => STOPBN_A::Bit2,
                    })
                });
            }
        }
    };
}
pub(crate) use halUsart;

halUsart! { pac::USART1, Serial1, Rx1, Tx1 }
halUsart! { pac::USART2, Serial2, Rx2, Tx2 }

#[cfg(feature = "usart3")]
halUsart! { pac::USART3, Serial3, Rx3, Tx3 }

impl<UART: CommonPins> Rx<UART, u8> {
    pub(crate) fn with_u16_data(self) -> Rx<UART, u16> {
        Rx::new(self.pin)
    }
}

impl<UART: CommonPins> Rx<UART, u16> {
    pub(crate) fn with_u8_data(self) -> Rx<UART, u8> {
        Rx::new(self.pin)
    }
}

impl<UART: CommonPins> Tx<UART, u8> {
    pub(crate) fn with_u16_data(self) -> Tx<UART, u16> {
        Tx::new(self.usart, self.pin)
    }
}

impl<UART: CommonPins> Tx<UART, u16> {
    pub(crate) fn with_u8_data(self) -> Tx<UART, u8> {
        Tx::new(self.usart, self.pin)
    }
}

impl<UART: CommonPins, WORD> Rx<UART, WORD> {
    pub(crate) fn new(pin: UART::Rx<PushPull>) -> Self {
        Self {
            _word: PhantomData,
            pin,
        }
    }

    pub fn join(self, tx: Tx<UART, WORD>) -> Serial<UART, WORD> {
        Serial { tx, rx: self }
    }
}

impl<UART: CommonPins, WORD> Tx<UART, WORD> {
    pub(crate) fn new(usart: UART, pin: UART::Tx<PushPull>) -> Self {
        Self {
            _word: PhantomData,
            usart,
            pin,
        }
    }

    pub fn join(self, rx: Rx<UART, WORD>) -> Serial<UART, WORD> {
        Serial { tx: self, rx }
    }
}

impl<UART: Instance, WORD> AsRef<Tx<UART, WORD>> for Serial<UART, WORD> {
    #[inline(always)]
    fn as_ref(&self) -> &Tx<UART, WORD> {
        &self.tx
    }
}

impl<UART: Instance, WORD> AsRef<Rx<UART, WORD>> for Serial<UART, WORD> {
    #[inline(always)]
    fn as_ref(&self) -> &Rx<UART, WORD> {
        &self.rx
    }
}

impl<UART: Instance, WORD> AsMut<Tx<UART, WORD>> for Serial<UART, WORD> {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Tx<UART, WORD> {
        &mut self.tx
    }
}

impl<UART: Instance, WORD> AsMut<Rx<UART, WORD>> for Serial<UART, WORD> {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Rx<UART, WORD> {
        &mut self.rx
    }
}

impl<UART: Instance> Serial<UART, u8> {
    /// Converts this Serial into a version that can read and write `u16` values instead of `u8`s
    ///
    /// This can be used with a word length of 9 bits.
    pub fn with_u16_data(self) -> Serial<UART, u16> {
        Serial {
            tx: self.tx.with_u16_data(),
            rx: self.rx.with_u16_data(),
        }
    }
}

impl<UART: Instance> Serial<UART, u16> {
    /// Converts this Serial into a version that can read and write `u8` values instead of `u16`s
    ///
    /// This can be used with a word length of 8 bits.
    pub fn with_u8_data(self) -> Serial<UART, u8> {
        Serial {
            tx: self.tx.with_u8_data(),
            rx: self.rx.with_u8_data(),
        }
    }
}
