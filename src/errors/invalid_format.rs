use super::AstrolabeError;
use std::fmt;

/// An error indicating that the string to be parsed is invalid.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InvalidFormat(String);

impl fmt::Display for InvalidFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub(crate) fn create_invalid_format(message: String) -> AstrolabeError {
    AstrolabeError::InvalidFormat(InvalidFormat(message))
}
