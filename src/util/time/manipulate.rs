use super::convert::{nanos_to_time, set_subsecond_value, time_nanos_to_nanos};
use crate::{
    errors::{out_of_range::create_conditional_oor, AstrolabeError},
    util::constants::{NANOS_PER_SEC, SECS_PER_HOUR_U64, SECS_PER_MINUTE_U64},
};

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

pub(crate) fn add_hours(nanos: u64, hours: u32) -> u64 {
    let hours_as_nanos = hours as u64 * SECS_PER_HOUR_U64 * NANOS_PER_SEC;

    nanos + hours_as_nanos
}

pub(crate) fn add_minutes(nanos: u64, minutes: u32) -> u64 {
    let minutes_as_nanos = minutes as u64 * SECS_PER_MINUTE_U64 * NANOS_PER_SEC;

    nanos + minutes_as_nanos
}

pub(crate) fn add_seconds(nanos: u64, seconds: u32) -> u64 {
    let minutes_as_nanos = seconds as u64 * NANOS_PER_SEC;

    nanos + minutes_as_nanos
}

pub(crate) fn add_millis(nanos: u64, millis: u32) -> u64 {
    let millis_as_nanos = millis as u64 * 1_000_000;

    nanos + millis_as_nanos
}

pub(crate) fn add_micros(nanos: u64, micros: u32) -> u64 {
    let micros_as_nanos = micros as u64 * 1_000;

    nanos + micros_as_nanos
}

pub(crate) fn sub_hours(nanos: i64, hours: u32) -> i64 {
    let hours_as_nanos = hours as i64 * SECS_PER_HOUR_U64 as i64 * NANOS_PER_SEC as i64;

    nanos - hours_as_nanos
}

pub(crate) fn sub_minutes(nanos: i64, minutes: u32) -> i64 {
    let minutes_as_nanos = minutes as i64 * SECS_PER_MINUTE_U64 as i64 * NANOS_PER_SEC as i64;

    nanos - minutes_as_nanos
}

pub(crate) fn sub_seconds(nanos: i64, seconds: u32) -> i64 {
    let minutes_as_nanos = seconds as i64 * NANOS_PER_SEC as i64;

    nanos - minutes_as_nanos
}

pub(crate) fn sub_millis(nanos: i64, millis: u32) -> i64 {
    let millis_as_nanos = millis as i64 * 1_000_000;

    nanos - millis_as_nanos
}

pub(crate) fn sub_micros(nanos: i64, micros: u32) -> i64 {
    let micros_as_nanos = micros as i64 * 1_000;

    nanos - micros_as_nanos
}

pub(crate) fn clear_nanos_until_minute(nanos: u64) -> u64 {
    let hour = nanos_to_time(nanos).0;
    time_nanos_to_nanos(hour, 0, 0, 0)
}

pub(crate) fn clear_nanos_until_second(nanos: u64) -> u64 {
    let (hour, minute, _) = nanos_to_time(nanos);
    time_nanos_to_nanos(hour, minute, 0, 0)
}

pub(crate) fn clear_nanos_until_milli(nanos: u64) -> u64 {
    let (hour, minute, second) = nanos_to_time(nanos);
    time_nanos_to_nanos(hour, minute, second, 0)
}

pub(crate) fn clear_nanos_until_micro(nanos: u64) -> u64 {
    let (hour, minute, second) = nanos_to_time(nanos);
    time_nanos_to_nanos(hour, minute, second, nanos / 1_000_000 * 1_000_000)
}

pub(crate) fn clear_nanos_until_nanos(nanos: u64) -> u64 {
    let (hour, minute, second) = nanos_to_time(nanos);
    time_nanos_to_nanos(hour, minute, second, nanos / 1_000 * 1_000)
}
