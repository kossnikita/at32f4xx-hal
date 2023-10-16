//! General Purpose Input / Output
//!
//! The GPIO pins are organised into groups of 16 pins which can be accessed through the
//! `gpioa`, `gpiob`... modules. To get access to the pins, you first need to convert them into a
//! HAL designed struct from the `pac` struct using the [split](trait.GpioExt.html#tymethod.split) function.
//! ```rust
//! // Acquire the GPIOC peripheral
//! // NOTE: `dp` is the device peripherals from the `PAC` crate
//! let mut gpioa = dp.GPIOA.split();
//! ```
//!
//! This gives you a struct containing all the pins `px0..px15`.
//! By default pins are in floating input mode. You can change their modes.
//! For example, to set `pa5` high, you would call
//!
//! ```rust
//! let output = gpioa.pa5.into_push_pull_output();
//! output.set_high();
//! ```
//!
//! ## Modes
//!
//! Each GPIO pin can be set to various modes:
//!
//! - **Alternate**: Pin mode required when the pin is driven by other peripherals
//! - **Analog**: Analog input to be used with ADC.
//! - **Dynamic**: Pin mode is selected at runtime. See changing configurations for more details
//! - Input
//!     - **PullUp**: Input connected to high with a weak pull up resistor. Will be high when nothing
//!     is connected
//!     - **PullDown**: Input connected to high with a weak pull up resistor. Will be low when nothing
//!     is connected
//!     - **Floating**: Input not pulled to high or low. Will be undefined when nothing is connected
//! - Output
//!     - **PushPull**: Output which either drives the pin high or low
//!     - **OpenDrain**: Output which leaves the gate floating, or pulls it do ground in drain
//!     mode. Can be used as an input in the `open` configuration
//!
//! ## Changing modes
//! The simplest way to change the pin mode is to use the `into_<mode>` functions. These return a
//! new struct with the correct mode that you can use the input or output functions on.
//!
//! If you need a more temporary mode change, and can not use the `into_<mode>` functions for
//! ownership reasons, you can use the closure based `with_<mode>` functions to temporarily change the pin type, do
//! some output or input, and then have it change back once done.
//!
//! ### Dynamic Mode Change
//! The above mode change methods guarantee that you can only call input functions when the pin is
//! in input mode, and output when in output modes, but can lead to some issues. Therefore, there
//! is also a mode where the state is kept track of at runtime, allowing you to change the mode
//! often, and without problems with ownership, or references, at the cost of some performance and
//! the risk of runtime errors.
//!
//! To make a pin dynamic, use the `into_dynamic` function, and then use the `make_<mode>` functions to
//! change the mode

use core::convert::Infallible;
use core::marker::PhantomData;

pub mod alt;
mod convert;
pub use convert::PinMode;
mod partially_erased;
pub use partially_erased::{PEPin, PartiallyErasedPin};
mod erased;
pub use erased::{EPin, ErasedPin};
mod dynamic;
pub use dynamic::{Dynamic, DynamicPin};

pub use embedded_hal::digital::v2::*;

use core::fmt;

/// A filler pin type
#[derive(Debug, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct NoPin<Otype = PushPull>(PhantomData<Otype>);
impl<Otype> NoPin<Otype> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

/// Extension trait to split a GPIO peripheral in independent pins and registers
pub trait GpioExt {
    /// The parts to split the GPIO into
    type Parts;

    /// Splits the GPIO block into independent pins and registers
    fn split(self) -> Self::Parts;
}

/// Id, port and mode for any pin
pub trait PinExt {
    /// Current pin mode
    type Mode;
    /// Pin number
    fn pin_id(&self) -> u8;
    /// Port number starting from 0
    fn port_id(&self) -> u8;
}

/// Some alternate mode (type state)
#[derive(Debug, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Alternate<const A: u8, Otype = PushPull>(PhantomData<Otype>);

/// Input mode (type state)
#[derive(Debug, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Input;

/// Pull setting for an input.
#[derive(Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pull {
    /// Floating
    None = 0,
    /// Pulled up
    Up = 1,
    /// Pulled down
    Down = 2,
}

