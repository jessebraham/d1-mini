# d1-mini

[![Crates.io](https://img.shields.io/crates/v/d1-mini.svg)](https://crates.io/crates/d1-mini)
[![Docs](https://docs.rs/d1-mini/badge.svg)](https://docs.rs/d1-mini/)
[![License](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue)](LICENSE)

> A Board Support Package (BSP) which provides a type-safe API for the [WEMOS/LOLIN D1 mini](https://docs.wemos.cc/en/latest/d1/d1_mini.html).

## [Documentation]

[Documentation]: https://docs.rs/d1-mini/

## Resources

* [LOLIN D1 mini official documentation](https://docs.wemos.cc/en/latest/d1/d1_mini.html)
* [esp8266-hal](https://github.com/esp-rs/esp8266-hal)
* [esp8266](https://github.com/esp-rs/esp8266)
* [xtensa-lx106-rt](https://github.com/icewind1991/xtensa-lx106-rt)

## Getting Started

Because the Xtensa target is not yet officially supported, you must build a custom version of Rust. For more information on this process, please refer the the [rust-xtensa] project's README. I've create a [Dockerfile] which handles setting up the environment for you, if you'd prefer that approach.

You will also need either [xargo] or [cargo-xbuild] to cross-compile your projects.

[rust-xtensa]: https://github.com/MabezDev/rust-xtensa
[Dockerfile]: https://github.com/jessebraham/docker-rust-esp
[xargo]: https://github.com/japaric/xargo
[cargo-xbuild]: https://github.com/rust-osdev/cargo-xbuild

### Build the Examples

To build the examples, you will need the Xtensa toolchain. You can download pre-built binaries from [Espressif][espressifdl], or build them yourself using [esp-open-sdk][espopensdk]. In either case, make sure that the resulting binaries are in your `$PATH`.

You can then build all examples at once, or just build one at a time:

```bash
$ xargo build --release --examples
$ xargo build --release --example=blinky
```

[espressifdl]: https://docs.espressif.com/projects/esp8266-rtos-sdk/en/latest/get-started/index.html#setup-toolchain
[espopensdk]: https://github.com/pfalcon/esp-open-sdk

### Flash the Device

### espflash

[espflash] is a community developed tool (written entirely in Rust) for flashing _ESP32-_ and _ESP8266-based_ devices. It provides a CLI tool for flashing the aforementioned devices as well as an additional crate, [cargo-espflash], which provides a cargo subcommand which compiles and flashes your device with one command.

Using `espflash`:

```bash
$ espflash [--ram] <path to serial> <path to elf image>
```

Using `cargo-espflash`:

```bash
$ cargo espflash [--ram] [--release] [--example EXAMPLE] --chip {esp32,esp8266} <serial>
```

[espflash]: https://github.com/icewind1991/espflash
[cargo-espflash]: https://github.com/icewind1991/espflash/tree/master/cargo-espflash

### esptool

You can use the official tool from Espressif, [esptool.py], to flash the firmware to your device. Make sure this has been installed and `esptool.py` is available in your `$PATH`.

First convert the ELF-formatted binary to a flashable image:

```bash
$ esptool.py \
>   --chip esp8266 \
>   elf2image \
>   target/xtensa-esp8266-none/release/examples/blinky
```

Then you're ready to flash the image to the device; make sure you replace `<PORT>` with the appropriate value prior to running the command.

```bash
$ esptool.py \
>   --port <PORT> \
>   write_flash \
>   -fm dio \
>   -fs 32m \
>   0x00000 \
>   target/xtensa-esp8266-none/release/examples/blinky-0x00000.bin
```

[esptool.py]: https://github.com/espressif/esptool

## License

Licensed under either of:

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in
the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without
any additional terms or conditions.
