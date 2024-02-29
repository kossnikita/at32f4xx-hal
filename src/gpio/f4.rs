use super::*;

pub use super::Input as DefaultMode;

impl<const P: char, const N: u8, MODE> Pin<P, N, MODE>
where
    MODE: marker::OutputSpeed,
{
    /// Set pin speed
    pub fn set_speed(&mut self, speed: Speed) {
        let offset = 2 * { N };
        unsafe {
            (*Gpio::<P>::ptr())
                .odrvr()
                .modify(|r, w| w.bits((r.bits() & !(0b11 << offset)) | ((speed as u32) << offset)));
        }
    }

    /// Set pin speed
    pub fn speed(mut self, speed: Speed) -> Self {
        self.set_speed(speed);
        self
    }
}

impl<const P: char, const N: u8, MODE> Pin<P, N, MODE>
where
    MODE: marker::Active,
{
    /// Set the internal pull-up and pull-down resistor
    pub fn set_internal_resistor(&mut self, resistor: Pull) {
        let offset = 2 * { N };
        let value = resistor as u32;
        unsafe {
            (*Gpio::<P>::ptr())
                .pull()
                .modify(|r, w| w.bits((r.bits() & !(0b11 << offset)) | (value << offset)));
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

#[cfg(feature = "at32f421")]
gpio!(GPIOA, gpioa, PA, 'A', PAn, [
    PA0: (pa0, 0, [1, 4, 5, 7]),
    PA1: (pa1, 1, [0, 1, 4, 5]),
    PA2: (pa2, 2, [0, 1]),
    PA3: (pa3, 3, [0, 1, 5]),
    PA4: (pa4, 4, [0, 1, 4]),
    PA5: (pa5, 5, [0]),
    PA6: (pa6, 6, [0, 1, 2, 3, 5, 6, 7]),
    PA7: (pa7, 7, [0, 1, 2, 4, 5, 6]),
    PA8: (pa8, 8, [0, 1, 2, 3, 4, 7]),
    PA9: (pa9, 9, [0, 1, 2, 4, 5, 7]),
    PA10: (pa10, 10, [0, 1, 2, 4]),
    PA11: (pa11, 11, [0, 1, 2, 4, 5, 7]),
    PA12: (pa12, 12, [0, 1, 2, 5]),
    PA13: (pa13, 13, [0, 1, 6], super::Debugger), // SWDIO, PullUp VeryHigh speed
    PA14: (pa14, 14, [0, 1, 6], super::Debugger), // SWCLK, PullDown
    PA15: (pa15, 15, [0, 1, 3, 6]),
]);

#[cfg(feature = "at32f421")]
gpio!(GPIOB, gpiob, PB, 'B', PBn, [
    PB0: (pb0, 0, [0, 1, 2, 3, 6]),
    PB1: (pb1, 1, [0, 1, 2, 6]),
    PB2: (pb2, 2, [2]),
    PB3: (pb3, 3, [0, 1, 6]),
    PB4: (pb4, 4, [0, 1, 2, 5, 6, 7]),
    PB5: (pb5, 5, [0, 1, 2, 3, 6]),
    PB6: (pb6, 6, [0, 1, 2, 6]),
    PB7: (pb7, 7, [0, 1, 2]),
    PB8: (pb8, 8, [1, 2]),
    PB9: (pb9, 9, [0, 1, 2, 3, 5, 7]),
    PB10: (pb10, 10, [1, 7]),
    PB11: (pb11, 11, [0, 1]),
    PB12: (pb12, 12, [0, 1, 2, 5, 7]),
    PB13: (pb13, 13, [0, 2, 5]),
    PB14: (pb14, 14, [0, 1, 2, 5]),
    PB15: (pb15, 15, [0, 1, 2, 3]),
]);

#[cfg(feature = "at32f421")]
gpio!(GPIOF, gpiof, PF, 'F', PFn, [
    PF0: (pf0, 0, [1]),
    PF1: (pf1, 1, [1]),
    PF6: (pf6, 6, [0]),
    PF7: (pf7, 7, [0]),
]);
