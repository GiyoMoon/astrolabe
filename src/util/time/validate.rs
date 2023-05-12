use crate::errors::{out_of_range::create_simple_oor, AstrolabeError};

pub(crate) fn validate_time(hour: u32, min: u32, sec: u32) -> Result<(), AstrolabeError> {
    if hour > 23 {
        return Err(create_simple_oor("hour", 0, 23, hour as i128));
    }

    if min > 59 {
        return Err(create_simple_oor("min", 0, 59, min as i128));
    }

    if sec > 59 {
        return Err(create_simple_oor("sec", 0, 59, sec as i128));
    };

    Ok(())
}
