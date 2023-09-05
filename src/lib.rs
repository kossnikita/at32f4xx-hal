#![no_std]
#![allow(non_camel_case_types)]

#[cfg(not(any(
    feature = "at32a403a",
    feature = "at32f402",
    feature = "at32f403",
    feature = "at32f403a",
    feature = "at32f405",
    feature = "at32f407",
    feature = "at32f413",
    feature = "at32f415",
    feature = "at32f421",
    feature = "at32f423",
    feature = "at32f425",
    feature = "at32f435",
    feature = "at32f437",
    feature = "at32wb415"
)))]
compile_error!(
    "This crate requires one of the following device features enabled:
        at32a403a
        at32f402
        at32f403
        at32f403a
        at32f405
        at32f407
        at32f413
        at32f415
        at32f421
        at32f423
        at32f425
        at32f435
        at32f437
        at32wb415"
);

pub use embedded_hal as hal;
pub use nb;
pub use nb::block;

#[cfg(feature = "at32a403a")]
/// Re-export of the [svd2rust](https://crates.io/crates/svd2rust) auto-generated API for the at32a403a peripherals.
pub use at32f4xx_pac::at32a403a as pac;

#[cfg(feature = "at32f402")]
/// Re-export of the [svd2rust](https://crates.io/crates/svd2rust) auto-generated API for the at32f402 peripherals.
pub use at32f4xx_pac::at32f402 as pac;

#[cfg(feature = "at32f403")]
/// Re-export of the [svd2rust](https://crates.io/crates/svd2rust) auto-generated API for the at32f403 peripherals.
pub use at32f4xx_pac::at32f403 as pac;

#[cfg(feature = "at32f403a")]
/// Re-export of the [svd2rust](https://crates.io/crates/svd2rust) auto-generated API for the at32f403a peripherals.
pub use at32f4xx_pac::at32f403a as pac;

#[cfg(feature = "at32f405")]
/// Re-export of the [svd2rust](https://crates.io/crates/svd2rust) auto-generated API for the at32f405 peripherals.
pub use at32f4xx_pac::at32f405 as pac;

#[cfg(feature = "at32f407")]
/// Re-export of the [svd2rust](https://crates.io/crates/svd2rust) auto-generated API for the at32f407 peripherals.
pub use at32f4xx_pac::at32f407 as pac;

#[cfg(feature = "at32f413")]
/// Re-export of the [svd2rust](https://crates.io/crates/svd2rust) auto-generated API for the at32f413 peripherals.
pub use at32f4xx_pac::at32f413 as pac;

#[cfg(feature = "at32f415")]
/// Re-export of the [svd2rust](https://crates.io/crates/svd2rust) auto-generated API for the at32f415 peripherals.
pub use at32f4xx_pac::at32f415 as pac;

#[cfg(feature = "at32f421")]
/// Re-export of the [svd2rust](https://crates.io/crates/svd2rust) auto-generated API for the at32f421 peripherals.
pub use at32f4xx_pac::at32f421 as pac;

#[cfg(feature = "at32f423")]
/// Re-export of the [svd2rust](https://crates.io/crates/svd2rust) auto-generated API for the at32f423 peripherals.
pub use at32f4xx_pac::at32f423 as pac;

#[cfg(feature = "at32f425")]
/// Re-export of the [svd2rust](https://crates.io/crates/svd2rust) auto-generated API for the at32f425 peripherals.
pub use at32f4xx_pac::at32f425 as pac;

#[cfg(feature = "at32f435")]
/// Re-export of the [svd2rust](https://crates.io/crates/svd2rust) auto-generated API for the at32f435 peripherals.
pub use at32f4xx_pac::at32f435 as pac;

#[cfg(feature = "at32f437")]
/// Re-export of the [svd2rust](https://crates.io/crates/svd2rust) auto-generated API for the at32f437 peripherals.
pub use at32f4xx_pac::at32f437 as pac;

#[cfg(feature = "at32wb415")]
/// Re-export of the [svd2rust](https://crates.io/crates/svd2rust) auto-generated API for the at32wb415 peripherals.
pub use at32f4xx_pac::at32wb415 as pac;

// Enable use of interrupt macro
pub use crate::pac::interrupt;

pub mod bb;
pub mod crm;
pub mod flash;
pub mod gpio;
pub mod prelude;
pub mod signature;
pub mod timer;

mod sealed {
    pub trait Sealed {}
}
pub(crate) use sealed::Sealed;

fn stripped_type_name<T>() -> &'static str {
    let s = core::any::type_name::<T>();
    let p = s.split("::");
    p.last().unwrap()
}
