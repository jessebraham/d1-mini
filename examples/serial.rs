#![no_std]
#![no_main]

use core::fmt::Write;

use panic_halt as _;

use d1_mini::{hal, target, Pins};
use hal::prelude::*;

#[entry]
fn main() -> ! {
    let peripherals = target::Peripherals::take().unwrap();
    let pins = Pins::new(peripherals.GPIO);

    let (mut timer1, _) = peripherals.TIMER.timers();

    // UART0 (tx, rx) - read/write
    // NOTE: also connected to the USB port via a CH340
    let mut uart0 = peripherals.UART0.serial(pins.tx, pins.rx);

    // UART1 (tx) - write-only
    let tx = pins.d4.into_uart();
    let mut uart1 = peripherals.UART1.serial(tx);

    loop {
        uart0.write_str("foo bar baz\r\n").unwrap();
        uart1.write_str("spam ham eggs\r\n").unwrap();

        timer1.delay_ms(1_000);
    }
}
