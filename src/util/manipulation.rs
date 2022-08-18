use super::{
    convert::{
        date_to_days, days_to_date, month_to_ymdays, nanos_as_unit, nanos_to_time, nanos_to_unit,
        valid_range,
    },
    leap::is_leap_year,
};
use crate::{
    shared::{
        NANOS_PER_SEC, SECS_PER_HOUR, SECS_PER_HOUR_U64, SECS_PER_MINUTE, SECS_PER_MINUTE_U64,
    },
    AstrolabeError, DateUnit, TimeUnit,
};

/// Applies (add/subtract) a specified [`DateUnit`] to days
pub(crate) fn apply_date_unit(
    old_days: i32,
    amount: i64,
    unit: DateUnit,
) -> Result<i32, AstrolabeError> {
    let amount_i32: i32 = amount.try_into().map_err(|_| AstrolabeError::OutOfRange)?;
    Ok(match unit {
        DateUnit::Year => {
            let (year, month, mut day) = days_to_date(old_days);
            let target_year: i32 = year + amount_i32;
            if is_leap_year(year) && !is_leap_year(target_year) && month == 2 && day == 29 {
                day = 28;
            }
            valid_range(target_year, month, day)?;
            // Using unwrap because it's safe to assume that the values provided are valid
            date_to_days(target_year, month, day).unwrap()
        }
        DateUnit::Month => {
            let (year, month, day) = days_to_date(old_days);
            let target_year = (year * 12 + month as i32 + amount_i32 - 1) / 12;
            let target_month = if (month as i32 + amount_i32) % 12 == 0 {
                12
            } else {
                (month as i32 + amount_i32).unsigned_abs() % 12
            };
            let target_day = match day {
                day if day < 29 => day,
                _ => {
                    // Using unwrap because it's safe to assume that month is valid
                    let (_, mdays) = month_to_ymdays(target_year, target_month).unwrap();
                    if day > mdays {
                        mdays
                    } else {
                        day
                    }
                }
            };
            valid_range(target_year, target_month, target_day)?;
            // Using unwrap because it's safe to assume that month and day is valid
            date_to_days(target_year, target_month, target_day).unwrap()
        }
        DateUnit::Day => old_days
            .checked_add(amount_i32)
            .ok_or(AstrolabeError::OutOfRange)?,
    })
}

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

/// Sets a specific [`DateUnit`] to the provided value
pub(crate) fn set_date_unit(
    old_days: i32,
    value: i32,
    unit: DateUnit,
) -> Result<i32, AstrolabeError> {
    Ok(match unit {
        DateUnit::Year => {
            let (_, month, day) = days_to_date(old_days);

            date_to_days(value, month, day)?
        }
        DateUnit::Month => {
            let (year, _, day) = days_to_date(old_days);
            if value.is_negative() {
                return Err(AstrolabeError::OutOfRange);
            }

            date_to_days(year, value.unsigned_abs(), day)?
        }
        DateUnit::Day => {
            let (year, month, _) = days_to_date(old_days);
            if value.is_negative() {
                return Err(AstrolabeError::OutOfRange);
            }

            date_to_days(year, month, value.unsigned_abs())?
        }
    })
}

/// Sets a specific [`TimeUnit`] to the provided value
pub(crate) fn set_time_unit(
    old_nanos: u64,
    value: u32,
    unit: TimeUnit,
) -> Result<u64, AstrolabeError> {
    Ok(match unit {
        TimeUnit::Hour => {
            if value > 23 {
                return Err(AstrolabeError::OutOfRange);
            }
            let (_, min, sec) = nanos_to_time(old_nanos);

            (value * SECS_PER_HOUR + min * SECS_PER_MINUTE + sec) as u64 * NANOS_PER_SEC
                + nanos_to_unit(old_nanos, TimeUnit::Nanos)
        }
        TimeUnit::Min => {
            if value > 59 {
                return Err(AstrolabeError::OutOfRange);
            }
            let (hour, _, sec) = nanos_to_time(old_nanos);

            (hour * SECS_PER_HOUR + value * SECS_PER_MINUTE + sec) as u64 * NANOS_PER_SEC
                + nanos_to_unit(old_nanos, TimeUnit::Nanos)
        }
        TimeUnit::Sec => {
            if value > 59 {
                return Err(AstrolabeError::OutOfRange);
            }
            let (hour, min, _) = nanos_to_time(old_nanos);

            (hour * SECS_PER_HOUR + min * SECS_PER_MINUTE + value) as u64 * NANOS_PER_SEC
                + nanos_to_unit(old_nanos, TimeUnit::Nanos)
        }
        TimeUnit::Centis => {
            if value > 99 {
                return Err(AstrolabeError::OutOfRange);
            }

            nanos_as_unit(old_nanos, TimeUnit::Sec) * NANOS_PER_SEC
                + value as u64 * 10_000_000
                + old_nanos % 10_000_000
        }
        TimeUnit::Millis => {
            if value > 999 {
                return Err(AstrolabeError::OutOfRange);
            }

            nanos_as_unit(old_nanos, TimeUnit::Sec) * NANOS_PER_SEC
                + value as u64 * 1_000_000
                + old_nanos % 1_000_000
        }
        TimeUnit::Micros => {
            if value > 999_999 {
                return Err(AstrolabeError::OutOfRange);
            }

            nanos_as_unit(old_nanos, TimeUnit::Sec) * NANOS_PER_SEC
                + value as u64 * 1_000
                + old_nanos % 1_000
        }
        TimeUnit::Nanos => {
            if value > 999_999_999 {
                return Err(AstrolabeError::OutOfRange);
            }

            nanos_as_unit(old_nanos, TimeUnit::Sec) * NANOS_PER_SEC + value as u64
        }
    })
}
