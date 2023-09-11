//! Provides basic Pulse-width modulation (PWM) capabilities
//!
//! There are 2 main structures [`Pwm`] and [`PwmHz`]. Both structures implement [`embedded_hal::Pwm`] and have some additional API.
//!
//! First one is based on [`FTimer`] with fixed prescaler
//! and easy to use with [`fugit::TimerDurationU32`] for setting pulse width and period without advanced calculations.
//!
//! Second one is based on [`Timer`] with dynamic internally calculated prescaler and require [`fugit::Hertz`] to set period.
//!
//! You can [`split`](Pwm::split) any of those structures on independent `PwmChannel`s if you need that implement [`embedded_hal::PwmPin`]
//! but can't change PWM period.
//!
//! Also there is [`PwmExt`] trait implemented on `pac::TMRx` to simplify creating new structure.
//!
//! You need to pass one or tuple of channels with pins you plan to use and initial `time`/`frequency` corresponding PWM period.
//! Pins can be collected with [`ChannelBuilder`]s in sequence corresponding to the channel number. Smaller channel number first.
//! Each channel group can contain 1 or several main pins and 0, 1 or several complementary pins.
//! Start constructing channel with `new(first_main_pin)`.
//! Then use `.with(other_main_pin)` and `.with_complementary(other_complementary_pin)` accordingly
//! to add advanced pins on same channel.
//!
//! For example:
//! ```rust
//! let channels = (
//!     Channel1::new(gpioa.pa8),
//!     Channel2::new(gpioa.pa9), // use Channel2OD` for `OpenDrain` pin
//! );
//! ```
//! or
//! ```rust,ignore
//! let channels = Channel1::new(gpioa.pa8).with_complementary(gpioa.pa7); // (CH1, CHN1)
//! ```
//!
//! where `CHx` and `CHx_n` are main pins of PWM channel `x` and `CHNx` are complementary pins of PWM channel `x`.
//!
//! After creating structures you can dynamically enable main or complementary channels with `enable` and `enable_complementary`
//! and change their polarity with `set_polarity` and `set_complementary_polarity`.

use super::{compute_arr_presc, Advanced, CPin, Channel, FTimer, IdleState, Instance, Ocm, Polarity, Timer, WithPwm,};
pub use super::{Ch, C1, C2, C3, C4};
use crate::gpio::{OpenDrain, PushPull};
use crate::crm::Clocks;
use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};
use fugit::{HertzU32 as Hertz, TimerDurationU32};

pub type Channel1<TMR, const COMP: bool = false> = ChannelBuilder<TMR, C1, COMP, PushPull>;
pub type Channel2<TMR, const COMP: bool = false> = ChannelBuilder<TMR, C2, COMP, PushPull>;
pub type Channel3<TMR, const COMP: bool = false> = ChannelBuilder<TMR, C3, COMP, PushPull>;
pub type Channel4<TMR, const COMP: bool = false> = ChannelBuilder<TMR, C4, COMP, PushPull>;

pub struct ChannelBuilder<TMR, const C: u8, const COMP: bool = false, Otype = PushPull> {
    pub(super) _tmr: PhantomData<(TMR, Otype)>,
}

impl<TMR, Otype, const C: u8> ChannelBuilder<TMR, C, false, Otype>
where
    TMR: CPin<C>,
{
    pub fn new(pin: impl Into<TMR::Ch<Otype>>) -> Self {
        let _pin = pin.into();
        Self { _tmr: PhantomData }
    }
}
impl<TMR, Otype, const C: u8, const COMP: bool> ChannelBuilder<TMR, C, COMP, Otype>
where
    TMR: CPin<C>,
{
    pub fn with(self, pin: impl Into<TMR::Ch<Otype>>) -> Self {
        let _pin = pin.into();
        self
    }
}

impl<TMR, Otype, const C: u8, const COMP: bool> sealed::Split
    for ChannelBuilder<TMR, C, COMP, Otype>
{
    type Channels = PwmChannel<TMR, C, COMP>;
    fn split() -> Self::Channels {
        PwmChannel::new()
    }
}

