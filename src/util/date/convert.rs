use super::validate::{validate_date, validate_doy};
use crate::{
    errors::{
        out_of_range::{create_conditional_oor, create_simple_oor},
        AstrolabeError,
    },
    util::leap::{is_leap_year, leap_years},
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
    validate_date(year, month, day)?;

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
        doy = if is_leap_year(year) { 366 } else { 365 } - doy;
        (year + 1) * 365 - leap_years as i32 - doy as i32
    } else {
        (year.abs() - 1) * 365 + leap_years as i32 + doy as i32
    })
}

/// Converts year and day of year to days since 01. January 0001
pub(crate) fn year_doy_to_days(year: i32, mut doy: u32) -> Result<i32, AstrolabeError> {
    validate_doy(year, doy)?;
    let leap_years = leap_years(year);

    doy -= 1;

    Ok(if year.is_negative() {
        doy = if is_leap_year(year) { 366 } else { 365 } - doy;
        (year + 1) * 365 - leap_years as i32 - doy as i32
    } else {
        (year.abs() - 1) * 365 + leap_years as i32 + doy as i32
    })
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

/// Converts days to week of year
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

/// Returns the years between two dates, considering day of year and subday nanoseconds
pub(crate) fn years_between(
    first_days: i32,
    first_nanos: u64,
    second_days: i32,
    second_nanos: u64,
) -> i32 {
    let first_year = days_to_date(first_days).0;
    let first_doy = days_to_doy(first_days);

    let second_year = days_to_date(second_days).0;
    let second_doy = days_to_doy(second_days);

    let mut years_between = first_year - second_year;

    // Fix needed as year 0 doesn't exist
    if first_year >= 1 && second_year < 1 {
        years_between -= 1
    } else if first_year < 1 && second_year >= 1 {
        years_between += 1
    };

    let extra_year = if years_between == 0 {
        0
    } else if first_year > second_year
        && (first_doy < second_doy || (first_doy == second_doy && first_nanos < second_nanos))
    {
        -1
    } else if first_year < second_year
        && (first_doy > second_doy || (first_doy == second_doy && first_nanos > second_nanos))
    {
        1
    } else {
        0
    };

    years_between + extra_year
}

/// Returns the months between two dates, considering day of month and subday nanoseconds
pub(crate) fn months_between(
    first_days: i32,
    first_nanos: u64,
    second_days: i32,
    second_nanos: u64,
) -> i32 {
    let (first_year, first_month, first_day) = days_to_date(first_days);
    let (second_year, second_month, second_day) = days_to_date(second_days);

    let mut years_between = first_year - second_year;

    // Fix needed as year 0 doesn't exist
    if first_year >= 1 && second_year < 1 {
        years_between -= 1
    } else if first_year < 1 && second_year >= 1 {
        years_between += 1
    };

    let months_between = years_between * 12 + first_month as i32 - second_month as i32;

    let extra_month = if months_between == 0 {
        0
    } else if first_year > second_year
        && (first_day < second_day || (first_day == second_day && first_nanos < second_nanos))
    {
        -1
    } else if first_year < second_year
        && (first_day > second_day || (first_day == second_day && first_nanos > second_nanos))
    {
        1
    } else {
        0
    };

    months_between + extra_month
}
