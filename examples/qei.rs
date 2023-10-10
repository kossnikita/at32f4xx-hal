#![deny(unsafe_code)]
#![no_main]
#![no_std]

// Demonstrates the use of a rotary encoder. This example was tested
// on AT-START-F415 board
//
// The rotary encoder A and B pins are connected to pins A0 and A1,
// and they each have a 10K ohm pull-up resistor.

use defmt_rtt as _;
use panic_probe as _;

use at32f4xx_hal::{pac, prelude::*, qei::Qei};
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().expect("Failed to get at32 peripherals");
    let cp = cortex_m::peripheral::Peripherals::take().expect("Failed to get cortex_m peripherals");

    // Set up the system clock.
    let crm = dp.CRM.constrain();
    let clocks = crm.cfgr.freeze();

    // Create a delay abstraction based on SysTick.
    let mut delay = cp.SYST.delay(&clocks);

    let gpioa = dp.GPIOA.split();

    // Connect a rotary encoder to pins A0 and A1.
    let rotary_encoder_pins = (gpioa.pa0, gpioa.pa1);
    let rotary_encoder_timer = dp.TMR2;
    let rotary_encoder = Qei::new(rotary_encoder_timer, rotary_encoder_pins);

    let mut current_count = rotary_encoder.count();
    loop {
        let new_count = rotary_encoder.count();

        if new_count != current_count {
            defmt::println!("{}", new_count);
            current_count = new_count;
        }

        delay.delay(1.millis());
    }
}
