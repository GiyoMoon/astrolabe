use super::convert::{ts_to_d_units, ts_to_t_units, ts_to_yday};
use crate::{util::convert::ts_to_wday, DateTimeError};

/// Formats a number as a zero padded string
pub(crate) fn zero_padded(number: u64, length: usize) -> String {
    format!("{:0width$}", number, width = length)
}

/// Formats a number as an ordinal number
pub(crate) fn add_ordinal_indicator(number: u64) -> String {
    match number {
        number if (number - 1) % 10 == 0 && number != 11 => format!("{number}st"),
        number if (number - 2) % 10 == 0 && number != 12 => format!("{number}nd"),
        number if (number - 3) % 10 == 0 && number != 13 => format!("{number}rd"),
        _ => format!("{number}th"),
    }
}

/// Determines length of formatting part based on actual, default and max length
pub(crate) fn get_length(length: usize, default: usize, max: usize) -> usize {
    if length > max {
        default
    } else {
        length
    }
}

/// Formats string parts based on https://www.unicode.org/reports/tr35/tr35-dates.html#table-date-field-symbol-table
/// **Note**: Not all field types/symbols are implemented.
pub fn format_part(chars: &str, timestamp: u64) -> Result<String, DateTimeError> {
    let first_char = chars.chars().next().ok_or(DateTimeError::InvalidFormat)?;
    Ok(match first_char {
        // SystemTime can only be year 1970 or later, which is always AD
        'G' => match chars.len() {
            1 | 2 | 3 => "AD".to_string(),
            5 => "A".to_string(),
            _ => "Anno Domini".to_string(),
        },
        'y' => match chars.len() {
            2 => {
                let year = ts_to_d_units(timestamp).0.to_string();
                let last_two = &year[year.len() - 2..];
                last_two.to_string()
            }
            _ => zero_padded(ts_to_d_units(timestamp).0, chars.len()),
        },
        'q' => {
            let quarter = (ts_to_d_units(timestamp).1 - 1) / 3 + 1;
            match chars.len() {
                1 | 2 => zero_padded(quarter, chars.len()),
                3 => format!("Q{quarter}"),
                4 => {
                    let ordinal = add_ordinal_indicator(quarter);
                    format!("{ordinal} quarter")
                }
                _ => zero_padded(quarter, 1),
            }
        }
        'M' => format_month(chars.len(), timestamp)?,
        'd' => zero_padded(ts_to_d_units(timestamp).2, get_length(chars.len(), 2, 2)),
        'D' => zero_padded(ts_to_yday(timestamp), get_length(chars.len(), 1, 3)),
        'e' => format_wday(chars.len(), timestamp)?,
        'h' => {
            let hour = if ts_to_t_units(timestamp).0 % 12 == 0 {
                12
            } else {
                ts_to_t_units(timestamp).0 % 12
            };
            zero_padded(hour, get_length(chars.len(), 2, 2))
        }
        'H' => zero_padded(ts_to_t_units(timestamp).0, get_length(chars.len(), 2, 2)),
        'K' => zero_padded(
            ts_to_t_units(timestamp).0 % 12,
            get_length(chars.len(), 2, 2),
        ),
        'k' => {
            let hour = if ts_to_t_units(timestamp).0 == 0 {
                24
            } else {
                ts_to_t_units(timestamp).0
            };
            zero_padded(hour, get_length(chars.len(), 2, 2))
        }
        'm' => zero_padded(ts_to_t_units(timestamp).1, get_length(chars.len(), 2, 2)),
        's' => zero_padded(ts_to_t_units(timestamp).2, get_length(chars.len(), 2, 2)),
        _ => chars.to_string(),
    })
}

/// Formats the month of a date based on https://www.unicode.org/reports/tr35/tr35-dates.html#dfst-month
fn format_month(length: usize, timestamp: u64) -> Result<String, DateTimeError> {
    let month = ts_to_d_units(timestamp).1;
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

/// Formats the week day of a date based on https://www.unicode.org/reports/tr35/tr35-dates.html#dfst-month
fn format_wday(length: usize, timestamp: u64) -> Result<String, DateTimeError> {
    const MONTH_ABBREVIATED: [&str; 7] = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
    const MONTH_WIDE: [&str; 7] = [
        "Sunday",
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
    ];
    const MONTH_NARROW: [&str; 7] = ["S", "M", "T", "W", "T", "F", "S"];
    const MONTH_SHORT: [&str; 7] = ["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"];

    Ok(match length {
        1 | 2 => zero_padded(ts_to_wday(timestamp, false) + 1, length),
        3 => MONTH_ABBREVIATED
            .into_iter()
            .nth(ts_to_wday(timestamp, false) as usize)
            .ok_or(DateTimeError::InvalidFormat)?
            .to_string(),
        4 => MONTH_WIDE
            .into_iter()
            .nth(ts_to_wday(timestamp, false) as usize)
            .ok_or(DateTimeError::InvalidFormat)?
            .to_string(),
        5 => MONTH_NARROW
            .into_iter()
            .nth(ts_to_wday(timestamp, false) as usize)
            .ok_or(DateTimeError::InvalidFormat)?
            .to_string(),
        6 => MONTH_SHORT
            .into_iter()
            .nth(ts_to_wday(timestamp, false) as usize)
            .ok_or(DateTimeError::InvalidFormat)?
            .to_string(),
        7 => zero_padded(ts_to_wday(timestamp, true) + 1, 1),
        8 => zero_padded(ts_to_wday(timestamp, true) + 1, 2),
        _ => zero_padded(ts_to_wday(timestamp, false) + 1, 1),
    })
}
