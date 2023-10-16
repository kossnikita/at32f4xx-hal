//! Timers

use core::convert::TryFrom;
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m::peripheral::SYST;

use crate::bb;
use crate::pac;

use crate::crm::{self, Clocks};
use fugit::HertzU32 as Hertz;

pub mod counter;
pub use counter::*;
pub mod delay;
pub use delay::*;
pub mod pwm;
pub use pwm::*;

pub mod hal;
pub use hal::*;

/// Timer wrapper.
///
/// This wrapper can be used both for the system timer (SYST) or the
/// general-purpose timer (TMRx).
///
/// Note: If you want to use the timer to sleep a certain amount of timer, use
/// [`Delay`](`crate::timer::delay::Delay`).
pub struct Timer<TMR> {
    pub(crate) tmr: TMR,
    pub(crate) clk: Hertz,
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum Channel {
    C1 = 0,
    C2 = 1,
    C3 = 2,
    C4 = 3,
}

pub use crate::gpio::alt::TmrCPin as CPin;

/// Channel wrapper
pub struct Ch<const C: u8, const COMP: bool>;
pub const C1: u8 = 0;
pub const C2: u8 = 1;
pub const C3: u8 = 2;
pub const C4: u8 = 3;

/// Enum for IO polarity
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Polarity {
    ActiveHigh,
    ActiveLow,
}

/// Output Idle state
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleState {
    Reset,
    Set,
}

bitflags::bitflags! {
    pub struct Event: u32 {
        const Update  = 1 << 0;
        const C1 = 1 << 1;
        const C2 = 1 << 2;
        const C3 = 1 << 3;
        const C4 = 1 << 4;
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    /// Timer is disabled
    Disabled,
    WrongAutoReload,
}

pub trait TimerExt: Sized {
    /// Non-blocking [Counter] with custom fixed precision
    fn counter<const FREQ: u32>(self, clocks: &Clocks) -> Counter<Self, FREQ>;
    /// Non-blocking [Counter] with fixed precision of 1 ms (1 kHz sampling)
    ///
    /// Can wait from 2 ms to 65 sec for 16-bit timer and from 2 ms to 49 days for 32-bit timer.
    ///
    /// NOTE: don't use this if your system frequency more than 65 MHz
    fn counter_ms(self, clocks: &Clocks) -> CounterMs<Self> {
        self.counter::<1_000>(clocks)
    }
    /// Non-blocking [Counter] with fixed precision of 1 μs (1 MHz sampling)
    ///
    /// Can wait from 2 μs to 65 ms for 16-bit timer and from 2 μs to 71 min for 32-bit timer.
    fn counter_us(self, clocks: &Clocks) -> CounterUs<Self> {
        self.counter::<1_000_000>(clocks)
    }
    /// Non-blocking [Counter] with dynamic precision which uses `Hertz` as Duration units
    fn counter_hz(self, clocks: &Clocks) -> CounterHz<Self>;

    /// Blocking [Delay] with custom fixed precision
    fn delay<const FREQ: u32>(self, clocks: &Clocks) -> Delay<Self, FREQ>;
    /// Blocking [Delay] with fixed precision of 1 ms (1 kHz sampling)
    ///
    /// Can wait from 2 ms to 49 days.
    ///
    /// NOTE: don't use this if your system frequency more than 65 MHz
    fn delay_ms(self, clocks: &Clocks) -> DelayMs<Self> {
        self.delay::<1_000>(clocks)
    }
    /// Blocking [Delay] with fixed precision of 1 μs (1 MHz sampling)
    ///
    /// Can wait from 2 μs to 71 min.
    fn delay_us(self, clocks: &Clocks) -> DelayUs<Self> {
        self.delay::<1_000_000>(clocks)
    }
}

impl<TMR: Instance> TimerExt for TMR {
    fn counter<const FREQ: u32>(self, clocks: &Clocks) -> Counter<Self, FREQ> {
        FTimer::new(self, clocks).counter()
    }
    fn counter_hz(self, clocks: &Clocks) -> CounterHz<Self> {
        Timer::new(self, clocks).counter_hz()
    }
    fn delay<const FREQ: u32>(self, clocks: &Clocks) -> Delay<Self, FREQ> {
        FTimer::new(self, clocks).delay()
    }
}

pub trait SysTimerExt: Sized {
    /// Creates timer which takes [Hertz] as Duration
    fn counter_hz(self, clocks: &Clocks) -> SysCounterHz;

    /// Creates timer with custom precision (core frequency recommended is known)
    fn counter<const FREQ: u32>(self, clocks: &Clocks) -> SysCounter<FREQ>;
    /// Creates timer with precision of 1 μs (1 MHz sampling)
    fn counter_us(self, clocks: &Clocks) -> SysCounterUs {
        self.counter::<1_000_000>(clocks)
    }
    /// Blocking [Delay] with custom precision
    fn delay(self, clocks: &Clocks) -> SysDelay;
}

impl SysTimerExt for SYST {
    fn counter_hz(self, clocks: &Clocks) -> SysCounterHz {
        Timer::syst(self, clocks).counter_hz()
    }
    fn counter<const FREQ: u32>(self, clocks: &Clocks) -> SysCounter<FREQ> {
        Timer::syst(self, clocks).counter()
    }
    fn delay(self, clocks: &Clocks) -> SysDelay {
        Timer::syst_external(self, clocks).delay()
    }
}

/// Interrupt events
#[derive(Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SysEvent {
    /// [Timer] timed out / count down ended
    Update,
}

impl Timer<SYST> {
    /// Initialize SysTick timer
    pub fn syst(mut tmr: SYST, clocks: &Clocks) -> Self {
        tmr.set_clock_source(SystClkSource::Core);
        Self {
            tmr,
            clk: clocks.hclk(),
        }
    }

