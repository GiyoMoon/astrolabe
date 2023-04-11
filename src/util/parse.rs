use super::format::get_length;
use crate::{
    errors::{invalid_format::create_invalid_format, AstrolabeError},
    shared::{
        MONTH_ABBREVIATED, MONTH_WIDE, SECS_PER_HOUR, SECS_PER_HOUR_U64, SECS_PER_MINUTE,
        SECS_PER_MINUTE_U64, WDAY_WIDE,
    },
    Date, DateUnit,
};

/// Parses the offset part from an RFC 3339 timestamp string to offset seconds
pub(crate) fn parse_offset(string: &str) -> Result<i32, AstrolabeError> {
    if string.starts_with('Z') {
        return Ok(0);
    }
    if string.len() != 6 {
        return Err(create_invalid_format(
            "Failed parsing the offset from the RFC 3339 string. Format should be +XX:XX or -XX:XX"
                .to_string(),
        ));
    }

    let hour = string[1..3].parse::<u32>().map_err(|_| {
        create_invalid_format(
            "Failed parsing the hour of the offset from the RFC 3339 string".to_string(),
        )
    })?;
    let min = string[4..6].parse::<u32>().map_err(|_| {
        create_invalid_format(
            "Failed parsing the minute of the offset from the RFC 3339 string".to_string(),
        )
    })?;

    if hour > 23 {
        return Err(create_invalid_format(
            "Failed parsing the hour of the offset from the RFC 3339 string. Hour has to be less than 24".to_string(),
        ));
    } else if min > 59 {
        return Err(create_invalid_format(
            "Failed parsing the minute of the offset from the RFC 3339 string. Minute has to be less than 60".to_string(),
        ));
    }

    let offset = hour * SECS_PER_HOUR + min * SECS_PER_MINUTE;

    Ok(if string.starts_with('+') {
        offset as i32
    } else {
        -(offset as i32)
    })
}

/// Parse a format string and return parts to format
pub(crate) fn parse_format_string(format: &str) -> Vec<String> {
    let escaped_format = format.replace("''", "\u{0000}");

    let mut parts: Vec<String> = Vec::new();
    let mut currently_escaped = false;

    for char in escaped_format.chars() {
        match char {
            '\'' => {
                if !currently_escaped {
                    parts.push(char.to_string());
                } else {
                    // Using unwrap because it's safe to assume that parts has a length of at least 1
                    parts.last_mut().unwrap().push(char);
                }
                currently_escaped = !currently_escaped;
            }
            _ => {
                if currently_escaped || parts.last().unwrap_or(&"".to_string()).starts_with(char) {
                    // Using unwrap because it's safe to assume that parts has a length of at least 1
                    parts.last_mut().unwrap().push(char);
                } else {
                    parts.push(char.to_string());
                }
            }
        };
    }
    parts
}

pub(crate) struct ParsedPart {
    pub(crate) value: i64,
    pub(crate) unit: ParseUnit,
}

pub(crate) enum ParseUnit {
    Year,
    Month,
    DayOfMonth,
    DayOfYear,
    Hour,
    Period,
    PeriodHour,
    Min,
    Sec,
    Decis,
    Centis,
    Millis,
    Micros,
    Nanos,
    Offset,
}

#[derive(Default)]
pub(crate) struct ParsedDate {
    pub(crate) year: Option<i32>,
    pub(crate) month: Option<u32>,
    pub(crate) day_of_month: Option<u32>,
    pub(crate) day_of_year: Option<u32>,
}

#[derive(Default)]
pub(crate) struct ParsedTime {
    pub(crate) hour: Option<u64>,
    pub(crate) period_hour: Option<u64>,
    pub(crate) period: Option<Period>,
    pub(crate) min: Option<u64>,
    pub(crate) sec: Option<u64>,
    pub(crate) decis: Option<u64>,
    pub(crate) centis: Option<u64>,
    pub(crate) millis: Option<u64>,
    pub(crate) micros: Option<u64>,
    pub(crate) nanos: Option<u64>,
    pub(crate) offset: Option<i32>,
}

pub(crate) enum Period {
    AM = 0,
    PM = 12,
}

