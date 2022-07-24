use super::convert::{SECS_PER_HOUR, SECS_PER_MINUTE};
use crate::DateTimeError;

// Parses the offset part from an RFC3339 timestamp string to offset seconds
pub(crate) fn parse_offset(string: &str) -> Result<i64, DateTimeError> {
    if string.starts_with('Z') {
        return Ok(0);
    }
    if string.len() != 6 {
        return Err(DateTimeError::InvalidFormat);
    }

    let hour = string[1..3]
        .parse::<u64>()
        .map_err(|_| DateTimeError::InvalidFormat)?;
    let min = string[4..6]
        .parse::<u64>()
        .map_err(|_| DateTimeError::InvalidFormat)?;

    if hour > 23 || min > 59 {
        return Err(DateTimeError::OutOfRange);
    }

    let offset = hour * SECS_PER_HOUR + min * SECS_PER_MINUTE;

    Ok(if string.starts_with('+') {
        offset as i64
    } else {
        -(offset as i64)
    })
}
