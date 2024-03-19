use core::ops::Deref;

use crate::crm::{Clocks, Enable, Reset};
use crate::gpio;
use crate::pac::{self, i2c1};
use embedded_hal::i2c::{ErrorKind, ErrorType, NoAcknowledgeSource, Operation};

use fugit::{HertzU32 as Hertz, RateExtU32};

#[derive(Debug, Eq, PartialEq)]
pub enum DutyCycle {
    Ratio2to1,
    Ratio16to9,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Mode {
    Standard {
        frequency: Hertz,
    },
    Fast {
        frequency: Hertz,
        duty_cycle: DutyCycle,
    },
}

impl Mode {
    pub fn standard(frequency: Hertz) -> Self {
        Self::Standard { frequency }
    }

    pub fn fast(frequency: Hertz, duty_cycle: DutyCycle) -> Self {
        Self::Fast {
            frequency,
            duty_cycle,
        }
    }

    pub fn get_frequency(&self) -> Hertz {
        match *self {
            Self::Standard { frequency } => frequency,
            Self::Fast { frequency, .. } => frequency,
        }
    }
}

impl From<Hertz> for Mode {
    fn from(frequency: Hertz) -> Self {
        let k100: Hertz = 100.kHz();
        if frequency <= k100 {
            Self::Standard { frequency }
        } else {
            Self::Fast {
                frequency,
                duty_cycle: DutyCycle::Ratio2to1,
            }
        }
    }
}

/// I2C abstraction
pub struct I2c<I2C: Instance> {
    i2c: I2C,
    pins: (I2C::Scl, I2C::Sda),
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[non_exhaustive]
pub enum Error {
    Overrun,
    NoAcknowledge(NoAcknowledgeSource),
    Timeout,
    // Note: The Bus error type is not currently returned, but is maintained for compatibility.
    Bus,
    Crc,
    ArbitrationLoss,
}

impl Error {
    pub(crate) fn nack_addr(self) -> Self {
        match self {
            Error::NoAcknowledge(NoAcknowledgeSource::Unknown) => {
                Error::NoAcknowledge(NoAcknowledgeSource::Address)
            }
            e => e,
        }
    }
    pub(crate) fn nack_data(self) -> Self {
        match self {
            Error::NoAcknowledge(NoAcknowledgeSource::Unknown) => {
                Error::NoAcknowledge(NoAcknowledgeSource::Data)
            }
            e => e,
        }
    }
}

impl embedded_hal::i2c::Error for Error {
    fn kind(&self) -> ErrorKind {
        match *self {
            Self::Overrun => ErrorKind::Overrun,
            Self::Bus => ErrorKind::Bus,
            Self::ArbitrationLoss => ErrorKind::ArbitrationLoss,
            Self::NoAcknowledge(nack) => ErrorKind::NoAcknowledge(nack),
            Self::Crc | Self::Timeout => ErrorKind::Other,
        }
    }
}

impl<I2C: Instance> ErrorType for I2c<I2C> {
    type Error = Error;
}

pub trait Instance:
    crate::Sealed + Deref<Target = i2c1::RegisterBlock> + Enable + Reset + gpio::alt::I2cCommon
{
    #[doc(hidden)]
    fn ptr() -> *const i2c1::RegisterBlock;
}

// Implemented by all I2C instances
macro_rules! i2c {
    ($I2C:ty: $I2c:ident) => {
        pub type $I2c = I2c<$I2C>;

        impl Instance for $I2C {
            fn ptr() -> *const i2c1::RegisterBlock {
                <$I2C>::ptr() as *const _
            }
        }
    };
}

i2c! { pac::I2C1: I2c1 }
i2c! { pac::I2C2: I2c2 }

pub trait I2cExt: Sized + Instance {
    fn i2c(
        self,
        pins: (impl Into<Self::Scl>, impl Into<Self::Sda>),
        mode: impl Into<Mode>,
        clocks: &Clocks,
    ) -> I2c<Self>;
}

impl<I2C: Instance> I2cExt for I2C {
    fn i2c(
        self,
        pins: (impl Into<Self::Scl>, impl Into<Self::Sda>),
        mode: impl Into<Mode>,
        clocks: &Clocks,
    ) -> I2c<Self> {
        I2c::new(self, pins, mode, clocks)
    }
}

impl<I2C> I2c<I2C>
where
    I2C: Instance,
{
    pub fn new(
        i2c: I2C,
        pins: (impl Into<I2C::Scl>, impl Into<I2C::Sda>),
        mode: impl Into<Mode>,
        clocks: &Clocks,
    ) -> Self {
        unsafe {
            // Enable and reset clock.
            I2C::enable_unchecked();
            I2C::reset_unchecked();
        }

        let pins = (pins.0.into(), pins.1.into());

        let i2c = I2c { i2c, pins };
        i2c.i2c_init(mode, clocks.pclk1());
        i2c
    }

    pub fn release(self) -> (I2C, (I2C::Scl, I2C::Sda)) {
        (self.i2c, self.pins)
    }
}

impl<I2C: Instance> I2c<I2C> {
    fn i2c_init(&self, mode: impl Into<Mode>, pclk: Hertz) {
        let mode = mode.into();
        // Make sure the I2C unit is disabled so we can configure it
        self.i2c.ctrl1().modify(|_, w| w.i2cen().disable());

        // Calculate settings for I2C speed modes
        let clock = pclk.raw();
        let clc_mhz = clock / 1_000_000;
        assert!((2..=50).contains(&clc_mhz));

        #[cfg(feature = "i2c-v1")]
        // Configure bus frequency into I2C peripheral
        self.i2c
            .ctrl2()
            .write(|w| unsafe { w.clkfreq().bits(clc_mhz as u8) });

        #[cfg(feature = "i2c-v1")]
        let trise = match mode {
            Mode::Standard { .. } => clc_mhz + 1,
            Mode::Fast { .. } => clc_mhz * 300 / 1000 + 1,
        };

        #[cfg(feature = "i2c-v1")]
        // Configure correct rise times
        self.i2c.tmrise().write(|w| w.risetime().bits(trise as u8));

        match mode {
            // I2C clock control calculation
            Mode::Standard { frequency } => {
                let speed: u32 = (clock / (frequency.raw() * 2)).max(4);

                #[cfg(feature = "i2c-v1")]
                // Set clock to standard mode with appropriate parameters for selected speed
                self.i2c.clkctrl().write(|w| {
                    w.speedmode()
                        .standard()
                        .dutymode()
                        .duty2_1()
                        .speed()
                        .bits(speed as u16)
                });
            }
            Mode::Fast {
                frequency,
                duty_cycle,
            } => match duty_cycle {
                DutyCycle::Ratio2to1 => {
                    let speed: u32 = (clock / (frequency.raw() * 3)).max(1);

                    // Set clock to fast mode with appropriate parameters for selected speed (2:1 duty cycle)
                    self.i2c.clkctrl().write(|w| {
                        w.speedmode()
                            .fast()
                            .dutymode()
                            .duty2_1()
                            .speed()
                            .bits(speed as u16)
                    });
                }
                DutyCycle::Ratio16to9 => {
                    let speed = (clock / (frequency.raw() * 25)).max(1);

                    // Set clock to fast mode with appropriate parameters for selected speed (16:9 duty cycle)
                    self.i2c.clkctrl().write(|w| {
                        w.speedmode()
                            .fast()
                            .dutymode()
                            .duty16_9()
                            .speed()
                            .bits(speed as u16)
                    });
                }
            },
        }

        // Enable the I2C processing
        self.i2c.ctrl1().modify(|_, w| w.i2cen().enable());
    }