    /// Initialize SysTick timer and set it frequency to `HCLK / 8`
    pub fn syst_external(mut tmr: SYST, clocks: &Clocks) -> Self {
        tmr.set_clock_source(SystClkSource::External);
        Self {
            tmr,
            clk: clocks.hclk() / 8,
        }
    }

    pub fn configure(&mut self, clocks: &Clocks) {
        self.tmr.set_clock_source(SystClkSource::Core);
        self.clk = clocks.hclk();
    }

    pub fn configure_external(&mut self, clocks: &Clocks) {
        self.tmr.set_clock_source(SystClkSource::External);
        self.clk = clocks.hclk() / 8;
    }

    pub fn release(self) -> SYST {
        self.tmr
    }

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
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum Ocm {
    Frozen = 0,
    ActiveOnMatch = 1,
    InactiveOnMatch = 2,
    Toggle = 3,
    ForceInactive = 4,
    ForceActive = 5,
    PwmMode1 = 6,
    PwmMode2 = 7,
}

mod sealed {
    use super::{Channel, Event, IdleState, Ocm, Polarity};
    pub trait General {
        type Width: Into<u32> + From<u16>;
        fn max_auto_reload() -> u32;
        unsafe fn set_auto_reload_unchecked(&mut self, pr: u32);
        fn set_auto_reload(&mut self, pr: u32) -> Result<(), super::Error>;
        fn read_auto_reload() -> u32;
        fn enable_preload(&mut self, b: bool);
        fn enable_counter(&mut self);
        fn disable_counter(&mut self);
        fn is_counter_enabled(&self) -> bool;
        fn reset_counter(&mut self);
        fn set_prescaler(&mut self, div: u16);
        fn read_prescaler(&self) -> u16;
        fn trigger_update(&mut self);
        fn clear_interrupt_flag(&mut self, event: Event);
        fn listen_interrupt(&mut self, event: Event, b: bool);
        fn get_interrupt_flag(&self) -> Event;
        fn read_count(&self) -> Self::Width;
        fn write_count(&mut self, value: Self::Width);
        fn start_one_pulse(&mut self);
        fn start_free(&mut self, update: bool);
        fn ctrl1_reset(&mut self);
        fn cnt_reset(&mut self);
    }

    pub trait WithPwmCommon: General {
        const CH_NUMBER: u8;
        const COMP_CH_NUMBER: u8;
        fn read_cc_value(channel: u8) -> u32;
        fn set_cc_value(channel: u8, value: u32);
        fn enable_channel(channel: u8, b: bool);
        fn set_channel_polarity(channel: u8, p: Polarity);
        fn set_nchannel_polarity(channel: u8, p: Polarity);
    }

    pub trait Advanced: WithPwmCommon {
        fn enable_nchannel(channel: u8, b: bool);
        fn set_dtc_value(value: u8);
        fn read_dtc_value() -> u8;
        fn idle_state(channel: u8, comp: bool, s: IdleState);
        fn enable_output(&mut self);
        fn disable_output(&mut self);
    }