/// Formats string parts based on https://www.unicode.org/reports/tr35/tr35-dates.html#table-date-field-symbol-table
/// **Note**: Not all field types/symbols are implemented.
pub(crate) fn parse_part(
    chars: &str,
    string: &mut String,
) -> Result<Option<ParsedPart>, AstrolabeError> {
    // Using unwrap because it's safe to assume that chars has a length of at least 1
    let first_char = chars.chars().next().unwrap();
    Ok(match first_char {
        'G' | 'y' | 'q' | 'M' | 'w' | 'd' | 'D' | 'e' => parse_date_part(chars, string)?,
        'a' | 'b' | 'h' | 'H' | 'K' | 'k' | 'm' | 's' | 'n' | 'X' | 'x' => {
            parse_time_part(chars, string)?
        }
        _ => {
            remove_part(chars.len(), string)?;
            None
        }
    })
}

/// Parse string parts based on https://www.unicode.org/reports/tr35/tr35-dates.html#table-date-field-symbol-table
/// This function only parses date parts while ignoring time related parts (E.g. hour, minute)
pub(crate) fn parse_date_part(
    chars: &str,
    string: &mut String,
) -> Result<Option<ParsedPart>, AstrolabeError> {
    // Using unwrap because it's safe to assume that chars has a length of at least 1
    let first_char = chars.chars().next().unwrap();
    Ok(match first_char {
        'G' => match chars.len() {
            1 | 2 | 3 => {
                remove_part(2, string)?;
                None
            }
            5 => {
                remove_part(1, string)?;
                None
            }
            _ => {
                if string.starts_with("Before Christ") {
                    // Using unwrap because it's safe to assume that the string is long enough
                    remove_part("Before Christ".len(), string).unwrap();
                    None
                } else if string.starts_with("Anno Domini") {
                    // Using unwrap because it's safe to assume that the string is long enough
                    remove_part("Anno Domini".len(), string).unwrap();
                    None
                } else {
                    return Err(create_invalid_format(format!(
                        "Could not parse '{}' from given string.",
                        chars
                    )));
                }
            }
        },
        'y' => match chars.len() {
            2 => {
                if string.starts_with('-') {
                    let year = pick_part::<i32>(3, string, "year")?;
                    Some(ParsedPart {
                        value: year as i64,
                        unit: ParseUnit::Year,
                    })
                } else {
                    let sub_century_year = pick_part::<i32>(2, string, "year")?;
                    let current_century = Date::now().get(DateUnit::Year) / 1000 * 1000;
                    Some(ParsedPart {
                        value: (current_century + sub_century_year) as i64,
                        unit: ParseUnit::Year,
                    })
                }
            }
            1 | 3 | 4 => {
                let mut year_length = usize::from(string.starts_with('-'));
                let string_length = string.chars().count();
                while string_length > year_length
                    && string.chars().nth(year_length).unwrap().is_ascii_digit()
                {
                    year_length += 1;
                }

                let year = pick_part::<i32>(year_length, string, "year")?;

                Some(ParsedPart {
                    value: year as i64,
                    unit: ParseUnit::Year,
                })
            }
            _ => {
                let year = if string.starts_with('-') {
                    pick_part::<i32>(chars.len() + 1, string, "year")?
                } else {
                    pick_part::<i32>(chars.len(), string, "year")?
                };

                Some(ParsedPart {
                    value: year as i64,
                    unit: ParseUnit::Year,
                })
            }
        },
        'q' => match chars.len() {
            1 | 2 => {
                remove_part(chars.len(), string)?;
                None
            }
            3 => {
                remove_part(2, string)?;
                None
            }
            4 => {
                for quarter in ["1st quarter", "2nd quarter", "3rd quarter", "4th quarter"] {
                    if string.starts_with(quarter) {
                        // Using unwrap because it's safe to assume that the string is long enough
                        remove_part(quarter.len(), string).unwrap();
                        return Ok(None);
                    };
                }
                return Err(create_invalid_format(format!(
                    "Could not parse '{}' from given string.",
                    chars
                )));
            }
            _ => {
                remove_part(1, string)?;
                None
            }
        },
        'M' => parse_month(chars.len(), string)?,
        'w' => match chars.len() {
            1 => match string.chars().nth(1) {
                Some(char) if char.is_ascii_digit() => {
                    // Using unwrap because it's safe to assume that the string is long enough
                    remove_part(2, string).unwrap();
                    None
                }
                _ => {
                    remove_part(1, string)?;
                    None
                }
            },
            _ => {
                remove_part(get_length(chars.len(), 2, 2), string)?;
                None
            }
        },
        'd' => match chars.len() {
            1 => match string.chars().nth(1) {
                Some(char) if char.is_ascii_digit() => {
                    let day = pick_part::<u32>(2, string, "day of month")?;

                    Some(ParsedPart {
                        value: day as i64,
                        unit: ParseUnit::DayOfMonth,
                    })
                }
                _ => {
                    let day = pick_part::<u32>(1, string, "day of month")?;

                    Some(ParsedPart {
                        value: day as i64,
                        unit: ParseUnit::DayOfMonth,
                    })
                }
            },
            _ => {
                let day = pick_part::<u32>(2, string, "day of month")?;

                Some(ParsedPart {
                    value: day as i64,
                    unit: ParseUnit::DayOfMonth,
                })
            }
        },
        'D' => match chars.len() {
            2 => match string.chars().nth(2) {
                Some(char) if char.is_ascii_digit() => {
                    // Using unwrap because it's safe to assume that the string is long enough
                    let day = pick_part::<u32>(3, string, "day of year").unwrap();

                    Some(ParsedPart {
                        value: day as i64,
                        unit: ParseUnit::DayOfYear,
                    })
                }
                _ => {
                    let day = pick_part::<u32>(2, string, "day of year")?;

                    Some(ParsedPart {
                        value: day as i64,
                        unit: ParseUnit::DayOfYear,
                    })
                }
            },
            3 => {
                let day = pick_part::<u32>(3, string, "day of year")?;

                Some(ParsedPart {
                    value: day as i64,
                    unit: ParseUnit::DayOfYear,
                })
            }
            _ => match string.chars().nth(1) {
                Some(char) if char.is_ascii_digit() => match string.chars().nth(2) {
                    Some(char) if char.is_ascii_digit() => {
                        // Using unwrap because it's safe to assume that the string is long enough
                        let day = pick_part::<u32>(3, string, "day of year").unwrap();

                        Some(ParsedPart {
                            value: day as i64,
                            unit: ParseUnit::DayOfYear,
                        })
                    }
                    _ => {
                        // Using unwrap because it's safe to assume that the string is long enough
                        let day = pick_part::<u32>(2, string, "day of year").unwrap();

                        Some(ParsedPart {
                            value: day as i64,
                            unit: ParseUnit::DayOfYear,
                        })
                    }
                },
                _ => {
                    let day = pick_part::<u32>(1, string, "day of year")?;

                    Some(ParsedPart {
                        value: day as i64,
                        unit: ParseUnit::DayOfYear,
                    })
                }
            },
        },
        'e' => parse_wday(chars.len(), string)?,
        _ => {
            remove_part(chars.len(), string)?;
            None
        }
    })
}

