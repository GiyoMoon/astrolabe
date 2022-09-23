use super::leap::{is_leap_year, leap_years};
use crate::{
    errors::{
        out_of_range::{create_conditional_oor, create_simple_oor, OutOfRange},
        AstrolabeError,
    },
    shared::{
        MAX_DATE, MIN_DATE, NANOS_PER_DAY, NANOS_PER_SEC, SECS_PER_DAY_U64, SECS_PER_HOUR,
        SECS_PER_HOUR_U64, SECS_PER_MINUTE, SECS_PER_MINUTE_U64,
    },
    DateTimeUnit, DateUnit, TimeUnit,
};

/// Converts days (since 01. January 0001) to a date (year, month, day of month). Days can be negative.
///
/// Logic originally released by the musl project (http://www.musl-libc.org/) under the MIT license. Taken from https://git.musl-libc.org/cgit/musl/tree/src/time/__secs_to_tm.c
pub(crate) fn days_to_date(days: i32) -> (i32, u32, u32) {
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

    let q_cycles = remdays / DAYS_PER_4Y;

    // Doesn't seem to occur if days is of type i32
    // if q_cycles == 25 {
    //     q_cycles -= 1;
    // }

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

    // As there is no year 0, subtract one if year is lower than 1
    if year < 1 {
        year -= 1;
    }

    (year as i32, mon as u32, mday as u32)
}

/// Converts a date (year, month and day of month) to days since 01. January 0001
pub(crate) fn date_to_days(year: i32, month: u32, day: u32) -> Result<i32, AstrolabeError> {
    valid_range(year, month, day)?;

    let leap_years = leap_years(year);
    let (mut doy, mdays) = year_month_to_doy(year, month)?;

    if day > mdays || day == 0 {
        return Err(create_conditional_oor(
            "day",
            1,
            mdays as i128,
            day as i128,
            format!("because month is {}", month),
        ));
    }
    doy += day - 1;

    Ok(if year.is_negative() {
        doy = 365 - if is_leap_year(year) { doy + 1 } else { doy };
        (year + 1) * 365 - leap_years as i32 - doy as i32
    } else {
        (year.abs() - 1) * 365 + leap_years as i32 + doy as i32
    })
}

/// Converts year and day of year to days since 01. January 0001
pub(crate) fn year_doy_to_days(year: i32, mut doy: u32) -> Result<i32, AstrolabeError> {
    valid_range_doy(year, doy)?;
    let leap_years = leap_years(year);

    doy -= 1;

    Ok(if year.is_negative() {
        doy = 365 - if is_leap_year(year) { doy + 1 } else { doy };
        (year + 1) * 365 - leap_years as i32 - doy as i32
    } else {
        (year.abs() - 1) * 365 + leap_years as i32 + doy as i32
    })
}

/// Converts nanoseconds to time values (hour, min, sec)
pub(crate) fn nanos_to_time(nanos: u64) -> (u32, u32, u32) {
    let as_seconds = (nanos / NANOS_PER_SEC) as u32;
    let hour = as_seconds / SECS_PER_HOUR;
    let min = as_seconds / SECS_PER_MINUTE % SECS_PER_MINUTE;
    let sec = as_seconds % SECS_PER_MINUTE;
    (hour, min, sec)
}

/// Converts time values (hour, minute and seconds) to day seconds
pub(crate) fn time_to_day_seconds(hour: u32, min: u32, sec: u32) -> Result<u32, AstrolabeError> {
    valid_time(hour, min, sec)?;

    Ok(hour * SECS_PER_HOUR + min * SECS_PER_MINUTE + sec)
}

/// Returns a given [`TimeUnit`] from nanoseconds
pub(crate) fn nanos_to_unit(nanos: u64, unit: TimeUnit) -> u64 {
    match unit {
        TimeUnit::Hour => nanos / NANOS_PER_SEC / SECS_PER_HOUR_U64,
        TimeUnit::Min => nanos / NANOS_PER_SEC / SECS_PER_MINUTE_U64 % SECS_PER_MINUTE_U64,
        TimeUnit::Sec => nanos / NANOS_PER_SEC % 60,
        TimeUnit::Centis => nanos / 10_000_000 % 100,
        TimeUnit::Millis => nanos / 1_000_000 % 1_000,
        TimeUnit::Micros => nanos / 1_000 % 1_000_000,
        TimeUnit::Nanos => nanos % NANOS_PER_SEC,
    }
}

