use super::format::get_length;
use crate::{
    errors::{invalid_format::create_invalid_format, AstrolabeError},
    shared::{MONTH_ABBREVIATED, MONTH_WIDE, SECS_PER_HOUR, SECS_PER_MINUTE, WDAY_WIDE},
    Date, DateUnit,
};

// Parses the offset part from an RFC3339 timestamp string to offset seconds
pub(crate) fn parse_offset(string: &str) -> Result<i32, AstrolabeError> {
    if string.starts_with('Z') {
        return Ok(0);
    }
    if string.len() != 6 {
        return Err(create_invalid_format(
            "Failed parsing the offset from the RFC3339 string. Format should be +XX:XX or -XX:XX"
                .to_string(),
        ));
    }

    let hour = string[1..3].parse::<u32>().map_err(|_| {
        create_invalid_format(
            "Failed parsing the hour of the offset from the RFC3339 string".to_string(),
        )
    })?;
    let min = string[4..6].parse::<u32>().map_err(|_| {
        create_invalid_format(
            "Failed parsing the minute of the offset from the RFC3339 string".to_string(),
        )
    })?;

    if hour > 23 {
        return Err(create_invalid_format(
            "Failed parsing the hour of the offset from the RFC3339 string. Hour has to be less than 24".to_string(),
        ));
    } else if min > 59 {
        return Err(create_invalid_format(
            "Failed parsing the minute of the offset from the RFC3339 string. Minute has to be less than 60".to_string(),
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
    // TODO: Remove
    #[allow(dead_code)]
    Hour,
    // TODO: Remove
    #[allow(dead_code)]
    Min,
    // TODO: Remove
    #[allow(dead_code)]
    Sec,
    // TODO: Remove
    #[allow(dead_code)]
    Centis,
    // TODO: Remove
    #[allow(dead_code)]
    Millis,
    // TODO: Remove
    #[allow(dead_code)]
    Micros,
    // TODO: Remove
    #[allow(dead_code)]
    Nanos,
}

#[derive(Default)]
pub(crate) struct ParsedDate {
    pub(crate) year: Option<i32>,
    pub(crate) month: Option<u32>,
    pub(crate) day_of_month: Option<u32>,
    pub(crate) day_of_year: Option<u32>,
}

/// Formats string parts based on https://www.unicode.org/reports/tr35/tr35-dates.html#table-date-field-symbol-table
/// **Note**: Not all field types/symbols are implemented.
// TODO: Remove
// #[allow(dead_code)]
// pub(crate) fn parse_part(
//     chars: &str,
//     string: &mut String,
// ) -> Result<Option<ParsedPart>, AstrolabeError> {
//     // Using unwrap because it's safe to assume that chars has a length of at least 1
//     let first_char = chars.chars().next().unwrap();
//     Ok(match first_char {
//         'G' | 'y' | 'q' | 'M' | 'w' | 'd' | 'D' | 'e' => parse_date_part(chars, string)?,
//         // 'a' | 'b' | 'h' | 'H' | 'K' | 'k' | 'm' | 's' | 'X' | 'x' | 'n' => {}
//         _ => {
//             remove_part(chars.len(), string)?;
//             None
//         }
//     })
// }

/// Parse string parts based on https://www.unicode.org/reports/tr35/tr35-dates.html#table-date-field-symbol-table
/// This function only formats date parts while ignoring time related parts (E.g. hour, minute)
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
                        "Could not parse {} from given string.",
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
                let mut year_length = if string.starts_with('-') { 1 } else { 0 };
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
                    "Could not parse {} from given string.",
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
