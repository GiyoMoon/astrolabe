use super::{
    convert::{days_to_d_units, month_to_ymdays, valid_range},
    leap::is_leap_year,
};
use crate::{AstrolabeError, Date, DateUnit};

/// Applies (add/subtract) a specified [`DateUnit`] to [`Date`]
pub(crate) fn apply_date_unit(
    old: &Date,
    amount: i32,
    unit: DateUnit,
) -> Result<Date, AstrolabeError> {
    Ok(match unit {
        DateUnit::Year => {
            let (year, month, mut day) = days_to_d_units(old.days());
            if is_leap_year(year) && month == 2 && day == 29 {
                day = 28;
            }
            let target_year = year + amount;
            valid_range(target_year, month, day)?;
            // Using unwrap because it's safe to assume that the values provided are valid
            Date::from_ymd(target_year, month, day).unwrap()
        }
        DateUnit::Month => {
            let (year, month, day) = days_to_d_units(old.days());
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
                .days()
                .checked_add(amount)
                .ok_or(AstrolabeError::OutOfRange)?;
            Date::from_days(new_days)
        }
    })
}
