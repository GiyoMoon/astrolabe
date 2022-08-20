use crate::{
    shared::{SECS_PER_HOUR, SECS_PER_MINUTE},
    AstrolabeError,
};

// Parses the offset part from an RFC3339 timestamp string to offset seconds
pub(crate) fn parse_offset(string: &str) -> Result<i32, AstrolabeError> {
    if string.starts_with('Z') {
        return Ok(0);
    }
    if string.len() != 6 {
        return Err(AstrolabeError::InvalidFormat);
    }

    let hour = string[1..3]
        .parse::<u32>()
        .map_err(|_| AstrolabeError::InvalidFormat)?;
    let min = string[4..6]
        .parse::<u32>()
        .map_err(|_| AstrolabeError::InvalidFormat)?;

    if hour > 23 || min > 59 {
        return Err(AstrolabeError::OutOfRange);
    }

    let offset = hour * SECS_PER_HOUR + min * SECS_PER_MINUTE;

    Ok(if string.starts_with('+') {
        offset as i32
    } else {
        -(offset as i32)
    })
}
