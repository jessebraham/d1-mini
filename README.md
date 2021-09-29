# d1-mini

[![Crates.io](https://img.shields.io/crates/v/d1-mini.svg)](https://crates.io/crates/d1-mini)
[![Docs](https://docs.rs/d1-mini/badge.svg)](https://docs.rs/d1-mini/)
[![License](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue)](LICENSE)

> A Board Support Package (BSP) which provides a type-safe API for the [WEMOS/LOLIN D1 mini](https://docs.wemos.cc/en/latest/d1/d1_mini.html).

## [Documentation]

[documentation]: https://docs.rs/d1-mini/

## Resources

- [LOLIN D1 mini official documentation](https://docs.wemos.cc/en/latest/d1/d1_mini.html)
- [esp8266-hal](https://github.com/esp-rs/esp8266-hal)
- [esp8266](https://github.com/esp-rs/esp8266)
- [xtensa-lx-rt](https://github.com/esp-rs/xtensa-lx-rt)
- [xtensa-lx](https://github.com/esp-rs/xtensa-lx)

## Getting Started

**NOTE:** This crate's dependencies are still in the early stages of development, and various features may be incomplete or missing altogether. With this being pre-`1.0` software, the public API is subject to change at any time.

### Prerequisites

Because the Xtensa target is not officially supported, you must use a custom fork of Rust. Pre-built artifacts for common operating systems are available via the [esp-rs/rust-build] repository's releases page. Alternatively, you can build the compiler from source; for more information on this process, please refer the the [esp-rs/rust] fork's README, which has detailed instructions for most popular operating systems.

You will additionally need the appropriate Xtensa toolchain. You can download pre-built binaries from [Espressif], or build them yourself using [esp-open-sdk]. In either case, make sure that the resulting binaries are in your `$PATH`.

[esp-rs/rust]: https://github.com/MabezDev/rust-xtensa
[esp-rs/rust-build]: https://github.com/MabezDev/rust-build
[espressif]: https://docs.espressif.com/projects/esp8266-rtos-sdk/en/latest/get-started/index.html#setup-toolchain
[esp-open-sdk]: https://github.com/pfalcon/esp-open-sdk

### Build the Examples

You can build all examples at once, or just build one at a time:

```bash
$ cargo build --release --examples
$ cargo build --release --example=blinky
```

### Flash the Device

### espflash

[espflash] is a community developed tool (written entirely in Rust) for flashing _ESP32-_ and _ESP8266-based_ devices. It provides a CLI tool for flashing the aforementioned devices as well as an additional crate, [cargo-espflash], which provides a cargo subcommand which compiles and flashes your device with one command.

For more information on these tools, please refer to their respective `README`s.

#### Examples

Make sure you replace `<PORT>` with the appropriate value prior to running either of the below commands.

Using `espflash`:

```bash
$ espflash <PORT> target/xtensa-esp8266-none/release/examples/blinky
```

Using `cargo-espflash`:

```bash
$ cargo espflash --release --example=blinky <PORT>
```

[espflash]: https://github.com/esp-rs/espflash
[cargo-espflash]: https://github.com/esp-rs/espflash/tree/master/cargo-espflash

### esptool

You can use the official tool from Espressif, [esptool.py], to flash the firmware to your device. Make sure this has been installed and `esptool.py` is available in your `$PATH`.

First convert the ELF-formatted binary to a flashable image:

```bash
$ esptool.py \
>   --chip esp8266 \
>   elf2image \
>   target/xtensa-esp8266-none/release/examples/blinky
```

Then you're ready to flash the image to the device; make sure you replace `<PORT>` with the appropriate value prior to running the command:

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

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in
the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without
any additional terms or conditions.
