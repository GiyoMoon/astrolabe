use super::convert::{days_to_d_units, days_to_wyear, days_to_yday, nanos_to_t_units};
use crate::{
    shared::{NANOS_PER_SEC, SECS_PER_DAY, SECS_PER_HOUR, SECS_PER_MINUTE},
    util::convert::days_to_wday,
    AstrolabeError,
};

// Parse a format string and return parts to format
pub fn parse_format_string(format: &str) -> Result<Vec<String>, AstrolabeError> {
    let escaped_format = format.replace("''", "\u{0000}");

    let mut parts: Vec<String> = Vec::new();
    let mut currently_escaped = false;
    for char in escaped_format.chars() {
        match char {
            '\'' => {
                if !currently_escaped {
                    parts.push(char.to_string());
                } else {
                    parts
                        .last_mut()
                        .ok_or(AstrolabeError::InvalidFormat)?
                        .push(char);
                }
                currently_escaped = !currently_escaped;
            }
            _ => {
                if (currently_escaped || parts.last().unwrap_or(&"".to_string()).starts_with(char))
                    && parts.last().is_some()
                {
                    parts
                        .last_mut()
                        .ok_or(AstrolabeError::InvalidFormat)?
                        .push(char);
                } else {
                    parts.push(char.to_string());
                }
            }
        };
    }
    Ok(parts)
}

/// Formats string parts based on https://www.unicode.org/reports/tr35/tr35-dates.html#table-date-field-symbol-table
/// **Note**: Not all field types/symbols are implemented.
pub fn format_part(
    chars: &str,
    days: i32,
    nanoseconds: u64,
    // offset: i32,
) -> Result<String, AstrolabeError> {
    let first_char = chars.chars().next().ok_or(AstrolabeError::InvalidFormat)?;
    Ok(match first_char {
        'G' | 'y' | 'q' | 'M' | 'w' | 'd' | 'D' | 'e' => format_date_part(chars, days)?,
        'a' | 'b' | 'h' | 'H' | 'K' | 'k' | 'm' | 's' => format_time_part(chars, nanoseconds)?,
        // 'X' => format_zone(offset, chars.len(), true),
        // 'x' => format_zone(offset, chars.len(), false),
        _ => chars.to_string(),
    })
}

/// Formats string parts based on https://www.unicode.org/reports/tr35/tr35-dates.html#table-date-field-symbol-table
/// This function only formats date parts while ignoring time related parts (E.g. hour, minute)
pub fn format_date_part(chars: &str, days: i32) -> Result<String, AstrolabeError> {
    let first_char = chars.chars().next().ok_or(AstrolabeError::InvalidFormat)?;
    Ok(match first_char {
        'G' => match chars.len() {
            1 | 2 | 3 => {
                if days.is_negative() {
                    "BC".to_string()
                } else {
                    "AD".to_string()
                }
            }
            5 => {
                if days.is_negative() {
                    "B".to_string()
                } else {
                    "A".to_string()
                }
            }
            _ => {
                if days.is_negative() {
                    "Before Christ".to_string()
                } else {
                    "Anno Domini".to_string()
                }
            }
        },
        'y' => match chars.len() {
            2 => {
                let mut year = days_to_d_units(days).0;
                let year_string = year.to_string();

                if year_string.len() > 2 {
                    let last_two = &year_string[year_string.len() - 2..];
                    year = last_two.parse::<i32>().unwrap();
                }
                zero_padded_i(year, 2)
            }
            _ => zero_padded_i(days_to_d_units(days).0, chars.len()),
        },
        'q' => {
            let quarter = (days_to_d_units(days).1 - 1) / 3 + 1;
            match chars.len() {
                1 | 2 => zero_padded(quarter, chars.len()),
                3 => format!("Q{}", quarter),
                4 => {
                    let ordinal = add_ordinal_indicator(quarter);
                    format!("{} quarter", ordinal)
                }
                _ => zero_padded(quarter, 1),
            }
        }
        'M' => format_month(chars.len(), days)?,
        'w' => zero_padded(days_to_wyear(days), get_length(chars.len(), 2, 2)),
        'd' => zero_padded(days_to_d_units(days).2, get_length(chars.len(), 2, 2)),
        'D' => zero_padded(days_to_yday(days), get_length(chars.len(), 1, 3)),
        'e' => format_wday(days, chars.len())?,
        _ => chars.to_string(),
    })
}

