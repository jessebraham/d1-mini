pub use bitbang_hal::i2c::I2cBB;

use esp8266_hal::ehal::timer::{CountDown, Periodic};
use esp8266_hal::gpio::*;

pub fn i2c_master<TIM>(
    d1: Gpio5<Input<Floating>>,
    d2: Gpio4<Input<Floating>>,
    timer: TIM,
) -> I2cBB<Gpio5<Output<OpenDrain>>, Gpio4<Output<OpenDrain>>, TIM>
where
    TIM: CountDown + Periodic,
{
    // The official documentation for the board labels d1 and d2 as SCL and SDA
    // respectively, despite the fact the ESP8266 has no actual I2C hardware; we'll
    // use these pins regardless.
    let scl = d1.into_open_drain_output();
    let sda = d2.into_open_drain_output();

    I2cBB::new(scl, sda, timer)
}
