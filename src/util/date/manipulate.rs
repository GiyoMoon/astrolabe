use super::convert::{date_to_days, days_to_date, year_doy_to_days, year_month_to_doy};
use crate::{
    errors::{out_of_range::create_custom_oor, AstrolabeError},
    util::leap::is_leap_year,
};

pub(crate) fn set_year(days: i32, year: i32) -> Result<i32, AstrolabeError> {
    let (_, month, day) = days_to_date(days);

    date_to_days(year, month, day)
}

pub(crate) fn set_month(days: i32, month: u32) -> Result<i32, AstrolabeError> {
    let (year, _, day) = days_to_date(days);

    date_to_days(year, month, day)
}

pub(crate) fn set_day(days: i32, day: u32) -> Result<i32, AstrolabeError> {
    let (year, month, _) = days_to_date(days);

    date_to_days(year, month, day)
}

pub(crate) fn set_day_of_year(days: i32, day_of_year: u32) -> Result<i32, AstrolabeError> {
    let year = days_to_date(days).0;

    year_doy_to_days(year, day_of_year)
}

pub(crate) fn add_years(days: i32, years: u32) -> Result<i32, AstrolabeError> {
    let (year, month, mut day) = days_to_date(days);
    let mut target_year: i32 = year + years as i32;
    // Skip year 0
    if year < 0 && target_year >= 0 {
        target_year += 1;
    }

    if is_leap_year(year) && !is_leap_year(target_year) && month == 2 && day == 29 {
        day = 28;
    }

    date_to_days(target_year, month, day)
}

pub(crate) fn add_months(days: i32, months: u32) -> Result<i32, AstrolabeError> {
    let (year, month, day) = days_to_date(days);
    let mut total_months = year * 12 + month as i32 + months as i32 - 1;
    // Skip year 0
    if total_months <= 11 {
        total_months += 12;
    }

    let target_year = total_months / 12;
    let target_month = if (month + months) % 12 == 0 {
        12
    } else {
        (month + months) % 12
    };
    let target_day = match day {
        day if day < 29 => day,
        _ => {
            let (_, mdays) = year_month_to_doy(target_year, target_month).unwrap();
            if day > mdays {
                mdays
            } else {
                day
            }
        }
    };
    date_to_days(target_year, target_month, target_day)
}

pub(crate) fn add_days(old_days: i32, days: u32) -> Result<i32, AstrolabeError> {
    old_days.checked_add(days as i32).ok_or_else(|| {
        create_custom_oor(format!(
            "Instance would result into an overflow if {} days were added.",
            days,
        ))
    })
}

pub(crate) fn sub_years(days: i32, years: u32) -> Result<i32, AstrolabeError> {
    let (year, month, mut day) = days_to_date(days);
    let mut target_year: i32 = year - years as i32;
    // Skip year 0
    if year > 0 && target_year <= 0 {
        target_year -= 1;
    }

    if is_leap_year(year) && !is_leap_year(target_year) && month == 2 && day == 29 {
        day = 28;
    }

    date_to_days(target_year, month, day)
}

pub(crate) fn sub_months(days: i32, months: u32) -> Result<i32, AstrolabeError> {
    let (year, month, day) = days_to_date(days);
    let mut total_months = year * 12 + month as i32 - months as i32 - 1;
    // Skip year 0
    if total_months <= 11 {
        if year > 0 {
            total_months -= 24;
        } else {
            total_months -= 12;
        }
    }

    let target_year = total_months / 12;
    let target_month = if (month - months) % 12 == 0 {
        12
    } else {
        (month - months) % 12
    };
    let target_day = match day {
        day if day < 29 => day,
        _ => {
            let (_, mdays) = year_month_to_doy(target_year, target_month).unwrap();
            if day > mdays {
                mdays
            } else {
                day
            }
        }
    };
    date_to_days(target_year, target_month, target_day)
}

pub(crate) fn sub_days(old_days: i32, days: u32) -> Result<i32, AstrolabeError> {
    old_days.checked_sub(days as i32).ok_or_else(|| {
        create_custom_oor(format!(
            "Instance would result into an overflow if {} days were added.",
            days,
        ))
    })
}
