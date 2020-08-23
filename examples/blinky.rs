#![no_std]
#![no_main]

use panic_halt as _;
use xtensa_lx106_rt as _;

use d1_mini::{hal, target, Pins};
use hal::prelude::*;

#[no_mangle]
fn main() -> ! {
    let peripherals = unsafe { target::Peripherals::steal() };
    let pins = Pins::new(peripherals.GPIO);

    let mut led = pins.d4.into_push_pull_output();
    led.set_high().unwrap();

    let (mut timer1, _) = peripherals.TIMER.timers(80u32.mhz());
    timer1.start(100u32.ms());

    loop {
        nb::block!(timer1.wait()).unwrap();
        led.toggle().unwrap();
    }
}
