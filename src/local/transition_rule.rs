use std::{num::ParseIntError, str::FromStr};

use crate::{
    util::{
        constants::{BUG_MSG, SECS_PER_DAY},
        date::convert::{weekdays_in_month, year_doy_to_days, year_month_to_doy},
    },
    DateTime, DateUtilities,
};

use super::{cursor::Cursor, errors::TimeZoneError, timezone::LocalTimeType};

/// Transition rule of a TZif file (Present in footer of Version 2/3 TZif files)
#[derive(Debug, PartialEq)]
pub(super) enum TransitionRule {
    /// Fixed local time type
    Fixed(LocalTimeType),
    /// Alternate local time types
    Alternate(AlternateLocalTimeType),
}

impl TransitionRule {
    pub(super) fn from_tz_string(
        footer: &[u8],
        string_extensions: bool,
    ) -> Result<Option<Self>, TimeZoneError> {
        let footer = std::str::from_utf8(footer)?;

        if !footer.starts_with('\n') || !footer.ends_with('\n') {
            return Err(TimeZoneError::InvalidTzFile("Invalid footer"));
        };

        let tz_string = footer.trim_matches(|c: char| c.is_ascii_whitespace());

        if tz_string.starts_with(':') || tz_string.contains('\0') {
            return Err(TimeZoneError::InvalidTzFile("Invalid footer"));
        }

        let mut cursor = Cursor::new(tz_string.as_bytes());

        remove_designation(&mut cursor)?;

        let std_offset = parse_tz_string_offset(&mut cursor)?;

        if cursor.empty() {
            return Ok(Some(TransitionRule::Fixed(LocalTimeType::new(
                -std_offset,
                false,
            ))));
        }

        remove_designation(&mut cursor)?;

        let dst_offset = match cursor.remaining().first() {
            Some(&b',') => std_offset - 3600,
            Some(_) => parse_tz_string_offset(&mut cursor)?,
            None => return Err(TimeZoneError::InvalidTzFile("Invalid footer")),
        };

        cursor.read_tag(b",")?;

        let (std_end, std_end_time) = parse_tz_string_rule(&mut cursor, string_extensions)?;

        cursor.read_tag(b",")?;

        let (dst_end, dst_end_time) = parse_tz_string_rule(&mut cursor, string_extensions)?;

        Ok(Some(TransitionRule::Alternate(
            AlternateLocalTimeType::new(
                LocalTimeType::new(-std_offset, false),
                std_end,
                std_end_time,
                LocalTimeType::new(-dst_offset, true),
                dst_end,
                dst_end_time,
            ),
        )))
    }
}

/// Alternate local time type
#[derive(Debug, PartialEq)]
pub(super) struct AlternateLocalTimeType {
    pub(super) std: LocalTimeType,
    std_end: RuleDay,
    std_end_time: u32,
    pub(super) dst: LocalTimeType,
    dst_end: RuleDay,
    dst_end_time: u32,
}

impl AlternateLocalTimeType {
    pub(super) fn new(
        std: LocalTimeType,
        std_end: RuleDay,
        std_end_time: u32,
        dst: LocalTimeType,
        dst_end: RuleDay,
        dst_end_time: u32,
    ) -> Self {
        Self {
            std,
            std_end,
            std_end_time,
            dst,
            dst_end,
            dst_end_time,
        }
    }

    pub(super) fn local_std_end_timestamp(&self, timestamp: i64) -> i64 {
        rule_to_local_timestamp(&self.std_end, self.std_end_time as i32, timestamp)
    }

    pub(super) fn local_dst_end_timestamp(&self, timestamp: i64) -> i64 {
        rule_to_local_timestamp(&self.dst_end, self.dst_end_time as i32, timestamp)
    }
}

fn rule_to_local_timestamp(start: &RuleDay, time: i32, timestamp: i64) -> i64 {
    let date_days = match start {
        RuleDay::JulianDayWithoutLeap(doy) => {
            let year = DateTime::from_timestamp(timestamp).year();
            year_doy_to_days(year, *doy, true).unwrap()
        }
        RuleDay::JulianDayWithLeap(doy) => {
            let year = DateTime::from_timestamp(timestamp).year();
            year_doy_to_days(year, doy + 1, false).unwrap()
        }
        RuleDay::MonthWeekDay(month, week, day) => {
            let year = DateTime::from_timestamp(timestamp).year();

            let weekdays_in_month = weekdays_in_month(year, *month as u32, *day);

            let day_of_month = match week {
                5 => weekdays_in_month.last().unwrap(),
                _ => &weekdays_in_month[*week as usize - 1],
            };

            let (start, _) = year_month_to_doy(year, *month as u32).unwrap();
            year_doy_to_days(year, start + day_of_month, false).unwrap()
        }
    };
    let time = time as i64;
    DateTime::from_seconds(date_days as i64 * SECS_PER_DAY as i64 + time)
        .unwrap()
        .timestamp()
}

fn remove_designation(cursor: &mut Cursor) -> Result<(), TimeZoneError> {
    if cursor.get_next()? == b'<' {
        cursor.read_until('>');
        cursor.read_exact(1)?;
    } else {
        cursor.read_while(|c: &u8| c.is_ascii_alphabetic());
    };
    Ok(())
}