    fn check_and_clear_error_flags(&self) -> Result<i2c1::sts1::R, Error> {
        // Note that flags should only be cleared once they have been registered. If flags are
        // cleared otherwise, there may be an inherent race condition and flags may be missed.
        let sts1 = self.i2c.sts1().read();

        if sts1.tmout().is_timeout() {
            self.i2c.sts1().modify(|_, w| w.tmout().clear());
            return Err(Error::Timeout);
        }

        if sts1.pecerr().is_error() {
            self.i2c.sts1().modify(|_, w| w.pecerr().clear());
            return Err(Error::Crc);
        }

        if sts1.ouf().is_overrun() {
            self.i2c.sts1().modify(|_, w| w.ouf().clear());
            return Err(Error::Overrun);
        }

        if sts1.ackfail().is_failure() {
            self.i2c.sts1().modify(|_, w| w.ackfail().clear());
            return Err(Error::NoAcknowledge(NoAcknowledgeSource::Unknown));
        }

        if sts1.arlost().is_lost() {
            self.i2c.sts1().modify(|_, w| w.arlost().clear());
            return Err(Error::ArbitrationLoss);
        }

        // The errata indicates that BERR may be incorrectly detected. It recommends ignoring and
        // clearing the BERR bit instead.
        // Check this for Artery later
        if sts1.buserr().is_error() {
            self.i2c.sts1().modify(|_, w| w.buserr().clear());
        }

        Ok(sts1)
    }