/// Formats string parts based on https://www.unicode.org/reports/tr35/tr35-dates.html#table-date-field-symbol-table
/// This function only formats time parts while ignoring date related parts (E.g. year, day)
pub fn format_time_part(chars: &str, nanoseconds: u64) -> Result<String, AstrolabeError> {
    let first_char = chars.chars().next().ok_or(AstrolabeError::InvalidFormat)?;
    Ok(match first_char {
        'a' => format_period(nanoseconds, get_length(chars.len(), 3, 5), false),
        'b' => format_period(nanoseconds, get_length(chars.len(), 3, 5), true),
        'h' => {
            let hour = if nanos_to_t_units(nanoseconds).0 % 12 == 0 {
                12
            } else {
                nanos_to_t_units(nanoseconds).0 % 12
            };
            zero_padded(hour, get_length(chars.len(), 2, 2))
        }
        'H' => zero_padded(
            nanos_to_t_units(nanoseconds).0,
            get_length(chars.len(), 2, 2),
        ),
        'K' => zero_padded(
            nanos_to_t_units(nanoseconds).0 % 12,
            get_length(chars.len(), 2, 2),
        ),
        'k' => {
            let hour = if nanos_to_t_units(nanoseconds).0 == 0 {
                24
            } else {
                nanos_to_t_units(nanoseconds).0
            };
            zero_padded(hour, get_length(chars.len(), 2, 2))
        }
        'm' => zero_padded(
            nanos_to_t_units(nanoseconds).1,
            get_length(chars.len(), 2, 2),
        ),
        's' => zero_padded(
            nanos_to_t_units(nanoseconds).2,
            get_length(chars.len(), 2, 2),
        ),
        _ => chars.to_string(),
    })
}

/// Formats the month of a date based on https://www.unicode.org/reports/tr35/tr35-dates.html#dfst-month
fn format_month(length: usize, days: i32) -> Result<String, AstrolabeError> {
    let month = days_to_d_units(days).1;
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
            .ok_or(AstrolabeError::InvalidFormat)?
            .to_string(),
        5 => MONTH_NARROW
            .into_iter()
            .nth((month - 1) as usize)
            .ok_or(AstrolabeError::InvalidFormat)?
            .to_string(),
        _ => MONTH_WIDE
            .into_iter()
            .nth((month - 1) as usize)
            .ok_or(AstrolabeError::InvalidFormat)?
            .to_string(),
    })
}

/// Formats the week day of a date based on https://www.unicode.org/reports/tr35/tr35-dates.html#dfst-month
fn format_wday(days: i32, length: usize) -> Result<String, AstrolabeError> {
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
        1 | 2 => zero_padded(days_to_wday(days, false) + 1, length),
        3 => MONTH_ABBREVIATED
            .into_iter()
            .nth(days_to_wday(days, false) as usize)
            .ok_or(AstrolabeError::InvalidFormat)?
            .to_string(),
        4 => MONTH_WIDE
            .into_iter()
            .nth(days_to_wday(days, false) as usize)
            .ok_or(AstrolabeError::InvalidFormat)?
            .to_string(),
        5 => MONTH_NARROW
            .into_iter()
            .nth(days_to_wday(days, false) as usize)
            .ok_or(AstrolabeError::InvalidFormat)?
            .to_string(),
        6 => MONTH_SHORT
            .into_iter()
            .nth(days_to_wday(days, false) as usize)
            .ok_or(AstrolabeError::InvalidFormat)?
            .to_string(),
        7 => zero_padded(days_to_wday(days, true) + 1, 1),
        8 => zero_padded(days_to_wday(days, true) + 1, 2),
        _ => zero_padded(days_to_wday(days, false) + 1, 1),
    })
}

