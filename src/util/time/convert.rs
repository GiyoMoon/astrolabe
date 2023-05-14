use super::validate::validate_time;
use crate::{
    errors::{out_of_range::create_simple_oor, AstrolabeError},
    util::constants::{
        NANOS_PER_DAY, NANOS_PER_SEC, SECS_PER_DAY_U64, SECS_PER_HOUR, SECS_PER_HOUR_U64,
        SECS_PER_MINUTE, SECS_PER_MINUTE_U64,
    },
    TimeUnit,
};

/// Converts nanoseconds to time units (hour, min, sec)
pub(crate) fn nanos_to_time(nanos: u64) -> (u32, u32, u32) {
    let as_seconds = (nanos / NANOS_PER_SEC) as u32;
    let hour = as_seconds / SECS_PER_HOUR;
    let min = as_seconds / SECS_PER_MINUTE % SECS_PER_MINUTE;
    let sec = as_seconds % SECS_PER_MINUTE;
    (hour, min, sec)
}

/// Converts time units (hour, minute and seconds) to day seconds
pub(crate) fn time_to_day_seconds(hour: u32, min: u32, sec: u32) -> Result<u32, AstrolabeError> {
    validate_time(hour, min, sec)?;

    Ok(hour * SECS_PER_HOUR + min * SECS_PER_MINUTE + sec)
}

/// Returns a given [`TimeUnit`] from nanoseconds
pub(crate) fn nanos_to_unit(nanos: u64, unit: TimeUnit) -> u64 {
    match unit {
        TimeUnit::Hour => nanos / NANOS_PER_SEC / SECS_PER_HOUR_U64,
        TimeUnit::Min => nanos / NANOS_PER_SEC / SECS_PER_MINUTE_U64 % SECS_PER_MINUTE_U64,
        TimeUnit::Sec => nanos / NANOS_PER_SEC % 60,
        TimeUnit::Centis => nanos / 10_000_000 % 100,
        TimeUnit::Millis => nanos / 1_000_000 % 1_000,
        TimeUnit::Micros => nanos / 1_000 % 1_000_000,
        TimeUnit::Nanos => nanos % NANOS_PER_SEC,
    }
}

/// Converts days and nanoseconds to seconds
pub(crate) fn days_nanos_to_secs(mut days: i32, day_nanos: u64) -> i64 {
    let adjusted_day_seconds = if days.is_negative() {
        days += 1;
        -(SECS_PER_DAY_U64 as i64 - day_nanos as i64 / NANOS_PER_SEC as i64)
    } else {
        (day_nanos / NANOS_PER_SEC) as i64
    };
    days as i64 * SECS_PER_DAY_U64 as i64 + adjusted_day_seconds
}

/// Converts seconds to days and nanoseconds
pub(crate) fn secs_to_days_nanos(seconds: i64) -> Result<(i32, u64), AstrolabeError> {
    let day_seconds = seconds.unsigned_abs() % SECS_PER_DAY_U64;

    let days_i64 = if seconds.is_negative() && day_seconds != 0 {
        seconds / SECS_PER_DAY_U64 as i64 - 1
    } else {
        seconds / SECS_PER_DAY_U64 as i64
    };

    let days = days_i64.try_into().map_err(|_| {
        create_simple_oor(
            "seconds",
            i32::MIN as i128 * SECS_PER_DAY_U64 as i128,
            i32::MAX as i128 * SECS_PER_DAY_U64 as i128 + SECS_PER_DAY_U64 as i128 - 1,
            seconds as i128,
        )
    })?;

    let adjusted_day_seconds = if seconds.is_negative() && day_seconds != 0 {
        SECS_PER_DAY_U64 - day_seconds
    } else {
        day_seconds
    };

    Ok((days, adjusted_day_seconds * NANOS_PER_SEC))
}

/// Converts days and nanoseconds to nanoseconds
pub(crate) fn days_nanos_to_nanos(mut days: i32, day_nanos: u64) -> i128 {
    let adjusted_day_nanos = if days.is_negative() {
        days += 1;
        -(NANOS_PER_DAY as i128 - day_nanos as i128)
    } else {
        day_nanos as i128
    };
    days as i128 * NANOS_PER_DAY as i128 + adjusted_day_nanos
}

/// Converts nanoseconds to days and nanoseconds
pub(crate) fn nanos_to_days_nanos(nanoseconds: i128) -> Result<(i32, u64), AstrolabeError> {
    let day_nanos = (nanoseconds.unsigned_abs() % NANOS_PER_DAY as u128) as u64;

    let days_i128 = if nanoseconds.is_negative() && day_nanos != 0 {
        nanoseconds / NANOS_PER_DAY as i128 - 1
    } else {
        nanoseconds / NANOS_PER_DAY as i128
    };

    let days = days_i128.try_into().map_err(|_| {
        create_simple_oor(
            "nanoseconds",
            i32::MIN as i128 * NANOS_PER_DAY as i128,
            i32::MAX as i128 * NANOS_PER_DAY as i128 + NANOS_PER_DAY as i128 - 1,
            nanoseconds,
        )
    })?;

    let adjusted_day_nanos = if nanoseconds.is_negative() && day_nanos != 0 {
        NANOS_PER_DAY - day_nanos
    } else {
        day_nanos
    };

    Ok((days, adjusted_day_nanos))
}