/// Parse string parts based on https://www.unicode.org/reports/tr35/tr35-dates.html#table-date-field-symbol-table
/// This function only parses time parts while ignoring date related parts (E.g. year, day)
pub(crate) fn parse_time_part(
    chars: &str,
    string: &mut String,
) -> Result<Option<ParsedPart>, AstrolabeError> {
    // Using unwrap because it's safe to assume that chars has a length of at least 1
    let first_char = chars.chars().next().unwrap();
    Ok(match first_char {
        'a' => match chars.len() {
            4 => {
                let period = pick_part::<String>(4, string, "period")?;
                match period.as_str() {
                    "a.m." => Some(ParsedPart {
                        value: 0,
                        unit: ParseUnit::Period,
                    }),
                    "p.m." => Some(ParsedPart {
                        value: 1,
                        unit: ParseUnit::Period,
                    }),
                    _ => {
                        return Err(create_invalid_format(format!(
                            "Could not parse '{}' from given string.",
                            chars
                        )));
                    }
                }
            }
            5 => {
                let period = pick_part::<String>(1, string, "period")?;
                match period.as_str() {
                    "a" => Some(ParsedPart {
                        value: 0,
                        unit: ParseUnit::Period,
                    }),
                    "p" => Some(ParsedPart {
                        value: 1,
                        unit: ParseUnit::Period,
                    }),
                    _ => {
                        return Err(create_invalid_format(format!(
                            "Could not parse '{}' from given string.",
                            chars
                        )));
                    }
                }
            }
            _ => {
                let period = pick_part::<String>(2, string, "period")?;
                match period.as_str() {
                    "am" | "AM" => Some(ParsedPart {
                        value: 0,
                        unit: ParseUnit::Period,
                    }),
                    "pm" | "PM" => Some(ParsedPart {
                        value: 1,
                        unit: ParseUnit::Period,
                    }),
                    _ => {
                        return Err(create_invalid_format(format!(
                            "Could not parse '{}' from given string.",
                            chars
                        )));
                    }
                }
            }
        },
        'b' => match chars.len() {
            4 => {
                for (n, period) in ["a.m.", "midnight", "p.m.", "noon"].iter().enumerate() {
                    if string.starts_with(period) {
                        // Using unwrap because it's safe to assume that the string is long enough
                        remove_part(period.len(), string).unwrap();
                        return Ok(Some(ParsedPart {
                            value: if n <= 1 { 0 } else { 1 },
                            unit: ParseUnit::Period,
                        }));
                    };
                }
                return Err(create_invalid_format(format!(
                    "Could not parse '{}' from given string.",
                    chars
                )));
            }
            5 => {
                for (n, period) in ["a", "mi", "p", "n"].iter().enumerate() {
                    if string.starts_with(period) {
                        // Using unwrap because it's safe to assume that the string is long enough
                        remove_part(period.len(), string).unwrap();
                        return Ok(Some(ParsedPart {
                            value: if n <= 1 { 0 } else { 1 },
                            unit: ParseUnit::Period,
                        }));
                    };
                }
                return Err(create_invalid_format(format!(
                    "Could not parse '{}' from given string.",
                    chars
                )));
            }
            _ => {
                for (n, period) in ["am", "AM", "midnight", "pm", "PM", "noon"]
                    .iter()
                    .enumerate()
                {
                    if string.starts_with(period) {
                        // Using unwrap because it's safe to assume that the string is long enough
                        remove_part(period.len(), string).unwrap();
                        return Ok(Some(ParsedPart {
                            value: if n <= 2 { 0 } else { 1 },
                            unit: ParseUnit::Period,
                        }));
                    };
                }
                return Err(create_invalid_format(format!(
                    "Could not parse '{}' from given string.",
                    chars
                )));
            }
        },
        'h' => match chars.len() {
            1 => match string.chars().nth(1) {
                Some(char) if char.is_ascii_digit() => {
                    let hour = pick_part::<u32>(2, string, "hour")?;
                    Some(ParsedPart {
                        value: if hour == 12 { 0 } else { hour } as i64,
                        unit: ParseUnit::PeriodHour,
                    })
                }
                _ => {
                    let hour = pick_part::<u32>(1, string, "hour")?;
                    Some(ParsedPart {
                        // Hour cannot be 12
                        value: hour as i64,
                        unit: ParseUnit::PeriodHour,
                    })
                }
            },
            _ => {
                let hour = pick_part::<u32>(2, string, "hour")?;
                Some(ParsedPart {
                    value: if hour == 12 { 0 } else { hour } as i64,
                    unit: ParseUnit::PeriodHour,
                })
            }
        },
        'H' => match chars.len() {
            1 => match string.chars().nth(1) {
                Some(char) if char.is_ascii_digit() => {
                    let hour = pick_part::<u32>(2, string, "hour")?;
                    println!("{}", hour);
                    Some(ParsedPart {
                        value: hour as i64,
                        unit: ParseUnit::Hour,
                    })
                }
                _ => {
                    let hour = pick_part::<u32>(1, string, "hour")?;
                    Some(ParsedPart {
                        value: hour as i64,
                        unit: ParseUnit::Hour,
                    })
                }
            },
            _ => {
                let hour = pick_part::<u32>(2, string, "hour")?;
                Some(ParsedPart {
                    value: hour as i64,
                    unit: ParseUnit::Hour,
                })
            }
        },
        'K' => match chars.len() {
            1 => match string.chars().nth(1) {
                Some(char) if char.is_ascii_digit() => {
                    let hour = pick_part::<u32>(2, string, "hour")?;
                    Some(ParsedPart {
                        value: hour as i64,
                        unit: ParseUnit::PeriodHour,
                    })
                }
                _ => {
                    let hour = pick_part::<u32>(1, string, "hour")?;
                    Some(ParsedPart {
                        value: hour as i64,
                        unit: ParseUnit::PeriodHour,
                    })
                }
            },
            _ => {
                let hour = pick_part::<u32>(2, string, "hour")?;
                Some(ParsedPart {
                    value: hour as i64,
                    unit: ParseUnit::PeriodHour,
                })
            }
        },
        'k' => match chars.len() {
            1 => match string.chars().nth(1) {
                Some(char) if char.is_ascii_digit() => {
                    let hour = pick_part::<u32>(2, string, "hour")?;
                    Some(ParsedPart {
                        value: if hour == 24 { 0 } else { hour } as i64,
                        unit: ParseUnit::Hour,
                    })
                }
                _ => {
                    let hour = pick_part::<u32>(1, string, "hour")?;
                    Some(ParsedPart {
                        // Hour cannot be 24
                        value: hour as i64,
                        unit: ParseUnit::Hour,
                    })
                }
            },
            _ => {
                let hour = pick_part::<u32>(2, string, "hour")?;
                Some(ParsedPart {
                    value: if hour == 24 { 0 } else { hour } as i64,
                    unit: ParseUnit::Hour,
                })
            }
        },
        'm' => match chars.len() {
            1 => match string.chars().nth(1) {
                Some(char) if char.is_ascii_digit() => {
                    let minute = pick_part::<u32>(2, string, "minute")?;
                    Some(ParsedPart {
                        value: minute as i64,
                        unit: ParseUnit::Min,
                    })
                }
                _ => {
                    let minute = pick_part::<u32>(1, string, "minute")?;
                    Some(ParsedPart {
                        value: minute as i64,
                        unit: ParseUnit::Min,
                    })
                }
            },
            _ => {
                let minute = pick_part::<u32>(2, string, "minute")?;
                Some(ParsedPart {
                    value: minute as i64,
                    unit: ParseUnit::Min,
                })
            }
        },
        's' => match chars.len() {
            1 => match string.chars().nth(1) {
                Some(char) if char.is_ascii_digit() => {
                    let seconds = pick_part::<u32>(2, string, "seconds")?;
                    Some(ParsedPart {
                        value: seconds as i64,
                        unit: ParseUnit::Sec,
                    })
                }
                _ => {
                    let seconds = pick_part::<u32>(1, string, "seconds")?;
                    Some(ParsedPart {
                        value: seconds as i64,
                        unit: ParseUnit::Sec,
                    })
                }
            },
            _ => {
                let seconds = pick_part::<u32>(2, string, "seconds")?;
                Some(ParsedPart {
                    value: seconds as i64,
                    unit: ParseUnit::Sec,
                })
            }
        },
        'n' => match chars.len() {
            1 => {
                let subsecond = pick_part::<u32>(1, string, "subseconds")?;

                Some(ParsedPart {
                    value: subsecond as i64,
                    unit: ParseUnit::Decis,
                })
            }
            2 => {
                let subsecond = pick_part::<u32>(2, string, "subseconds")?;

                Some(ParsedPart {
                    value: subsecond as i64,
                    unit: ParseUnit::Centis,
                })
            }
            4 => {
                let subsecond = pick_part::<u32>(6, string, "subseconds")?;

                Some(ParsedPart {
                    value: subsecond as i64,
                    unit: ParseUnit::Micros,
                })
            }
            5 => {
                let subsecond = pick_part::<u32>(9, string, "subseconds")?;

                Some(ParsedPart {
                    value: subsecond as i64,
                    unit: ParseUnit::Nanos,
                })
            }
            _ => {
                let subsecond = pick_part::<u32>(3, string, "subseconds")?;

                Some(ParsedPart {
                    value: subsecond as i64,
                    unit: ParseUnit::Millis,
                })
            }
        },
        'X' => parse_zone(chars.len(), string, true)?,
        'x' => parse_zone(chars.len(), string, false)?,
        _ => {
            remove_part(chars.len(), string)?;
            None
        }
    })
}

