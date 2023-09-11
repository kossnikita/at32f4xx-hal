//! Blinks LEDs
//!
//! This example should run on AT-START-F415 board

#![no_std]
#![no_main]

use cortex_m_rt::entry;
use defmt_rtt as _;
use panic_halt as _;

use at32f4xx_hal::{pac, prelude::*};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    let gpioc = dp.GPIOC.split();
    let mut led2 = gpioc.pc2.into_push_pull_output();
    let mut led3 = gpioc.pc3.into_push_pull_output();
    let mut led4 = gpioc.pc5.into_push_pull_output();

    defmt::println!("Hello World!");

    loop {
        cortex_m::asm::delay(1_000_000);
        led2.toggle();
        cortex_m::asm::delay(1_000_000);
        led3.toggle();
        cortex_m::asm::delay(1_000_000);
        led4.toggle();
    }
}
