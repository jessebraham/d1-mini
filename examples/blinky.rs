#![no_std]
#![no_main]

use panic_halt as _;

use d1_mini::{hal, target, Pins};
use hal::prelude::*;
use xtensa_lx106_rt::entry;

#[entry]
fn main() -> ! {
    let peripherals = unsafe { target::Peripherals::steal() };
    let pins = Pins::new(peripherals.GPIO);

    let (mut timer1, _) = peripherals.TIMER.timers();

    let mut led = pins.d4.into_push_pull_output();
    led.set_high().unwrap();

    loop {
        timer1.delay_ms(100);
        led.toggle().unwrap();
    }
}
