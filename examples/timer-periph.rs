//! Start and stop a periodic peripheral timer.

#![no_std]
#![no_main]

use panic_halt as _;

use at32f4xx_hal as hal;
use cortex_m_rt::entry;
use hal::timer::Error;

use crate::hal::{pac, prelude::*};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let crm = dp.CRM.constrain();
    let clocks = crm.cfgr.sysclk(24.MHz()).freeze();

    // Create a timer based on SysTick
    let mut timer = dp.TIM1.counter_ms(&clocks);
    timer.start(1.secs()).unwrap();

    hprintln!("hello!");
    // wait until timer expires
    nb::block!(timer.wait()).unwrap();
    hprintln!("timer expired 1");

    // the function counter_ms() creates a periodic timer, so it is automatically
    // restarted
    nb::block!(timer.wait()).unwrap();
    hprintln!("timer expired 2");

    // cancel current timer
    timer.cancel().unwrap();

    // start it again
    timer.start(1.secs()).unwrap();
    nb::block!(timer.wait()).unwrap();
    hprintln!("timer expired 3");

    timer.cancel().unwrap();
    let cancel_outcome = timer.cancel();
    assert_eq!(cancel_outcome, Err(Error::Disabled));
    hprintln!("ehy, you cannot cancel a timer two times!");
    // this time the timer was not restarted, therefore this function should
    // wait forever
    nb::block!(timer.wait()).unwrap();
    // you should never see this print
    hprintln!("if you see this there is something wrong");
    panic!();
}