mod sealed {
    pub trait Split {
        type Channels;
        fn split() -> Self::Channels;
    }
    macro_rules! split {
        ($($T:ident),+) => {
            impl<$($T),+> Split for ($($T),+)
            where
                $($T: Split,)+
            {
                type Channels = ($($T::Channels),+);
                fn split() -> Self::Channels {
                    ($($T::split()),+)
                }
            }
        };
    }
    split!(T1, T2);
    split!(T1, T2, T3);
    split!(T1, T2, T3, T4);
}
pub trait Pins<TMR>: sealed::Split {
    const C1: bool = false;
    const C2: bool = false;
    const C3: bool = false;
    const C4: bool = false;
    const NC1: bool = false;
    const NC2: bool = false;
    const NC3: bool = false;
    const NC4: bool = false;

    fn check_used(c: Channel) -> Channel {
        if (c == Channel::C1 && Self::C1)
            || (c == Channel::C2 && Self::C2)
            || (c == Channel::C3 && Self::C3)
            || (c == Channel::C4 && Self::C4)
        {
            c
        } else {
            panic!("Unused channel")
        }
    }

    fn check_complementary_used(c: Channel) -> Channel {
        if (c == Channel::C1 && Self::NC1)
            || (c == Channel::C2 && Self::NC2)
            || (c == Channel::C3 && Self::NC3)
            || (c == Channel::C4 && Self::NC4)
        {
            c
        } else {
            panic!("Unused channel")
        }
    }
}

macro_rules! pins_impl {
    ( $( $(($Otype:ident, $ENCHX:ident, $COMP:ident)),+; )+ ) => {
        $(
            #[allow(unused_parens)]
            impl<TMR, $($Otype, const $COMP: bool,)+> Pins<TMR> for ($(ChannelBuilder<TMR, $ENCHX, $COMP, $Otype>),+) {
                $(
                    const $ENCHX: bool = true;
                    const $COMP: bool = $COMP;
                )+
            }
        )+
    };
}

pins_impl!(
    (O1, C1, NC1), (O2, C2, NC2), (O3, C3, NC3), (O4, C4, NC4);

                   (O2, C2, NC2), (O3, C3, NC3), (O4, C4, NC4);
    (O1, C1, NC1),                (O3, C3, NC3), (O4, C4, NC4);
    (O1, C1, NC1), (O2, C2, NC2),                (O4, C4, NC4);
    (O1, C1, NC1), (O2, C2, NC2), (O3, C3, NC3);

                                  (O3, C3, NC3), (O4, C4, NC4);
                   (O2, C2, NC2),                (O4, C4, NC4);
                   (O2, C2, NC2), (O3, C3, NC3);
    (O1, C1, NC1),                               (O4, C4, NC4);
    (O1, C1, NC1),                (O3, C3, NC3);
    (O1, C1, NC1), (O2, C2, NC2);

    (O1, C1, NC1);
                   (O2, C2, NC2);
                                  (O3, C3, NC3);
                                                 (O4, C4, NC4);
);

pub struct PwmChannel<TMR, const C: u8, const COMP: bool = false> {
    pub(super) _tmr: PhantomData<TMR>,
}

pub trait PwmExt
where
    Self: Sized + Instance + WithPwm,
{
    fn pwm<PINS, const FREQ: u32>(
        self,
        pins: PINS,
        time: TimerDurationU32<FREQ>,
        clocks: &Clocks,
    ) -> Pwm<Self, PINS, FREQ>
    where
        PINS: Pins<Self>;

    fn pwm_hz<PINS>(self, pins: PINS, freq: Hertz, clocks: &Clocks) -> PwmHz<Self, PINS>
    where
        PINS: Pins<Self>;

    fn pwm_us<PINS>(
        self,
        pins: PINS,
        time: TimerDurationU32<1_000_000>,
        clocks: &Clocks,
    ) -> Pwm<Self, PINS, 1_000_000>
    where
        PINS: Pins<Self>,
    {
        self.pwm::<_, 1_000_000>(pins, time, clocks)
    }
}

impl<TMR> PwmExt for TMR
where
    Self: Sized + Instance + WithPwm,
{
    fn pwm<PINS, const FREQ: u32>(
        self,
        pins: PINS,
        time: TimerDurationU32<FREQ>,
        clocks: &Clocks,
    ) -> Pwm<TMR, PINS, FREQ>
    where
        PINS: Pins<Self>,
    {
        FTimer::<Self, FREQ>::new(self, clocks).pwm(pins, time)
    }

    fn pwm_hz<PINS>(self, pins: PINS, time: Hertz, clocks: &Clocks) -> PwmHz<TMR, PINS>
    where
        PINS: Pins<Self>,
    {
        Timer::new(self, clocks).pwm_hz(pins, time)
    }
}

