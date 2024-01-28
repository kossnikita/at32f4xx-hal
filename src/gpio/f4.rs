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

#[cfg(any(feature = "at32f435", feature = "at32f437"))]
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
    PA13: (pa13, 13, []),   // JTMS-SWDIO, PullUp VeryHigh speed
    PA14: (pa14, 14, []),   // JTCK-SWCLK, PullDown
    PA15: (pa15, 15, []),   // JTDI, PullUp
]);

#[cfg(any(feature = "at32f435", feature = "at32f437"))]
gpio!(GPIOB, gpiob, PB, 'B', PBn, [
    PB0: (pb0, 0, []),
    PB1: (pb1, 1, []),
    PB2: (pb2, 2, []),
    PB3: (pb3, 3, []),      // JTDO-SWO, VeryHigh speed
    PB4: (pb4, 4, []),      // JTRST, PullUp
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

#[cfg(any(feature = "at32f435", feature = "at32f437"))]
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

#[cfg(any(feature = "at32f435", feature = "at32f437"))]
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

#[cfg(any(feature = "at32f435", feature = "at32f437"))]
gpio!(GPIOE, gpioe, PE, 'E', PEn, [
    PE0: (pe0, 0, []),
    PE1: (pe1, 1, []),
    PE2: (pe2, 2, []),
    PE3: (pe3, 3, []),
    PE4: (pe4, 4, []),
    PE5: (pe5, 5, []),
    PE6: (pe6, 6, []),
    PE7: (pe7, 7, []),
    PE8: (pe8, 8, []),
    PE9: (pe9, 9, []),
    PE10: (pe10, 10, []),
    PE11: (pe11, 11, []),
    PE12: (pe12, 12, []),
    PE13: (pe13, 13, []),
    PE14: (pe14, 14, []),
    PE15: (pe15, 15, []),
]);

#[cfg(any(feature = "at32f435", feature = "at32f437"))]
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

#[cfg(any(feature = "at32f435", feature = "at32f437"))]
gpio!(GPIOG, gpiog, PG, 'G', PGn, [
    PG0: (pg0, 0, []),
    PG1: (pg1, 1, []),
    PG2: (pg2, 2, []),
    PG3: (pg3, 3, []),
    PG4: (pg4, 4, []),
    PG5: (pg5, 5, []),
    PG6: (pg6, 6, []),
    PG7: (pg7, 7, []),
    PG8: (pg8, 8, []),
    PG9: (pg9, 9, []),
    PG10: (pg10, 10, []),
    PG11: (pg11, 11, []),
    PG12: (pg12, 12, []),
    PG13: (pg13, 13, []),
    PG14: (pg14, 14, []),
    PG15: (pg15, 15, []),
]);

#[cfg(any(feature = "at32f435", feature = "at32f437"))]
gpio!(GPIOH, gpioh, PH, 'H', PHn, [
    PH0: (ph0, 0, []),
    PH1: (ph1, 1, []),
    PH2: (ph2, 2, []),
]);
