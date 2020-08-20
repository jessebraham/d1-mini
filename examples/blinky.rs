#![no_std]
#![no_main]

use panic_halt as _;
use xtensa_lx106_rt as _;

use d1_mini::{hal, target, Pins};
use hal::prelude::*;
use hal::timer::Nanoseconds;

const CORE_HZ: u32 = 80_000_000;

#[no_mangle]
fn main() -> ! {
    let peripherals = unsafe { target::Peripherals::steal() };
    let pins = Pins::new(peripherals.GPIO);

    let mut led = pins.d4.into_push_pull_output();
    led.set_high().unwrap();

    let (mut timer1, _) = peripherals.TIMER.timers(CORE_HZ);
    timer1.start(Nanoseconds(100_000_000));

    loop {
        nb::block!(timer1.wait()).unwrap();
        led.toggle().unwrap();
    }
}
