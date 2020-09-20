//! A Board Support Package (BSP) which provides a type-safe API for the
//! [WEMOS/LOLIN D1 mini].
//!
//! [WEMOS/LOLIN D1 mini]: https://docs.wemos.cc/en/latest/d1/d1_mini.html

#![no_std]

mod pins;

pub use esp8266_hal::{self as hal, target};
pub use pins::Pins;

use bitbang_hal::i2c::I2cBB;
use esp8266_hal::ehal::timer::{CountDown, Periodic};
use esp8266_hal::gpio::*;
use esp8266_hal::prelude::*;
use esp8266_hal::spi::{SPI1Master, SpiClock};
use esp8266_hal::target::SPI1;

/// Convenience function for setting up the D1/D2 pins to operate as I²C SCL/SDA
/// respectively.
///
/// Since the ESP8266 does not have any I²C hardware, we accomplish this using
/// [bitbang-hal]. The [official documentation] lists D1/D2 as SCL/SDA
/// respectively, so by default we will use these pins, but you can use
/// whichever you like in practice.
///
/// [bitbang-hal]: https://github.com/sajattack/bitbang-hal
/// [official documentation]: https://docs.wemos.cc/en/latest/d1/d1_mini.html#pin
pub fn i2c_master<TIM>(
    d1: Gpio5<Input<Floating>>,
    d2: Gpio4<Input<Floating>>,
    timer: TIM,
) -> I2cBB<Gpio5<Output<OpenDrain>>, Gpio4<Output<OpenDrain>>, TIM>
where
    TIM: CountDown + Periodic,
{
    let scl = d1.into_open_drain_output();
    let sda = d2.into_open_drain_output();

    I2cBB::new(scl, sda, timer)
}

/// Convenience function for setting up the D5/D6/D7 pins to operate as SPI
/// SCK/MISO/MOSI respectively, running at the provided frequency.
pub fn spi_master(
    spi1: SPI1,
    d5: Gpio14<UnInitialized>,
    d6: Gpio12<UnInitialized>,
    d7: Gpio13<UART>,
    frequency: SpiClock,
) -> SPI1Master {
    let sck = d5.into_hspi();
    let miso = d6.into_hspi();
    let mosi = d7.into_hspi();

    spi1.spi(sck, miso, mosi, frequency)
}
