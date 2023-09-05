//! Blinks LEDs

#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;

use at32f4xx_hal::{pac, prelude::*};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    let mut crm = dp.CRM.constrain();
    let clocks = crm.cfgr.freeze();

    loop {
        cortex_m::asm::nop();
    }
}
