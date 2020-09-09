#![no_std]
#![no_main]

use panic_halt as _;

use core::fmt::Write;
use d1_mini::{hal, target, Pins};
use hal::prelude::*;
use xtensa_lx106_rt::entry;

#[entry]
fn main() -> ! {
    let peripherals = unsafe { target::Peripherals::steal() };
    let pins = Pins::new(peripherals.GPIO);

    let (mut timer1, _) = peripherals.TIMER.timers();

    // UART0 (txd, rxd) - read/write
    // NOTE: also connected to the USB port via a CH340
    let mut uart0 = peripherals.UART0.serial(pins.tx, pins.rx);

    // UART1 (txd) - write-only
    let txd = pins.d4.into_uart();
    let mut uart1 = peripherals.UART1.serial(txd);

    loop {
        uart0.write_str("foo bar baz\r\n").unwrap();
        uart1.write_str("spam ham eggs\r\n").unwrap();

        timer1.delay_ms(1_000);
    }
}
