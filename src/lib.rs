//! Astrolabe is a date and time library for Rust which aims to be feature rich, lightweight (zero dependencies) and easy-to-use. It implements formatting and manipulating functions for date and time values.
//!
//! ### Features:
//! // TODO
//!
//! ### Example
//! // TODO

#![doc(html_logo_url = "https://raw.githubusercontent.com/giyomoon/astrolabe/main/assets/logo.svg")]
#![forbid(unsafe_code)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![allow(clippy::many_single_char_names)]

mod date;
mod datetime;
mod shared;
mod time;
mod util;

pub use self::date::{Date, DateUnit};
pub use self::datetime::{DateTime, DateTimeUnit};
pub use self::shared::AstrolabeError;
pub use self::time::{Time, TimeUnit};
