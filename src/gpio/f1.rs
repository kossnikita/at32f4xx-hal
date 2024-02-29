use super::*;

pub use super::Input as DefaultMode;

pub impl<const P: char, const N: u8, MODE> Pin<P, N, MODE>
where
    MODE: marker::OutputSpeed,
{
    /// Set pin speed
    pub fn set_speed(&mut self, speed: Speed) {
        let offset = 4 * { N };
        match N {
            0..=7 => unsafe {
                (*Gpio::<P>::ptr()).cfglr().modify(|r, w| {
                    w.bits((r.bits() & !(0b11 << offset)) | ((speed as u32) << offset))
                });
            },
            8..=15 => unsafe {
                (*Gpio::<P>::ptr()).cfghr().modify(|r, w| {
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

pub impl<const P: char, const N: u8, MODE> Pin<P, N, MODE>
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
                        .cfglr()
                        .modify(|r, w| w.bits(r.bits() & !(0b1 << (offset + 3))))
                },
                8..=15 => unsafe {
                    (*Gpio::<P>::ptr())
                        .cfghr()
                        .modify(|r, w| w.bits(r.bits() & !(0b1 << (offset + 3))))
                },
                _ => unreachable!(),
            }
        } else {
            match N {
                0..=7 => unsafe {
                    (*Gpio::<P>::ptr()).cfglr().modify(|r, w| {
                        w.bits(r.bits() & !(0b11 << (offset + 2)) | (0b10 << (offset + 2)))
                    })
                },
                8..=15 => unsafe {
                    (*Gpio::<P>::ptr()).cfghr().modify(|r, w| {
                        w.bits(r.bits() & !(0b11 << (offset + 2)) | (0b10 << (offset + 2)))
                    })
                },
                _ => unreachable!(),
            }
            unsafe {
                (*Gpio::<P>::ptr())
                    .odt()
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

#[cfg(feature = "at32f415")]
gpio!(GPIOA, gpioa, PA, 'A', PAn, [
    PA0: (pa0, 0, []),
    PA1: (pa1, 1, []),
    PA2: (pa2, 2, []),
    PA3: (pa3, 3, []),
    PA4: (pa4, 4, []),
    PA5: (pa5, 5, []),
    PA6: (pa6, 6, []),
    PA7: (pa7, 7, []),
    PA8: (pa8, 8, []),
    PA9: (pa9, 9, []),
    PA10: (pa10, 10, []),
    PA11: (pa11, 11, []),
    PA12: (pa12, 12, []),
    PA13: (pa13, 13, []), // JTMS-SWDIO, PullUp VeryHigh speed
    PA14: (pa14, 14, []), // JTCK-SWCLK, PullDown
    PA15: (pa15, 15, []), // JTDI, PullUp
]);

#[cfg(feature = "at32f415")]
gpio!(GPIOB, gpiob, PB, 'B', PBn, [
    PB0: (pb0, 0, []),
    PB1: (pb1, 1, []),
    PB2: (pb2, 2, []),
    PB3: (pb3, 3, []), // JTDO-SWO, VeryHigh speed
    PB4: (pb4, 4, []), // JTRST, PullUp
    PB5: (pb5, 5, []),
    PB6: (pb6, 6, []),
    PB7: (pb7, 7, []),
    PB8: (pb8, 8, []),
    PB9: (pb9, 9, []),
    PB10: (pb10, 10, []),
    PB11: (pb11, 11, []),
    PB12: (pb12, 12, []),
    PB13: (pb13, 13, []),
    PB14: (pb14, 14, []),
    PB15: (pb15, 15, []),
]);

#[cfg(feature = "at32f415")]
gpio!(GPIOC, gpioc, PC, 'C', PCn, [
    PC0: (pc0, 0, []),
    PC1: (pc1, 1, []),
    PC2: (pc2, 2, []),
    PC3: (pc3, 3, []),
    PC4: (pc4, 4, []),
    PC5: (pc5, 5, []),
    PC6: (pc6, 6, []),
    PC7: (pc7, 7, []),
    PC8: (pc8, 8, []),
    PC9: (pc9, 9, []),
    PC10: (pc10, 10, []),
    PC11: (pc11, 11, []),
    PC12: (pc12, 12, []),
    PC13: (pc13, 13, []),
    PC14: (pc14, 14, []),
    PC15: (pc15, 15, []),
]);

#[cfg(feature = "at32f415")]
gpio!(GPIOD, gpiod, PD, 'D', PDn, [
    PD0: (pd0, 0, []),
    PD1: (pd1, 1, []),
    PD2: (pd2, 2, []),
    PD3: (pd3, 3, []),
]);

#[cfg(feature = "at32f415")]
gpio!(GPIOF, gpiof, PF, 'F', PFn, [
    PF4: (pf4, 4, []),
    PF5: (pf5, 5, []),
    PF6: (pf6, 6, []),
    PF7: (pf7, 7, []),
]);
