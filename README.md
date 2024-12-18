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
  <a href="https://github.com/rust-lang/rust/releases/tag/1.60.0" target="_blank">
    <img src="https://img.shields.io/badge/MSRV-1.60-fa6733.svg?style=flat"
    alt="MSRV" />
  </a>
  <!-- Github Actions -->
  <a href="https://github.com/giyomoon/astrolabe/actions">
    <img src="https://img.shields.io/github/actions/workflow/status/giyomoon/astrolabe/checks.yml?branch=main&style=flat" alt="Actions Status" />
  </a>
  <!-- Code coverage -->
  <a href="https://app.codecov.io/gh/GiyoMoon/astrolabe">
    <img src="https://img.shields.io/codecov/c/gh/giyomoon/astrolabe?style=flat" alt="Code Coverage" />
  </a>
  <!-- Dependencies -->
  <a href="https://deps.rs/repo/github/giyomoon/astrolabe" target="_blank">
    <img src="https://deps.rs/repo/github/giyomoon/astrolabe/status.svg?style=flat"
    alt="Dependencies" />
  </a>
  <!-- License -->
  <a href="https://github.com/giyomoon/astrolabe#License" target="_blank">
    <img src="https://img.shields.io/crates/l/astrolabe?style=flat" alt="License">
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
    <span> | </span>
    <a href="#examples">
      Examples
    </a>
  </h4>
</div>

## Overview
Astrolabe is a date and time library for Rust which aims to be feature rich, lightweight (zero dependencies) and easy-to-use. It implements formatting, parsing and manipulating functions for date and time values.

### Features
- **Formatting** and **parsing** with format strings based on [Unicode Date Field Symbols](https://www.unicode.org/reports/tr35/tr35-dates.html#Date_Field_Symbol_Table)
- **RFC 3339** timestamp parsing and formatting
- **Manipulation** functions to add, subtract, set and clear date units
- **Cron** expression parser
- **Timezone** offset
- **Local** timezone on UNIX platforms
- **Zero** dependencies
- **Serde** serializing and deserializing (With feature flag `serde`)
- **sqlx** postgres encoding and decoding (With feature flag `sqlx-postgres`)

## Examples
### Basic
A basic example which demonstrates creating, formatting and manipulating a `DateTime` instance.

```rust
use astrolabe::{DateTime, TimeUtilities, Precision};

// Create a DateTime instance from year, month, and days (day of month)
let date_time = DateTime::from_ymd(2022, 5, 2).unwrap();

// Use the format function to freely format your DateTime instance
assert_eq!("2022/05/02", date_time.format("yyyy/MM/dd"));

// Create a new instance with a modified DateTime
// The previous instance is not modified and is still in scope
let modified_dt = date_time.add_hours(11).add_minutes(23);

assert_eq!("2022/05/02 11:23:00", modified_dt.format("yyyy/MM/dd HH:mm:ss"));
assert_eq!("2022-05-02T11:23:00Z", modified_dt.format_rfc3339(Precision::Seconds));
```
To see all implementations for the `DateTime` struct, check out it's [documentation](https://docs.rs/astrolabe/latest/astrolabe/struct.DateTime.html).

### Local timezone (UNIX systems only)
Astrolabe can parse the timezone from `/etc/localtime` to get the local UTC offset. This only works on UNIX systems.

```rust
use astrolabe::{DateTime, Offset, OffsetUtilities, Precision};

// Equivalent to `DateTime::now().set_offset(Offset::Local)`
let now = DateTime::now_local();

// Prints for example:
// 2023-10-08T08:30:00+02:00
println!("{}", now.format_rfc3339(Precision::Seconds));
assert_eq!(Offset::Local, now.get_offset());
```
See [`Offset`](https://docs.rs/astrolabe/latest/astrolabe/enum.Offset.html)

### CRON parsing
```rust
use astrolabe::CronSchedule;

// Every 5 minutes
let schedule = CronSchedule::parse("*/5 * * * *").unwrap();
for date in schedule.take(3) {
   println!("{}", date);
}
// Prints for example:
// 2022-05-02 16:15:00
// 2022-05-02 16:20:00
// 2022-05-02 16:25:00

// Every weekday at 10:00
let schedule = CronSchedule::parse("0 10 * * Mon-Fri").unwrap();
for date in schedule.take(3) {
   println!("{}", date.format("yyyy-MM-dd HH:mm:ss eeee"));
}
// Prints for example:
// 2022-05-03 10:00:00 Tuesday
// 2022-05-04 10:00:00 Wednesday
// 2022-05-05 10:00:00 Thursday
```
See [`CronSchedule`](https://docs.rs/astrolabe/latest/astrolabe/struct.CronSchedule.html)

## MSRV
This crate uses the Rust 2021 Edition and requires at least version `1.60`. When using features that require third-party crates (`serde` or `sqlx`), please refer to their MSRV.

## License
Licensed under either of
 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
