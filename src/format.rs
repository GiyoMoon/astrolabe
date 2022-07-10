use crate::DateTimeError;

pub fn format_part(chars: &str, timestamp: u64) -> Result<String, DateTimeError> {
    let first_char = chars.chars().next().ok_or(DateTimeError::InvalidFormat)?;
    Ok(match first_char {
        'y' => match chars.len() {
            2 => {
                let year = get_date_val(timestamp, DateValue::Year).to_string();
                let last_two = &year[year.len() - 2..];
                last_two.to_string()
            }
            _ => zero_padded(get_date_val(timestamp, DateValue::Year), chars.len()),
        },
        'M' => format_month(chars.len(), timestamp)?,
        'd' => format_days(chars.len(), timestamp),
        'h' => {
            let hour = if get_time_val(timestamp, TimeValue::Hour) % 12 == 0 {
                12
            } else {
                get_time_val(timestamp, TimeValue::Hour) % 12
            };
            zero_padded(hour, get_length(chars.len(), 2, 2))
        }
        'H' => zero_padded(
            get_time_val(timestamp, TimeValue::Hour),
            get_length(chars.len(), 2, 2),
        ),
        'K' => zero_padded(
            get_time_val(timestamp, TimeValue::Hour) % 12,
            get_length(chars.len(), 2, 2),
        ),
        'k' => {
            let hour = if get_time_val(timestamp, TimeValue::Hour) == 0 {
                24
            } else {
                get_time_val(timestamp, TimeValue::Hour)
            };
            zero_padded(hour, get_length(chars.len(), 2, 2))
        }
        'm' => zero_padded(
            get_time_val(timestamp, TimeValue::Min),
            get_length(chars.len(), 2, 2),
        ),
        's' => zero_padded(
            get_time_val(timestamp, TimeValue::Sec),
            get_length(chars.len(), 2, 2),
        ),
        _ => chars.to_string(),
    })
}

pub fn zero_padded(number: u64, length: usize) -> String {
    format!("{:0width$}", number, width = length)
}

fn get_length(length: usize, default: usize, max: usize) -> usize {
    if length > max {
        default
    } else {
        length
    }
}

fn format_month(length: usize, secs_since_epoch: u64) -> Result<String, DateTimeError> {
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

    Ok(match length {
        1 | 2 => zero_padded(month, length),
        3 => MONTH_ABBREVIATED
            .into_iter()
            .nth((month - 1) as usize)
            .ok_or(DateTimeError::InvalidFormat)?
            .to_string(),
        4 => MONTH_WIDE
            .into_iter()
            .nth((month - 1) as usize)
            .ok_or(DateTimeError::InvalidFormat)?
            .to_string(),
        5 => MONTH_NARROW
            .into_iter()
            .nth((month - 1) as usize)
            .ok_or(DateTimeError::InvalidFormat)?
            .to_string(),
        _ => MONTH_WIDE
            .into_iter()
            .nth((month - 1) as usize)
            .ok_or(DateTimeError::InvalidFormat)?
            .to_string(),
    })
}

fn format_days(length: usize, secs_since_epoch: u64) -> String {
    let days = get_date_val(secs_since_epoch, DateValue::Day);

    zero_padded(days, get_length(length, 2, 2))
}

// Most of the logic is taken from https://git.musl-libc.org/cgit/musl/tree/src/time/__secs_to_tm.c (MIT license)
pub fn date_from_timestamp(timestamp: u64) -> (u64, u64, u64) {
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

pub fn time_from_timestamp(timestamp: u64) -> (u64, u64, u64) {
    const SECS_PER_MINUTE: u64 = 60;
    const SECS_PER_HOUR: u64 = 60 * SECS_PER_MINUTE;
    const SECS_PER_DAY: u64 = 24 * SECS_PER_HOUR;
    let remaining_secs = timestamp % SECS_PER_DAY;
    let hour = remaining_secs / 3600;
    let min = remaining_secs / 60 % 60;
    let sec = remaining_secs % 60;
    (hour, min, sec)
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
    Min,
    Sec,
}

fn get_date_val(timestamp: u64, value: DateValue) -> u64 {
    match value {
        DateValue::Year => date_from_timestamp(timestamp).0,
        DateValue::Month => date_from_timestamp(timestamp).1,
        DateValue::Day => date_from_timestamp(timestamp).2,
    }
}

fn get_time_val(timestamp: u64, value: TimeValue) -> u64 {
    match value {
        TimeValue::Hour => time_from_timestamp(timestamp).0,
        TimeValue::Min => time_from_timestamp(timestamp).1,
        TimeValue::Sec => time_from_timestamp(timestamp).2,
    }
}
