use super::*;
use crate::bb;

macro_rules! bus_enable {
    ($PER:ident => $bit:literal) => {
        impl Enable for crate::pac::$PER {
            #[inline(always)]
            fn enable(rcc: &RccRB) {
                unsafe {
                    bb::set(Self::Bus::enr(rcc), $bit);
                }
                // Stall the pipeline to work around erratum 2.1.13 (DM00037591)
                cortex_m::asm::dsb();
            }
            #[inline(always)]
            fn disable(rcc: &RccRB) {
                unsafe {
                    bb::clear(Self::Bus::enr(rcc), $bit);
                }
            }
            #[inline(always)]
            fn is_enabled() -> bool {
                let rcc = pac::CRM::ptr();
                (Self::Bus::enr(unsafe { &*rcc }).read().bits() >> $bit) & 0x1 != 0
            }
        }
    };
}

macro_rules! bus_reset {
    ($PER:ident => $bit:literal) => {
        impl Reset for crate::pac::$PER {
            #[inline(always)]
            fn reset(rcc: &RccRB) {
                unsafe {
                    bb::set(Self::Bus::rstr(rcc), $bit);
                    bb::clear(Self::Bus::rstr(rcc), $bit);
                }
            }
        }
    };
}

macro_rules! bus {
    ($($PER:ident => ($busX:ty, $bit:literal),)+) => {
        $(
            impl crate::Sealed for crate::pac::$PER {}
            impl RccBus for crate::pac::$PER {
                type Bus = $busX;
            }
            bus_enable!($PER => $bit);
            bus_reset!($PER => $bit);
        )+
    }
}

bus! {
    CRC => (AHB, 12),
    DMA1 => (AHB, 21),
    DMA2 => (AHB, 22),
}

bus! {
    GPIOA => (AHB, 0),
    GPIOB => (AHB, 1),
    GPIOC => (AHB, 2),
    GPIOD => (AHB, 3),
    GPIOF => (AHB, 4),
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
