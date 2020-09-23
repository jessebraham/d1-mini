#![no_std]
#![no_main]

use core::fmt::Write;

use panic_halt as _;

use d1_mini::{hal, i2c_master, target, Pins};
use hal::prelude::*;
use lis3dh::accelerometer::{RawAccelerometer, Tracker};
use lis3dh::{Lis3dh, Range, SlaveAddr};

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

#[entry]
fn main() -> ! {
    let peripherals = target::Peripherals::take().unwrap();
    let pins = Pins::new(peripherals.GPIO);

    let (timer1, mut timer2) = peripherals.TIMER.timers();
    let mut uart0 = peripherals.UART0.serial(pins.tx, pins.rx);

    // The `i2c_master` helper function assumes that d1 and d2 are being used for
    // SCL and SDA respectively. If you would like to use other GPIOs instead, the
    // `I2cBB` struct from bitbang-hal has been re-exported as well.
    let i2c = i2c_master(pins.d1, pins.d2, timer1);
    let mut lis3dh = Lis3dh::new(i2c, SlaveAddr::Default).unwrap();
    lis3dh.set_range(Range::G8).unwrap();

    // The value passed into Tracker's `new` function was obtained experimentally.
    let mut tracker = Tracker::new(3700.0);

    loop {
        let raw = lis3dh.accel_raw().unwrap();
        let orientation = tracker.update(raw);

        uprintln!(
            uart0,
            "Orientation: {:?}  Raw: ({:?}, {:?}, {:?})",
            orientation,
            raw.x,
            raw.y,
            raw.z
        )
        .unwrap();

        timer2.delay_ms(100);
    }
}
