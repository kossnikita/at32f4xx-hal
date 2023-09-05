use super::{Error, Event, FTimer, Instance, SysEvent, Timer};
use crate::pac::SYST;
use core::ops::{Deref, DerefMut};
use fugit::{HertzU32 as Hertz, TimerDurationU32, TimerInstantU32};

/// Hardware timers
pub struct CounterHz<TMR>(pub(super) Timer<TMR>);

impl<T> Deref for CounterHz<T> {
    type Target = Timer<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for CounterHz<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<TMR: Instance> CounterHz<TMR> {
    /// Releases the TMR peripheral
    pub fn release(mut self) -> Timer<TMR> {
        // stop timer
        self.tmr.ctrl1_reset();
        self.0
    }
}

impl<TMR: Instance> CounterHz<TMR> {
    pub fn start(&mut self, timeout: Hertz) -> Result<(), Error> {
        todo!();

        Ok(())
    }

    pub fn wait(&mut self) -> nb::Result<(), Error> {
        if self.tmr.get_interrupt_flag().contains(Event::Update) {
            self.tmr.clear_interrupt_flag(Event::Update);
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }

    pub fn cancel(&mut self) -> Result<(), Error> {
        if !self.tmr.is_counter_enabled() {
            return Err(Error::Disabled);
        }

        // disable counter
        self.tmr.disable_counter();
        Ok(())
    }
}

/// Periodic non-blocking timer that implements [embedded_hal::timer::CountDown]
pub struct Counter<TMR, const FREQ: u32>(pub(super) FTimer<TMR, FREQ>);

impl<T, const FREQ: u32> Deref for Counter<T, FREQ> {
    type Target = FTimer<T, FREQ>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, const FREQ: u32> DerefMut for Counter<T, FREQ> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// `Counter` with precision of 1 μs (1 MHz sampling)
pub type CounterUs<TMR> = Counter<TMR, 1_000_000>;

/// `Counter` with precision of of 1 ms (1 kHz sampling)
///
/// NOTE: don't use this if your system frequency more than 65 MHz
pub type CounterMs<TMR> = Counter<TMR, 1_000>;

impl<TMR: Instance, const FREQ: u32> Counter<TMR, FREQ> {
    /// Releases the TMR peripheral
    pub fn release(mut self) -> FTimer<TMR, FREQ> {
        // stop counter
        self.tmr.ctrl1_reset();
        self.0
    }

    pub fn now(&self) -> TimerInstantU32<FREQ> {
        TimerInstantU32::from_ticks(self.tmr.read_count().into())
    }

    pub fn start(&mut self, timeout: TimerDurationU32<FREQ>) -> Result<(), Error> {
        // pause
        self.tmr.disable_counter();
        // reset counter
        self.tmr.reset_counter();
        self.tmr.clear_interrupt_flag(Event::Update);

        self.tmr.set_auto_reload(timeout.ticks() - 1)?;

        // Trigger update event to load the registers
        self.tmr.trigger_update();

        // start counter
        self.tmr.enable_counter();

        Ok(())
    }

    pub fn wait(&mut self) -> nb::Result<(), Error> {
        if self.tmr.get_interrupt_flag().contains(Event::Update) {
            self.tmr.clear_interrupt_flag(Event::Update);
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }

    pub fn cancel(&mut self) -> Result<(), Error> {
        if !self.tmr.is_counter_enabled() {
            return Err(Error::Disabled);
        }

        // disable counter
        self.tmr.disable_counter();
        Ok(())
    }
}

impl<TMR: Instance, const FREQ: u32> fugit_timer::Timer<FREQ> for Counter<TMR, FREQ> {
    type Error = Error;

    fn now(&mut self) -> TimerInstantU32<FREQ> {
        Self::now(self)
    }

    fn start(&mut self, duration: TimerDurationU32<FREQ>) -> Result<(), Self::Error> {
        self.start(duration)
    }

    fn cancel(&mut self) -> Result<(), Self::Error> {
        self.cancel()
    }

