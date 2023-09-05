//! Blinks LEDs

#![no_std]
#![no_main]

use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::entry;
use panic_halt as _;

use at32f4xx_hal::{pac, prelude::*};

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    // Set up the SysTick peripheral.
    let mut syst = cp.SYST;
    syst.set_clock_source(SystClkSource::Core);
    syst.set_reload(500_000); //internal in clock ticks
    syst.enable_counter();

    let gpioc = dp.GPIOC.split();
    let mut led2 = gpioc.pc2.into_push_pull_output();
    let mut led3 = gpioc.pc3.into_push_pull_output();
    let mut led4 = gpioc.pc5.into_push_pull_output();

    syst.clear_current();

    loop {
        while !syst.has_wrapped() {}
        led2.toggle();
        while !syst.has_wrapped() {}
        led3.toggle();
        while !syst.has_wrapped() {}
        led4.toggle();
    }
}
