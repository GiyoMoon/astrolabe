//! Astrolabe is a date and time library for Rust which aims to be feature rich, lightweight (zero dependencies) and easy-to-use. It implements formatting, parsing and manipulating functions for date and time values.
//!
//! ### Features
//! - **Formatting** and **parsing** with format strings based on [Unicode Date Field Symbols](https://www.unicode.org/reports/tr35/tr35-dates.html#Date_Field_Symbol_Table)
//! - **RFC 3339** timestamp parsing and formatting
//! - **Manipulation** functions to add, subtract, set and clear date units
//! - **Cron** expression parser (With feature flag `cron`)
//! - **Timezone** offset
//! - **Zero** dependencies
//! - **Serde** serializing and deserializing (With feature flag `serde`)
//!
//! ### Example
//! A basic example which demonstrates creating, formatting and manipulating a [`DateTime`] instance.
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
//! let modified_dt = date_time
//!     .add_hours(11).unwrap()
//!     .add_minutes(23).unwrap();
//!
//! assert_eq!("2022/05/02 11:23:00", modified_dt.format("yyyy/MM/dd HH:mm:ss"));
//! assert_eq!("2022-05-02T11:23:00Z", modified_dt.format_rfc3339(Precision::Seconds));
//! ```
//! To see all implementations for the [`DateTime`] struct, check out it's [documentation](DateTime).

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/giyomoon/astrolabe/main/assets/logo.svg",
    html_favicon_url = "https://raw.githubusercontent.com/giyomoon/astrolabe/main/assets/logo.svg"
)]
#![forbid(unsafe_code)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![allow(clippy::many_single_char_names)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "cron")]
#[cfg_attr(docsrs, doc(cfg(feature = "cron")))]
mod cron;
mod date;
mod datetime;
pub mod errors;
mod shared;
mod time;
mod util;

#[cfg(feature = "cron")]
pub use self::cron::CronSchedule;
pub use self::date::Date;
pub use self::datetime::DateTime;
pub use self::shared::{DateUtilities, OffsetUtilities, Precision, TimeUtilities};
pub use self::time::Time;
