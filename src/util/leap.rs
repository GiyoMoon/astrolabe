/// Returns the number of leap years since 1970 (excluding the specified year)
pub(crate) fn leaps_since_epoch(mut year: u64) -> u64 {
    year -= 1;
    (year - 1968) / 4 - (year - 1900) / 100 + (year - 1600) / 400
}

/// Returns true if year is a leap year
pub(crate) fn is_leap_year(year: u64) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}
