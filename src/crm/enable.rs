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

#[allow(unused)]
macro_rules! bus_lpenable {
    ($PER:ident => $bit:literal) => {
        impl LPEnable for crate::pac::$PER {
            #[inline(always)]
            fn enable_in_low_power(rcc: &RccRB) {
                unsafe {
                    bb::set(Self::Bus::lpenr(rcc), $bit);
                }
                // Stall the pipeline to work around erratum 2.1.13 (DM00037591)
                cortex_m::asm::dsb();
            }
            #[inline(always)]
            fn disable_in_low_power(rcc: &RccRB) {
                unsafe {
                    bb::clear(Self::Bus::lpenr(rcc), $bit);
                }
            }
            #[inline(always)]
            fn is_enabled_in_low_power() -> bool {
                let rcc = pac::RCC::ptr();
                (Self::Bus::lpenr(unsafe { &*rcc }).read().bits() >> $bit) & 0x1 != 0
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

#[cfg(any(feature = "at32f415", feature = "at32f421"))]
mod f1;
