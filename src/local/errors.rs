use std::{array::TryFromSliceError, fmt::Display, num::ParseIntError, str::Utf8Error};

/// TZif parsing errors
#[derive(Debug)]
pub(crate) enum TimeZoneError {
    Cursor(&'static str),
    InvalidTzFile(&'static str),
    UnsupportedTzFile(&'static str),
    TryFromSliceError(TryFromSliceError),
    Utf8Error(Utf8Error),
    ParseIntError(ParseIntError),
}

impl Display for TimeZoneError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TimeZoneError::Cursor(error) => {
                write!(f, "Error when parsing a TZif file: Cursor Error: {}", error)
            }
            TimeZoneError::InvalidTzFile(error) => {
                write!(
                    f,
                    "Error when parsing a TZif file: Invalid Tzif file: {}",
                    error
                )
            }
            TimeZoneError::UnsupportedTzFile(error) => {
                write!(
                    f,
                    "Error when parsing a TZif file: Unsupported Tzif file: {}",
                    error
                )
            }
            TimeZoneError::TryFromSliceError(error) => {
                write!(f, "Error when parsing a TZif file: {}", error)
            }
            TimeZoneError::Utf8Error(error) => {
                write!(f, "Error when parsing a TZif file: {}", error)
            }
            TimeZoneError::ParseIntError(error) => {
                write!(f, "Error when parsing a TZif file: {}", error)
            }
        }
    }
}

impl From<TryFromSliceError> for TimeZoneError {
    fn from(e: TryFromSliceError) -> Self {
        TimeZoneError::TryFromSliceError(e)
    }
}

impl From<Utf8Error> for TimeZoneError {
    fn from(e: Utf8Error) -> Self {
        TimeZoneError::Utf8Error(e)
    }
}

impl From<ParseIntError> for TimeZoneError {
    fn from(e: ParseIntError) -> Self {
        TimeZoneError::ParseIntError(e)
    }
}

#[cfg(test)]
mod tests {
    use crate::local::errors::TimeZoneError;

    #[test]
    fn display() {
        let string = " 0";
        println!(
            "{}",
            string
                .parse::<i32>()
                .map_err(TimeZoneError::from)
                .unwrap_err()
        );

        let invalid_utf8 = b"\xFF";
        #[allow(invalid_from_utf8)]
        let parsed = std::str::from_utf8(invalid_utf8)
            .map_err(TimeZoneError::from)
            .unwrap_err();
        println!("{}", parsed);
        println!("{:?}", parsed);

        let bytes: &[u8] = b"12";
        #[allow(invalid_from_utf8)]
        let slice_result: Result<[u8; 4], _> = bytes.try_into().map_err(TimeZoneError::from);
        assert!(slice_result.is_err());
        println!("{}", slice_result.unwrap_err());
    }
}