impl<TMR, const C: u8, const COMP: bool> PwmChannel<TMR, C, COMP> {
    pub(crate) fn new() -> Self {
        Self {
            _tmr: core::marker::PhantomData,
        }
    }
}

impl<TMR: Instance + WithPwm, const C: u8, const COMP: bool> PwmChannel<TMR, C, COMP> {
    /// Disable PWM channel
    #[inline]
    pub fn disable(&mut self) {
        TMR::enable_channel(C, false);
    }

    /// Enable PWM channel
    #[inline]
    pub fn enable(&mut self) {
        TMR::enable_channel(C, true);
    }

    /// Set PWM channel polarity
    #[inline]
    pub fn set_polarity(&mut self, p: Polarity) {
        TMR::set_channel_polarity(C, p);
    }

    /// Get PWM channel duty cycle
    #[inline]
    pub fn get_duty(&self) -> u16 {
        TMR::read_cc_value(C) as u16
    }

    /// Get the maximum duty cycle value of the PWM channel
    ///
    /// If `0` returned means max_duty is 2^16
    #[inline]
    pub fn get_max_duty(&self) -> u16 {
        (TMR::read_auto_reload() as u16).wrapping_add(1)
    }

    /// Set PWM channel duty cycle
    #[inline]
    pub fn set_duty(&mut self, duty: u16) {
        TMR::set_cc_value(C, duty as u32)
    }

    /// Set complementary PWM channel polarity
    #[inline]
    pub fn set_complementary_polarity(&mut self, p: Polarity) {
        TMR::set_nchannel_polarity(C, p);
    }
}

impl<TMR: Instance + WithPwm + Advanced, const C: u8> PwmChannel<TMR, C, true> {
    /// Disable complementary PWM channel
    #[inline]
    pub fn disable_complementary(&mut self) {
        TMR::enable_nchannel(C, false);
    }

    /// Enable complementary PWM channel
    #[inline]
    pub fn enable_complementary(&mut self) {
        TMR::enable_nchannel(C, true);
    }

    /// Set PWM channel idle state
    #[inline]
    pub fn set_idle_state(&mut self, s: IdleState) {
        TMR::idle_state(C, false, s);
    }

    /// Set complementary PWM channel idle state
    #[inline]
    pub fn set_complementary_idle_state(&mut self, s: IdleState) {
        TMR::idle_state(C, true, s);
    }
}

pub struct PwmHz<TMR, PINS>
where
    TMR: Instance + WithPwm,
    PINS: Pins<TMR>,
{
    timer: Timer<TMR>,
    _pins: PhantomData<PINS>,
}

impl<TMR, PINS> PwmHz<TMR, PINS>
where
    TMR: Instance + WithPwm,
    PINS: Pins<TMR>,
{
    pub fn release(mut self) -> Timer<TMR> {
        // stop timer
        self.tmr.ctrl1_reset();
        self.timer
    }

    pub fn split(self) -> PINS::Channels {
        PINS::split()
    }
}

impl<TMR, PINS> Deref for PwmHz<TMR, PINS>
where
    TMR: Instance + WithPwm,
    PINS: Pins<TMR>,
{
    type Target = Timer<TMR>;
    fn deref(&self) -> &Self::Target {
        &self.timer
    }
}

impl<TMR, PINS> DerefMut for PwmHz<TMR, PINS>
where
    TMR: Instance + WithPwm,
    PINS: Pins<TMR>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.timer
    }
}

