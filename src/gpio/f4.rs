use super::*;

pub use super::Input as DefaultMode;

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
    PA13: (pa13, 13, [], super::Debugger), // SWDIO, PullUp VeryHigh speed
    PA14: (pa14, 14, [], super::Debugger), // SWCLK, PullDown
    PA15: (pa15, 15, [], super::Debugger), // JTDI, PullUp
]);

#[cfg(feature = "at32f415")]
gpio!(GPIOB, gpiob, PB, 'B', PBn, [
    PB0: (pb0, 0, []),
    PB1: (pb1, 1, []),
    PB2: (pb2, 2, []),
    PB3: (pb3, 3, [], super::Debugger), // SWO, VeryHigh speed
    PB4: (pb4, 4, [], super::Debugger), // JTRST, PullUp
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
    PD4: (pd4, 4, []),
    PD5: (pd5, 5, []),
    PD6: (pd6, 6, []),
    PD7: (pd7, 7, []),
    PD8: (pd8, 8, []),
    PD9: (pd9, 9, []),
    PD10: (pd10, 10, []),
    PD11: (pd11, 11, []),
    PD12: (pd12, 12, []),
    PD13: (pd13, 13, []),
    PD14: (pd14, 14, []),
    PD15: (pd15, 15, []),
]);

#[cfg(feature = "at32f415")]
gpio!(GPIOF, gpiof, PF, 'F', PFn, [
    PF0: (pf0, 0, []),
    PF1: (pf1, 1, []),
    PF2: (pf2, 2, []),
    PF3: (pf3, 3, []),
    PF4: (pf4, 4, []),
    PF5: (pf5, 5, []),
    PF6: (pf6, 6, []),
    PF7: (pf7, 7, []),
    PF8: (pf8, 8, []),
    PF9: (pf9, 9, []),
    PF10: (pf10, 10, []),
    PF11: (pf11, 11, []),
    PF12: (pf12, 12, []),
    PF13: (pf13, 13, []),
    PF14: (pf14, 14, []),
    PF15: (pf15, 15, []),
]);
