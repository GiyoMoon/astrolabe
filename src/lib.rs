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
//!   * Use modify functions like [`DateTime::add`] or [`DateTime::sub`] to create a new, modified [`DateTime`] struct
//! * **Timezone**
//!   * Specify a timezone offset which will be applied in any format function
//! * Zero dependencies

#![forbid(unsafe_code)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]

mod datetime;
mod format;

pub use self::datetime::{DateTime, Error as DateTimeError, Precision, Unit};
