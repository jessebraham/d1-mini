#![no_std]
#![no_main]

use core::fmt::Write;

use panic_halt as _;
use xtensa_lx106_rt as _;

use d1_mini::{hal, target, Pins};
use hal::prelude::*;

macro_rules! uprint {
    ($serial:expr, $($arg:tt)*) => {
        $serial.write_fmt(format_args!($($arg)*))
    };
}

macro_rules! uprintln {
    ($serial:expr, $fmt:expr) => {
        uprint!($serial, concat!($fmt, "\r\n"))
    };
    ($serial:expr, $fmt:expr, $($arg:tt)*) => {
        uprint!($serial, concat!($fmt, "\r\n"), $($arg)*)
    };
}

#[no_mangle]
fn main() -> ! {
    let peripherals = unsafe { target::Peripherals::steal() };
    let pins = Pins::new(peripherals.GPIO);

    let (mut timer1, _) = peripherals.TIMER.timers();
    timer1.start(1u32.s());

    // UART0 (txd, rxd) - read/write
    let mut uart0 = peripherals.UART0.serial(pins.tx, pins.rx);

    // UART1 (txd) - write-only
    let txd = pins.d4.into_uart();
    let mut uart1 = peripherals.UART1.serial(txd);

    loop {
        uprintln!(uart0, "foo bar baz").unwrap();
        uprintln!(uart1, "spam ham eggs").unwrap();

        nb::block!(timer1.wait()).unwrap();
    }
}