/// Open drain input or output (type state)
#[derive(Debug, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct OpenDrain;

/// Output mode (type state)
#[derive(Debug, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Output<MODE = PushPull> {
    _mode: PhantomData<MODE>,
}

/// Push pull output (type state)
#[derive(Debug, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct PushPull;

/// Analog mode (type state)
#[derive(Debug, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Analog;

/// JTAG/SWD mote (type state)
pub type Debugger = Alternate<0, PushPull>;

pub(crate) mod marker {
    /// Marker trait that show if `ExtiPin` can be implemented
    pub trait Interruptible {}
    /// Marker trait for readable pin modes
    pub trait Readable {}
    /// Marker trait for slew rate configurable pin modes
    pub trait OutputSpeed {}
    /// Marker trait for active pin modes
    pub trait Active {}
    /// Marker trait for all pin modes except alternate
    pub trait NotAlt {}
    /// Marker trait for pins with alternate function `A` mapping
    pub trait IntoAf<const A: u8> {}
}

impl<MODE> marker::Interruptible for Output<MODE> {}
impl marker::Interruptible for Input {}
impl marker::Readable for Input {}
impl marker::Readable for Output<OpenDrain> {}
impl<const A: u8, Otype> marker::Interruptible for Alternate<A, Otype> {}
impl<const A: u8, Otype> marker::Readable for Alternate<A, Otype> {}
impl marker::Active for Input {}
impl<Otype> marker::OutputSpeed for Output<Otype> {}
impl<const A: u8, Otype> marker::OutputSpeed for Alternate<A, Otype> {}
impl<Otype> marker::Active for Output<Otype> {}
impl<const A: u8, Otype> marker::Active for Alternate<A, Otype> {}
impl marker::NotAlt for Input {}
impl<Otype> marker::NotAlt for Output<Otype> {}
impl marker::NotAlt for Analog {}

/// GPIO Pin speed selection
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Speed {
    /// Medium speed
    Medium = 1,
    /// Low speed
    Low = 2,
    /// High speed
    High = 3,
}

/// GPIO interrupt trigger edge selection
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Edge {
    /// Rising edge of voltage
    Rising,
    /// Falling edge of voltage
    Falling,
    /// Rising and falling edge of voltage
    RisingFalling,
}

macro_rules! af {
    ($($i:literal: $MUXi:ident),+) => {
        $(
            #[doc = concat!("Alternate function ", $i, " (type state)" )]
            pub type $MUXi<Otype = PushPull> = Alternate<$i, Otype>;
        )+
    };
}

af!(
    0: MUX0,
    1: MUX1,
    2: MUX2,
    3: MUX3,
    4: MUX4,
    5: MUX5,
    6: MUX6,
    7: MUX7,
    8: MUX8,
    9: MUX9,
    10: MUX10,
    11: MUX11,
    12: MUX12,
    13: MUX13,
    14: MUX14,
    15: MUX15
);

/// Generic pin type
///
/// - `MODE` is one of the pin modes (see [Modes](crate::gpio#modes) section).
/// - `P` is port name: `A` for GPIOA, `B` for GPIOB, etc.
/// - `N` is pin number: from `0` to `15`.
pub struct Pin<const P: char, const N: u8, MODE = DefaultMode> {
    _mode: PhantomData<MODE>,
}
impl<const P: char, const N: u8, MODE> Pin<P, N, MODE> {
    const fn new() -> Self {
        Self { _mode: PhantomData }
    }
}

impl<const P: char, const N: u8, MODE> PinExt for Pin<P, N, MODE> {
    type Mode = MODE;

    #[inline(always)]
    fn pin_id(&self) -> u8 {
        N
    }
    #[inline(always)]
    fn port_id(&self) -> u8 {
        P as u8 - b'A'
    }
}

pub trait PinSpeed: Sized {
    /// Set pin speed
    fn set_speed(&mut self, speed: Speed);

    #[inline(always)]
    fn speed(mut self, speed: Speed) -> Self {
        self.set_speed(speed);
        self
    }
}

pub trait PinPull: Sized {
    /// Set the internal pull-up and pull-down resistor
    fn set_internal_resistor(&mut self, resistor: Pull);