/// Converts year and month to the days until this month and days in the current month
pub(crate) fn year_month_to_doy(year: i32, month: u32) -> Result<(u32, u32), AstrolabeError> {
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
            _ => return Err(create_simple_oor("month", 1, 12, month as i128)),
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
            _ => return Err(create_simple_oor("month", 1, 12, month as i128)),
        })
    }
}

/// Converts days to day of year
pub(crate) fn days_to_doy(days: i32) -> u32 {
    let (year, month, day) = days_to_date(days);
    // Using unwrap because it's safe to assume that month is valid
    let (doy, _) = year_month_to_doy(year, month).unwrap();
    doy + day
}

/// Converts days to day of week
pub(crate) fn days_to_wday(days: i32, monday_first: bool) -> u32 {
    (days.unsigned_abs() % 7 + if monday_first { 0 } else { 1 }) % 7
}

/// Converts a timestamp to week of year
/// Formula taken from https://tondering.dk/claus/cal/week.php#calcweekno
pub(crate) fn days_to_wyear(days: i32) -> u32 {
    let (year, month, day) = days_to_date(days);
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

/// Converts a [`DateTimeUnit`] to [`DateUnit`]. If there is no corresponding date unit, the default, [`DateUnit::Day`], will be used.
pub(crate) fn dtu_to_du(unit: DateTimeUnit) -> DateUnit {
    match unit {
        DateTimeUnit::Year => DateUnit::Year,
        DateTimeUnit::Month => DateUnit::Month,
        _ => DateUnit::Day,
    }
}

/// Converts a [`DateTimeUnit`] to [`TimeUnit`]. If there is no corresponding time unit, the default, [`TimeUnit::Sec`], will be used.
pub(crate) fn dtu_to_tu(unit: DateTimeUnit) -> TimeUnit {
    match unit {
        DateTimeUnit::Hour => TimeUnit::Hour,
        DateTimeUnit::Min => TimeUnit::Min,
        DateTimeUnit::Centis => TimeUnit::Centis,
        DateTimeUnit::Millis => TimeUnit::Millis,
        DateTimeUnit::Micros => TimeUnit::Micros,
        DateTimeUnit::Nanos => TimeUnit::Nanos,
        _ => TimeUnit::Sec,
    }
}

/// Checks if the given date (year, month and day of month) is in the valid range for the [`Date`]/[`DateTime`] struct
pub(crate) fn valid_range(year: i32, month: u32, day: u32) -> Result<(), AstrolabeError> {
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
pub(crate) fn valid_range_doy(year: i32, doy: u32) -> Result<(), AstrolabeError> {
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

pub(crate) fn valid_time(hour: u32, min: u32, sec: u32) -> Result<(), AstrolabeError> {
    if hour > 23 {
        return Err(create_simple_oor("hour", 0, 23, hour as i128));
    }

    if min > 59 {
        return Err(create_simple_oor("min", 0, 59, min as i128));
    }

    if sec > 59 {
        return Err(create_simple_oor("sec", 0, 59, sec as i128));
    };

    Ok(())
}

/// Adds a given offest to nanoseconds
pub(crate) fn add_offset_to_nanos(nanoseconds: u64, offset: i32) -> u64 {
    (((nanoseconds as i64 + offset as i64 * NANOS_PER_SEC as i64) + NANOS_PER_DAY as i64)
        % NANOS_PER_DAY as i64)
        .unsigned_abs()
}

/// Removes a given offest from nanoseconds
pub(crate) fn remove_offset_from_nanos(nanoseconds: u64, offset: i32) -> u64 {
    (((nanoseconds as i64 - offset as i64 * NANOS_PER_SEC as i64) + NANOS_PER_DAY as i64)
        % NANOS_PER_DAY as i64)
        .unsigned_abs()
}

/// Adds a given offset to days and nanoseconds
pub(crate) fn add_offset_to_dn(days: i32, nanoseconds: u64, offset: i32) -> (i32, u64) {
    let mut nanos = days_nanos_to_nanos(days, nanoseconds);
    nanos += offset as i128 * NANOS_PER_SEC as i128;
    let (days, nanoseconds) = nanos_to_days_nanos(nanos).unwrap();
    (days, nanoseconds)
}

/// Removes a given offset from days and nanoseconds
pub(crate) fn remove_offset_from_dn(days: i32, nanoseconds: u64, offset: i32) -> (i32, u64) {
    let mut nanos = days_nanos_to_nanos(days, nanoseconds);
    nanos -= offset as i128 * NANOS_PER_SEC as i128;
    nanos_to_days_nanos(nanos).unwrap()
}

/// Converts days and nanoseconds to seconds
pub(crate) fn days_nanos_to_secs(mut days: i32, day_nanos: u64) -> i64 {
    let adjusted_day_seconds = if days.is_negative() {
        days += 1;
        -(SECS_PER_DAY_U64 as i64 - day_nanos as i64 / NANOS_PER_SEC as i64)
    } else {
        (day_nanos / NANOS_PER_SEC) as i64
    };
    days as i64 * SECS_PER_DAY_U64 as i64 + adjusted_day_seconds
}

/// Converts seconds to days and nanoseconds
pub(crate) fn secs_to_days_nanos(seconds: i64) -> Result<(i32, u64), AstrolabeError> {
    let day_seconds = seconds.unsigned_abs() % SECS_PER_DAY_U64;

    let days_i64 = if seconds.is_negative() && day_seconds != 0 {
        seconds / SECS_PER_DAY_U64 as i64 - 1
    } else {
        seconds / SECS_PER_DAY_U64 as i64
    };

    let days = days_i64.try_into().map_err(|_| {
        create_simple_oor(
            "seconds",
            i32::MIN as i128 * SECS_PER_DAY_U64 as i128,
            i32::MAX as i128 * SECS_PER_DAY_U64 as i128 + SECS_PER_DAY_U64 as i128 - 1,
            seconds as i128,
        )
    })?;

    let adjusted_day_seconds = if seconds.is_negative() && day_seconds != 0 {
        SECS_PER_DAY_U64 - day_seconds
    } else {
        day_seconds
    };

    Ok((days, adjusted_day_seconds * NANOS_PER_SEC))
}

/// Converts days and nanoseconds to nanoseconds
pub(crate) fn days_nanos_to_nanos(mut days: i32, day_nanos: u64) -> i128 {
    let adjusted_day_nanos = if days.is_negative() {
        days += 1;
        -(NANOS_PER_DAY as i128 - day_nanos as i128)
    } else {
        day_nanos as i128
    };
    days as i128 * NANOS_PER_DAY as i128 + adjusted_day_nanos
}

/// Converts nanoseconds to days and nanoseconds
pub(crate) fn nanos_to_days_nanos(nanoseconds: i128) -> Result<(i32, u64), AstrolabeError> {
    let day_nanos = (nanoseconds.unsigned_abs() % NANOS_PER_DAY as u128) as u64;

    let days_i128 = if nanoseconds.is_negative() && day_nanos != 0 {
        nanoseconds / NANOS_PER_DAY as i128 - 1
    } else {
        nanoseconds / NANOS_PER_DAY as i128
    };

    let days = days_i128.try_into().map_err(|_| {
        create_simple_oor(
            "nanoseconds",
            i32::MIN as i128 * NANOS_PER_DAY as i128,
            i32::MAX as i128 * NANOS_PER_DAY as i128 + NANOS_PER_DAY as i128 - 1,
            nanoseconds,
        )
    })?;

    let adjusted_day_nanos = if nanoseconds.is_negative() && day_nanos != 0 {
        NANOS_PER_DAY - day_nanos
    } else {
        day_nanos
    };

    Ok((days, adjusted_day_nanos))
}
