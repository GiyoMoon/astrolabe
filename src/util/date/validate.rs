use crate::{
    errors::{
        out_of_range::{create_conditional_oor, create_simple_oor},
        AstrolabeError, OutOfRange,
    },
    shared::{MAX_DATE, MIN_DATE},
    util::leap::is_leap_year,
};

/// Checks if the given date (year, month and day of month) is in the valid range for the [`Date`]/[`DateTime`] struct
pub(crate) fn validate_date(year: i32, month: u32, day: u32) -> Result<(), AstrolabeError> {
    if year == 0 {
        return Err(AstrolabeError::OutOfRange(OutOfRange {
            name: "year",
            min: MIN_DATE.0 as i128,
            max: MAX_DATE.0 as i128,
            value: year as i128,
            custom: Some("Year cannot be 0. After the year -1 comes 1.".to_string()),
            conditional: None,
        }));
    } else if year < MIN_DATE.0 {
        return Err(create_simple_oor(
            "year",
            MIN_DATE.0 as i128,
            MAX_DATE.0 as i128,
            year as i128,
        ));
    } else if year == MIN_DATE.0 && month < MIN_DATE.1 {
        return Err(create_conditional_oor(
            "month",
            MIN_DATE.1 as i128,
            12,
            month as i128,
            format!("because year is {}", year),
        ));
    } else if year == MIN_DATE.0 && month == MIN_DATE.1 && day < MIN_DATE.2 {
        return Err(create_conditional_oor(
            "day",
            MIN_DATE.2 as i128,
            30,
            day as i128,
            format!("because year is {} and month is {}", year, month),
        ));
    } else if year > MAX_DATE.0 {
        return Err(create_simple_oor(
            "year",
            MIN_DATE.0 as i128,
            MAX_DATE.0 as i128,
            year as i128,
        ));
    } else if year == MAX_DATE.0 && month > MAX_DATE.1 {
        return Err(create_conditional_oor(
            "month",
            1,
            MAX_DATE.1 as i128,
            month as i128,
            format!("because year is {}", year),
        ));
    } else if year == MAX_DATE.0 && month == MAX_DATE.1 && day > MAX_DATE.2 {
        return Err(create_conditional_oor(
            "day",
            1,
            MAX_DATE.2 as i128,
            day as i128,
            format!("because year is {} and month is {}", year, month),
        ));
    }

    Ok(())
}

/// Checks if the given year and day of year is in the valid range for the [`Date`]/[`DateTime`] struct
pub(crate) fn validate_doy(year: i32, doy: u32) -> Result<(), AstrolabeError> {
    if year == 0 {
        return Err(AstrolabeError::OutOfRange(OutOfRange {
            name: "year",
            min: MIN_DATE.0 as i128,
            max: MAX_DATE.0 as i128,
            value: year as i128,
            custom: Some("Year cannot be 0. After the year -1 comes 1.".to_string()),
            conditional: None,
        }));
    } else if year < MIN_DATE.0 {
        return Err(create_simple_oor(
            "year",
            MIN_DATE.0 as i128,
            MAX_DATE.0 as i128,
            year as i128,
        ));
    } else if year == MIN_DATE.0 && doy < MIN_DATE.3 {
        return Err(create_conditional_oor(
            "day of year",
            MIN_DATE.3 as i128,
            365,
            doy as i128,
            format!("because year is {}", year),
        ));
    } else if year > MAX_DATE.0 {
        return Err(create_simple_oor(
            "year",
            MIN_DATE.0 as i128,
            MAX_DATE.0 as i128,
            year as i128,
        ));
    } else if year == MAX_DATE.0 && doy > MAX_DATE.3 {
        return Err(create_conditional_oor(
            "day of year",
            1,
            MAX_DATE.3 as i128,
            doy as i128,
            format!("because year is {}", year),
        ));
    } else if is_leap_year(year) && doy > 366 {
        return Err(create_conditional_oor(
            "day of year",
            1,
            366,
            doy as i128,
            format!("because year is {}", year),
        ));
    } else if !is_leap_year(year) && doy > 365 {
        return Err(create_conditional_oor(
            "day of year",
            1,
            365,
            doy as i128,
            format!("because year is {}", year),
        ));
    } else if doy < 1 {
        return Err(create_conditional_oor(
            "day of year",
            1,
            if is_leap_year(year) { 366 } else { 365 },
            doy as i128,
            format!("because year is {}", year),
        ));
    }

    Ok(())
}
