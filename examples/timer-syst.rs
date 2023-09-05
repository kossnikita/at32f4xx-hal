//! Start and stop a periodic system timer.

#![no_std]
#![no_main]

// use panic_halt as _;
use panic_probe as _;

use at32f4xx_hal as hal;
use cortex_m_rt::entry;
use hal::timer::Error;

use defmt_rtt as _;

use crate::hal::{pac, prelude::*};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::peripheral::Peripherals::take().unwrap();
    let crm = dp.CRM.constrain();
    let clocks = crm.cfgr.sclk(24.MHz()).freeze();

    // Create a timer based on SysTick
    let mut timer = cp.SYST.counter_us(&clocks);
    timer.start(42.millis()).unwrap();

    defmt::println!("hello!");
    // wait until timer expires
    nb::block!(timer.wait()).unwrap();
    defmt::println!("timer expired 1");

    // the function syst() creates a periodic timer, so it is automatically
    // restarted
    nb::block!(timer.wait()).unwrap();
    defmt::println!("timer expired 2");

    // cancel current timer
    timer.cancel().unwrap();

    // start it again
    timer.start(42.millis()).unwrap();
    nb::block!(timer.wait()).unwrap();
    defmt::println!("timer expired 3");

    timer.cancel().unwrap();
    let cancel_outcome = timer.cancel();
    assert_eq!(cancel_outcome, Err(Error::Disabled));
    defmt::println!("ehy, you cannot cancel a timer two times!");
    // this time the timer was not restarted, therefore this function should
    // wait forever
    nb::block!(timer.wait()).unwrap();
    // you should never see this print
    defmt::error!("if you see this there is something wrong");
    panic!();
}