    /// Sends START and Address for writing
    #[inline(always)]
    fn prepare_write(&self, addr: u8) -> Result<(), Error> {
        // Wait until a previous STOP condition finishes
        while self.i2c.ctrl1().read().genstop().is_stop() {}

        // Send a START condition
        self.i2c.ctrl1().modify(|_, w| w.genstart().start());

        // Wait until START condition was generated
        while self.check_and_clear_error_flags()?.startf().is_no_start() {}

        // Also wait until signalled we're master and everything is waiting for us
        loop {
            self.check_and_clear_error_flags()?;

            let sts2 = self.i2c.sts2().read();
            if !(sts2.trmode().is_slave() && sts2.busyf().is_idle()) {
                break;
            }
        }

        // Set up current address, we're trying to talk to
        self.i2c
            .dt()
            .write(|w| unsafe { w.bits(u32::from(addr) << 1) });

        // Wait until address was sent
        loop {
            // Check for any I2C errors. If a NACK occurs, the ADDR bit will never be set.
            let sts1 = self
                .check_and_clear_error_flags()
                .map_err(Error::nack_addr)?;

            // Wait for the address to be acknowledged
            if sts1.addr7f().is_match() {
                break;
            }
        }

        // Clear condition by reading STS2
        self.i2c.sts2().read();

        Ok(())
    }

    /// Sends START and Address for reading
    fn prepare_read(&self, addr: u8) -> Result<(), Error> {
        // Wait until a previous STOP condition finishes
        while self.i2c.ctrl1().read().genstop().is_stop() {}

        // Send a START condition and set ACK bit
        self.i2c
            .ctrl1()
            .modify(|_, w| w.genstart().start().acken().enable());

        // Wait until START condition was generated
        while self.i2c.sts1().read().startf().is_no_start() {}

        // Also wait until signalled we're master and everything is waiting for us
        while {
            let sts2 = self.i2c.sts2().read();
            sts2.trmode().is_slave() && sts2.busyf().is_idle()
        } {}

        // Set up current address, we're trying to talk to
        self.i2c
            .dt()
            .write(|w| unsafe { w.bits((u32::from(addr) << 1) + 1) });

        // Wait until address was sent
        loop {
            self.check_and_clear_error_flags()
                .map_err(Error::nack_addr)?;
            if self.i2c.sts1().read().addr7f().is_match() {
                break;
            }
        }

        // Clear condition by reading STS2
        self.i2c.sts2().read();

        Ok(())
    }

    fn write_bytes(&mut self, bytes: impl Iterator<Item = u8>) -> Result<(), Error> {
        // Send bytes
        for c in bytes {
            self.send_byte(c)?;
        }

        // Fallthrough is success
        Ok(())
    }

    fn send_byte(&self, byte: u8) -> Result<(), Error> {
        // Wait until we're ready for sending
        // Check for any I2C errors. If a NACK occurs, the ADDR bit will never be set.
        while self
            .check_and_clear_error_flags()
            .map_err(Error::nack_addr)?
            .tdbe()
            .is_not_empty()
        {}

        // Push out a byte of data
        self.i2c.dt().write(|w| unsafe { w.bits(u32::from(byte)) });

        // Wait until byte is transferred
        // Check for any potential error conditions.
        while self
            .check_and_clear_error_flags()
            .map_err(Error::nack_data)?
            .tdc()
            .is_not_finished()
        {}

        Ok(())
    }

    fn recv_byte(&self) -> Result<u8, Error> {
        loop {
            // Check for any potential error conditions.
            self.check_and_clear_error_flags()
                .map_err(Error::nack_data)?;

            if self.i2c.sts1().read().rdbf().is_not_empty() {
                break;
            }
        }

        let value = self.i2c.dt().read().bits() as u8;
        Ok(value)
    }

    fn read_bytes(&mut self, buffer: &mut [u8]) -> Result<(), Error> {
        // Receive bytes into buffer
        for c in buffer {
            *c = self.recv_byte()?;
        }

        Ok(())
    }

    /// Reads like normal but does'n generate start and don't send address
    fn read_wo_prepare(&mut self, buffer: &mut [u8]) -> Result<(), Error> {
        if let Some((last, buffer)) = buffer.split_last_mut() {
            // Read all bytes but not last
            self.read_bytes(buffer)?;

            // Prepare to send NACK then STOP after next byte
            self.i2c
                .ctrl1()
                .modify(|_, w| w.acken().disable().genstop().stop());

            // Receive last byte
            *last = self.recv_byte()?;

            // Fallthrough is success
            Ok(())
        } else {
            Err(Error::Overrun)
        }
    }

    pub fn read(&mut self, addr: u8, buffer: &mut [u8]) -> Result<(), Error> {
        if buffer.is_empty() {
            return Err(Error::Overrun);
        }

        self.prepare_read(addr)?;
        self.read_wo_prepare(buffer)
    }

    /// Writes like normal but does'n generate start and don't send address
    fn write_wo_prepare(&mut self, bytes: &[u8]) -> Result<(), Error> {
        self.write_bytes(bytes.iter().cloned())?;

        // Send a STOP condition
        self.i2c.ctrl1().modify(|_, w| w.genstop().stop());

        // Fallthrough is success
        Ok(())
    }

