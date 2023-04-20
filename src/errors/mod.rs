//! Various error types returned by functions in the astrolabe crate.

pub(crate) mod invalid_format;
pub(crate) mod out_of_range;
pub use self::{invalid_format::InvalidFormat, out_of_range::OutOfRange};
use std::fmt;

/// Custom error enum for the astrolabe crate.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AstrolabeError {
    /// An error indicating that some given parameter is out of range or resulted in an out of range date/time value.
    OutOfRange(OutOfRange),
    /// An error indicating that the string to be parsed is invalid.
    InvalidFormat(InvalidFormat),
}

impl fmt::Display for AstrolabeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OutOfRange(e) => e.fmt(f),
            Self::InvalidFormat(e) => e.fmt(f),
        }
    }
}

impl From<AstrolabeError> for String {
    fn from(e: AstrolabeError) -> Self {
        e.to_string()
    }
}
