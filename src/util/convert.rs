use super::leap::{is_leap_year, leaps_since_epoch};
use crate::DateTimeError;

const SECS_PER_MINUTE: u64 = 60;
const SECS_PER_HOUR: u64 = 60 * SECS_PER_MINUTE;
const SECS_PER_DAY: u64 = 24 * SECS_PER_HOUR;
const SECS_PER_WEEK: u64 = 7 * SECS_PER_DAY;

/// Converts a unix timestamp to date units (year, month and day of month)
/// Logic originally released by the musl project (http://www.musl-libc.org/) under the MIT license. Taken from https://git.musl-libc.org/cgit/musl/tree/src/time/__secs_to_tm.c
pub(crate) fn ts_to_d_units(timestamp: u64) -> (u64, u64, u64) {
    // 2000-03-01
    const LEAPOCH: i64 = 11017;
    const DAYS_PER_400Y: i64 = 365 * 400 + 97;
    const DAYS_PER_100Y: i64 = 365 * 100 + 24;
    const DAYS_PER_4Y: i64 = 365 * 4 + 1;
    const SECS_PER_DAY: i64 = 60 * 60 * 24;
    const MONTH_DAYS: [i64; 12] = [31, 30, 31, 30, 31, 31, 30, 31, 30, 31, 31, 29];

    let days = (timestamp as i64 / SECS_PER_DAY) - LEAPOCH;

    let mut qc_cycles = days / DAYS_PER_400Y;
    let mut remdays = days % DAYS_PER_400Y;

    if remdays < 0 {
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
    (year as u64, mon, mday as u64)
}

/// Converts a unix timestamp to subday time units (hour, min, sec)
pub(crate) fn ts_to_t_units(timestamp: u64) -> (u64, u64, u64) {
    let subday_sec = (timestamp % SECS_PER_DAY as u64) as u64;
    let hour = subday_sec / 3600;
    let min = subday_sec / 60 % 60;
    let sec = subday_sec % 60;
    (hour, min, sec)
}

/// Converts a date (year, month and day of month) to days since January 1, 1970
pub(crate) fn date_to_days(year: u64, month: u64, day: u64) -> Result<u64, DateTimeError> {
    if year < 1970 {
        return Err(DateTimeError::OutOfRange);
    }

    let leap_years = leaps_since_epoch(year);
    let (mut ydays, mdays) = month_to_ymdays(year, month)?;

    if day > mdays || day == 0 {
        return Err(DateTimeError::OutOfRange);
    }
    ydays += day - 1;

    Ok((year - 1970) * 365 + leap_years + ydays as u64)
}

/// Converts year and month to the days until this month and days in the current month
pub(crate) fn month_to_ymdays(year: u64, month: u64) -> Result<(u64, u64), DateTimeError> {
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
            _ => return Err(DateTimeError::OutOfRange),
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
            _ => return Err(DateTimeError::OutOfRange),
        })
    }
}

/// Converts a timestamp to day of year
pub(crate) fn ts_to_yday(timestamp: u64) -> u64 {
    let (year, month, day) = ts_to_d_units(timestamp);
    // Using unwrap because it's safe to assume that month is valid
    let (ydays, _) = month_to_ymdays(year, month).unwrap();
    ydays + day
}

/// Converts a timestamp to day of week
pub(crate) fn ts_to_wday(timestamp: u64, monday_first: bool) -> u64 {
    (timestamp % SECS_PER_WEEK / SECS_PER_DAY + if monday_first { 3 } else { 4 }) % 7
}

/// Converts a timestamp to week of year
/// Formula taken from https://tondering.dk/claus/cal/week.php#calcweekno
pub(crate) fn ts_to_wyear(timestamp: u64) -> u64 {
    let (year, month, day) = ts_to_d_units(timestamp);
    let year = year as i64;
    let month = month as i64;
    let day = day as i64;

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
        n if n < 0 => (53 - (g - s) / 5) as u64,
        n if n > 364 + s => 1,
        _ => (n / 7 + 1) as u64,
    }
}
