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
    USART1 => (APB2, 4),
    USART2 => (APB1, 17),
}

bus! {
    ADC1 => (APB2, 8),
}

bus! {
    TMR1 => (APB2, 0),
}