    #[inline(always)]
    fn internal_resistor(mut self, resistor: Pull) -> Self {
        self.set_internal_resistor(resistor);
        self
    }
}

impl<const P: char, const N: u8, MODE> PinSpeed for Pin<P, N, MODE>
where
    MODE: marker::OutputSpeed,
{
    #[inline(always)]
    fn set_speed(&mut self, speed: Speed) {
        self.set_speed(speed)
    }
}

impl<const P: char, const N: u8, MODE> Pin<P, N, MODE>
where
    MODE: marker::OutputSpeed,
{
    /// Set pin speed
    pub fn set_speed(&mut self, speed: Speed) {
        let offset = 4 * { N };
        match N {
            0..=7 => unsafe {
                (*Gpio::<P>::ptr()).cfglr.modify(|r, w| {
                    w.bits((r.bits() & !(0b11 << offset)) | ((speed as u32) << offset))
                });
            },
            8..=15 => unsafe {
                (*Gpio::<P>::ptr()).cfghr.modify(|r, w| {
                    w.bits((r.bits() & !(0b11 << offset)) | ((speed as u32) << offset))
                });
            },
            _ => unreachable!(),
        }
    }

    /// Set pin speed
    pub fn speed(mut self, speed: Speed) -> Self {
        self.set_speed(speed);
        self
    }
}

impl<const P: char, const N: u8, MODE> PinPull for Pin<P, N, MODE>
where
    MODE: marker::Active,
{
    #[inline(always)]
    fn set_internal_resistor(&mut self, resistor: Pull) {
        self.set_internal_resistor(resistor)
    }
}

impl<const P: char, const N: u8, MODE> Pin<P, N, MODE>
where
    MODE: marker::Active,
{
    /// Set the internal pull-up and pull-down resistor
    pub fn set_internal_resistor(&mut self, resistor: Pull) {
        let offset = 4 * { N };
        let value = match resistor {
            Pull::Down => 0,
            Pull::Up => 1,
            _ => 2,
        };

        if resistor == Pull::None {
            match N {
                0..=7 => unsafe {
                    (*Gpio::<P>::ptr())
                        .cfglr
                        .modify(|r, w| w.bits(r.bits() & !(0b1 << (offset + 3))))
                },
                8..=15 => unsafe {
                    (*Gpio::<P>::ptr())
                        .cfghr
                        .modify(|r, w| w.bits(r.bits() & !(0b1 << (offset + 3))))
                },
                _ => unreachable!(),
            }
        } else {
            match N {
                0..=7 => unsafe {
                    (*Gpio::<P>::ptr()).cfglr.modify(|r, w| {
                        w.bits(r.bits() & !(0b11 << (offset + 2)) | (0b10 << (offset + 2)))
                    })
                },
                8..=15 => unsafe {
                    (*Gpio::<P>::ptr()).cfghr.modify(|r, w| {
                        w.bits(r.bits() & !(0b11 << (offset + 2)) | (0b10 << (offset + 2)))
                    })
                },
                _ => unreachable!(),
            }
            unsafe {
                (*Gpio::<P>::ptr())
                    .odt
                    .modify(|r, w| w.bits(r.bits() & !(0b1 << N) | (value << N)))
            }
        }
    }

    /// Set the internal pull-up and pull-down resistor
    pub fn internal_resistor(mut self, resistor: Pull) -> Self {
        self.set_internal_resistor(resistor);
        self
    }

    /// Enables / disables the internal pull up
    pub fn internal_pull_up(self, on: bool) -> Self {
        if on {
            self.internal_resistor(Pull::Up)
        } else {
            self.internal_resistor(Pull::None)
        }
    }

    /// Enables / disables the internal pull down
    pub fn internal_pull_down(self, on: bool) -> Self {
        if on {
            self.internal_resistor(Pull::Down)
        } else {
            self.internal_resistor(Pull::None)
        }
    }
}

impl<const P: char, const N: u8, MODE> Pin<P, N, MODE> {
    /// Erases the pin number from the type
    ///
    /// This is useful when you want to collect the pins into an array where you
    /// need all the elements to have the same type
    pub fn erase_number(self) -> PartiallyErasedPin<P, MODE> {
        PartiallyErasedPin::new(N)
    }

