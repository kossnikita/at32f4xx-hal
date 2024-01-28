use super::*;
use crate::bb;

macro_rules! bus_enable {
    ($PER:ident => $bit:literal) => {
        impl Enable for crate::pac::$PER {
            #[inline(always)]
            fn enable(crm: &CrmRB) {
                unsafe {
                    bb::set(Self::Bus::enr(crm), $bit);
                }
                // Stall the pipeline to work around erratum 2.1.13 (DM00037591)
                cortex_m::asm::dsb();
            }
            #[inline(always)]
            fn disable(crm: &CrmRB) {
                unsafe {
                    bb::clear(Self::Bus::enr(crm), $bit);
                }
            }
            #[inline(always)]
            fn is_enabled() -> bool {
                let crm = pac::CRM::ptr();
                (Self::Bus::enr(unsafe { &*crm }).read().bits() >> $bit) & 0x1 != 0
            }
        }
    };
}

macro_rules! bus_reset {
    ($PER:ident => $bit:literal) => {
        impl Reset for crate::pac::$PER {
            #[inline(always)]
            fn reset(crm: &CrmRB) {
                unsafe {
                    bb::set(Self::Bus::rstr(crm), $bit);
                    bb::clear(Self::Bus::rstr(crm), $bit);
                }
            }
        }
    };
}

macro_rules! bus {
    ($($PER:ident => ($busX:ty, $bit:literal),)+) => {
        $(
            impl crate::Sealed for crate::pac::$PER {}
            impl CrmBus for crate::pac::$PER {
                type Bus = $busX;
            }
            bus_enable!($PER => $bit);
            bus_reset!($PER => $bit);
        )+
    }
}

bus! {
    TMR2 => (APB1, 0),
}

bus! {
    TMR3 => (APB1, 1),
}

bus! {
    TMR4 => (APB1, 2),
}

bus! {
    TMR5 => (APB1, 3),
}

bus! {
    TMR6 => (APB1, 4),
}

bus! {
    TMR7 => (APB1, 5),
}

bus! {
    TMR12 => (APB1, 6),
}

bus! {
    TMR13 => (APB1, 7),
}

bus! {
    TMR14 => (APB1, 8),
}

#[cfg(feature = "at32f415")]
bus! {
    DMA1 => (AHB, 0),
    DMA2 => (AHB, 1),
    CRC => (AHB, 6),
}

#[cfg(feature = "at32f415")]
bus! {
    GPIOA => (APB2, 2),
    GPIOB => (APB2, 3),
    GPIOC => (APB2, 4),
}

#[cfg(all(feature = "iomux", feature = "at32f415"))]
bus! {
    IOMUX => (APB2, 0),
}

#[cfg(all(feature = "gpiod", feature = "at32f415"))]
bus! {
    GPIOD => (APB2, 5),
}

#[cfg(all(feature = "gpioe", feature = "at32f415"))]
bus! {
    GPIOE => (APB2, 6),
}

#[cfg(all(feature = "gpiof", feature = "at32f415"))]
bus! {
    GPIOF => (APB2, 7),
}

#[cfg(feature = "at32f415")]
bus! {
    SPI1 => (APB2, 12),
    SPI2 => (APB1, 14),
}

#[cfg(feature = "at32f415")]
bus! {
    I2C1 => (APB1, 21),
    I2C2 => (APB1, 22),
}

#[cfg(feature = "at32f415")]
bus! {
    USART1 => (APB2, 14),
    USART2 => (APB1, 17),
}

#[cfg(all(feature = "usart3", feature = "at32f415"))]
bus! {
    USART3 => (APB1, 18),
}

#[cfg(all(feature = "uart4", feature = "at32f415"))]
bus! {
    UART4 => (APB1, 19),
    UART5 => (APB1, 20),
}

#[cfg(feature = "at32f415")]
bus! {
    ADC1 => (APB2, 8),
}

#[cfg(all(feature = "tmr1", feature = "at32f415"))]
bus! {
    TMR1 => (APB2, 11),
}

#[cfg(all(feature = "tmr8", feature = "at32f415"))]
bus! {
    TMR8 => (APB2, 13),
}

#[cfg(all(feature = "tmr9", feature = "at32f415"))]
bus! {
    TMR9 => (APB2, 19),
}

#[cfg(all(feature = "tmr10", feature = "at32f415"))]
bus! {
    TMR10 => (APB2, 10),
}

