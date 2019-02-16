# `debounced-pin`

[![Build Status](https://travis-ci.org/Winseven4lyf/rust-debounced-pin.svg)](https://travis-ci.org/Winseven4lyf/rust-debounced-pin)
[![Crate](https://img.shields.io/crates/v/debounced-pin.svg)](https://crates.io/crates/debounced-pin)
[![Docs](https://docs.rs/debounced-pin/badge.svg)](https://docs.rs/debounced-pin)

Adds a wrapper for an `InputPin` that debounces it's `is_high()` and `is_low()` methods.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
debounced-pin = "0.1.0"
```

This crate currently requires [`embedded-hal`] to be built using the `unproven` feature, for access to the `InputPin` trait.

## License

This project is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  [online](https://www.apache.org/licenses/LICENSE-2.0))
- MIT license ([LICENSE-MIT](LICENSE-MIT) or
  [online](https://opensource.org/licenses/MIT))

at your option.

[`embedded-hal`]: https://docs.rs/crate/embedded-hal/0.2.2