/// Parses the month of a date based on https://www.unicode.org/reports/tr35/tr35-dates.html#dfst-month
fn parse_month(length: usize, string: &mut String) -> Result<Option<ParsedPart>, AstrolabeError> {
    Ok(match length {
        1 | 2 => {
            let month = pick_part::<u32>(length, string, "month")?;

            Some(ParsedPart {
                value: month as i64,
                unit: ParseUnit::Month,
            })
        }
        3 => {
            for (n, month) in MONTH_ABBREVIATED.iter().enumerate() {
                if string.starts_with(month) {
                    // Using unwrap because it's safe to assume that the string is long enough
                    remove_part(month.len(), string).unwrap();
                    return Ok(Some(ParsedPart {
                        value: (n + 1) as i64,
                        unit: ParseUnit::Month,
                    }));
                };
            }
            return Err(create_invalid_format(
                "Could not parse month from given string.".to_string(),
            ));
        }
        // Narrow month parsing doesn't work as there are multiple months with the same letter
        5 => {
            remove_part(1, string)?;
            None
        }
        _ => {
            for (n, month) in MONTH_WIDE.iter().enumerate() {
                if string.starts_with(month) {
                    // Using unwrap because it's safe to assume that the string is long enough
                    remove_part(month.len(), string).unwrap();
                    return Ok(Some(ParsedPart {
                        value: (n + 1) as i64,
                        unit: ParseUnit::Month,
                    }));
                };
            }
            return Err(create_invalid_format(
                "Could not parse month from given string.".to_string(),
            ));
        }
    })
}