    /// Erases the pin number and the port from the type
    ///
    /// This is useful when you want to collect the pins into an array where you
    /// need all the elements to have the same type
    pub fn erase(self) -> ErasedPin<MODE> {
        ErasedPin::new(P as u8 - b'A', N)
    }
}

impl<const P: char, const N: u8, MODE> From<Pin<P, N, MODE>> for PartiallyErasedPin<P, MODE> {
    /// Pin-to-partially erased pin conversion using the [`From`] trait.
    ///
    /// Note that [`From`] is the reciprocal of [`Into`].
    fn from(p: Pin<P, N, MODE>) -> Self {
        p.erase_number()
    }
}

impl<const P: char, const N: u8, MODE> From<Pin<P, N, MODE>> for ErasedPin<MODE> {
    /// Pin-to-erased pin conversion using the [`From`] trait.
    ///
    /// Note that [`From`] is the reciprocal of [`Into`].
    fn from(p: Pin<P, N, MODE>) -> Self {
        p.erase()
    }
}

impl<const P: char, const N: u8, MODE> Pin<P, N, MODE> {
    /// Set the output of the pin regardless of its mode.
    /// Primarily used to set the output value of the pin
    /// before changing its mode to an output to avoid
    /// a short spike of an incorrect value
    #[inline(always)]
    fn _set_state(&mut self, state: PinState) {
        match state {
            PinState::High => self._set_high(),
            PinState::Low => self._set_low(),
        }
    }
    #[inline(always)]
    fn _set_high(&mut self) {
        // NOTE(unsafe) atomic write to a stateless register
        unsafe { (*Gpio::<P>::ptr()).scr.write(|w| w.bits(1 << N)) }
    }
    #[inline(always)]
    fn _set_low(&mut self) {
        // NOTE(unsafe) atomic write to a stateless register
        unsafe { (*Gpio::<P>::ptr()).scr.write(|w| w.bits(1 << (16 + N))) }
    }
    #[inline(always)]
    fn _is_set_low(&self) -> bool {
        // NOTE(unsafe) atomic read with no side effects
        unsafe { (*Gpio::<P>::ptr()).odt.read().bits() & (1 << N) == 0 }
    }
    #[inline(always)]
    fn _is_low(&self) -> bool {
        // NOTE(unsafe) atomic read with no side effects
        unsafe { (*Gpio::<P>::ptr()).idt.read().bits() & (1 << N) == 0 }
    }
}

impl<const P: char, const N: u8, MODE> Pin<P, N, Output<MODE>> {
    /// Drives the pin high
    #[inline(always)]
    pub fn set_high(&mut self) {
        self._set_high()
    }

    /// Drives the pin low
    #[inline(always)]
    pub fn set_low(&mut self) {
        self._set_low()
    }

    /// Is the pin in drive high or low mode?
    #[inline(always)]
    pub fn get_state(&self) -> PinState {
        if self.is_set_low() {
            PinState::Low
        } else {
            PinState::High
        }
    }

    /// Drives the pin high or low depending on the provided value
    #[inline(always)]
    pub fn set_state(&mut self, state: PinState) {
        match state {
            PinState::Low => self.set_low(),
            PinState::High => self.set_high(),
        }
    }

    /// Is the pin in drive high mode?
    #[inline(always)]
    pub fn is_set_high(&self) -> bool {
        !self.is_set_low()
    }

    /// Is the pin in drive low mode?
    #[inline(always)]
    pub fn is_set_low(&self) -> bool {
        self._is_set_low()
    }

    /// Toggle pin output
    #[inline(always)]
    pub fn toggle(&mut self) {
        if self.is_set_low() {
            self.set_high()
        } else {
            self.set_low()
        }
    }
}

pub trait ReadPin {
    #[inline(always)]
    fn is_high(&self) -> bool {
        !self.is_low()
    }
    fn is_low(&self) -> bool;
}

