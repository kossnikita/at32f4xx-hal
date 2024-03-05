#![no_std]
#![no_main]

use core::result;

use cortex_m_rt::entry;
use defmt_rtt as _;
use panic_probe as _;

use at32f4xx_hal::{
    adc::{config::*, Adc},
    crm::Enable,
    pac,
    prelude::*,
};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    let gpioa = dp.GPIOA.split();

    let fb = gpioa.pa5.into_analog();

    let config = AdcConfig::default().sequence(SequenceMode::Enabled);
    let mut adc = Adc::adc1(dp.ADC1, true, config);
    let mut result;
    adc.configure_channel(&fb, Sequence::One, SampleTime::Cycles_112);
    loop {
        cortex_m::asm::delay(1_000_000);
        adc.start_conversion();
        result = adc.current_sample();
        defmt::info!("ADC conversion {}", result);
    }
}
