//! Date and time library for Rust. Aims to be feature rich, lightweight (zero dependencies) and easy-to-use.
//!
//! ### Features:
//!
//! * [`DateTime`], a wrapper around [`std::time::SystemTime`] which implements formatting and manipulation functions
//! * **Formatting**
//!   * RFC3339 timestamp with [`DateTime::format_rfc3339`]
//!   * Formatting with format strings based on [Unicode Date Field Symbols](https://www.unicode.org/reports/tr35/tr35-dates.html#Date_Field_Symbol_Table).
//!     (Which allows formatting [`SystemTime`](std::time::SystemTime) into basically any string format).
//!     <br>See [`DateTime::format`]
//! * **Manipulation**
//!   * Manipulation functions like [`DateTime::add`] or [`DateTime::sub`] to create a new, modified [`DateTime`] instance
//! * **Timezone**
//!   * Specify a timezone offset which will be applied in any format function
//! * Zero dependencies
//!
//! ### Example
//! ```rust
//! use astrolabe::{DateTime, Precision, Unit};
//!
//! // Create a DateTime instance from year, month, and days (day of month)
//! let date_time = DateTime::from_ymd(2022, 5, 2).unwrap();
//!
//! // Use the format function to freely format your DateTime instance
//! assert_eq!("2022/05/02", date_time.format("yyyy/MM/dd").unwrap());
//!
//! // Create a new instance with a modified DateTime
//! // The previous instance is not modified and is still in scope
//! let modified_dt = date_time.add(11, Unit::Hour).add(23, Unit::Min);
//!
//! assert_eq!("2022/05/02 11:23:00", modified_dt.format("yyyy/MM/dd HH:mm:ss").unwrap());
//! assert_eq!("2022-05-02T11:23:00Z", modified_dt.format_rfc3339(Precision::Seconds));
//! ```
//! To see all implementations for the [`DateTime`] struct, check out it's [documentation](DateTime).

#![doc(html_logo_url = "https://raw.githubusercontent.com/giyomoon/astrolabe/main/assets/logo.svg")]
#![forbid(unsafe_code)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]

mod datetime;
mod util;

pub use self::datetime::{DateTime, DateTimeError, Offset, Precision, Unit};