    pub trait WithPwm: WithPwmCommon {
        fn preload_output_channel_in_mode(&mut self, channel: Channel, mode: Ocm);
        fn start_pwm(&mut self);
    }

    pub trait MasterTimer: General {
        type Ptos;
        fn master_mode(&mut self, mode: Self::Ptos);
    }
}
pub(crate) use sealed::{Advanced, General, MasterTimer, WithPwm, WithPwmCommon};

pub trait Instance:
    crate::Sealed + crm::Enable + crm::Reset + crm::BusTimerClock + General
{
}

macro_rules! tmr {
    ($TMR:ty: [
        $Timer:ident,
        $bits:ty,
        $(dmar: $memsize:ty,)?
        $(c: ($cnum:tt $(, $aoe:ident)?),)?
        $(m: $tmrbase:ident,)?
    ]) => {
        impl Instance for $TMR { }
        pub type $Timer = Timer<$TMR>;

        impl General for $TMR {
            type Width = $bits;

            #[inline(always)]
            fn max_auto_reload() -> u32 {
                <$bits>::MAX as u32
            }
            #[inline(always)]
            unsafe fn set_auto_reload_unchecked(&mut self, pr: u32) {
                self.pr.write(|w| w.bits(pr.try_into().unwrap()))
            }
            #[inline(always)]
            fn set_auto_reload(&mut self, pr: u32) -> Result<(), Error> {
                // Note: Make it impossible to set the ARR value to 0, since this
                // would cause an infinite loop.
                if pr > 0 && pr <= Self::max_auto_reload().try_into().unwrap() {
                    Ok(unsafe { self.set_auto_reload_unchecked(pr.try_into().unwrap()) })
                } else {
                    Err(Error::WrongAutoReload)
                }
            }
            #[inline(always)]
            fn read_auto_reload() -> u32 {
                let tmr = unsafe { &*<$TMR>::ptr() };
                tmr.pr.read().bits().into()
            }
            #[inline(always)]
            fn enable_preload(&mut self, b: bool) {
                self.ctrl1.modify(|_, w| w.prben().bit(b));
            }
            #[inline(always)]
            fn enable_counter(&mut self) {
                self.ctrl1.modify(|_, w| w.tmren().set_bit());
            }
            #[inline(always)]
            fn disable_counter(&mut self) {
                self.ctrl1.modify(|_, w| w.tmren().clear_bit());
            }
            #[inline(always)]
            fn is_counter_enabled(&self) -> bool {
                self.ctrl1.read().tmren().bit()
            }
            #[inline(always)]
            fn reset_counter(&mut self) {
                self.cval.reset();
            }
            #[inline(always)]
            #[allow(unused_unsafe)]
            fn set_prescaler(&mut self, div: u16) {
                self.div.write(|w| unsafe {w.bits(div)} );
            }
            #[inline(always)]
            fn read_prescaler(&self) -> u16 {
                self.div.read().bits()
            }
            #[inline(always)]
            fn trigger_update(&mut self) {
                self.ctrl1.modify(|_, w| w.ovfs().set_bit());
                self.swevt.write(|w| w.ovfswtr().set_bit());
                self.ctrl1.modify(|_, w| w.ovfs().clear_bit());
            }
            #[inline(always)]
            fn clear_interrupt_flag(&mut self, event: Event) {
                self.ists.write(|w| unsafe { w.bits(0xffff & !event.bits()) });
            }
            #[inline(always)]
            fn listen_interrupt(&mut self, event: Event, b: bool) {
                if b {
                    self.iden.modify(|r, w| unsafe { w.bits(r.bits() | event.bits()) });
                } else {
                    self.iden.modify(|r, w| unsafe { w.bits(r.bits() & !event.bits()) });
                }
            }
            #[inline(always)]
            fn get_interrupt_flag(&self) -> Event {
                Event::from_bits_truncate(self.ists.read().bits())
            }
            #[inline(always)]
            fn read_count(&self) -> Self::Width {
                self.cval.read().bits() as Self::Width
            }
            #[inline(always)]
            #[allow(unused_unsafe)]
            fn write_count(&mut self, value:Self::Width) {
                self.cval.write(|w|unsafe{w.bits(value)});
            }
            #[inline(always)]
            fn start_one_pulse(&mut self) {
                self.ctrl1.modify(|_, w| unsafe { w.bits(1 << 3) }.tmren().set_bit());
            }
            #[inline(always)]
            fn start_free(&mut self, update: bool) {
                self.ctrl1.modify(|_, w| w.tmren().set_bit().ocmen().bit(!update));
            }
            #[inline(always)]
            fn ctrl1_reset(&mut self) {
                self.ctrl1.reset();
            }
            #[inline(always)]
            fn cnt_reset(&mut self) {
                self.cval.reset();
            }
        }



        $(with_dmar!($TMR, $memsize);)?

        $(
            impl WithPwmCommon for $TMR {
                const CH_NUMBER: u8 = $cnum;
                const COMP_CH_NUMBER: u8 = $cnum;

                #[inline(always)]
                fn read_cc_value(c: u8) -> u32 {
                    let tmr = unsafe { &*<$TMR>::ptr() };
                    if c < Self::CH_NUMBER {
                        tmr.cdt[c as usize].read().bits().try_into().unwrap()
                    } else {
                        0
                    }
                }

                #[inline(always)]
                #[allow(unused_unsafe)]
                fn set_cc_value(c: u8, value: u32) {
                    let tmr = unsafe { &*<$TMR>::ptr() };
                    if c < Self::CH_NUMBER {
                        tmr.cdt[c as usize].write(|w| unsafe { w.bits(value.try_into().unwrap()) })
                    }
                }

                #[inline(always)]
                fn enable_channel(c: u8, b: bool) {
                    let tmr = unsafe { &*<$TMR>::ptr() };
                    if c < Self::CH_NUMBER {
                        unsafe { bb::write(&tmr.cctrl, c*4, b); }
                    }
                }

                #[inline(always)]
                fn set_channel_polarity(c: u8, p: Polarity) {
                    let tmr = unsafe { &*<$TMR>::ptr() };
                    if c < Self::CH_NUMBER {
                        unsafe { bb::write(&tmr.cctrl, c*4 + 1, p == Polarity::ActiveLow); }
                    }
                }

                #[inline(always)]
                fn set_nchannel_polarity(c: u8, p: Polarity) {
                    let tmr = unsafe { &*<$TMR>::ptr() };
                    if c < Self::COMP_CH_NUMBER {
                        unsafe { bb::write(&tmr.cctrl, c*4 + 3, p == Polarity::ActiveLow); }
                    }
                }
            }

            $(
                impl Advanced for $TMR {
                    fn enable_nchannel(c: u8, b: bool) {
                        let $aoe = ();
                        let tmr = unsafe { &*<$TMR>::ptr() };
                        if c < Self::COMP_CH_NUMBER {
                            unsafe { bb::write(&tmr.cctrl, c*4 + 2, b); }
                        }
                    }
                    fn set_dtc_value(value: u8) {
                        let tmr = unsafe { &*<$TMR>::ptr() };
                        tmr.brk.modify(|_,w| w.dtc().bits(value));
                    }
                    fn read_dtc_value() -> u8 {
                        let tmr = unsafe { &*<$TMR>::ptr() };
                        tmr.brk.read().dtc().bits()
                    }
                    fn idle_state(c: u8, comp: bool, s: IdleState) {
                        let tmr = unsafe { &*<$TMR>::ptr() };
                        if !comp {
                            if c < Self::CH_NUMBER {
                                unsafe { bb::write(&tmr.ctrl2, c*2 + 8, s == IdleState::Set); }
                            }
                        } else {
                            if c < Self::COMP_CH_NUMBER {
                                unsafe { bb::write(&tmr.ctrl2, c*2 + 9, s == IdleState::Set); }
                            }
                        }
                    }

                    #[inline(always)]
                    fn enable_output(&mut self){
                        let tmr = unsafe { &*<$TMR>::ptr() };
                        tmr.brk.modify(|_, w|  w.oen().set_bit());
                    }

                    #[inline(always)]
                    fn disable_output(&mut self){
                        let tmr = unsafe { &*<$TMR>::ptr() };
                        tmr.brk.modify(|_, w|  w.oen().clear_bit());
                    }
                }
            )?

            with_pwm!($TMR: $cnum $(, $aoe)?);
        )?

        $(impl MasterTimer for $TMR {
            type Ptos = pac::$tmrbase::ctrl2::PTOS_A;
            fn master_mode(&mut self, mode: Self::Ptos) {
                self.ctrl2.modify(|_,w| w.ptos().variant(mode));
            }
        })?

    };
}
use tmr;

// macro_rules! with_dmar {
//     ($TMR:ty, $memsize:ty) => {
//         unsafe impl PeriAddress for DMAR<$TMR> {
//             #[inline(always)]
//             fn address(&self) -> u32 {
//                 &self.0.dmar as *const _ as u32
//             }

//             type MemSize = $memsize;
//         }
//     };
// }

macro_rules! with_pwm {
    ($TMR:ty: [$($Cx:ident, $ccmrx_output:ident, $cxoben:ident, $cxoctrl:ident;)+] $(, $aoe:ident)?) => {
        impl WithPwm for $TMR {
            #[inline(always)]
            #[allow(unused_unsafe)]
            fn preload_output_channel_in_mode(&mut self, channel: Channel, mode: Ocm) {
                match channel {
                    $(
                        Channel::$Cx => {
                            self.$ccmrx_output()
                            .modify(|_, w| unsafe {w.$cxoben().set_bit().$cxoctrl().bits(mode as _) });
                        }
                    )+
                    #[allow(unreachable_patterns)]
                    _ => {},
                }
            }

            #[inline(always)]
            fn start_pwm(&mut self) {
                $(let $aoe = self.brk.modify(|_, w| w.aoen().set_bit());)?
                self.ctrl1.modify(|_, w| w.tmren().set_bit());
            }
        }
    };
    ($TMR:ty: 1) => {
        with_pwm!($TMR: [
            C1, cm1_output, c1oben, c1octrl;
        ]);
    };
    ($TMR:ty: 2) => {
        with_pwm!($TMR: [
            C1, cm1_output, c1oben, c1octrl;
            C2, cm1_output, c2oben, c2octrl;
        ]);
    };
    ($TMR:ty: 4 $(, $aoe:ident)?) => {
        with_pwm!($TMR: [
            C1, cm1_output, c1oben, c1octrl;
            C2, cm1_output, c2oben, c2octrl;
            C3, cm2_output, c3oben, c3octrl;
            C4, cm2_output, c4oben, c4octrl;
        ] $(, $aoe)?);
    };
}

impl<TMR: Instance> Timer<TMR> {
    /// Initialize timer
    pub fn new(tmr: TMR, clocks: &Clocks) -> Self {
        unsafe {
            // Enable and reset the timer peripheral
            TMR::enable_unchecked();
            TMR::reset_unchecked();
        }

        Self {
            clk: TMR::timer_clock(clocks),
            tmr,
        }
    }

