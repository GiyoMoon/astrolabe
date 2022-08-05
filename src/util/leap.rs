/// Returns leap years between the year 0001 and the given year (exluding the year itself)
pub(crate) fn leap_years(mut year: i32) -> u32 {
    if year.is_positive() {
        year -= 1;
    }
    let year_abs = year.abs();
    let mut leaps = year_abs / 4 - year_abs / 100 + year_abs / 400;
    if year.is_negative() {
        leaps += 1;
    }
    leaps as u32
}

/// Checks if the given year is a leap year
pub(crate) fn is_leap_year(year: i32) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}