impl<TMR: Instance + WithPwm> Timer<TMR> {
    pub fn pwm_hz<PINS>(mut self, _pins: PINS, freq: Hertz) -> PwmHz<TMR, PINS>
    where
        PINS: Pins<TMR>,
    {
        if PINS::C1 {
            self.tmr
                .preload_output_channel_in_mode(Channel::C1, Ocm::PwmMode1);
        }
        if PINS::C2 && TMR::CH_NUMBER > 1 {
            self.tmr
                .preload_output_channel_in_mode(Channel::C2, Ocm::PwmMode1);
        }
        if PINS::C3 && TMR::CH_NUMBER > 2 {
            self.tmr
                .preload_output_channel_in_mode(Channel::C3, Ocm::PwmMode1);
        }
        if PINS::C4 && TMR::CH_NUMBER > 3 {
            self.tmr
                .preload_output_channel_in_mode(Channel::C4, Ocm::PwmMode1);
        }

        // The reference manual is a bit ambiguous about when enabling this bit is really
        // necessary, but since we MUST enable the preload for the output channels then we
        // might as well enable for the auto-reload too
        self.tmr.enable_preload(true);

        let (psc, arr) = compute_arr_presc(freq.raw(), self.clk.raw());
        self.tmr.set_prescaler(psc);
        self.tmr.set_auto_reload(arr).unwrap();

        // Trigger update event to load the registers
        self.tmr.trigger_update();

        self.tmr.start_pwm();

        PwmHz {
            timer: self,
            _pins: PhantomData,
        }
    }
}

impl<TMR, PINS> PwmHz<TMR, PINS>
where
    TMR: Instance + WithPwm,
    PINS: Pins<TMR>,
{
    /// Enable PWM output of the timer on channel `channel`
    #[inline]
    pub fn enable(&mut self, channel: Channel) {
        TMR::enable_channel(PINS::check_used(channel) as u8, true)
    }

    /// Disable PWM output of the timer on channel `channel`
    #[inline]
    pub fn disable(&mut self, channel: Channel) {
        TMR::enable_channel(PINS::check_used(channel) as u8, false)
    }

    /// Set the polarity of the active state for the primary PWM output of the timer on channel `channel`
    #[inline]
    pub fn set_polarity(&mut self, channel: Channel, p: Polarity) {
        TMR::set_channel_polarity(PINS::check_used(channel) as u8, p);
    }

    /// Get the current duty cycle of the timer on channel `channel`
    #[inline]
    pub fn get_duty(&self, channel: Channel) -> u16 {
        TMR::read_cc_value(PINS::check_used(channel) as u8) as u16
    }

    /// Set the duty cycle of the timer on channel `channel`
    #[inline]
    pub fn set_duty(&mut self, channel: Channel, duty: u16) {
        TMR::set_cc_value(PINS::check_used(channel) as u8, duty as u32)
    }

    /// Get the maximum duty cycle value of the timer
    ///
    /// If `0` returned means max_duty is 2^16
    pub fn get_max_duty(&self) -> u16 {
        (TMR::read_auto_reload() as u16).wrapping_add(1)
    }

    /// Get the PWM frequency of the timer in Hertz
    pub fn get_period(&self) -> Hertz {
        let clk = self.clk;
        let psc = self.tmr.read_prescaler() as u32;
        let arr = TMR::read_auto_reload();

        // Length in ms of an internal clock pulse
        clk / ((psc + 1) * (arr + 1))
    }

    /// Set the PWM frequency for the timer in Hertz
    pub fn set_period(&mut self, period: Hertz) {
        let clk = self.clk;

        let (psc, arr) = compute_arr_presc(period.raw(), clk.raw());
        self.tmr.set_prescaler(psc);
        self.tmr.set_auto_reload(arr).unwrap();
        self.tmr.cnt_reset();
    }

    /// Set the polarity of the active state for the complementary PWM output of the advanced timer on channel `channel`
    #[inline]
    pub fn set_complementary_polarity(&mut self, channel: Channel, p: Polarity) {
        TMR::set_channel_polarity(PINS::check_complementary_used(channel) as u8, p);
    }
}

