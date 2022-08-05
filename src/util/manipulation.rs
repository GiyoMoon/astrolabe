use super::{
    convert::{days_to_d_units, month_to_ymdays, valid_range},
    leap::is_leap_year,
};
use crate::{
    shared::{NANOS_PER_SEC, SECS_PER_HOUR_U64, SECS_PER_MINUTE_U64},
    AstrolabeError, Date, DateUnit, Time, TimeUnit,
};

/// Applies (add/subtract) a specified [`DateUnit`] to [`Date`]
pub(crate) fn apply_date_unit(
    old: &Date,
    amount: i32,
    unit: DateUnit,
) -> Result<Date, AstrolabeError> {
    Ok(match unit {
        DateUnit::Year => {
            let (year, month, mut day) = days_to_d_units(old.as_days());
            if is_leap_year(year) && month == 2 && day == 29 {
                day = 28;
            }
            let target_year = year + amount;
            valid_range(target_year, month, day)?;
            // Using unwrap because it's safe to assume that the values provided are valid
            Date::from_ymd(target_year, month, day).unwrap()
        }
        DateUnit::Month => {
            let (year, month, day) = days_to_d_units(old.as_days());
            let target_year = (year * 12 + month as i32 + amount - 1) / 12;
            let target_month = if (month as i32 + amount) % 12 == 0 {
                12
            } else {
                (month as i32 + amount).unsigned_abs() % 12
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
            Date::from_ymd(target_year, target_month, target_day).unwrap()
        }
        DateUnit::Day => {
            let new_days = old
                .as_days()
                .checked_add(amount)
                .ok_or(AstrolabeError::OutOfRange)?;
            Date::from_days(new_days)
        }
    })
}

/// Applies (add/subtract) a specified [`TimeUnit`] to [`Time`]
pub(crate) fn apply_time_unit(
    old: &Time,
    amount: i64,
    unit: TimeUnit,
) -> Result<Time, AstrolabeError> {
    Ok(match unit {
        TimeUnit::Hour => {
            let old_time = old.as_nano_seconds();
            let amount_nanos = amount.unsigned_abs() * SECS_PER_HOUR_U64 * NANOS_PER_SEC;

            let new_time = if amount.is_negative() {
                old_time.checked_sub(amount_nanos)
            } else {
                old_time.checked_add(amount_nanos)
            }
            .ok_or(AstrolabeError::OutOfRange)?;

            Time::from_nano_seconds(new_time)
        }
        TimeUnit::Min => {
            let old_time = old.as_nano_seconds();
            let amount_nanos = amount.unsigned_abs() * SECS_PER_MINUTE_U64 * NANOS_PER_SEC;

            let new_time = if amount.is_negative() {
                old_time.checked_sub(amount_nanos)
            } else {
                old_time.checked_add(amount_nanos)
            }
            .ok_or(AstrolabeError::OutOfRange)?;

            Time::from_nano_seconds(new_time)
        }
        TimeUnit::Sec => {
            let old_time = old.as_nano_seconds();
            let amount_nanos = amount.unsigned_abs() * NANOS_PER_SEC;

            let new_time = if amount.is_negative() {
                old_time.checked_sub(amount_nanos)
            } else {
                old_time.checked_add(amount_nanos)
            }
            .ok_or(AstrolabeError::OutOfRange)?;

            Time::from_nano_seconds(new_time)
        }
        TimeUnit::Centis => {
            let old_time = old.as_nano_seconds();
            let amount_nanos = amount.unsigned_abs() * 10_000_000;

            let new_time = if amount.is_negative() {
                old_time.checked_sub(amount_nanos)
            } else {
                old_time.checked_add(amount_nanos)
            }
            .ok_or(AstrolabeError::OutOfRange)?;

            Time::from_nano_seconds(new_time)
        }
        TimeUnit::Millis => {
            let old_time = old.as_nano_seconds();
            let amount_nanos = amount.unsigned_abs() * 1_000_000;

            let new_time = if amount.is_negative() {
                old_time.checked_sub(amount_nanos)
            } else {
                old_time.checked_add(amount_nanos)
            }
            .ok_or(AstrolabeError::OutOfRange)?;

            Time::from_nano_seconds(new_time)
        }
        TimeUnit::Micros => {
            let old_time = old.as_nano_seconds();
            let amount_nanos = amount.unsigned_abs() * 1_000;

            let new_time = if amount.is_negative() {
                old_time.checked_sub(amount_nanos)
            } else {
                old_time.checked_add(amount_nanos)
            }
            .ok_or(AstrolabeError::OutOfRange)?;

            Time::from_nano_seconds(new_time)
        }
        TimeUnit::Nanos => {
            let old_time = old.as_nano_seconds();
            let amount_nanos = amount.unsigned_abs();

            let new_time = if amount.is_negative() {
                old_time.checked_sub(amount_nanos)
            } else {
                old_time.checked_add(amount_nanos)
            }
            .ok_or(AstrolabeError::OutOfRange)?;

            Time::from_nano_seconds(new_time)
        }
    })
}
