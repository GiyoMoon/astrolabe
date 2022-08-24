use super::AstrolabeError;
use std::fmt;

/// An error indicating that some given parameter is out of range or resulted in an out of range date/time value.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OutOfRange {
    pub(crate) name: &'static str,
    pub(crate) min: i128,
    pub(crate) max: i128,
    pub(crate) value: i128,
    pub(crate) custom: Option<String>,
    pub(crate) conditional: Option<String>,
}

impl fmt::Display for OutOfRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(custom) = &self.custom {
            write!(f, "{}", custom).unwrap();
            return Ok(());
        }

        write!(
            f,
            "{} must be in the range {}..={}",
            self.name, self.min, self.max
        )
        .unwrap();

        if let Some(conditional) = &self.conditional {
            write!(f, ", {}", conditional).unwrap();
        }

        Ok(())
    }
}

pub(crate) fn create_simple_oor(
    name: &'static str,
    min: i128,
    max: i128,
    value: i128,
) -> AstrolabeError {
    AstrolabeError::OutOfRange(OutOfRange {
        name,
        min,
        max,
        value,
        custom: None,
        conditional: None,
    })
}

pub(crate) fn create_conditional_oor(
    name: &'static str,
    min: i128,
    max: i128,
    value: i128,
    conditional: String,
) -> AstrolabeError {
    AstrolabeError::OutOfRange(OutOfRange {
        name,
        min,
        max,
        value,
        custom: None,
        conditional: Some(conditional),
    })
}

pub(crate) fn create_custom_oor(custom: String) -> AstrolabeError {
    AstrolabeError::OutOfRange(OutOfRange {
        name: "",
        min: 0,
        max: 0,
        value: 0,
        custom: Some(custom),
        conditional: None,
    })
}