/// Parses the week day of a date based on https://www.unicode.org/reports/tr35/tr35-dates.html#dfst-month
fn parse_wday(length: usize, string: &mut String) -> Result<Option<ParsedPart>, AstrolabeError> {
    Ok(match length {
        2 | 3 => {
            remove_part(length, string)?;
            None
        }
        4 => {
            for wday in WDAY_WIDE {
                if string.starts_with(wday) {
                    // Using unwrap because it's safe to assume that the string is long enough
                    remove_part(wday.len(), string).unwrap();
                    return Ok(None);
                };
            }
            return Err(create_invalid_format(
                "Could not parse week day from given string.".to_string(),
            ));
        }
        6 | 8 => {
            remove_part(2, string)?;
            None
        }
        // 1, 5, 7 and 9+ all consist of 1 char
        _ => {
            remove_part(1, string)?;
            None
        }
    })
}

/// Parses the time zone
fn parse_zone(
    length: usize,
    string: &mut String,
    with_z: bool,
) -> Result<Option<ParsedPart>, AstrolabeError> {
    let prefix = pick_part::<String>(1, string, "timezone prefix")?;

    let multiplier = match prefix.as_str() {
        "Z" if with_z => {
            return Ok(Some(ParsedPart {
                value: 0,
                unit: ParseUnit::Offset,
            }));
        }
        "+" => 1,
        "-" => -1,
        _ => {
            return Err(create_invalid_format(
                "Couldn't parse prefix of timezone offset. Prefix has to be either '+' or '-'."
                    .to_string(),
            ))
        }
    };

    let hour = pick_part::<u32>(2, string, "timezone hour")?;

    Ok(match length {
        1 => match string.chars().next() {
            Some(char) if char.is_ascii_digit() => {
                let minute = pick_part::<u32>(2, string, "timezone minute")?;

                let offset = (hour * SECS_PER_HOUR_U64 as u32 + minute * SECS_PER_MINUTE_U64 as u32)
                    as i64
                    * multiplier;

                Some(ParsedPart {
                    value: offset,
                    unit: ParseUnit::Offset,
                })
            }
            _ => {
                let offset = (hour * SECS_PER_HOUR_U64 as u32) as i64 * multiplier;

                Some(ParsedPart {
                    value: offset,
                    unit: ParseUnit::Offset,
                })
            }
        },
        2 => {
            let minute = pick_part::<u32>(2, string, "timezone minute")?;

            let offset = (hour * SECS_PER_HOUR_U64 as u32 + minute * SECS_PER_MINUTE_U64 as u32)
                as i64
                * multiplier;

            Some(ParsedPart {
                value: offset,
                unit: ParseUnit::Offset,
            })
        }
        4 => match string.chars().nth(2) {
            Some(char) if char.is_ascii_digit() => {
                let minute = pick_part::<u32>(2, string, "timezone minute")?;
                let second = pick_part::<u32>(2, string, "timezone second")?;

                let offset = (hour * SECS_PER_HOUR_U64 as u32
                    + minute * SECS_PER_MINUTE_U64 as u32
                    + second) as i64
                    * multiplier;

                Some(ParsedPart {
                    value: offset,
                    unit: ParseUnit::Offset,
                })
            }
            _ => {
                let minute = pick_part::<u32>(2, string, "timezone minute")?;

                let offset = (hour * SECS_PER_HOUR_U64 as u32 + minute * SECS_PER_MINUTE_U64 as u32)
                    as i64
                    * multiplier;

                Some(ParsedPart {
                    value: offset,
                    unit: ParseUnit::Offset,
                })
            }
        },
        5 => match string.chars().nth(4) {
            Some(char) if char.is_ascii_digit() => {
                // Using unwrap because it's safe to assume that the string is long enough
                remove_part(1, string).unwrap();
                let minute = pick_part::<u32>(2, string, "timezone minute")?;
                // Using unwrap because it's safe to assume that the string is long enough
                remove_part(1, string).unwrap();
                let second = pick_part::<u32>(2, string, "timezone second")?;

                let offset = (hour * SECS_PER_HOUR_U64 as u32
                    + minute * SECS_PER_MINUTE_U64 as u32
                    + second) as i64
                    * multiplier;

                Some(ParsedPart {
                    value: offset,
                    unit: ParseUnit::Offset,
                })
            }
            _ => {
                remove_part(1, string)?;
                let minute = pick_part::<u32>(2, string, "timezone minute")?;

                let offset = (hour * SECS_PER_HOUR_U64 as u32 + minute * SECS_PER_MINUTE_U64 as u32)
                    as i64
                    * multiplier;

                Some(ParsedPart {
                    value: offset,
                    unit: ParseUnit::Offset,
                })
            }
        },
        _ => {
            remove_part(1, string)?;
            let minute = pick_part::<u32>(2, string, "timezone minute")?;

            let offset = (hour * SECS_PER_HOUR_U64 as u32 + minute * SECS_PER_MINUTE_U64 as u32)
                as i64
                * multiplier;

            Some(ParsedPart {
                value: offset,
                unit: ParseUnit::Offset,
            })
        }
    })
}

fn remove_part(length: usize, string: &mut String) -> Result<(), AstrolabeError> {
    if string.chars().count() < length {
        Err(create_invalid_format(
            "String to parse is too short. Please check your format string.".to_string(),
        ))
    } else {
        string.replace_range(0..length, "");
        Ok(())
    }
}

fn pick_part<T: std::str::FromStr>(
    length: usize,
    string: &mut String,
    part_name: &str,
) -> Result<T, AstrolabeError> {
    if string.chars().count() < length {
        Err(create_invalid_format(
            "String to parse is too short. Please check your format string.".to_string(),
        ))
    } else {
        let part = string[0..length].parse::<T>().map_err(|_| {
            create_invalid_format(format!(
                "Failed parsing {} from given string. Value is '{}'.",
                part_name,
                &string[0..length]
            ))
        })?;
        string.replace_range(0..length, "");
        Ok(part)
    }
}
