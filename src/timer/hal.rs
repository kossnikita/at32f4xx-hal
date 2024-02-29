//! Delay implementation based on general-purpose 32 bit timers and System timer (SysTick).
//!

use embedded_hal::delay::DelayNs;
use fugit::ExtU32;

use super::SysDelay;

impl DelayNs for SysDelay {
    fn delay_ns(&mut self, ns: u32) {
        self.delay_ns(ns.nanos())
    }
}