    pub fn write(&mut self, addr: u8, bytes: &[u8]) -> Result<(), Error> {
        self.prepare_write(addr)?;
        self.write_wo_prepare(bytes)
    }

    pub fn write_iter<B>(&mut self, addr: u8, bytes: B) -> Result<(), Error>
    where
        B: IntoIterator<Item = u8>,
    {
        self.prepare_write(addr)?;
        self.write_bytes(bytes.into_iter())?;

        // Send a STOP condition
        self.i2c.ctrl1().modify(|_, w| w.genstop().stop());

        // Fallthrough is success
        Ok(())
    }

    pub fn write_read(&mut self, addr: u8, bytes: &[u8], buffer: &mut [u8]) -> Result<(), Error> {
        self.prepare_write(addr)?;
        self.write_bytes(bytes.iter().cloned())?;
        self.read(addr, buffer)
    }

    pub fn write_iter_read<B>(&mut self, addr: u8, bytes: B, buffer: &mut [u8]) -> Result<(), Error>
    where
        B: IntoIterator<Item = u8>,
    {
        self.prepare_write(addr)?;
        self.write_bytes(bytes.into_iter())?;
        self.read(addr, buffer)
    }

    pub fn transaction<'a>(
        &mut self,
        addr: u8,
        mut ops: impl Iterator<Item = Operation<'a>>,
    ) -> Result<(), Error> {
        if let Some(mut prev_op) = ops.next() {
            // 1. Generate Start for operation
            match &prev_op {
                Operation::Read(_) => self.prepare_read(addr)?,
                Operation::Write(_) => self.prepare_write(addr)?,
            };

            for op in ops {
                // 2. Execute previous operations.
                match &mut prev_op {
                    Operation::Read(rb) => self.read_bytes(rb)?,
                    Operation::Write(wb) => self.write_bytes(wb.iter().cloned())?,
                };
                // 3. If operation changes type we must generate new start
                match (&prev_op, &op) {
                    (Operation::Read(_), Operation::Write(_)) => self.prepare_write(addr)?,
                    (Operation::Write(_), Operation::Read(_)) => self.prepare_read(addr)?,
                    _ => {} // No changes if operation have not changed
                }

                prev_op = op;
            }

            // 4. Now, prev_op is last command use methods variations that will generate stop
            match prev_op {
                Operation::Read(rb) => self.read_wo_prepare(rb)?,
                Operation::Write(wb) => self.write_wo_prepare(wb)?,
            };
        }

        // Fallthrough is success
        Ok(())
    }

    pub fn transaction_slice(
        &mut self,
        addr: u8,
        ops_slice: &mut [Operation<'_>],
    ) -> Result<(), Error> {
        let i2c = self;
        let mut ops = ops_slice.iter_mut();
        if let Some(mut prev_op) = ops.next() {
            // 1. Generate Start for operation
            match &prev_op {
                Operation::Read(_) => i2c.prepare_read(addr)?,
                Operation::Write(_) => i2c.prepare_write(addr)?,
            };

            for op in ops {
                // 2. Execute previous operations.
                match &mut prev_op {
                    Operation::Read(rb) => i2c.read_bytes(rb)?,
                    Operation::Write(wb) => i2c.write_bytes(wb.iter().cloned())?,
                };
                // 3. If operation changes type we must generate new start
                match (&prev_op, &op) {
                    (Operation::Read(_), Operation::Write(_)) => i2c.prepare_write(addr)?,
                    (Operation::Write(_), Operation::Read(_)) => i2c.prepare_read(addr)?,
                    _ => {} // No changes if operation have not changed
                }

                prev_op = op;
            }

            // 4. Now, prev_op is last command use methods variations that will generate stop
            match prev_op {
                Operation::Read(rb) => i2c.read_wo_prepare(rb)?,
                Operation::Write(wb) => i2c.write_wo_prepare(wb)?,
            };
        }
        // Fallthrough is success
        Ok(())
    }
}

mod blocking {
    use super::{I2c, Instance, Operation};

    impl<I2C: Instance> embedded_hal::i2c::I2c for I2c<I2C> {
        fn read(&mut self, addr: u8, buffer: &mut [u8]) -> Result<(), Self::Error> {
            self.read(addr, buffer)
        }

        fn write(&mut self, addr: u8, bytes: &[u8]) -> Result<(), Self::Error> {
            self.write(addr, bytes)
        }

        fn write_read(
            &mut self,
            addr: u8,
            bytes: &[u8],
            buffer: &mut [u8],
        ) -> Result<(), Self::Error> {
            self.write_read(addr, bytes, buffer)
        }

        fn transaction(
            &mut self,
            addr: u8,
            operations: &mut [Operation<'_>],
        ) -> Result<(), Self::Error> {
            self.transaction_slice(addr, operations)
        }
    }
}
