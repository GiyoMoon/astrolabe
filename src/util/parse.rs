use crate::{
    errors::{invalid_format::create_invalid_format, AstrolabeError},
    shared::{SECS_PER_HOUR, SECS_PER_MINUTE},
};

// Parses the offset part from an RFC3339 timestamp string to offset seconds
pub(crate) fn parse_offset(string: &str) -> Result<i32, AstrolabeError> {
    if string.starts_with('Z') {
        return Ok(0);
    }
    if string.len() != 6 {
        return Err(create_invalid_format(
            "Failed parsing the offset from the RFC3339 string. Format should be +XX:XX or -XX:XX",
        ));
    }

    let hour = string[1..3].parse::<u32>().map_err(|_| {
        create_invalid_format("Failed parsing the hour of the offset from the RFC3339 string")
    })?;
    let min = string[4..6].parse::<u32>().map_err(|_| {
        create_invalid_format("Failed parsing the minute of the offset from the RFC3339 string")
    })?;

    if hour > 23 {
        return Err(create_invalid_format(
            "Failed parsing the hour of the offset from the RFC3339 string. Hour has to be less than 24",
        ));
    } else if min > 59 {
        return Err(create_invalid_format(
            "Failed parsing the minute of the offset from the RFC3339 string. Minute has to be less than 60",
        ));
    }

    let offset = hour * SECS_PER_HOUR + min * SECS_PER_MINUTE;

    Ok(if string.starts_with('+') {
        offset as i32
    } else {
        -(offset as i32)
    })
}
