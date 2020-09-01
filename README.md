# d1-mini

[![Crates.io](https://img.shields.io/crates/v/d1-mini.svg)](https://crates.io/crates/d1-mini)
[![Docs](https://docs.rs/d1-mini/badge.svg)](https://docs.rs/d1-mini/)
![License](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue)

> A Board Support Package (BSP) which provides a type-safe API for the [WEMOS/LOLIN D1 mini](https://docs.wemos.cc/en/latest/d1/d1_mini.html).

## [Documentation]

[Documentation]: https://docs.rs/d1-mini/

## Getting Started

In order to target the Xtensa ISA, you must first build a custom version of Rust. For more information on this process, please refer the the [rust-xtensa](https://github.com/MabezDev/rust-xtensa) project's README. I've also create a [Dockerfile](https://github.com/jessebraham/docker-rust-esp) which handles setting up the environment for you, if you'd prefer that approach.

You will need to use [xargo](https://github.com/japaric/xargo) or [cargo-xbuild](https://github.com/rust-osdev/cargo-xbuild) to cross-compile your projects.

### Build the Examples

You can build all examples at once, or just build one at a time:

```bash
$ xargo build --release --examples
$ xargo build --release --example=blinky
```

### Flash the Device

You can use [esptool](https://github.com/espressif/esptool) to flash the firmware to your device. Make sure this has been installed and `esptool.py` is available on your `PATH`.

First convert the ELF-formatted binary to a flashable image:

```bash
$ esptool.py --chip esp8266 elf2image target/xtensa-esp8266-none/release/examples/blinky
```

Then you're ready to flash the image to the device; make sure you replace `<PORT>` with the appropriate value prior to running the command.

```bash
$ esptool.py \
>     --port <PORT> \
>     write_flash \
>     -fm dio \
>     -fs 32m \
>     0x00000 \
>     target/xtensa-esp8266-none/release/examples/blinky-0x00000.bin
```

## License

Licensed under either of:

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in
the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without
any additional terms or conditions.
