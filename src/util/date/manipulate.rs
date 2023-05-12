use super::{
    convert::{date_to_days, days_to_date, year_month_to_doy},
    validate::validate_date,
};
use crate::{
    errors::{out_of_range::create_custom_oor, AstrolabeError},
    util::leap::is_leap_year,
    DateUnit,
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
            validate_date(target_year, month, day)?;
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
            validate_date(target_year, target_month, target_day)?;
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
