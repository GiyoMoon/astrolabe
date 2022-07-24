use crate::DateTimeError;

pub(crate) fn parse_offset(string: &str) -> Result<i64, DateTimeError> {
    if string.starts_with('Z') {
        return Ok(0);
    }
    if string.len() != 6 {
        return Err(DateTimeError::InvalidFormat);
    }

    let hour = string[1..3]
        .parse::<i64>()
        .map_err(|_| DateTimeError::InvalidFormat)?;
    let min = string[4..6]
        .parse::<i64>()
        .map_err(|_| DateTimeError::InvalidFormat)?;

    if hour > 23 || min > 59 {
        return Err(DateTimeError::OutOfRange);
    }

    let offset = hour * 3600 + min * 60;

    Ok(if string.starts_with('+') {
        offset
    } else {
        -offset
    })
}
