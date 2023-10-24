#![no_main]
#![no_std]

use panic_halt as _;

use at32f4xx_hal as hal;
use cortex_m_rt::entry;

use crate::hal::{pac, prelude::*};

use core::fmt::Write; // for pretty formatting of the serial output

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();

    let dp = pac::Peripherals::take().unwrap();

    let gpioa = dp.GPIOA.split();

    let crm = dp.CRM.constrain();

    let clocks = crm.cfgr.freeze();

    let mut delay = cp.SYST.delay(&clocks);

    // define RX/TX pins
    let tx_pin = gpioa.pa9;

    // configure serial
    // let mut tx = Serial::tx(dp.USART1, tx_pin, 9600.bps(), &clocks).unwrap();
    // or
    let mut tx = dp.USART1.tx(tx_pin, 9600.bps(), &clocks).unwrap();

    let mut value: u8 = 0;

    loop {
        // print some value every 500 ms, value will overflow after 255
        writeln!(tx, "value: {value:02}\r").unwrap();
        value = value.wrapping_add(1);
        delay.delay(2.secs());
    }
}
