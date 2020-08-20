//! A Board Support Package (BSP) which provides a type-safe API for the [WEMOS/LOLIN D1 mini].
//!
//! [WEMOS/LOLIN D1 mini]: https://docs.wemos.cc/en/latest/d1/d1_mini.html
//!

#![no_std]

pub use esp8266;
pub use esp8266_hal;

mod pins;
pub use pins::Pins;