fn parse_hms(cursor: &mut Cursor) -> Result<(i32, i32, i32, i32), TimeZoneError> {
    let next = cursor.get_next()?;
    let direction = if next == b'-' {
        cursor.read_exact(1).expect(BUG_MSG);
        -1
    } else {
        if next == b'+' {
            cursor.read_exact(1).expect(BUG_MSG);
        };
        1
    };

    let hour: i32 = parse_int(cursor.read_while(|c: &u8| c.is_ascii_digit()))?;

    let mut minute = 0;
    let mut second = 0;

    if !cursor.empty() && cursor.get_next().expect(BUG_MSG) == b':' {
        cursor.read_exact(1).expect(BUG_MSG);
        minute = parse_int(cursor.read_while(|c: &u8| c.is_ascii_digit()))?;
        if !cursor.empty() && cursor.get_next().expect(BUG_MSG) == b':' {
            cursor.read_exact(1).expect(BUG_MSG);
            second = parse_int(cursor.read_while(|c: &u8| c.is_ascii_digit()))?;
        };
    };

    Ok((direction, hour, minute, second))
}

fn parse_tz_string_offset(cursor: &mut Cursor) -> Result<i32, TimeZoneError> {
    let (direction, hour, minute, second) = parse_hms(cursor)?;

    if !(0..=24).contains(&hour) {
        return Err(TimeZoneError::InvalidTzFile(
            "Invalid day time hour in footer",
        ));
    }
    if !(0..=59).contains(&minute) {
        return Err(TimeZoneError::InvalidTzFile(
            "Invalid day time minute in footer",
        ));
    }
    if !(0..=59).contains(&second) {
        return Err(TimeZoneError::InvalidTzFile(
            "Invalid day time second in footer",
        ));
    }

    Ok(direction * (hour * 3600 + minute * 60 + second))
}

fn parse_tz_string_offset_extended(cursor: &mut Cursor) -> Result<i32, TimeZoneError> {
    let (direction, hour, minute, second) = parse_hms(cursor)?;

    if !(-167..=167).contains(&hour) {
        return Err(TimeZoneError::InvalidTzFile(
            "Invalid day time hour in footer",
        ));
    }
    if !(0..=59).contains(&minute) {
        return Err(TimeZoneError::InvalidTzFile(
            "Invalid day time minute in footer",
        ));
    }
    if !(0..=59).contains(&second) {
        return Err(TimeZoneError::InvalidTzFile(
            "Invalid day time second in footer",
        ));
    }

    Ok(direction * (hour * 3600 + minute * 60 + second))
}

/// Parse integer from a slice of bytes
fn parse_int<T: FromStr<Err = ParseIntError>>(bytes: &[u8]) -> Result<T, TimeZoneError> {
    std::str::from_utf8(bytes)
        .expect(BUG_MSG)
        .parse()
        .map_err(TimeZoneError::from)
}

fn parse_tz_string_rule(
    cursor: &mut Cursor,
    string_extensions: bool,
) -> Result<(RuleDay, u32), TimeZoneError> {
    let day = match cursor.get_next()? {
        b'J' => {
            cursor.read_exact(1).expect(BUG_MSG);
            let day = parse_int(cursor.read_while(|c: &u8| c.is_ascii_digit()))?;
            RuleDay::JulianDayWithoutLeap(day)
        }
        byte if byte.is_ascii_digit() => {
            let day = parse_int(cursor.read_while(|c: &u8| c.is_ascii_digit())).expect(BUG_MSG);
            RuleDay::JulianDayWithLeap(day)
        }
        b'M' => {
            cursor.read_exact(1).expect(BUG_MSG);
            let month = parse_int(cursor.read_until('.'))?;

            cursor.read_exact(1)?;
            let week = parse_int(cursor.read_until('.'))?;

            cursor.read_exact(1)?;
            let day = parse_int(cursor.read_while(|c| c.is_ascii_digit()))?;

            RuleDay::MonthWeekDay(month, week, day)
        }
        _ => return Err(TimeZoneError::InvalidTzFile("Invalid footer")),
    };

    let time = if !cursor.empty() && cursor.get_next().expect(BUG_MSG) == b'/' {
        cursor.read_exact(1).expect(BUG_MSG);
        if string_extensions {
            parse_tz_string_offset_extended(cursor)? as u32
        } else {
            parse_tz_string_offset(cursor)? as u32
        }
    } else {
        2 * 3600
    };

    Ok((day, time))
}

#[derive(Debug, PartialEq)]
pub(super) enum RuleDay {
    /// Julian day (1..=365). February 29 is never counted
    JulianDayWithoutLeap(u32),
    /// Zero based Julian day (0..=365). February 29 is counted in leap years
    JulianDayWithLeap(u32),
    /// Month, week, day (1..=12, 1..=5, 0..=6)
    /// Week 5 means the last week of the month
    /// Day zero is Sunday
    MonthWeekDay(u8, u8, u8),
}