    pub fn configure(&mut self, clocks: &Clocks) {
        self.clk = TMR::timer_clock(clocks);
    }

    pub fn counter_hz(self) -> CounterHz<TMR> {
        CounterHz(self)
    }

    pub fn release(self) -> TMR {
        self.tmr
    }

    /// Starts listening for an `event`
    ///
    /// Note, you will also have to enable the TMR2 interrupt in the NVIC to start
    /// receiving events.
    pub fn listen(&mut self, event: Event) {
        self.tmr.listen_interrupt(event, true);
    }

    /// Clears interrupt associated with `event`.
    ///
    /// If the interrupt is not cleared, it will immediately retrigger after
    /// the ISR has finished.
    pub fn clear_interrupt(&mut self, event: Event) {
        self.tmr.clear_interrupt_flag(event);
    }

    /// Stops listening for an `event`
    pub fn unlisten(&mut self, event: Event) {
        self.tmr.listen_interrupt(event, false);
    }
}

impl<TMR: Instance + MasterTimer> Timer<TMR> {
    pub fn set_master_mode(&mut self, mode: TMR::Ptos) {
        self.tmr.master_mode(mode)
    }
}

/// Timer wrapper for fixed precision timers.
///
/// Uses `fugit::TimerDurationU32` for most of operations
pub struct FTimer<TMR, const FREQ: u32> {
    tmr: TMR,
}

/// `FTimer` with precision of 1 μs (1 MHz sampling)
pub type FTimerUs<TMR> = FTimer<TMR, 1_000_000>;

/// `FTimer` with precision of 1 ms (1 kHz sampling)
///
/// NOTE: don't use this if your system frequency more than 65 MHz
pub type FTimerMs<TMR> = FTimer<TMR, 1_000>;

impl<TMR: Instance, const FREQ: u32> FTimer<TMR, FREQ> {
    /// Initialize timer
    pub fn new(tmr: TMR, clocks: &Clocks) -> Self {
        unsafe {
            // Enable and reset the timer peripheral
            TMR::enable_unchecked();
            TMR::reset_unchecked();
        }

        let mut t = Self { tmr };
        t.configure(clocks);
        t
    }

