#![no_main]
#![no_std]

use defmt_rtt as _;
use panic_probe as _;

use at32f4xx_hal as hal;
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::{entry, exception};

use crate::hal::{pac, prelude::*, uart::Serial};

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();

    let dp = pac::Peripherals::take().unwrap();

    let gpioa = dp.GPIOA.split();

    let crm = dp.CRM.constrain();

    let clocks = crm.cfgr.freeze();

    let mut syst = cp.SYST;

    // configures the system timer to trigger a SysTick exception every second
    syst.set_clock_source(SystClkSource::Core);
    syst.set_reload(8_000_000); // period = 1s
    syst.enable_counter();
    syst.enable_interrupt();

    // define RX/TX pins
    let rx_pin = gpioa.pa10;
    let tx_pin = gpioa.pa9;

    // configure serial
    // let mut tx = Serial::tx(dp.USART1, tx_pin, 9600.bps(), &clocks).unwrap();
    // or
    let serial: Serial<pac::USART1> = dp
        .USART1
        .serial((tx_pin, rx_pin), 9600.bps(), &clocks)
        .unwrap();

    let (mut tx, mut rx) = serial.split();

    let mut value: u8 = 0;
    defmt::println!("Init\r");
    loop {}
}

#[exception]
fn SysTick() {
    defmt::println!("SysTick\r");
}
