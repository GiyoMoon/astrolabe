<h1 align="center">Astrolabe</h1>
<div align="center">
 <strong>
  Date and time library for Rust. Feature rich and easy-to-use.
 </strong>
</div>

<br />

<div align="center">
  <!-- Downloads -->
  <a href="https://crates.io/crates/astrolabe" target="_blank">
    <img src="https://img.shields.io/crates/d/astrolabe.svg?style=flat-square"
      alt="Download" />
  </a>
  <!-- Version -->
  <a href="https://crates.io/crates/astrolabe" target="_blank">
    <img src="https://img.shields.io/crates/v/astrolabe.svg?style=flat-square"
    alt="Crates.io version" />
  </a>
  <!-- Github Actions -->
  <a href = "https://github.com/giyomoon/astrolabe/actions">
    <img src="https://img.shields.io/github/workflow/status/giyomoon/astrolabe/checks/main?style=flat-square" alt="actions status" />
  </a>
  <!-- Dependencies -->
  <a href="https://deps.rs/repo/github/giyomoon/astrolabe" target="_blank">
    <img src="https://deps.rs/repo/github/giyomoon/astrolabe/status.svg?style=flat-square"
    alt="Crates.io version" />
  </a>
  <br/>
  <!-- License -->
  <a href="https://github.com/giyomoon/astrolabe#License" target="_blank">
    <img src="https://img.shields.io/badge/License-APACHE--2.0%2FMIT-blue?style=flat-square" alt="License">
  </a>
</div>

<div align="center">
  <h4>
    <a href="https://docs.rs/astrolabe" target="_blank">
      Documentation
    </a>
    <span> | </span>
    <a href="https://github.com/giyomoon/astrolabe" target="_blank">
      Github
    </a>
    <span> | </span>
    <a href="https://crates.io/crates/astrolabe" target="_blank">
      Crate
    </a>
  </h4>
</div>

<br />

Astrolabe is a date and time library for Rust which aims to be feature rich and easy-to-use.

## Status
Astrolabe is currently in **heavy development**. Please do not use the crate in production yet.
#### Roadmap:
* [x] Create new `DateTime` based on current time or specific dates
* [ ] (In progress) Formatting with specified format strings
* [ ] Formatting as RFC3339 timestamp
* [ ] Manipulation of `DateTime` with `add()` and `sub()` functions

## Features
> At this stage of development, not all functions have been fully implemented.
* `DateTime`, a wrapper around `std::time::SystemTime` which implements formatting and manipulation functions
* Formatting as RFC3339 timestamp
* Formatting with specific format strings based on [Unicode Date Field Symbols](https://www.unicode.org/reports/tr35/tr35-dates.html#Date_Field_Symbol_Table).
* Zero dependencies if used without the `format` feature

## MSRV
This crate uses Rust 2021 edition and requires at least version `1.60`.

## License
Licensed under either of
 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
at your option.

### Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