impl<const P: char, const N: u8, MODE> ReadPin for Pin<P, N, MODE>
where
    MODE: marker::Readable,
{
    #[inline(always)]
    fn is_low(&self) -> bool {
        self.is_low()
    }
}

impl<const P: char, const N: u8, MODE> Pin<P, N, MODE>
where
    MODE: marker::Readable,
{
    /// Is the input pin high?
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        !self.is_low()
    }

    /// Is the input pin low?
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        self._is_low()
    }
}

impl<const P: char, const N: u8, MODE> OutputPin for Pin<P, N, Output<MODE>> {
    type Error = Infallible;

    #[inline(always)]
    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.set_high();
        Ok(())
    }

    #[inline(always)]
    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.set_low();
        Ok(())
    }
}

impl<const P: char, const N: u8, MODE> StatefulOutputPin for Pin<P, N, Output<MODE>> {
    #[inline(always)]
    fn is_set_high(&self) -> Result<bool, Self::Error> {
        Ok(self.is_set_high())
    }

    #[inline(always)]
    fn is_set_low(&self) -> Result<bool, Self::Error> {
        Ok(self.is_set_low())
    }
}

impl<const P: char, const N: u8, MODE> ToggleableOutputPin for Pin<P, N, Output<MODE>> {
    type Error = Infallible;

    #[inline(always)]
    fn toggle(&mut self) -> Result<(), Self::Error> {
        self.toggle();
        Ok(())
    }
}

impl<const P: char, const N: u8, MODE> InputPin for Pin<P, N, MODE>
where
    MODE: marker::Readable,
{
    type Error = Infallible;

    #[inline(always)]
    fn is_high(&self) -> Result<bool, Self::Error> {
        Ok(self.is_high())
    }

    #[inline(always)]
    fn is_low(&self) -> Result<bool, Self::Error> {
        Ok(self.is_low())
    }
}

macro_rules! gpio {
    ($GPIOX:ident, $gpiox:ident, $PEPin:ident, $port_id:expr, $PXn:ident, [
        $($PXi:ident: ($pxi:ident, $i:expr, [$($A:literal),*] $(, $MODE:ty)?),)+
    ]) => {
        /// GPIO
        pub mod $gpiox {
            use crate::pac::$GPIOX;
            use crate::crm::{Enable, Reset};

            /// GPIO parts
            pub struct Parts {
                $(
                    /// Pin
                    pub $pxi: $PXi $(<$MODE>)?,
                )+
            }

            impl super::GpioExt for $GPIOX {
                type Parts = Parts;

                fn split(self) -> Parts {
                    unsafe {
                        // Enable clock.
                        $GPIOX::enable_unchecked();
                        $GPIOX::reset_unchecked();
                    }
                    Parts {
                        $(
                            $pxi: $PXi::new(),
                        )+
                    }
                }
            }

            #[doc="Common type for "]
            #[doc=stringify!($GPIOX)]
            #[doc=" related pins"]
            pub type $PXn<MODE> = super::PartiallyErasedPin<$port_id, MODE>;

            $(
                #[doc=stringify!($PXi)]
                #[doc=" pin"]
                pub type $PXi<MODE = super::DefaultMode> = super::Pin<$port_id, $i, MODE>;

                $(
                    impl<MODE> super::marker::IntoAf<$A> for $PXi<MODE> { }
                )*
            )+

        }

        pub use $gpiox::{ $($PXi,)+ };
    }
}
use gpio;

mod f4;
pub use f4::*;

struct Gpio<const P: char>;
impl<const P: char> Gpio<P> {
    const fn ptr() -> *const crate::pac::gpioa::RegisterBlock {
        match P {
            'A' => crate::pac::GPIOA::ptr(),
            'B' => crate::pac::GPIOB::ptr() as _,
            'C' => crate::pac::GPIOC::ptr() as _,
            #[cfg(feature = "gpioe")]
            'E' => crate::pac::GPIOE::ptr() as _,
            #[cfg(feature = "gpiod")]
            'D' => crate::pac::GPIOD::ptr() as _,
            #[cfg(feature = "gpiof")]
            'F' => crate::pac::GPIOF::ptr() as _,
            _ => panic!("Unknown GPIO port"),
        }
    }
}
