use super::validate::validate_time;
use crate::{
    errors::{out_of_range::create_simple_oor, AstrolabeError},
    util::constants::{
        NANOS_PER_DAY, NANOS_PER_HOUR, NANOS_PER_MINUTE, NANOS_PER_SEC, SECS_PER_DAY_U64,
        SECS_PER_HOUR, SECS_PER_MINUTE,
    },
};

/// Converts nanoseconds to time units (hour, min, sec)
pub(crate) fn nanos_to_time(nanos: u64) -> (u32, u32, u32) {
    let as_seconds = (nanos / NANOS_PER_SEC) as u32;
    let hour = as_seconds / SECS_PER_HOUR;
    let min = as_seconds / SECS_PER_MINUTE % SECS_PER_MINUTE;
    let sec = as_seconds % SECS_PER_MINUTE;
    (hour, min, sec)
}

/// Converts nanoseconds to subsecond units (millis, micros, nanos)
pub(crate) fn nanos_to_subsecond(nanos: u64) -> (u32, u32, u32) {
    let millis = nanos % NANOS_PER_SEC / 1_000_000;
    let micros = nanos % NANOS_PER_SEC / 1_000;
    let nanos = nanos % NANOS_PER_SEC;
    (millis as u32, micros as u32, nanos as u32)
}

/// Converts time units (hour, minute and seconds) to day seconds
pub(crate) fn time_to_day_seconds(
    hour: u32,
    minute: u32,
    second: u32,
) -> Result<u32, AstrolabeError> {
    validate_time(hour, minute, second)?;

    Ok(hour * SECS_PER_HOUR + minute * SECS_PER_MINUTE + second)
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

/// Converts time units (hour, minute and seconds) and nanoseconds to nanoseconds. Only the subsecond nanoseconds are used.
pub(crate) fn time_nanos_to_nanos(hour: u32, minute: u32, second: u32, nanos: u64) -> u64 {
    let time_seconds = time_to_day_seconds(hour, minute, second).unwrap();
    time_seconds as u64 * NANOS_PER_SEC + nanos % NANOS_PER_SEC
}

/// Inserts a subsecond value into nanoseconds. For example, it inserts milliseconds (e.g. `999`) with a divisor of `1_000_000` into nanoseconds (e.g. `1_222_333_444`) and returns `1_999_333_444`.
pub(crate) fn set_subsecond_value(nanos: u64, value: u64, divisor: u64) -> u64 {
    let upper_nanos = nanos / NANOS_PER_SEC * NANOS_PER_SEC;
    let subdevider_nanos = nanos % divisor;
    upper_nanos + value * divisor + subdevider_nanos
}

pub(crate) fn days_nanos_to_hours(days: i32, nanos: u64) -> i64 {
    days as i64 * 24 + nanos_to_time(nanos).0 as i64
}

pub(crate) fn nanos_to_subhour_nanos(nanoseconds: u64) -> i64 {
    (nanoseconds % NANOS_PER_HOUR) as i64
}

pub(crate) fn days_nanos_to_minutes(days: i32, nanos: u64) -> i64 {
    let (subday_hours, subhour_minutes, _) = nanos_to_time(nanos);
    days as i64 * 24 * 60 + subday_hours as i64 * 60 + subhour_minutes as i64
}

pub(crate) fn nanos_to_subminute_nanos(nanoseconds: u64) -> i64 {
    (nanoseconds % NANOS_PER_MINUTE) as i64
}

pub(crate) fn days_nanos_to_seconds(days: i32, nanos: u64) -> i64 {
    let (subday_hours, subhour_minutes, subminute_seconds) = nanos_to_time(nanos);
    days as i64 * 24 * 60 * 60
        + subday_hours as i64 * 60 * 60
        + subhour_minutes as i64 * 60
        + subminute_seconds as i64
}

pub(crate) fn nanos_to_subsecond_nanos(nanoseconds: u64) -> i64 {
    (nanoseconds % NANOS_PER_SEC) as i64
}

pub(crate) fn days_nanos_to_millis(days: i32, nanos: u64) -> i128 {
    let total_seconds = days_nanos_to_seconds(days, nanos);
    total_seconds as i128 * 1_000 + nanos as i128 / 1_000_000
}

pub(crate) fn nanos_to_submilli_nanos(nanoseconds: u64) -> i64 {
    (nanoseconds % 1_000_000) as i64
}

pub(crate) fn days_nanos_to_micros(days: i32, nanos: u64) -> i128 {
    let total_seconds = days_nanos_to_seconds(days, nanos);
    total_seconds as i128 * 1_000_000 + nanos as i128 / 1_000
}

pub(crate) fn nanos_to_submicro_nanos(nanoseconds: u64) -> i64 {
    (nanoseconds % 1_000) as i64
}