fn format_period(nanos: u64, length: usize, seperate_12: bool) -> String {
    const FORMATS: [[&str; 4]; 5] = [
        ["AM", "PM", "noon", "midnight"],
        ["AM", "PM", "noon", "midnight"],
        ["am", "pm", "noon", "midnight"],
        ["a.m.", "p.m.", "noon", "midnight"],
        ["a", "p", "n", "mi"],
    ];
    let time = (nanos / NANOS_PER_SEC) as u32 % SECS_PER_DAY;

    match time {
        time if seperate_12 && time == 0 => {
            FORMATS.into_iter().nth(length - 1).unwrap()[3].to_string()
        }
        time if seperate_12 && time == 43200 => {
            FORMATS.into_iter().nth(length - 1).unwrap()[2].to_string()
        }
        time if time < 43200 => FORMATS.into_iter().nth(length - 1).unwrap()[0].to_string(),
        _ => FORMATS.into_iter().nth(length - 1).unwrap()[1].to_string(),
    }
}

#[allow(dead_code)]
fn format_zone(offset: i32, length: usize, with_z: bool) -> String {
    if with_z && offset == 0 {
        return "Z".to_string();
    }

    let hour = offset.unsigned_abs() / SECS_PER_HOUR;
    let min = offset.unsigned_abs() % SECS_PER_HOUR / SECS_PER_MINUTE;
    let sec = offset.unsigned_abs() % SECS_PER_HOUR % SECS_PER_MINUTE;
    let prefix = if offset.is_negative() { "-" } else { "+" };

    match length {
        1 => {
            format!(
                "{}{}{}",
                prefix,
                zero_padded(hour, 2),
                if min != 0 {
                    zero_padded(min, 2)
                } else {
                    "".to_string()
                }
            )
        }
        2 => {
            format!("{}{}{}", prefix, zero_padded(hour, 2), zero_padded(min, 2))
        }
        4 => {
            format!(
                "{}{}{}{}",
                prefix,
                zero_padded(hour, 2),
                zero_padded(min, 2),
                if sec != 0 {
                    zero_padded(sec, 2)
                } else {
                    "".to_string()
                }
            )
        }
        5 => {
            format!(
                "{}{}:{}{}",
                prefix,
                zero_padded(hour, 2),
                zero_padded(min, 2),
                if sec != 0 {
                    format!(":{}", zero_padded(sec, 2))
                } else {
                    "".to_string()
                }
            )
        }
        _ => {
            format!("{}{}:{}", prefix, zero_padded(hour, 2), zero_padded(min, 2))
        }
    }
}

/// Formats a number as a zero padded string
pub(crate) fn zero_padded_i(number: i32, length: usize) -> String {
    format!(
        "{}{}",
        if number.is_negative() { "-" } else { "" },
        zero_padded(number.unsigned_abs(), length)
    )
}

/// Formats a number as a zero padded string
pub(crate) fn zero_padded(number: u32, length: usize) -> String {
    format!("{:0width$}", number, width = length)
}

/// Determines length of formatting part based on actual, default and max length
pub(crate) fn get_length(length: usize, default: usize, max: usize) -> usize {
    if length > max {
        default
    } else {
        length
    }
}

/// Formats a number as an ordinal number
pub(crate) fn add_ordinal_indicator(number: u32) -> String {
    match number {
        number if (number - 1) % 10 == 0 && number != 11 => format!("{}st", number),
        number if (number - 2) % 10 == 0 && number != 12 => format!("{}nd", number),
        number if (number - 3) % 10 == 0 && number != 13 => format!("{}rd", number),
        _ => format!("{}th", number),
    }
}
