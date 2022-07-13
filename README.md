<div align="center"> <img src="https://raw.githubusercontent.com/giyomoon/astrolabe/main/assets/logo.svg" width=200 /></div>
<h1 align="center">Astrolabe</h1>
<div align="center">
 <strong>
  Date and time library for Rust. Feature rich, lightweight and easy-to-use.
 </strong>
</div>

<br />

<div align="center">
  <!-- Downloads -->
  <a href="https://crates.io/crates/astrolabe" target="_blank">
    <img src="https://img.shields.io/crates/d/astrolabe.svg?style=flat"
      alt="Download" />
  </a>
  <!-- Version -->
  <a href="https://crates.io/crates/astrolabe" target="_blank">
    <img src="https://img.shields.io/crates/v/astrolabe.svg?style=flat"
    alt="Crates.io version" />
  </a>
  <!-- MSRV -->
  <a href="https://github.com/rust-lang/rust/releases/tag/1.56.1" target="_blank">
    <img src="https://img.shields.io/badge/MSRV-1.56.1-fa6733.svg?style=flat"
    alt="MSRV" />
  </a>
  <!-- Github Actions -->
  <a href = "https://github.com/giyomoon/astrolabe/actions">
    <img src="https://img.shields.io/github/workflow/status/giyomoon/astrolabe/checks/main?style=flat" alt="actions status" />
  </a>
  <!-- Dependencies -->
  <a href="https://deps.rs/repo/github/giyomoon/astrolabe" target="_blank">
    <img src="https://deps.rs/repo/github/giyomoon/astrolabe/status.svg?style=flat"
    alt="Crates.io version" />
  </a>
  <br/>
  <!-- License -->
  <a href="https://github.com/giyomoon/astrolabe#License" target="_blank">
    <img src="https://img.shields.io/badge/License-APACHE--2.0%2FMIT-blue?style=flat" alt="License">
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
  <h4>
    <a href="#example">
      Example
    </a>
  </h4>
</div>

## Status
Astrolabe is currently in **heavy development**. Please do not use the crate in production yet.

## Overview

Astrolabe is a date and time library for Rust which aims to be feature rich, lightweight (zero dependencies) and easy-to-use.

### Features
At this stage of development, not all features have been fully implemented.
* [`DateTime`](https://docs.rs/astrolabe/latest/astrolabe/struct.DateTime.html), a wrapper around [`std::time::SystemTime`](https://doc.rust-lang.org/std/time/struct.SystemTime.html) which implements formatting and manipulation functions
* **Formatting**
  * RFC3339 timestamp
  * Formatting with format strings based on [Unicode Date Field Symbols](https://www.unicode.org/reports/tr35/tr35-dates.html#Date_Field_Symbol_Table). (Which allows formatting `SystemTime` into basically any string format)
* **Manipulation**
  * Manipulation functions like [`DateTime::add`](https://docs.rs/astrolabe/latest/astrolabe/struct.DateTime.html#method.add) or [`DateTime::sub`](https://docs.rs/astrolabe/latest/astrolabe/struct.DateTime.html#method.sub) to create a new, modified `DateTime` struct
* **Timezone**
  * Specify a timezone offset which will be applied in any format function
* Zero dependencies

## Example

A basic example which demonstrates creating, formatting and manipulating a `DateTime` instance.

```rust
use astrolabe::{DateTime, Precision, Unit};

// Create a DateTime instance from year, month, and days (day of month)
let date_time = DateTime::from_ymd(2022, 5, 2).unwrap();

// Use the format function to freely format your `DateTime` instance
assert_eq!("2022/05/02", date_time.format("yyyy/MM/dd").unwrap());

// Create a new instance with a modified `DateTime`
// The previous instance is not modified and is still in scope
let modified_dt = date_time.add(11, Unit::Hour).add(23, Unit::Min);

assert_eq!("2022/05/02 11:23:00", modified_dt.format("yyyy/MM/dd HH:mm:ss").unwrap());
assert_eq!("2022-05-02T11:23:00Z", modified_dt.format_rfc3339(Precision::Seconds));
```
To see all implementations for the `DateTime` struct, check out it's [documentation](https://docs.rs/astrolabe/latest/astrolabe/struct.DateTime.html).

## MSRV
This crate uses the Rust 2021 Edition and requires at least version `1.56.1`.

## License
Licensed under either of
 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
