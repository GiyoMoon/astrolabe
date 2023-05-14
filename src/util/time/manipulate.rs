use super::convert::{nanos_to_time, set_subsecond_value, time_nanos_to_nanos};
use crate::{
    errors::{out_of_range::create_conditional_oor, AstrolabeError},
    util::constants::{NANOS_PER_SEC, SECS_PER_HOUR_U64, SECS_PER_MINUTE_U64},
    TimeUnit,
};

/// Applies (add/subtract) a specified [`TimeUnit`] to nanoseconds
pub(crate) fn apply_time_unit(old_nanos: i128, amount: i64, unit: TimeUnit) -> i128 {
    match unit {
        TimeUnit::Hour => {
            let amount_nanos = amount as i128 * SECS_PER_HOUR_U64 as i128 * NANOS_PER_SEC as i128;

            old_nanos + amount_nanos
        }
        TimeUnit::Min => {
            let amount_nanos = amount as i128 * SECS_PER_MINUTE_U64 as i128 * NANOS_PER_SEC as i128;

            old_nanos + amount_nanos
        }
        TimeUnit::Sec => {
            let amount_nanos = amount as i128 * NANOS_PER_SEC as i128;

            old_nanos + amount_nanos
        }
        TimeUnit::Centis => {
            let amount_nanos = amount as i128 * 10_000_000;

            old_nanos + amount_nanos
        }
        TimeUnit::Millis => {
            let amount_nanos = amount as i128 * 1_000_000;

            old_nanos + amount_nanos
        }
        TimeUnit::Micros => {
            let amount_nanos = amount as i128 * 1_000;

            old_nanos + amount_nanos
        }
        TimeUnit::Nanos => {
            let amount_nanos = amount as i128;

            old_nanos + amount_nanos
        }
    }
}

pub(crate) fn set_hour(nanos: u64, hour: u32) -> Result<u64, AstrolabeError> {
    if hour > 23 {
        return Err(create_conditional_oor(
            "value",
            0,
            23,
            hour as i128,
            "because unit is \"Hour\"".to_string(),
        ));
    }
    let (_, minute, second) = nanos_to_time(nanos);

    Ok(time_nanos_to_nanos(hour, minute, second, nanos))
}

pub(crate) fn set_minute(nanos: u64, minute: u32) -> Result<u64, AstrolabeError> {
    if minute > 59 {
        return Err(create_conditional_oor(
            "value",
            0,
            59,
            minute as i128,
            "because unit is \"Minute\"".to_string(),
        ));
    }
    let (hour, _, second) = nanos_to_time(nanos);

    Ok(time_nanos_to_nanos(hour, minute, second, nanos))
}

pub(crate) fn set_second(nanos: u64, second: u32) -> Result<u64, AstrolabeError> {
    if second > 59 {
        return Err(create_conditional_oor(
            "value",
            0,
            59,
            second as i128,
            "because unit is \"Second\"".to_string(),
        ));
    }
    let (hour, minute, _) = nanos_to_time(nanos);

    Ok(time_nanos_to_nanos(hour, minute, second, nanos))
}

pub(crate) fn set_milli(nanos: u64, milli: u32) -> Result<u64, AstrolabeError> {
    if milli > 999 {
        return Err(create_conditional_oor(
            "value",
            0,
            999,
            milli as i128,
            "because unit is \"Millis\"".to_string(),
        ));
    }

    Ok(set_subsecond_value(nanos, milli as u64, 1_000_000))
}

pub(crate) fn set_micro(nanos: u64, micro: u32) -> Result<u64, AstrolabeError> {
    if micro > 999_999 {
        return Err(create_conditional_oor(
            "value",
            0,
            999_999,
            micro as i128,
            "because unit is \"Micros\"".to_string(),
        ));
    }

    Ok(set_subsecond_value(nanos, micro as u64, 1_000))
}

pub(crate) fn set_nano(nanos: u64, nano: u32) -> Result<u64, AstrolabeError> {
    if nano > 999_999_999 {
        return Err(create_conditional_oor(
            "value",
            0,
            999_999_999,
            nano as i128,
            "because unit is \"Nanos\"".to_string(),
        ));
    }

    Ok(set_subsecond_value(nanos, nano as u64, 1))
}
