# `arduino-nano33-iot-quickstart`

> A template for building applications for Arduino Nano 33 iot BOARD

## Dependencies

To build embedded programs using this template you'll need:

- Rust 1.31, 1.30-beta, nightly-2018-09-13 or a newer toolchain. e.g. `rustup
  default beta`

- The `cargo generate` subcommand. [Installation
  instructions](https://github.com/ashleygwilliams/cargo-generate#installation).

- `rust-std` components (pre-compiled `core` crate) for the ARM Cortex-M
  targets. Run:

- `arm-thumbv6m-none-eabi` target.
``` console
$ rustup target add thumbv6m-none-eabi
```

## Using this template

**NOTE**: This is the very short version that only covers building programs. For
the long version, which additionally covers flashing, running and debugging
programs, check [the embedded Rust book][book].

[book]: https://rust-embedded.github.io/book

``` console
$ cargo generate --git https://github.com/fredpit/arduino-nano33-iot-quickstart
```
