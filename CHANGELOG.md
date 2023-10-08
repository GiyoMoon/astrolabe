# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [0.5.0] - 2023-10-08
Astrolabe can now get the local timezone offset on UNIX systems.
```rust
use astrolabe::{DateTime, Offset, OffsetUtilities, Precision};

// Equivalent to `DateTime::now().set_offset(Offset::Local)`
let now = DateTime::now_local();
// Prints for example:
// 2023-10-08T08:30:00+02:00
println!("{}", now.format_rfc3339(Precision::Seconds));
assert_eq!(Offset::Local, now.get_offset());
```

Also, we decided to panic in case of a date overflow. This should not happen if the library is used correctly, as the valid date range is between `30. June -5879611`..=`12. July 5879611`. This change makes the API much easier to use, as many functions only returned a `Result` because of this edge case.

### Added
- Enums
  - [`Offset`](https://docs.rs/astrolabe/0.5.0/astrolabe/enum.Offset.html) - Represents an offset from UTC. `Fixed` or `Local`

### Changed
- Updated the [`OffsetUtilities`](https://docs.rs/astrolabe/0.5.0/astrolabe/trait.OffsetUtilities.html) trait to use the [`Offset`](https://docs.rs/astrolabe/0.5.0/astrolabe/enum.Offset.html) enum
- The `add_*` and `sub_*` functions from the [`DateUtilities`](https://docs.rs/astrolabe/0.5.0/astrolabe/trait.DateUtilities.html) and [`TimeUtilities`](https://docs.rs/astrolabe/0.5.0/astrolabe/trait.TimeUtilities.html) traits now return `Self` instead of `Result`
- `from_timestamp` from the [`DateUtilities`](https://docs.rs/astrolabe/0.5.0/astrolabe/trait.DateUtilities.html) trait now return `Self` instead of `Result`

## [0.4.0] - 2023-05-18
### Added
- Structs
  - [`CronSchedule`](https://docs.rs/astrolabe/0.4.0/astrolabe/struct.CronSchedule.html)
- Traits
  - [`DateUtilities`](https://docs.rs/astrolabe/0.4.0/astrolabe/trait.DateUtilities.html) and [`TimeUtilities`](https://docs.rs/astrolabe/0.4.0/astrolabe/trait.TimeUtilities.html)
    - Defines get and manipulation functions for date and time units functions.
  - [`OffsetUtilities`](https://docs.rs/astrolabe/0.4.0/astrolabe/trait.OffsetUtilities.html)
    - Defines get and manipulation functions for the offset.
- `DateTime`
  - Impl `From<Date>`, `From<&Date>`, `From<Time>` and `From<&Time>`
  - Impl `Add<Time>`, `AddAssign<Time>`, `Sub<Time>` and `AddAssign<Time>`
  - Impl `Add<Duration>`, `AddAssign<Duration>`, `Sub<Duration>` and `AddAssign<Duration>`
  - Impl `DateUtilities`
  - Impl `TimeUtilities`
  - `as_ymdhms()`, `as_ymd()`, `as_hms()`
  - `duration_between()`
  - `get_offset_hms()`
- `Date`
  - Impl `From<DateTime>`, `From<&DateTime>`
  - Impl `Add<Duration>`, `AddAssign<Duration>`, `Sub<Duration>` and `AddAssign<Duration>`
  - Impl `DateUtilities`
  - `as_ymd()`
  - `duration_between()`
- `Time`
  - Impl `From<DateTime>`, `From<&DateTime>`
  - Impl `Add`, `AddAssign`, `Sub` and `AddAssign`
  - Impl `Add<Duration>`, `AddAssign<Duration>`, `Sub<Duration>` and `AddAssign<Duration>`
  - Impl `TimeUtilities`
  - `as_hms()`
  - `duration_between()`
  - `get_offset_hms()`

### Changed
- `DateTime::timestamp` now comes from `DateUtilities`
- `Date::timestamp` now comes from `DateUtilities`
- `Time::as_seconds` now returns `u32` instead of `u64`
- `DateTime::set_time` now accepts `Time` instead of `u64`
- All offset functions now come from the `OffsetUtilities` trait
- Renamed `set_offset_time()` to `set_offset_hms()`
- Renamed `as_offset_time()` to `as_offset_hms()`
- Renamed `Time::from_nanoseconds` to `Time::from_nanos`
- Renamed `Time::as_nanoseconds` to `Time::as_nanos`

### Removed
- `DateTime`
  - `DateTime::from_days`
  - `DateTime::as_days`
  - `DateTime::from_seconds`
  - `DateTime::as_seconds`
  - `DateTime::from_nanoseconds`
  - `DateTime::as_nanoseconds`
  - `DateTime::date` in favor of `Date::from<DateTime>`
  - `DateTime::time` in favor of `Time::from<DateTime>`
  - `DateTime::between` in favor of `DateTime::seconds_since`
  - `DateTime::get` in favor of the `DateUtilities` and `TimeUtilities` get functions
  - `DateTime::set` in favor of the `DateUtilities` and `TimeUtilities` set functions
  - `DateTime::apply` in favor of the `DateUtilities` and `TimeUtilities` add and sub functions
- `Date`
  - `Date::from_days`
  - `Date::as_days`
  - `Date::between` in favor of `Date::days_since`
  - `Date::get` in favor of the `DateUtilities` get functions
  - `Date::set` in favor of the `DateUtilities` set functions
  - `Date::apply` in favor of the `DateUtilities` add and sub functions
- `Time`
  - `Time::between` in favor of `Time::nanos_since`
  - `Time::get` in favor of the `TimeUtilities` get functions
  - `Time::set` in favor of the `TimeUtilities` set functions
  - `Time::apply` in favor of the `TimeUtilities` add and sub functions
- `DateTimeUnit`
- `DateUnit`
- `TimeUnit`

## [0.3.0] - 2023-04-11
### Added
- Impl `Ord`, `PartialOrd`, `FromStr` for `DateTime`, `Date` and `Time`
- Impl Serde `Serialize` and `Deserialize` for `DateTime`, `Date` and `Time`
- `DateTime::parse`
- `Date::parse`
- `Time::parse`

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