    /// Calculate prescaler depending on `Clocks` state
    pub fn configure(&mut self, clocks: &Clocks) {
        let clk = TMR::timer_clock(clocks);
        assert!(clk.raw() % FREQ == 0);
        let psc = clk.raw() / FREQ;
        self.tmr.set_prescaler(u16::try_from(psc - 1).unwrap());
    }

    /// Creates `Counter` that implements [embedded_hal::timer::CountDown]
    pub fn counter(self) -> Counter<TMR, FREQ> {
        Counter(self)
    }

    /// Creates `Delay` that implements [embedded_hal::blocking::delay] traits
    pub fn delay(self) -> Delay<TMR, FREQ> {
        Delay(self)
    }

    /// Releases the TMR peripheral
    pub fn release(self) -> TMR {
        self.tmr
    }

    /// Starts listening for an `event`
    ///
    /// Note, you will also have to enable the TMR2 interrupt in the NVIC to start
    /// receiving events.
    pub fn listen(&mut self, event: Event) {
        self.tmr.listen_interrupt(event, true);
    }

    /// Clears interrupt associated with `event`.
    ///
    /// If the interrupt is not cleared, it will immediately retrigger after
    /// the ISR has finished.
    pub fn clear_interrupt(&mut self, event: Event) {
        self.tmr.clear_interrupt_flag(event);
    }