    fn wait(&mut self) -> nb::Result<(), Self::Error> {
        self.wait()
    }
}

impl Timer<SYST> {
    /// Creates [SysCounterHz] which takes [Hertz] as Duration
    pub fn counter_hz(self) -> SysCounterHz {
        SysCounterHz(self)
    }

    /// Creates [SysCounter] with custom precision (core frequency recommended is known)
    pub fn counter<const FREQ: u32>(self) -> SysCounter<FREQ> {
        SysCounter(self)
    }

    /// Creates [SysCounter] 1 microsecond precision
    pub fn counter_us(self) -> SysCounterUs {
        SysCounter(self)
    }
}

/// Hardware timers
pub struct SysCounterHz(Timer<SYST>);

impl Deref for SysCounterHz {
    type Target = Timer<SYST>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SysCounterHz {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl SysCounterHz {
    pub fn start(&mut self, timeout: Hertz) -> Result<(), Error> {
        let rvr = self.clk.raw() / timeout.raw() - 1;

        if rvr >= (1 << 24) {
            return Err(Error::WrongAutoReload);
        }

        self.tmr.set_reload(rvr);
        self.tmr.clear_current();
        self.tmr.enable_counter();

        Ok(())
    }

    pub fn wait(&mut self) -> nb::Result<(), Error> {
        if self.tmr.has_wrapped() {
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }

    pub fn cancel(&mut self) -> Result<(), Error> {
        if !self.tmr.is_counter_enabled() {
            return Err(Error::Disabled);
        }

        self.tmr.disable_counter();
        Ok(())
    }
}

pub type SysCounterUs = SysCounter<1_000_000>;

/// SysTick timer with precision of 1 μs (1 MHz sampling)
pub struct SysCounter<const FREQ: u32>(Timer<SYST>);

impl<const FREQ: u32> Deref for SysCounter<FREQ> {
    type Target = Timer<SYST>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const FREQ: u32> DerefMut for SysCounter<FREQ> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<const FREQ: u32> SysCounter<FREQ> {
    /// Starts listening for an `event`
    pub fn listen(&mut self, event: SysEvent) {
        match event {
            SysEvent::Update => self.tmr.enable_interrupt(),
        }
    }

    /// Stops listening for an `event`
    pub fn unlisten(&mut self, event: SysEvent) {
        match event {
            SysEvent::Update => self.tmr.disable_interrupt(),
        }
    }

    pub fn now(&self) -> TimerInstantU32<FREQ> {
        TimerInstantU32::from_ticks(
            (SYST::get_reload() - SYST::get_current()) / (self.clk.raw() / FREQ),
        )
    }

    pub fn start(&mut self, timeout: TimerDurationU32<FREQ>) -> Result<(), Error> {
        let rvr = timeout.ticks() * (self.clk.raw() / FREQ) - 1;

        if rvr >= (1 << 24) {
            return Err(Error::WrongAutoReload);
        }

        self.tmr.set_reload(rvr);
        self.tmr.clear_current();
        self.tmr.enable_counter();

        Ok(())
    }

    pub fn wait(&mut self) -> nb::Result<(), Error> {
        if self.tmr.has_wrapped() {
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }

    pub fn cancel(&mut self) -> Result<(), Error> {
        if !self.tmr.is_counter_enabled() {
            return Err(Error::Disabled);
        }

        self.tmr.disable_counter();
        Ok(())
    }
}

impl<const FREQ: u32> fugit_timer::Timer<FREQ> for SysCounter<FREQ> {
    type Error = Error;

    fn now(&mut self) -> TimerInstantU32<FREQ> {
        Self::now(self)
    }

    fn start(&mut self, duration: TimerDurationU32<FREQ>) -> Result<(), Self::Error> {
        self.start(duration)
    }

    fn wait(&mut self) -> nb::Result<(), Self::Error> {
        self.wait()
    }

    fn cancel(&mut self) -> Result<(), Self::Error> {
        self.cancel()
    }
}