impl<TMR, PINS> PwmHz<TMR, PINS>
where
    TMR: Instance + WithPwm + Advanced,
    PINS: Pins<TMR>,
{
    /// Enable complementary PWM output of the timer on channel `channel`
    #[inline]
    pub fn enable_complementary(&mut self, channel: Channel) {
        TMR::enable_nchannel(PINS::check_complementary_used(channel) as u8, true)
    }

    /// Disable complementary PWM output of the timer on channel `channel`
    #[inline]
    pub fn disable_complementary(&mut self, channel: Channel) {
        TMR::enable_nchannel(PINS::check_complementary_used(channel) as u8, false)
    }

    /// Set number DTS ticks during that the primary and complementary PWM pins are simultaneously forced to their inactive states
    /// ( see [`Polarity`] setting ) when changing PWM state. This duration when both channels are in an 'off' state  is called 'dead time'.
    ///
    /// This is necessary in applications like motor control or power converters to prevent the destruction of the switching elements by
    /// short circuit in the moment of switching.
    #[inline]
    pub fn set_dead_time(&mut self, dts_ticks: u16) {
        let bits = pack_ceil_dead_time(dts_ticks);
        TMR::set_dtg_value(bits);
    }

    /// Set raw dead time (DTG) bits
    ///
    /// The dead time generation is nonlinear and constrained by the DTS tick duration. DTG register configuration and calculation of
    /// the actual resulting dead time is described in the application note RM0368 from ST Microelectronics
    #[inline]
    pub fn set_dead_time_bits(&mut self, bits: u8) {
        TMR::set_dtg_value(bits);
    }

    /// Return dead time for complementary pins in the unit of DTS ticks
    #[inline]
    pub fn get_dead_time(&self) -> u16 {
        unpack_dead_time(TMR::read_dtg_value())
    }

    /// Get raw dead time (DTG) bits
    #[inline]
    pub fn get_dead_time_bits(&self) -> u8 {
        TMR::read_dtg_value()
    }

    /// Set the pin idle state
    #[inline]
    pub fn set_idle_state(&mut self, channel: Channel, s: IdleState) {
        TMR::idle_state(PINS::check_used(channel) as u8, false, s);
    }

    /// Set the complementary pin idle state
    #[inline]
    pub fn set_complementary_idle_state(&mut self, channel: Channel, s: IdleState) {
        TMR::idle_state(PINS::check_complementary_used(channel) as u8, true, s);
    }
}

pub struct Pwm<TMR, PINS, const FREQ: u32>
where
    TMR: Instance + WithPwm,
    PINS: Pins<TMR>,
{
    timer: FTimer<TMR, FREQ>,
    _pins: PhantomData<PINS>,
}

impl<TMR, PINS, const FREQ: u32> Pwm<TMR, PINS, FREQ>
where
    TMR: Instance + WithPwm,
    PINS: Pins<TMR>,
{
    pub fn split(self) -> PINS::Channels {
        PINS::split()
    }

    pub fn release(mut self) -> FTimer<TMR, FREQ> {
        // stop counter
        self.tmr.ctrl1_reset();
        self.timer
    }
}

impl<TMR, PINS, const FREQ: u32> Deref for Pwm<TMR, PINS, FREQ>
where
    TMR: Instance + WithPwm,
    PINS: Pins<TMR>,
{
    type Target = FTimer<TMR, FREQ>;
    fn deref(&self) -> &Self::Target {
        &self.timer
    }
}

impl<TMR, PINS, const FREQ: u32> DerefMut for Pwm<TMR, PINS, FREQ>
where
    TMR: Instance + WithPwm,
    PINS: Pins<TMR>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.timer
    }
}

impl<TMR: Instance + WithPwm, const FREQ: u32> FTimer<TMR, FREQ> {
    pub fn pwm<PINS>(mut self, _pins: PINS, time: TimerDurationU32<FREQ>) -> Pwm<TMR, PINS, FREQ>
    where
        PINS: Pins<TMR>,
    {
        if PINS::C1 {
            self.tmr
                .preload_output_channel_in_mode(Channel::C1, Ocm::PwmMode1);
        }
        if PINS::C2 && TMR::CH_NUMBER > 1 {
            self.tmr
                .preload_output_channel_in_mode(Channel::C2, Ocm::PwmMode1);
        }
        if PINS::C3 && TMR::CH_NUMBER > 2 {
            self.tmr
                .preload_output_channel_in_mode(Channel::C3, Ocm::PwmMode1);
        }
        if PINS::C4 && TMR::CH_NUMBER > 3 {
            self.tmr
                .preload_output_channel_in_mode(Channel::C4, Ocm::PwmMode1);
        }

        // The reference manual is a bit ambiguous about when enabling this bit is really
        // necessary, but since we MUST enable the preload for the output channels then we
        // might as well enable for the auto-reload too
        self.tmr.enable_preload(true);

        self.tmr.set_auto_reload(time.ticks() - 1).unwrap();

        // Trigger update event to load the registers
        self.tmr.trigger_update();

        self.tmr.start_pwm();

        Pwm {
            timer: self,
            _pins: PhantomData,
        }
    }
}

