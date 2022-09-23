use super::{
    convert::{
        date_to_days, days_to_date, nanos_to_time, nanos_to_unit, valid_range, year_month_to_doy,
    },
    leap::is_leap_year,
};
use crate::{
    errors::{
        out_of_range::{create_conditional_oor, create_custom_oor},
        AstrolabeError,
    },
    shared::{
        NANOS_PER_SEC, SECS_PER_HOUR, SECS_PER_HOUR_U64, SECS_PER_MINUTE, SECS_PER_MINUTE_U64,
    },
    DateUnit, TimeUnit,
};

/// Applies (add/subtract) a specified [`DateUnit`] to days
pub(crate) fn apply_date_unit(
    old_days: i32,
    amount: i64,
    unit: DateUnit,
) -> Result<i32, AstrolabeError> {
    let amount_i32: i32 = amount.try_into().map_err(|_| {
        create_custom_oor(format!(
            "Amount has to fit into an i32 integer when using unit \"{:?}\"",
            unit,
        ))
    })?;
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
                    let (_, mdays) = year_month_to_doy(target_year, target_month).unwrap();
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
        DateUnit::Day => old_days.checked_add(amount_i32).ok_or_else(|| {
            create_custom_oor(format!(
                "Instance would result into an overflow if {} days were added.",
                amount_i32,
            ))
        })?,
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
            if value.is_negative() {
                return Err(create_custom_oor(
                    "Value cannot be negative because unit is \"Month\"".to_string(),
                ));
            }

            let (year, _, day) = days_to_date(old_days);

            date_to_days(year, value.unsigned_abs(), day)?
        }
        DateUnit::Day => {
            if value.is_negative() {
                return Err(create_custom_oor(
                    "Value cannot be negative because unit is \"Day\"".to_string(),
                ));
            }

            let (year, month, _) = days_to_date(old_days);

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
                return Err(create_conditional_oor(
                    "value",
                    0,
                    23,
                    value as i128,
                    "because unit is \"Hour\"".to_string(),
                ));
            }
            let (_, min, sec) = nanos_to_time(old_nanos);

            (value * SECS_PER_HOUR + min * SECS_PER_MINUTE + sec) as u64 * NANOS_PER_SEC
                + nanos_to_unit(old_nanos, TimeUnit::Nanos)
        }
        TimeUnit::Min => {
            if value > 59 {
                return Err(create_conditional_oor(
                    "value",
                    0,
                    59,
                    value as i128,
                    "because unit is \"Min\"".to_string(),
                ));
            }
            let (hour, _, sec) = nanos_to_time(old_nanos);

            (hour * SECS_PER_HOUR + value * SECS_PER_MINUTE + sec) as u64 * NANOS_PER_SEC
                + nanos_to_unit(old_nanos, TimeUnit::Nanos)
        }
        TimeUnit::Sec => {
            if value > 59 {
                return Err(create_conditional_oor(
                    "value",
                    0,
                    59,
                    value as i128,
                    "because unit is \"Sec\"".to_string(),
                ));
            }
            let (hour, min, _) = nanos_to_time(old_nanos);

            (hour * SECS_PER_HOUR + min * SECS_PER_MINUTE + value) as u64 * NANOS_PER_SEC
                + nanos_to_unit(old_nanos, TimeUnit::Nanos)
        }
        TimeUnit::Centis => {
            if value > 99 {
                return Err(create_conditional_oor(
                    "value",
                    0,
                    99,
                    value as i128,
                    "because unit is \"Centis\"".to_string(),
                ));
            }

            old_nanos / NANOS_PER_SEC * NANOS_PER_SEC
                + value as u64 * 10_000_000
                + old_nanos % 10_000_000
        }
        TimeUnit::Millis => {
            if value > 999 {
                return Err(create_conditional_oor(
                    "value",
                    0,
                    999,
                    value as i128,
                    "because unit is \"Millis\"".to_string(),
                ));
            }

            old_nanos / NANOS_PER_SEC * NANOS_PER_SEC
                + value as u64 * 1_000_000
                + old_nanos % 1_000_000
        }
        TimeUnit::Micros => {
            if value > 999_999 {
                return Err(create_conditional_oor(
                    "value",
                    0,
                    999_999,
                    value as i128,
                    "because unit is \"Micros\"".to_string(),
                ));
            }

            old_nanos / NANOS_PER_SEC * NANOS_PER_SEC + value as u64 * 1_000 + old_nanos % 1_000
        }
        TimeUnit::Nanos => {
            if value > 999_999_999 {
                return Err(create_conditional_oor(
                    "value",
                    0,
                    999_999_999,
                    value as i128,
                    "because unit is \"Nanos\"".to_string(),
                ));
            }

            old_nanos / NANOS_PER_SEC * NANOS_PER_SEC + value as u64
        }
    })
}
