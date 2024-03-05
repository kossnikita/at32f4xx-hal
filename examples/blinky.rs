//! Blinks LEDs
//!
//! This example should run on AT-START-F415 board

#![no_std]
#![no_main]

use cortex_m_rt::entry;
use defmt_rtt as _;
use panic_probe as _;

use at32f4xx_hal::{pac, prelude::*};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    let gpiob = dp.GPIOB.split();
    let gpiof = dp.GPIOF.split();
    let mut led2 = gpiof.pf6.into_push_pull_output();
    let mut led3 = gpiof.pf7.into_push_pull_output();
    let mut led4 = gpiob.pb11.into_push_pull_output();
    loop {
        cortex_m::asm::delay(1_000_000);
        defmt::info!("Toggle LED2");
        led2.toggle();
        cortex_m::asm::delay(1_000_000);
        defmt::info!("Toggle LED3");
        led3.toggle();
        cortex_m::asm::delay(1_000_000);
        defmt::info!("Toggle LED4");
        led4.toggle();
    }
}