#[cfg(all(feature = "tmr11", feature = "at32f415"))]
bus! {
    TMR11 => (APB2, 21),
}

#[cfg(all(feature = "tmr15", feature = "at32f415"))]
bus! {
    TMR15 => (APB2, 16),
}

#[cfg(all(feature = "tmr16", feature = "at32f415"))]
bus! {
    TMR16 => (APB2, 17),
}

#[cfg(all(feature = "tmr17", feature = "at32f415"))]
bus! {
    TMR17 => (APB2, 18),
}

#[cfg(all(feature = "tmr20", feature = "at32f415"))]
bus! {
    TMR20 => (APB2, 11),
}

#[cfg(any(feature = "at32f435", feature = "at32f437"))]
bus! {
    DMA1 => (AHB1, 22),
    DMA2 => (AHB1, 24),
    CRC => (AHB1, 12),
}

#[cfg(any(feature = "at32f435", feature = "at32f437"))]
bus! {
    GPIOA => (AHB1, 0),
    GPIOB => (AHB1, 1),
    GPIOC => (AHB1, 2),
}

#[cfg(all(feature = "gpiod", any(feature = "at32f435", feature = "at32f437")))]
bus! {
    GPIOD => (AHB1, 3),
}

#[cfg(all(feature = "gpioe", any(feature = "at32f435", feature = "at32f437")))]
bus! {
    GPIOE => (AHB1, 4),
}

#[cfg(all(feature = "gpiof", any(feature = "at32f435", feature = "at32f437")))]
bus! {
    GPIOF => (AHB1, 5),
}

#[cfg(all(feature = "gpiog", any(feature = "at32f435", feature = "at32f437")))]
bus! {
    GPIOG => (AHB1, 6),
}

#[cfg(all(feature = "gpioh", any(feature = "at32f435", feature = "at32f437")))]
bus! {
    GPIOH => (AHB1, 7),
}

#[cfg(any(feature = "at32f435", feature = "at32f437"))]
bus! {
    SPI1 => (APB2, 12),
    SPI2 => (APB1, 14),
    SPI3 => (APB1, 15),
    SPI4 => (APB2, 13),
}

#[cfg(any(feature = "at32f435", feature = "at32f437"))]
bus! {
    I2C1 => (APB1, 21),
    I2C2 => (APB1, 22),
    I2C3 => (APB1, 23),
}

#[cfg(any(feature = "at32f435", feature = "at32f437"))]
bus! {
    USART1 => (APB2, 4),
    USART2 => (APB1, 17),
    USART3 => (APB1, 18),
    UART4 => (APB1, 19),
    UART5 => (APB1, 20),
    USART6 => (APB2, 5),
    UART7 => (APB1, 30),
    UART8 => (APB1, 31),
}

// #[cfg(all(feature = "usart3", any(feature = "at32f435", feature = "at32f437")))]
// bus! {
//     USART3 => (APB1, 18),
// }

// #[cfg(all(feature = "uart4", any(feature = "at32f435", feature = "at32f437")))]
// bus! {
//     UART4 => (APB1, 19),
//     UART5 => (APB1, 20),
// }

#[cfg(any(feature = "at32f435", feature = "at32f437"))]
bus! {
    ADC1 => (APB2, 8),
    ADC2 => (APB2, 9),
    ADC3 => (APB2, 10),
}

#[cfg(all(feature = "tmr1", any(feature = "at32f435", feature = "at32f437")))]
bus! {
    TMR1 => (APB2, 0),
}

#[cfg(all(feature = "tmr8", any(feature = "at32f435", feature = "at32f437")))]
bus! {
    TMR8 => (APB2, 1),
}

#[cfg(all(feature = "tmr9", any(feature = "at32f435", feature = "at32f437")))]
bus! {
    TMR9 => (APB2, 16),
}

#[cfg(all(feature = "tmr10", any(feature = "at32f435", feature = "at32f437")))]
bus! {
    TMR10 => (APB2, 17),
}

#[cfg(all(feature = "tmr11", any(feature = "at32f435", feature = "at32f437")))]
bus! {
    TMR11 => (APB2, 18),
}

#[cfg(all(feature = "tmr20", any(feature = "at32f435", feature = "at32f437")))]
bus! {
    TMR20 => (APB2, 20),
}
