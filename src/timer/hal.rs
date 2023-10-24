//! Delay implementation based on general-purpose 32 bit timers and System timer (SysTick).
//!

use embedded_hal::delay::DelayUs;
use fugit::ExtU32;

use super::SysDelay;

impl DelayUs for SysDelay {
    fn delay_us(&mut self, us: u32) {
        self.delay(us.micros())
    }
}
