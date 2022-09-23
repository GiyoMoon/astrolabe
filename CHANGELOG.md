# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [0.2.1] - Unreleased
### Added
- Impl `Ord` and `PartialOrd` for `DateTime`, `Date` and `Time`
- `Date::parse`

## [0.2.0] - 2022-08-25
### Added
- **Structs**
  - [`Date`](https://docs.rs/astrolabe/0.2.0/astrolabe/struct.Date.html)
  - [`Time`](https://docs.rs/astrolabe/0.2.0/astrolabe/struct.Time.html)
- **Enums**
  - [`DateUnit`](https://docs.rs/astrolabe/0.2.0/astrolabe/enum.DateUnit.html)
  - [`TimeUnit`](https://docs.rs/astrolabe/0.2.0/astrolabe/enum.TimeUnit.html)
- `DateTime::as_days`
- `DateTime::as_nanoseconds`
- `DateTime::as_seconds`
- `DateTime::date`
- `DateTime::from_days`
- `DateTime::from_hms`
- `DateTime::from_nanoseconds`
- `DateTime::from_seconds`
- `DateTime::set_time`
- `DateTime::time`

### Changed
- Renamed `Unit` to `DateTimeUnit`
- More detailed error messages. See [errors module](https://docs.rs/astrolabe/0.2.0/astrolabe/errors/index.html).
- Combined `DateTime::add` and `DateTime::sub` to `DateTime::apply`
- `DateTime::as_offset` now accepts `i32` (was `i64`)
- `DateTime::as_offset_time` now accepts `u32` (was `u64`)
- `DateTime::between` now returns `u64` (was `Duration`)
- `DateTime::format` now returns `String` (was `Result`)
- `DateTime::from_timestamp` now accepts `i64` (was `u64`)
- `DateTime::from_ymd` now accepts `i32` and `u32` (was `i64` and `u64`)
- `DateTime::from_ymdhms` now accepts `i32` and `u32` (was `i64` and `u64`)
- `DateTime::get` now returns `i64` (was `u64`)
- `DateTime::get_offset` now returns `i32` (was `i64`)
- `DateTime::set` now accepts `i32` (was `u64`)
- `DateTime::set_offset` now accepts `i32` (was `i64`)
- `DateTime::set_offset_time` now accepts `u32` (was `u64`)
- `DateTime::timestamp` now returns `i64` (was `u64`)

### Removed
- `DateTime::add_dur`
- `DateTime::sub_dur`
- `DateTime::duration`