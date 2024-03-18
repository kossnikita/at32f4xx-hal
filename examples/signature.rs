//! Read MCU PID, name and flash size from signature

#![no_std]
#![no_main]

use cortex_m_rt::entry;
use defmt_rtt as _;
use panic_probe as _;

use at32f4xx_hal::signature::{FlashSize, IDCode};

#[entry]
fn main() -> ! {
    let id_code = IDCode::get();
    let pid = id_code.pid();
    let mcu = id_code.mcu();
    let flash_size = FlashSize::from_pid(pid).kilo_bytes();
    defmt::info!("MCU PID: 0x{:X}", pid);
    defmt::info!("MCU part number: {}", mcu);
    defmt::info!("Flash size: {} kb", flash_size);
    loop {}
}
