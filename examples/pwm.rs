//! Blinks LEDs

#![no_std]
#![no_main]

use panic_probe as _;

use at32f4xx_hal as hal;
use cortex_m_rt::entry;

use defmt_rtt as _;

use hal::{
    pac,
    prelude::*,
    timer::{Channel1, Channel2, PwmExt},
};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    let mut crm = dp.CRM.constrain();
    let clocks = crm.cfgr.freeze();

    let gpioa = dp.GPIOA.split();
    let channels = (Channel1::new(gpioa.pa8), Channel2::new(gpioa.pa9));

    let pwm = dp.TMR1.pwm_hz(channels, 20.kHz(), &clocks);
    let (mut ch1, _ch2) = pwm;
    let max_duty = ch1.get_max_duty();

    loop {
        cortex_m::asm::nop();
    }
}