impl<TMR, PINS, const FREQ: u32> Pwm<TMR, PINS, FREQ>
where
    TMR: Instance + WithPwm,
    PINS: Pins<TMR>,
{
    /// Enable PWM output of the timer on channel `channel`
    #[inline]
    pub fn enable(&mut self, channel: Channel) {
        TMR::enable_channel(PINS::check_used(channel) as u8, true)
    }

    /// Disable PWM output of the timer on channel `channel`
    #[inline]
    pub fn disable(&mut self, channel: Channel) {
        TMR::enable_channel(PINS::check_used(channel) as u8, false)
    }

    /// Set the polarity of the active state for the primary PWM output of the timer on channel `channel`
    #[inline]
    pub fn set_polarity(&mut self, channel: Channel, p: Polarity) {
        TMR::set_channel_polarity(PINS::check_used(channel) as u8, p);
    }

    /// Get the current duty cycle of the timer on channel `channel`
    #[inline]
    pub fn get_duty(&self, channel: Channel) -> u16 {
        TMR::read_cc_value(PINS::check_used(channel) as u8) as u16
    }
    /// Get the current duty cycle of the timer on channel `channel` and convert to a duration
    #[inline]
    pub fn get_duty_time(&self, channel: Channel) -> TimerDurationU32<FREQ> {
        TimerDurationU32::from_ticks(TMR::read_cc_value(PINS::check_used(channel) as u8))
    }

    /// Set the duty cycle of the timer on channel `channel`
    #[inline]
    pub fn set_duty(&mut self, channel: Channel, duty: u16) {
        TMR::set_cc_value(PINS::check_used(channel) as u8, duty.into())
    }

    /// Set the duty cycle of the timer on channel `channel` from a duration
    #[inline]
    pub fn set_duty_time(&mut self, channel: Channel, duty: TimerDurationU32<FREQ>) {
        TMR::set_cc_value(PINS::check_used(channel) as u8, duty.ticks())
    }

    /// Get the maximum duty cycle value of the timer
    ///
    /// If `0` returned means max_duty is 2^16
    pub fn get_max_duty(&self) -> u16 {
        (TMR::read_auto_reload() as u16).wrapping_add(1)
    }

    /// Get the PWM frequency of the timer as a duration
    pub fn get_period(&self) -> TimerDurationU32<FREQ> {
        TimerDurationU32::from_ticks(TMR::read_auto_reload() + 1)
    }

    /// Set the PWM frequency for the timer from a duration
    pub fn set_period(&mut self, period: TimerDurationU32<FREQ>) {
        self.tmr.set_auto_reload(period.ticks() - 1).unwrap();
        self.tmr.cnt_reset();
    }

    /// Set the polarity of the active state for the complementary PWM output of the advanced timer on channel `channel`
    #[inline]
    pub fn set_complementary_polarity(&mut self, channel: Channel, p: Polarity) {
        TMR::set_channel_polarity(PINS::check_complementary_used(channel) as u8, p);
    }
}

/// Convert number dead time ticks to raw DTG register bits.
/// Values greater than 1009 result in maximum dead time of 126 us
const fn pack_ceil_dead_time(dts_ticks: u16) -> u8 {
    match dts_ticks {
        0..=127 => dts_ticks as u8,
        128..=254 => ((((dts_ticks + 1) >> 1) - 64) as u8) | 0b_1000_0000,
        255..=504 => ((((dts_ticks + 7) >> 3) - 32) as u8) | 0b_1100_0000,
        505..=1008 => ((((dts_ticks + 15) >> 4) - 32) as u8) | 0b_1110_0000,
        1009.. => 0xff,
    }
}

/// Convert raw DTG register bits value to number of dead time ticks
const fn unpack_dead_time(bits: u8) -> u16 {
    if bits & 0b_1000_0000 == 0 {
        bits as u16
    } else if bits & 0b_0100_0000 == 0 {
        (((bits & !0b_1000_0000) as u16) + 64) * 2
    } else if bits & 0b_0010_0000 == 0 {
        (((bits & !0b_1100_0000) as u16) + 32) * 8
    } else {
        (((bits & !0b_1110_0000) as u16) + 32) * 16
    }
}
