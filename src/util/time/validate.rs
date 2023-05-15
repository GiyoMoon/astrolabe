use crate::errors::{out_of_range::create_simple_oor, AstrolabeError};

pub(crate) fn validate_time(hour: u32, minute: u32, second: u32) -> Result<(), AstrolabeError> {
    if hour > 23 {
        return Err(create_simple_oor("hour", 0, 23, hour as i128));
    }

    if minute > 59 {
        return Err(create_simple_oor("minute", 0, 59, minute as i128));
    }

    if second > 59 {
        return Err(create_simple_oor("second", 0, 59, second as i128));
    };

    Ok(())
}
