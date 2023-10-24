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
    DMA1 => (AHB, 0),
    DMA2 => (AHB, 1),
    CRC => (AHB, 6),
}

bus! {
    GPIOA => (APB2, 2),
    GPIOB => (APB2, 3),
    GPIOC => (APB2, 4),
}

#[cfg(feature = "iomux")]
bus! {
    IOMUX => (APB2, 0),
}

#[cfg(feature = "gpiod")]
bus! {
    GPIOD => (APB2, 5),
}

#[cfg(feature = "gpioe")]
bus! {
    GPIOE => (APB2, 6),
}

#[cfg(feature = "gpiof")]
bus! {
    GPIOF => (APB2, 7),
}

bus! {
    SPI1 => (APB2, 12),
    SPI2 => (APB1, 14),
}

bus! {
    I2C1 => (APB1, 21),
    I2C2 => (APB1, 22),
}

bus! {
    USART1 => (APB2, 14),
    USART2 => (APB1, 17),
}

bus! {
    ADC1 => (APB2, 8),
}

#[cfg(feature = "tmr1")]
bus! {
    TMR1 => (APB2, 11),
}

#[cfg(feature = "tmr2")]
bus! {
    TMR2 => (APB1, 0),
}

#[cfg(feature = "tmr3")]
bus! {
    TMR3 => (APB1, 1),
}

#[cfg(feature = "tmr4")]
bus! {
    TMR4 => (APB1, 2),
}

#[cfg(feature = "tmr5")]
bus! {
    TMR5 => (APB1, 3),
}

#[cfg(feature = "tmr6")]
bus! {
    TMR6 => (APB1, 4),
}

#[cfg(feature = "tmr7")]
bus! {
    TMR7 => (APB1, 5),
}

#[cfg(feature = "tmr8")]
bus! {
    TMR8 => (APB2, 13),
}

#[cfg(feature = "tmr9")]
bus! {
    TMR9 => (APB2, 19),
}

#[cfg(feature = "tmr10")]
bus! {
    TMR10 => (APB2, 10),
}

#[cfg(feature = "tmr11")]
bus! {
    TMR11 => (APB2, 21),
}

#[cfg(feature = "tmr12")]
bus! {
    TMR12 => (APB1, 6),
}

#[cfg(feature = "tmr13")]
bus! {
    TMR13 => (APB1, 7),
}

#[cfg(feature = "tmr14")]
bus! {
    TMR14 => (APB1, 8),
}

#[cfg(feature = "tmr15")]
bus! {
    TMR15 => (APB2, 16),
}

#[cfg(feature = "tmr16")]
bus! {
    TMR16 => (APB2, 17),
}

#[cfg(feature = "tmr17")]
bus! {
    TMR17 => (APB2, 18),
}

#[cfg(feature = "tmr20")]
bus! {
    TMR20 => (APB2, 11),
}
