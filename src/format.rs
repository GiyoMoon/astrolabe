use crate::DateTimeError;
use std::time::Duration;

pub fn format_part(chars: &str, duration: Duration) -> Result<String, DateTimeError> {
    let first_char = chars.chars().next().ok_or(DateTimeError::InvalidFormat)?;
    Ok(match first_char {
        'y' => match chars.len() {
            2 => {
                let year = get_date_val(duration.as_secs(), DateValue::Year).to_string();
                let last_two = &year[year.len() - 2..];
                last_two.to_string()
            }
            _ => format!(
                "{:0width$}",
                get_date_val(duration.as_secs(), DateValue::Year),
                width = chars.len()
            ),
        },
        'M' => format_month(chars.len(), duration.as_secs()),
        'd' => format_days(chars.len(), duration.as_secs()),
        'h' => {
            let hour = if get_time_val(duration.as_secs(), TimeValue::Hour) % 12 == 0 {
                12
            } else {
                get_time_val(duration.as_secs(), TimeValue::Hour) % 12
            };
            format!("{:0width$}", hour, width = get_length(chars.len(), 2, 2))
        }
        'H' => {
            format!(
                "{:0width$}",
                get_time_val(duration.as_secs(), TimeValue::Hour),
                width = get_length(chars.len(), 2, 2)
            )
        }
        'K' => format!(
            "{:0width$}",
            get_time_val(duration.as_secs(), TimeValue::Hour) % 12,
            width = get_length(chars.len(), 2, 2)
        ),
        'k' => {
            let hour = if get_time_val(duration.as_secs(), TimeValue::Hour) == 0 {
                24
            } else {
                get_time_val(duration.as_secs(), TimeValue::Hour)
            };
            format!("{:0width$}", hour, width = get_length(chars.len(), 2, 2))
        }
        'm' => format!(
            "{:0width$}",
            get_time_val(duration.as_secs(), TimeValue::Minute),
            width = get_length(chars.len(), 2, 2)
        ),
        's' => format!(
            "{:0width$}",
            get_time_val(duration.as_secs(), TimeValue::Second),
            width = get_length(chars.len(), 2, 2)
        ),
        _ => chars.to_string(),
    })
}

fn get_length(length: usize, default: usize, max: usize) -> usize {
    if length > max {
        default
    } else {
        length
    }
}

fn format_month(length: usize, secs_since_epoch: u64) -> String {
    let month = get_date_val(secs_since_epoch, DateValue::Month);
    const MONTH_ABBREVIATED: [&str; 12] = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];
    const MONTH_WIDE: [&str; 12] = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    const MONTH_NARROW: [&str; 12] = ["J", "F", "M", "A", "M", "J", "J", "A", "S", "O", "N", "D"];

    match length {
        1 | 2 => format!("{:0width$}", month, width = length),
        3 => MONTH_ABBREVIATED
            .into_iter()
            .nth((month - 1) as usize)
            .unwrap()
            .to_string(),
        4 => MONTH_WIDE
            .into_iter()
            .nth((month - 1) as usize)
            .unwrap()
            .to_string(),
        5 => MONTH_NARROW
            .into_iter()
            .nth((month - 1) as usize)
            .unwrap()
            .to_string(),
        _ => MONTH_WIDE
            .into_iter()
            .nth((month - 1) as usize)
            .unwrap()
            .to_string(),
    }
}

fn format_days(length: usize, secs_since_epoch: u64) -> String {
    let days = get_date_val(secs_since_epoch, DateValue::Day);

    format!("{:0width$}", days, width = get_length(length, 2, 2))
}

#[derive(PartialEq)]
enum DateValue {
    Year,
    Month,
    Day,
}

#[derive(PartialEq)]
enum TimeValue {
    Hour,
    Minute,
    Second,
}

// Most of the logic is taken from https://git.musl-libc.org/cgit/musl/tree/src/time/__secs_to_tm.c (MIT license)
fn get_date_val(secs_since_epoch: u64, value: DateValue) -> i64 {
    // 2000-03-01
    const LEAPOCH: i64 = 11017;
    const DAYS_PER_400Y: i64 = 365 * 400 + 97;
    const DAYS_PER_100Y: i64 = 365 * 100 + 24;
    const DAYS_PER_4Y: i64 = 365 * 4 + 1;
    const SECS_PER_DAY: u64 = 60 * 60 * 24;
    const MONTH_DAYS: [i64; 12] = [31, 30, 31, 30, 31, 31, 30, 31, 30, 31, 31, 29];

    let days = (secs_since_epoch / SECS_PER_DAY) as i64 - LEAPOCH;

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

    match value {
        DateValue::Year => year,
        DateValue::Month => mon,
        DateValue::Day => mday,
    }
}

fn get_time_val(secs_since_epoch: u64, value: TimeValue) -> i64 {
    const SECS_PER_MINUTE: u64 = 60;
    const SECS_PER_HOUR: u64 = 60 * SECS_PER_MINUTE;
    const SECS_PER_DAY: u64 = 24 * SECS_PER_HOUR;
    let remaining_secs = secs_since_epoch % SECS_PER_DAY;
    let hour = remaining_secs / 3600;
    let minute = remaining_secs / 60 % 60;
    let seconds = remaining_secs % 60;

    match value {
        TimeValue::Hour => hour as i64,
        TimeValue::Minute => minute as i64,
        TimeValue::Second => seconds as i64,
    }
}
