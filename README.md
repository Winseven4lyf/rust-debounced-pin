# `debounced-pin`

[![Build Status](https://travis-ci.org/Winseven4lyf/rust-debounced-pin.svg)](https://travis-ci.org/Winseven4lyf/rust-debounced-pin)
[![Crate](https://img.shields.io/crates/v/debounced-pin.svg)](https://crates.io/crates/debounced-pin)
[![Docs](https://docs.rs/debounced-pin/badge.svg)](https://docs.rs/debounced-pin)

A platform-agnostic debounce library.

This library provides an `update()` method to debounce a pin.

Implements approach 1 from [here](http://www.labbookpages.co.uk/electronics/debounce.html#soft)
([archived 2018-09-03](https://web.archive.org/web/20180903142143/http://www.labbookpages.co.uk/electronics/debounce.html#soft)).

It also adds a wrapper for an `InputPin` that debounces it's
`is_high()` and `is_low()` methods.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
debounced-pin = "0.3.0"
```

This crate currently requires [`embedded-hal`] to be built using the
`unproven` feature, for access to the `InputPin` trait.

## License

This project is licensed under either of

- Apache License, Version 2.0 ([`LICENSE-APACHE`](LICENSE-APACHE) or
  [online](https://www.apache.org/licenses/LICENSE-2.0))
- MIT License ([`LICENSE-MIT`](LICENSE-MIT) or
  [online](https://opensource.org/licenses/MIT))

at your option.

[`embedded-hal`]: https://docs.rs/crate/embedded-hal/0.2.3
