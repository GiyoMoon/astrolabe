use super::leap::{is_leap_year, leap_years};
use crate::{
    shared::{MAX_DATE, MIN_DATE, NANOS_PER_SEC, SECS_PER_HOUR, SECS_PER_MINUTE},
    AstrolabeError,
};

// Converts days (since 01. January 0001) to date units (year, month, day of month)
/// Logic originally released by the musl project (http://www.musl-libc.org/) under the MIT license. Taken from https://git.musl-libc.org/cgit/musl/tree/src/time/__secs_to_tm.c
pub(crate) fn days_to_d_units(days: i32) -> (i32, u32, u32) {
    // 2000-03-01. Days since 0001-01-01
    const LEAPOCH: i64 = 730_179;
    const DAYS_PER_400Y: i64 = 365 * 400 + 97;
    const DAYS_PER_100Y: i64 = 365 * 100 + 24;
    const DAYS_PER_4Y: i64 = 365 * 4 + 1;
    const MONTH_DAYS: [i64; 12] = [31, 30, 31, 30, 31, 31, 30, 31, 30, 31, 31, 29];

    let days = days as i64 - LEAPOCH;

    let mut qc_cycles = days / DAYS_PER_400Y;
    let mut remdays = days % DAYS_PER_400Y;

    if remdays.is_negative() {
        remdays += DAYS_PER_400Y;
        qc_cycles -= 1;
    }

    let mut c_cycles = remdays / DAYS_PER_100Y;
    if c_cycles == 4 {
        c_cycles -= 1;
    }
    remdays -= c_cycles * DAYS_PER_100Y;

    let mut q_cycles = remdays / DAYS_PER_4Y;
    if q_cycles == 25 {
        q_cycles -= 1;
    }
    remdays -= q_cycles * DAYS_PER_4Y;

    let mut remyears = remdays / 365;
    if remyears == 4 {
        remyears -= 1;
    }
    let mut year = 2000 + remyears + 4 * q_cycles + 100 * c_cycles + 400 * qc_cycles;

    remdays -= remyears * 365;

    let mut mon = 0;
    for mdays in MONTH_DAYS.iter() {
        mon += 1;
        if remdays < *mdays {
            break;
        }
        remdays -= *mdays;
    }
    let mday = remdays + 1;

    let mon = if mon + 2 > 12 {
        year += 1;
        mon - 10
    } else {
        mon + 2
    };
    (year as i32, mon as u32, mday as u32)
}

// Converts nano seconds to time units (hour, min, sec)
pub(crate) fn nanos_to_t_units(nanos: u64) -> (u32, u32, u32) {
    let as_seconds = (nanos / NANOS_PER_SEC) as u32;
    let hour = as_seconds / SECS_PER_HOUR;
    let min = as_seconds / SECS_PER_MINUTE % SECS_PER_MINUTE;
    let sec = as_seconds % SECS_PER_MINUTE;
    (hour, min, sec)
}

// Converts a date (year, month and day of month) to days since 01. January 0001
pub(crate) fn date_to_days(year: i32, month: u32, day: u32) -> Result<i32, AstrolabeError> {
    valid_range(year, month, day)?;

    let leap_years = leap_years(year);
    let (mut ydays, mdays) = month_to_ymdays(year, month)?;

    if day > mdays || day == 0 {
        return Err(AstrolabeError::OutOfRange);
    }
    ydays += day - 1;

    Ok(if year <= 0 {
        ydays = if is_leap_year(year) { 366 } else { 365 } - ydays;
        year * 365 - leap_years as i32 - ydays as i32
    } else {
        (year.abs() - 1) * 365 + leap_years as i32 + ydays as i32
    })
}

/// Converts time values (hour, minute and seconds) to day seconds
pub(crate) fn time_to_day_seconds(hour: u32, min: u32, sec: u32) -> Result<u32, AstrolabeError> {
    if hour > 23 || min > 59 || sec > 59 {
        return Err(AstrolabeError::OutOfRange);
    }
    Ok(hour * SECS_PER_HOUR + min * SECS_PER_MINUTE + sec)
}

/// Converts year and month to the days until this month and days in the current month
pub(crate) fn month_to_ymdays(year: i32, month: u32) -> Result<(u32, u32), AstrolabeError> {
    let is_leap_year = is_leap_year(year);
    if is_leap_year {
        Ok(match month {
            1 => (0, 31),
            2 => (31, 29),
            3 => (60, 31),
            4 => (91, 30),
            5 => (121, 31),
            6 => (152, 30),
            7 => (182, 31),
            8 => (213, 31),
            9 => (244, 30),
            10 => (274, 31),
            11 => (305, 30),
            12 => (335, 31),
            _ => return Err(AstrolabeError::OutOfRange),
        })
    } else {
        Ok(match month {
            1 => (0, 31),
            2 => (31, 28),
            3 => (59, 31),
            4 => (90, 30),
            5 => (120, 31),
            6 => (151, 30),
            7 => (181, 31),
            8 => (212, 31),
            9 => (243, 30),
            10 => (273, 31),
            11 => (304, 30),
            12 => (334, 31),
            _ => return Err(AstrolabeError::OutOfRange),
        })
    }
}

/// Converts days to day of year
pub(crate) fn days_to_yday(days: i32) -> u32 {
    let (year, month, day) = days_to_d_units(days);
    // Using unwrap because it's safe to assume that month is valid
    let (ydays, _) = month_to_ymdays(year, month).unwrap();
    ydays + day
}

/// Converts days to day of week
pub(crate) fn days_to_wday(days: i32, monday_first: bool) -> u32 {
    (days.unsigned_abs() % 7 + if monday_first { 0 } else { 1 }) % 7
}

/// Converts a timestamp to week of year
/// Formula taken from https://tondering.dk/claus/cal/week.php#calcweekno
pub(crate) fn days_to_wyear(days: i32) -> u32 {
    let (year, month, day) = days_to_d_units(days);
    let month = month as i32;
    let day = day as i32;

    let a = if month <= 2 { year - 1 } else { year };
    let b = a / 4 - a / 100 + a / 400;
    let c = (a - 1) / 4 - (a - 1) / 100 + (a - 1) / 400;
    let s = b - c;
    let e = if month <= 2 { 0 } else { s + 1 };
    let f = if month <= 2 {
        day - 1 + 31 * (month - 1)
    } else {
        day + (153 * (month - 3) + 2) / 5 + 58 + s
    };
    let g = (a + b) % 7;
    let d = (f + g - e) % 7;
    let n = f + 3 - d;
    match n {
        n if n.is_negative() => (53 - (g - s) / 5) as u32,
        n if n > 364 + s => 1,
        _ => (n / 7 + 1) as u32,
    }
}

// Checks if the given date (year, month and day of month) is in the valid range for the [`Date`] struct
pub(crate) fn valid_range(year: i32, month: u32, day: u32) -> Result<(), AstrolabeError> {
    if year < MIN_DATE.0
        || (year == MIN_DATE.0 && (month < MIN_DATE.1 || month == MIN_DATE.1 && day < MIN_DATE.2))
    {
        return Err(AstrolabeError::OutOfRange);
    }
    if year > MAX_DATE.0
        || (year == MAX_DATE.0 && (month > MAX_DATE.1 || month == MAX_DATE.1 && day > MAX_DATE.2))
    {
        return Err(AstrolabeError::OutOfRange);
    }
    Ok(())
}
