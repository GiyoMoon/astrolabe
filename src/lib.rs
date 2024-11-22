//! Astrolabe is a date and time library for Rust which aims to be feature rich, lightweight (zero dependencies) and easy-to-use. It implements formatting, parsing and manipulating functions for date and time values.
//!
//! ### Features
//! - **Formatting** and **parsing** with format strings based on [Unicode Date Field Symbols](https://www.unicode.org/reports/tr35/tr35-dates.html#Date_Field_Symbol_Table)
//! - **RFC 3339** timestamp parsing and formatting
//! - **Manipulation** functions to add, subtract, set and clear date units
//! - **Cron** expression parser
//! - **Timezone** offset
//! - **Local** timezone on UNIX platforms
//! - **Zero** dependencies
//! - **Serde** serializing and deserializing (With feature flag `serde`)
//! - **sqlx** postgres encoding and decoding (With feature flag `sqlx-postgres`)
//!
//! ## Examples
//! ### Basic
//! A basic example which demonstrates creating, formatting and manipulating a `DateTime` instance.
//!
//! ```rust
//! use astrolabe::{DateTime, TimeUtilities, Precision};
//!
//! // Create a DateTime instance from year, month, and days (day of month)
//! let date_time = DateTime::from_ymd(2022, 5, 2).unwrap();
//!
//! // Use the format function to freely format your DateTime instance
//! assert_eq!("2022/05/02", date_time.format("yyyy/MM/dd"));
//!
//! // Create a new instance with a modified DateTime
//! // The previous instance is not modified and is still in scope
//! let modified_dt = date_time.add_hours(11).add_minutes(23);
//!
//! assert_eq!("2022/05/02 11:23:00", modified_dt.format("yyyy/MM/dd HH:mm:ss"));
//! assert_eq!("2022-05-02T11:23:00Z", modified_dt.format_rfc3339(Precision::Seconds));
//! ```
//! To see all implementations for the `DateTime` struct, check out it's [documentation](https://docs.rs/astrolabe/latest/astrolabe/struct.DateTime.html).
//!
//! ### Local timezone (UNIX systems only)
//! Astrolabe can parse the timezone from `/etc/localtime` to get the local UTC offset. This only works on UNIX systems.
//!
//! ```rust
//! use astrolabe::{DateTime, Offset, OffsetUtilities, Precision};
//!
//! // Equivalent to `DateTime::now().set_offset(Offset::Local)`
//! let now = DateTime::now_local();
//!
//! // Prints for example:
//! // 2023-10-08T08:30:00+02:00
//! println!("{}", now.format_rfc3339(Precision::Seconds));
//! assert_eq!(Offset::Local, now.get_offset());
//! ```
//! See [`Offset`](https://docs.rs/astrolabe/latest/astrolabe/enum.Offset.html)
//!
//! ### CRON parsing
//! ```rust
//! use astrolabe::CronSchedule;
//!
//! // Every 5 minutes
//! let schedule = CronSchedule::parse("*/5 * * * *").unwrap();
//! for date in schedule.take(3) {
//!    println!("{}", date);
//! }
//! // Prints for example:
//! // 2022-05-02 16:15:00
//! // 2022-05-02 16:20:00
//! // 2022-05-02 16:25:00
//!
//! // Every weekday at 10:00
//! let schedule = CronSchedule::parse("0 10 * * Mon-Fri").unwrap();
//! for date in schedule.take(3) {
//!    println!("{}", date.format("yyyy-MM-dd HH:mm:ss eeee"));
//! }
//! // Prints for example:
//! // 2022-05-03 10:00:00 Tuesday
//! // 2022-05-04 10:00:00 Wednesday
//! // 2022-05-05 10:00:00 Thursday
//! ```
//! See [`CronSchedule`](https://docs.rs/astrolabe/latest/astrolabe/struct.CronSchedule.html)
//!

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/giyomoon/astrolabe/main/assets/logo.svg",
    html_favicon_url = "https://raw.githubusercontent.com/giyomoon/astrolabe/main/assets/logo.svg"
)]
#![forbid(unsafe_code)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![allow(clippy::many_single_char_names)]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod cron;
mod date;
mod datetime;
pub mod errors;
mod local;
mod offset;
#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
mod serde;
mod shared;
#[cfg(feature = "sqlx")]
#[cfg_attr(docsrs, doc(cfg(feature = "sqlx")))]
mod sqlx;
mod time;
mod util;

pub use self::cron::CronSchedule;
pub use self::date::Date;
pub use self::datetime::DateTime;
pub use self::offset::Offset;
pub use self::shared::{DateUtilities, OffsetUtilities, Precision, TimeUtilities};
pub use self::time::Time;
