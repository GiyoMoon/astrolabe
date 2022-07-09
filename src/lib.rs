//! Date and time library for Rust. Aims to be feature rich and easy-to-use.
//!
//! Features:
//!
//! * [`DateTime`], a wrapper around [`std::time::SystemTime`] which implements formatting and manipulation functions
//! * Formatting as RFC3339 timestamp
//! * Formatting with specific format strings based on [Unicode Date Field Symbols](https://www.unicode.org/reports/tr35/tr35-dates.html#Date_Field_Symbol_Table). See further documentation here: [`DateTime::format`]
//! * Zero dependencies if used without the `format` feature

#![forbid(unsafe_code)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]

mod datetime;
#[cfg(feature = "format")]
mod format;

pub use self::datetime::{DateTime, Error as DateTimeError, Precision};