    pub fn get_interrupt(&self) -> Event {
        self.tmr.get_interrupt_flag()
    }

    /// Stops listening for an `event`
    pub fn unlisten(&mut self, event: Event) {
        self.tmr.listen_interrupt(event, false);
    }
}

impl<TMR: Instance + MasterTimer, const FREQ: u32> FTimer<TMR, FREQ> {
    pub fn set_master_mode(&mut self, mode: TMR::Ptos) {
        self.tmr.master_mode(mode)
    }
}

#[inline(always)]
pub(crate) const fn compute_arr_presc(freq: u32, clock: u32) -> (u16, u32) {
    let ticks = clock / freq;
    let psc = (ticks - 1) / (1 << 16);
    let arr = ticks / (psc + 1) - 1;
    (psc as u16, arr)
}

#[cfg(feature = "tmr1")]
tmr!(pac::TMR1: [Timer1, u16, c: (4, _aoe), m: tmr1,]);

#[cfg(feature = "tmr2")]
tmr!(pac::TMR2: [Timer2, u32, c: (4), m: tmr2,]);

#[cfg(feature = "tmr3")]
tmr!(pac::TMR3: [Timer3, u16, c: (4), m: tmr3,]);

#[cfg(feature = "tmr4")]
tmr!(pac::TMR4: [Timer4, u16, c: (4), m: tmr4,]);

#[cfg(feature = "tmr5")]
tmr!(pac::TMR5: [Timer5, u32, c: (4), m: tmr5,]);

#[cfg(feature = "tmr6")]
tmr!(pac::TMR6: [Timer6, u16, c: (4), m: tmr6,]);

#[cfg(feature = "tmr7")]
tmr!(pac::TMR7: [Timer7, u16, c: (4), m: tmr7,]);

#[cfg(feature = "tmr8")]
tmr!(pac::TMR8: [Timer8, u16, c: (4), m: tmr8,]);

#[cfg(feature = "tmr9")]
tmr!(pac::TMR9: [Timer9, u16, c: (2),]);

// #[cfg(feature = "tmr10")]
// tmr!(pac::TMR10: [Timer10, u16, c: (4), m: tmr10,]);

// #[cfg(feature = "tmr11")]
// tmr!(pac::TMR11: [Timer11, u16, c: (4), m: tmr11,]);

#[cfg(feature = "tmr12")]
tmr!(pac::TMR12: [Timer12, u16, c: (4),]);

#[cfg(feature = "tmr13")]
tmr!(pac::TMR13: [Timer13, u16, c: (4), m:]);

#[cfg(feature = "tmr14")]
tmr!(pac::TMR14: [Timer14, u16, c: (4),]);

#[cfg(feature = "tmr15")]
tmr!(pac::TMR15: [Timer15, u16, c: (4), m: tmr15,]);

#[cfg(feature = "tmr16")]
tmr!(pac::TMR16: [Timer16, u16, c: (4), m: tmr16,]);

#[cfg(feature = "tmr17")]
tmr!(pac::TMR17: [Timer17, u16, c: (4), m: tmr17,]);

#[cfg(feature = "tmr20")]
tmr!(pac::TMR20: [Timer20, u16, c: (4), m: tmr20,]);
