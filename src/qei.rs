//! # Quadrature Encoder Interface
use crate::{
    crm,
    gpio::Input,
    pac,
    timer::{CPin, General},
};

pub trait QeiExt: Sized + Instance {
    fn qei(
        self,
        pins: (
            impl Into<<Self as CPin<0>>::Ch<Input>>,
            impl Into<<Self as CPin<1>>::Ch<Input>>,
        ),
    ) -> Qei<Self>;
}

impl<TMR: Instance> QeiExt for TMR {
    fn qei(
        self,
        pins: (
            impl Into<<Self as CPin<0>>::Ch<Input>>,
            impl Into<<Self as CPin<1>>::Ch<Input>>,
        ),
    ) -> Qei<Self> {
        Qei::new(self, pins)
    }
}

/// Hardware quadrature encoder interface peripheral
pub struct Qei<TMR: Instance> {
    tmr: TMR,
    pins: (<TMR as CPin<0>>::Ch<Input>, <TMR as CPin<1>>::Ch<Input>),
}

impl<TMR: Instance> Qei<TMR> {
    /// Configures a TMR peripheral as a quadrature encoder interface input
    pub fn new(
        mut tmr: TMR,
        pins: (
            impl Into<<TMR as CPin<0>>::Ch<Input>>,
            impl Into<<TMR as CPin<1>>::Ch<Input>>,
        ),
    ) -> Self {
        // Enable and reset clock.
        unsafe {
            TMR::enable_unchecked();
            TMR::reset_unchecked();
        }

        let pins1 = (pins.0.into(), pins.1.into());
        tmr.setup_qei();
        Qei { tmr, pins: pins1 }
    }

    /// Releases the TMR peripheral and QEI pins
    #[allow(clippy::type_complexity)]
    pub fn release(
        self,
    ) -> (
        TMR,
        (<TMR as CPin<0>>::Ch<Input>, <TMR as CPin<1>>::Ch<Input>),
    ) {
        (self.tmr, self.pins)
    }

    /// Set current count number
    pub fn set_count(&mut self, value: TMR::Width) -> &mut Self {
        self.tmr.write_count(value);
        self
    }
}

impl<TMR: Instance> embedded_hal::Qei for Qei<TMR> {
    type Count = TMR::Width;

    fn count(&self) -> Self::Count {
        self.tmr.read_count()
    }

    fn direction(&self) -> embedded_hal::Direction {
        if self.tmr.read_direction() {
            embedded_hal::Direction::Upcounting
        } else {
            embedded_hal::Direction::Downcounting
        }
    }
}

pub trait Instance: crate::Sealed + crm::Enable + crm::Reset + General + CPin<0> + CPin<1> {
    fn setup_qei(&mut self);

    fn read_direction(&self) -> bool;
}

macro_rules! hal {
    ($TMR:ty) => {
        impl Instance for $TMR {
            #![allow(unused_unsafe)]
            fn setup_qei(&mut self) {
                self.cm1_input().write(|w| unsafe {
                    w.c1df()
                        .bits(1)
                        .c2df()
                        .bits(1)
                        .c1c()
                        .c1ifp1()
                        .c2c()
                        .c2ifp2()
                });
                self.cctrl.write(|w| unsafe {
                    w.c1p().high().c2p().high().c1en().enable().c2en().enable()
                });
                // enable and configure to capture on rising edge
                self.stctrl.modify(|_, w| w.smsel().encoder_a());
                self.set_auto_reload(<$TMR as General>::Width::MAX as u32)
                    .unwrap();
                self.enable_counter();
            }

            fn read_direction(&self) -> bool {
                true
            }
        }
    };
}

#[cfg(feature = "tmr1")]
hal! { pac::TMR1 }
#[cfg(feature = "tmr2")]
hal! { pac::TMR2 }
#[cfg(feature = "tmr3")]
hal! { pac::TMR3 }
#[cfg(feature = "tmr4")]
hal! { pac::TMR4 }
#[cfg(feature = "tmr5")]
hal! { pac::TMR5 }
#[cfg(feature = "tmr8")]
hal! { pac::TMR8 }
#[cfg(feature = "new-gpio")]
#[cfg(feature = "tmr9")]
hal! { pac::TMR9 }
#[cfg(feature = "tmr12")]
hal! { pac::TMR12 }
