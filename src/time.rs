use crate::{
    errors::{
        out_of_range::{create_custom_oor, create_simple_oor},
        AstrolabeError,
    },
    util::{
        constants::{
            NANOS_PER_DAY, NANOS_PER_SEC, SECS_PER_DAY, SECS_PER_DAY_U64, SECS_PER_HOUR,
            SECS_PER_HOUR_U64, SECS_PER_MINUTE, SECS_PER_MINUTE_U64,
        },
        format::format_time_part,
        offset::{add_offset_to_nanos, remove_offset_from_nanos},
        parse::{parse_format_string, parse_time_part, ParseUnit, ParsedTime, Period},
        time::{
            convert::{
                days_nanos_to_hours, days_nanos_to_micros, days_nanos_to_millis,
                days_nanos_to_minutes, days_nanos_to_nanos, days_nanos_to_seconds,
                nanos_to_subhour_nanos, nanos_to_submicro_nanos, nanos_to_submilli_nanos,
                nanos_to_subminute_nanos, nanos_to_subsecond, nanos_to_subsecond_nanos,
                nanos_to_time, time_to_day_seconds,
            },
            manipulate::{
                add_hours, add_micros, add_millis, add_minutes, add_seconds,
                clear_nanos_until_micro, clear_nanos_until_milli, clear_nanos_until_nanos,
                clear_nanos_until_second, set_hour, set_micro, set_milli, set_minute, set_nano,
                set_second, sub_hours, sub_micros, sub_millis, sub_minutes, sub_seconds,
            },
        },
    },
    DateTime, OffsetUtilities, TimeUtilities,
};
use std::{
    cmp,
    fmt::Display,
    ops::{Add, AddAssign, Sub, SubAssign},
    str::FromStr,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

/// Clock time with nanosecond precision.
///
/// See the [`TimeUtilities`](#impl-TimeUtilities-for-Time) implementation for get, set and manipulation methods.
///
/// [`OffsetUtilities`](#impl-OffsetUtilities-for-Time) impements methods for setting and getting the offset.
#[derive(Debug, Default, Copy, Clone, Eq)]
pub struct Time {
    pub(crate) nanoseconds: u64,
    pub(crate) offset: i32,
}

impl Time {
    /// Creates a new [`Time`] instance with [`SystemTime::now()`].
    ///
    /// ```rust
    /// # use astrolabe::Time;
    /// let time = Time::now();
    /// println!("{}", time);
    /// ```
    pub fn now() -> Self {
        let duration = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        let nanoseconds =
            duration.as_secs() % SECS_PER_DAY_U64 * NANOS_PER_SEC + duration.subsec_nanos() as u64;
        Self {
            nanoseconds,
            offset: 0,
        }
    }

    /// Creates a new [`Time`] instance from hour, minute and seconds.
    ///
    /// Returns an [`OutOfRange`](AstrolabeError::OutOfRange) error if the provided time is invalid.
    ///
    /// ```rust
    /// # use astrolabe::Time;
    /// let time = Time::from_hms(12, 32, 01).unwrap();
    /// assert_eq!("12:32:01", time.format("HH:mm:ss"));
    /// ```
    pub fn from_hms(hour: u32, minute: u32, second: u32) -> Result<Self, AstrolabeError> {
        let seconds = time_to_day_seconds(hour, minute, second)? as u64;

        Ok(Self {
            nanoseconds: seconds * NANOS_PER_SEC,
            offset: 0,
        })
    }

    /// Returns the time as hour, minute and seconds.
    ///
    /// ```rust
    /// # use astrolabe::Time;
    /// let time = Time::from_hms(12, 12, 12).unwrap();
    /// let (hour, minute, second) = time.as_hms();
    /// assert_eq!(12, hour);
    /// assert_eq!(12, minute);
    /// assert_eq!(12, second);
    /// ```
    pub fn as_hms(&self) -> (u32, u32, u32) {
        let seconds = self.as_seconds();

        let hour = seconds / 3600;
        let minute = (seconds % 3600) / 60;
        let second = seconds % 60;

        (hour, minute, second)
    }

    /// Creates a new [`Time`] instance from seconds.
    ///
    /// Returns an [`OutOfRange`](AstrolabeError::OutOfRange) error if the provided seconds are invalid (over `86399`)
    ///
    /// ```rust
    /// # use astrolabe::Time;
    /// let time = Time::from_seconds(1_234).unwrap();
    /// assert_eq!("00:20:34", time.format("HH:mm:ss"));
    /// ```
    pub fn from_seconds(seconds: u32) -> Result<Self, AstrolabeError> {
        if seconds >= SECS_PER_DAY {
            return Err(create_simple_oor(
                "seconds",
                0,
                SECS_PER_DAY as i128 - 1,
                seconds as i128,
            ));
        }
        Ok(Self {
            nanoseconds: seconds as u64 * NANOS_PER_SEC,
            offset: 0,
        })
    }

    /// Returns the time as seconds.
    ///
    /// ```rust
    /// # use astrolabe::Time;
    /// let time = Time::from_hms(12, 12, 12).unwrap();
    /// assert_eq!(43932, time.as_seconds());
    /// ```
    pub fn as_seconds(&self) -> u32 {
        (self.nanoseconds / NANOS_PER_SEC) as u32
    }

    /// Creates a new [`Time`] instance from seconds.
    ///
    /// Returns an [`OutOfRange`](AstrolabeError::OutOfRange) error if the provided nanoseconds are invalid (over `86_399_999_999_999`)
    ///
    /// ```rust
    /// # use astrolabe::Time;
    /// let time = Time::from_nanos(1_234).unwrap();
    /// assert_eq!(1_234, time.as_nanos());
    /// ```
    pub fn from_nanos(nanos: u64) -> Result<Self, AstrolabeError> {
        if nanos >= SECS_PER_DAY_U64 * NANOS_PER_SEC {
            return Err(create_simple_oor(
                "nanoseconds",
                0,
                SECS_PER_DAY_U64 as i128 * NANOS_PER_SEC as i128 - 1,
                nanos as i128,
            ));
        }
        Ok(Self {
            nanoseconds: nanos,
            offset: 0,
        })
    }

    /// Returns the time as nanoseconds.
    ///
    /// ```rust
    /// # use astrolabe::Time;
    /// let time = Time::from_hms(12, 12, 12).unwrap();
    /// assert_eq!(43_932_000_000_000, time.as_nanos());
    /// ```
    pub fn as_nanos(&self) -> u64 {
        self.nanoseconds
    }

    /// Parses a string with a given format and creates a new [`Time`] instance from it. See [`Time::format`] for a list of available symbols.
    ///
    /// Returns an [`InvalidFormat`](AstrolabeError::InvalidFormat) error if the given string could not be parsed with the given format.
    ///
    /// ```rust
    /// # use astrolabe::Time;
    /// let date = Time::parse("12:32:01", "HH:mm:ss").unwrap();
    /// assert_eq!("12:32:01", date.format("HH:mm:ss"));
    /// ```
    pub fn parse(string: &str, format: &str) -> Result<Self, AstrolabeError> {
        let parts = parse_format_string(format);

        let mut time = ParsedTime::default();
        let mut string = string.to_string();

        for part in parts {
            // Escaped apostrophes
            if part.starts_with('\u{0000}') {
                string.replace_range(0..part.len(), "");
                continue;
            }

            // Escaped parts
            if part.starts_with('\'') {
                string.replace_range(0..part.len() - if part.ends_with('\'') { 2 } else { 1 }, "");
                continue;
            }

            let parsed_part = parse_time_part(&part, &mut string)?;
            if let Some(parsed_part) = parsed_part {
                match parsed_part.unit {
                    ParseUnit::Hour => time.hour = Some(parsed_part.value as u64),
                    ParseUnit::PeriodHour => time.period_hour = Some(parsed_part.value as u64),
                    ParseUnit::Period => {
                        time.period = Some(if parsed_part.value == 0 {
                            Period::AM
                        } else {
                            Period::PM
                        })
                    }
                    ParseUnit::Minute => time.minute = Some(parsed_part.value as u64),
                    ParseUnit::Second => time.second = Some(parsed_part.value as u64),
                    ParseUnit::Decis => time.decis = Some(parsed_part.value as u64),
                    ParseUnit::Centis => time.centis = Some(parsed_part.value as u64),
                    ParseUnit::Millis => time.millis = Some(parsed_part.value as u64),
                    ParseUnit::Micros => time.micros = Some(parsed_part.value as u64),
                    ParseUnit::Nanos => time.nanos = Some(parsed_part.value as u64),
                    // Can't be any other variant than `ParseUnit::Offset`
                    _ => time.offset = Some(parsed_part.value as i32),
                };
            };
        }

        let mut nanoseconds = 0;

        if time.hour.is_some() {
            nanoseconds += time.hour.unwrap_or(0) * SECS_PER_HOUR_U64 * NANOS_PER_SEC;
        } else {
            nanoseconds += (time.period_hour.unwrap_or(0)
                + time.period.unwrap_or(Period::AM) as u64)
                * SECS_PER_HOUR_U64
                * NANOS_PER_SEC;
        }
        nanoseconds += time.minute.unwrap_or(0) * SECS_PER_MINUTE_U64 * NANOS_PER_SEC;
        nanoseconds += time.second.unwrap_or(0) * NANOS_PER_SEC;
        nanoseconds += time.decis.unwrap_or(0) * 100_000_000;
        nanoseconds += time.centis.unwrap_or(0) * 10_000_000;
        nanoseconds += time.millis.unwrap_or(0) * 1_000_000;
        nanoseconds += time.micros.unwrap_or(0) * 1_000;
        nanoseconds += time.nanos.unwrap_or(0);

        Ok(if let Some(offset) = time.offset {
            Self::from_nanos(nanoseconds)?.set_offset(offset)?
        } else {
            Self::from_nanos(nanoseconds)?
        })
    }

    /// Formatting with format strings based on [Unicode Date Field Symbols](https://www.unicode.org/reports/tr35/tr35-dates.html#Date_Field_Symbol_Table).
    ///
    /// Please note that not all symbols are implemented. If you need something that is not implemented, please open an issue on [GitHub](https://github.com/GiyoMoon/astrolabe/issues) describing your need.
    ///
    /// # Available Symbols:
    ///
    /// | Field Type                 | Pattern | Examples                       | Hint                 |
    /// | -------------------------- | ------- | ------------------------------ | -------------------- |
    /// | AM, PM                     | a..aa   | AM, PM                         |                      |
    /// |                            | aaa     | am, pm                         | *                    |
    /// |                            | aaaa    | a.m., p.m.                     |                      |
    /// |                            | aaaaa   | a, p                           |                      |
    /// | AM, PM,<br/>noon, midnight | b..bb   | AM, PM,<br/>noon, midnight     |                      |
    /// |                            | bbb     | am, pm,<br/>noon, midnight     | *                    |
    /// |                            | bbbb    | a.m., p.m.,<br/>noon, midnight |                      |
    /// |                            | bbbbb   | a, p, n, mi                    |                      |
    /// | hour                       | h       | 1, 12                          | [1-12]               |
    /// |                            | hh      | 01, 12                         | *                    |
    /// |                            | H       | 0, 23                          | [0-23]               |
    /// |                            | HH      | 00, 23                         | *                    |
    /// |                            | K       | 0, 11                          | [0-11]               |
    /// |                            | KK      | 00, 11                         | *                    |
    /// |                            | k       | 1, 24                          | [1-24]               |
    /// |                            | kk      | 01, 24                         | *                    |
    /// | minute                     | m       | 0, 59                          |                      |
    /// |                            | mm      | 00, 59                         | *                    |
    /// | second                     | s       | 0, 59                          |                      |
    /// |                            | ss      | 00, 59                         | *                    |
    /// | subsecond values           | n       | 1, 9                           | Deciseconds          |
    /// |                            | nn      | 01, 99                         | Centiseconds         |
    /// |                            | nnn     | 001, 999                       | Milliseconds, *      |
    /// |                            | nnnn    | 000001, 999999                 | Microseconds         |
    /// |                            | nnnnn   | 000000001, 999999999           | Nanoseconds          |
    /// | zone                       | X       | -08, +0530, Z                  |                      |
    /// |                            | XX      | -0800, Z                       |                      |
    /// |                            | XXX     | -08:00, Z                      | *                    |
    /// |                            | XXXX    | -0800, -075258, Z              |                      |
    /// |                            | XXXXX   | -08:00, -07:52:58, Z           |                      |
    /// |                            | x       | -08, +0530, +00                | Like X but without Z |
    /// |                            | xx      | -0800, +0000                   |                      |
    /// |                            | xxx     | -08:00, +00:00                 | *                    |
    /// |                            | xxxx    | -0800, -075258, +0000          |                      |
    /// |                            | xxxxx   | -08:00, -07:52:58, +00:00      |                      |
    ///
    /// `*` = Default
    ///
    /// If the sequence is longer than listed in the table, the output will be the same as the default pattern for this unit (marked with `*`).
    ///
    /// Surround any character with apostrophes (`'`) to escape them.
    /// If you want escape `'`, write `''`.
    ///
    /// ```rust
    /// # use astrolabe::Time;
    /// let time = Time::from_hms(12, 32, 1).unwrap();
    /// assert_eq!("12:32:01", time.format("HH:mm:ss"));
    /// // Escape characters
    /// assert_eq!("12:mm:ss", time.format("HH:'mm:ss'"));
    /// assert_eq!("12:'32:01'", time.format("HH:''mm:ss''"));
    /// ```
    ///
    pub fn format(&self, format: &str) -> String {
        let parts = parse_format_string(format);
        parts
            .iter()
            .flat_map(|part| -> Vec<char> {
                // Escaped apostrophes
                if part.starts_with('\u{0000}') {
                    return part.replace('\u{0000}', "'").chars().collect::<Vec<char>>();
                }

                // Escape parts starting with apostrophe
                if part.starts_with('\'') {
                    let part = part.replace('\u{0000}', "'");
                    return part[1..part.len() - usize::from(part.ends_with('\''))]
                        .chars()
                        .collect::<Vec<char>>();
                }

                format_time_part(
                    part,
                    add_offset_to_nanos(self.nanoseconds, self.offset),
                    self.offset,
                )
                .chars()
                .collect::<Vec<char>>()
            })
            .collect::<String>()
    }

    /// Returns the duration between the provided time.
    pub fn duration_between(&self, other: &Self) -> Duration {
        let nanos = self.nanoseconds as i64 - other.nanoseconds as i64;

        Duration::from_nanos(nanos.unsigned_abs())
    }
}

// ########################################
//
//  TimeUtility trait implementation
//
// ########################################

impl TimeUtilities for Time {
    fn hour(&self) -> u32 {
        let nanos = add_offset_to_nanos(self.nanoseconds, self.offset);

        nanos_to_time(nanos).0
    }

    fn minute(&self) -> u32 {
        let nanos = add_offset_to_nanos(self.nanoseconds, self.offset);

        nanos_to_time(nanos).1
    }

    fn second(&self) -> u32 {
        let nanos = add_offset_to_nanos(self.nanoseconds, self.offset);

        nanos_to_time(nanos).2
    }

    fn milli(&self) -> u32 {
        let nanos = add_offset_to_nanos(self.nanoseconds, self.offset);

        nanos_to_subsecond(nanos).0
    }

    fn micro(&self) -> u32 {
        let nanos = add_offset_to_nanos(self.nanoseconds, self.offset);

        nanos_to_subsecond(nanos).1
    }

    fn nano(&self) -> u32 {
        let nanos = add_offset_to_nanos(self.nanoseconds, self.offset);

        nanos_to_subsecond(nanos).2
    }

    fn set_hour(&self, hour: u32) -> Result<Self, AstrolabeError> {
        let nanos = add_offset_to_nanos(self.nanoseconds, self.offset);

        let new_nanos = set_hour(nanos, hour)?;

        let new_nanos = remove_offset_from_nanos(new_nanos, self.offset);

        Ok(Self {
            nanoseconds: new_nanos,
            offset: self.offset,
        })
    }

    fn set_minute(&self, minute: u32) -> Result<Self, AstrolabeError> {
        let nanos = add_offset_to_nanos(self.nanoseconds, self.offset);

        let new_nanos = set_minute(nanos, minute)?;

        let new_nanos = remove_offset_from_nanos(new_nanos, self.offset);

        Ok(Self {
            nanoseconds: new_nanos,
            offset: self.offset,
        })
    }

    fn set_second(&self, second: u32) -> Result<Self, AstrolabeError> {
        let nanos = add_offset_to_nanos(self.nanoseconds, self.offset);

        let new_nanos = set_second(nanos, second)?;

        let new_nanos = remove_offset_from_nanos(new_nanos, self.offset);

        Ok(Self {
            nanoseconds: new_nanos,
            offset: self.offset,
        })
    }

    fn set_milli(&self, milli: u32) -> Result<Self, AstrolabeError> {
        let nanos = add_offset_to_nanos(self.nanoseconds, self.offset);

        let new_nanos = set_milli(nanos, milli)?;

        let new_nanos = remove_offset_from_nanos(new_nanos, self.offset);

        Ok(Self {
            nanoseconds: new_nanos,
            offset: self.offset,
        })
    }

    fn set_micro(&self, micro: u32) -> Result<Self, AstrolabeError> {
        let nanos = add_offset_to_nanos(self.nanoseconds, self.offset);

        let new_nanos = set_micro(nanos, micro)?;

        let new_nanos = remove_offset_from_nanos(new_nanos, self.offset);

        Ok(Self {
            nanoseconds: new_nanos,
            offset: self.offset,
        })
    }

    fn set_nano(&self, nano: u32) -> Result<Self, AstrolabeError> {
        let nanos = add_offset_to_nanos(self.nanoseconds, self.offset);

        let new_nanos = set_nano(nanos, nano)?;

        let new_nanos = remove_offset_from_nanos(new_nanos, self.offset);

        Ok(Self {
            nanoseconds: new_nanos,
            offset: self.offset,
        })
    }

    fn add_hours(&self, hours: u32) -> Result<Self, AstrolabeError> {
        Self::from_nanos(add_hours(self.nanoseconds, hours))?.set_offset(self.offset)
    }

    fn add_minutes(&self, minutes: u32) -> Result<Self, AstrolabeError> {
        Self::from_nanos(add_minutes(self.nanoseconds, minutes))?.set_offset(self.offset)
    }

    fn add_seconds(&self, seconds: u32) -> Result<Self, AstrolabeError> {
        Self::from_nanos(add_seconds(self.nanoseconds, seconds))?.set_offset(self.offset)
    }

    fn add_millis(&self, millis: u32) -> Result<Self, AstrolabeError> {
        Self::from_nanos(add_millis(self.nanoseconds, millis))?.set_offset(self.offset)
    }

    fn add_micros(&self, micros: u32) -> Result<Self, AstrolabeError> {
        Self::from_nanos(add_micros(self.nanoseconds, micros))?.set_offset(self.offset)
    }

    fn add_nanos(&self, nanos: u32) -> Result<Self, AstrolabeError> {
        Self::from_nanos(self.nanoseconds + nanos as u64)?.set_offset(self.offset)
    }

    fn sub_hours(&self, hours: u32) -> Result<Self, AstrolabeError> {
        Self::from_nanos(
            sub_hours(self.nanoseconds as i64, hours)
                .try_into()
                .map_err(|_| {
                    create_custom_oor(format!(
                        "Subtracting {} hours would result into an out of range time",
                        hours
                    ))
                })?,
        )?
        .set_offset(self.offset)
    }

    fn sub_minutes(&self, minutes: u32) -> Result<Self, AstrolabeError> {
        Self::from_nanos(
            sub_minutes(self.nanoseconds as i64, minutes)
                .try_into()
                .map_err(|_| {
                    create_custom_oor(format!(
                        "Subtracting {} minutes would result into an out of range time",
                        minutes
                    ))
                })?,
        )?
        .set_offset(self.offset)
    }

    fn sub_seconds(&self, seconds: u32) -> Result<Self, AstrolabeError> {
        Self::from_nanos(
            sub_seconds(self.nanoseconds as i64, seconds)
                .try_into()
                .map_err(|_| {
                    create_custom_oor(format!(
                        "Subtracting {} seconds would result into an out of range time",
                        seconds
                    ))
                })?,
        )?
        .set_offset(self.offset)
    }

    fn sub_millis(&self, millis: u32) -> Result<Self, AstrolabeError> {
        Self::from_nanos(
            sub_millis(self.nanoseconds as i64, millis)
                .try_into()
                .map_err(|_| {
                    create_custom_oor(format!(
                        "Subtracting {} milliseconds would result into an out of range time",
                        millis
                    ))
                })?,
        )?
        .set_offset(self.offset)
    }

    fn sub_micros(&self, micros: u32) -> Result<Self, AstrolabeError> {
        Self::from_nanos(
            sub_micros(self.nanoseconds as i64, micros)
                .try_into()
                .map_err(|_| {
                    create_custom_oor(format!(
                        "Subtracting {} microseconds would result into an out of range time",
                        micros
                    ))
                })?,
        )?
        .set_offset(self.offset)
    }

    fn sub_nanos(&self, nanos: u32) -> Result<Self, AstrolabeError> {
        Self::from_nanos(
            (self.nanoseconds as i64 - nanos as i64)
                .try_into()
                .map_err(|_| {
                    create_custom_oor(format!(
                        "Subtracting {} nanoseconds would result into an out of range time",
                        nanos
                    ))
                })?,
        )?
        .set_offset(self.offset)
    }

    fn clear_until_hour(&self) -> Self {
        let nanoseconds = remove_offset_from_nanos(0, self.offset);
        Self {
            nanoseconds,
            offset: self.offset,
        }
    }

    fn clear_until_minute(&self) -> Self {
        let nanoseconds = add_offset_to_nanos(self.nanoseconds, self.offset);
        let nanoseconds =
            remove_offset_from_nanos(clear_nanos_until_nanos(nanoseconds), self.offset);
        Self {
            nanoseconds,
            offset: self.offset,
        }
    }

    fn clear_until_second(&self) -> Self {
        let nanoseconds = add_offset_to_nanos(self.nanoseconds, self.offset);
        let nanoseconds =
            remove_offset_from_nanos(clear_nanos_until_second(nanoseconds), self.offset);
        Self {
            nanoseconds,
            offset: self.offset,
        }
    }

    fn clear_until_milli(&self) -> Self {
        let nanoseconds = add_offset_to_nanos(self.nanoseconds, self.offset);
        let nanoseconds =
            remove_offset_from_nanos(clear_nanos_until_milli(nanoseconds), self.offset);
        Self {
            nanoseconds,
            offset: self.offset,
        }
    }

    fn clear_until_micro(&self) -> Self {
        let nanoseconds = add_offset_to_nanos(self.nanoseconds, self.offset);
        let nanoseconds =
            remove_offset_from_nanos(clear_nanos_until_micro(nanoseconds), self.offset);
        Self {
            nanoseconds,
            offset: self.offset,
        }
    }

    fn clear_until_nano(&self) -> Self {
        let nanoseconds = add_offset_to_nanos(self.nanoseconds, self.offset);
        let nanoseconds =
            remove_offset_from_nanos(clear_nanos_until_nanos(nanoseconds), self.offset);
        Self {
            nanoseconds,
            offset: self.offset,
        }
    }

    type SubDayReturn = i32;

    fn hours_since(&self, compare: &Self) -> Self::SubDayReturn {
        let self_total_hours = days_nanos_to_hours(0, self.nanoseconds) as i32;
        let self_subhour_nanos = nanos_to_subhour_nanos(self.nanoseconds);

        let compare_total_hours = days_nanos_to_hours(0, compare.nanoseconds) as i32;
        let compare_subhour_nanos = nanos_to_subhour_nanos(compare.nanoseconds);

        self_total_hours
            - compare_total_hours
            - if self_subhour_nanos < compare_subhour_nanos {
                1
            } else {
                0
            }
    }

    fn minutes_since(&self, compare: &Self) -> Self::SubDayReturn {
        let self_total_minutes = days_nanos_to_minutes(0, self.nanoseconds) as i32;
        let self_subminute_nanos = nanos_to_subminute_nanos(self.nanoseconds);

        let compare_total_minutes = days_nanos_to_minutes(0, compare.nanoseconds) as i32;
        let compare_subminute_nanos = nanos_to_subminute_nanos(compare.nanoseconds);

        self_total_minutes
            - compare_total_minutes
            - if self_subminute_nanos < compare_subminute_nanos {
                1
            } else {
                0
            }
    }

    fn seconds_since(&self, compare: &Self) -> Self::SubDayReturn {
        let self_total_seconds = days_nanos_to_seconds(0, self.nanoseconds) as i32;
        let self_subsecond_nanos = nanos_to_subsecond_nanos(self.nanoseconds);

        let compare_total_seconds = days_nanos_to_seconds(0, compare.nanoseconds) as i32;
        let compare_subsecond_nanos = nanos_to_subsecond_nanos(compare.nanoseconds);

        self_total_seconds
            - compare_total_seconds
            - if self_subsecond_nanos < compare_subsecond_nanos {
                1
            } else {
                0
            }
    }

    type SubSecReturn = i64;

    fn millis_since(&self, compare: &Self) -> Self::SubSecReturn {
        let self_total_millis = days_nanos_to_millis(0, self.nanoseconds) as i64;
        let self_submilli_nanos = nanos_to_submilli_nanos(self.nanoseconds);

        let compare_total_millis = days_nanos_to_millis(0, compare.nanoseconds) as i64;
        let compare_submilli_nanos = nanos_to_submilli_nanos(compare.nanoseconds);

        self_total_millis
            - compare_total_millis
            - if self_submilli_nanos < compare_submilli_nanos {
                1
            } else {
                0
            }
    }

    fn micros_since(&self, compare: &Self) -> Self::SubSecReturn {
        let self_total_micros = days_nanos_to_micros(0, self.nanoseconds) as i64;
        let self_submicro_nanos = nanos_to_submicro_nanos(self.nanoseconds);

        let compare_total_micros = days_nanos_to_micros(0, compare.nanoseconds) as i64;
        let compare_submicro_nanos = nanos_to_submicro_nanos(compare.nanoseconds);

        self_total_micros
            - compare_total_micros
            - if self_submicro_nanos < compare_submicro_nanos {
                1
            } else {
                0
            }
    }

    fn nanos_since(&self, compare: &Self) -> Self::SubSecReturn {
        let self_total_nanos = days_nanos_to_nanos(0, self.nanoseconds) as i64;

        let compare_total_nanos = days_nanos_to_nanos(0, compare.nanoseconds) as i64;

        self_total_nanos - compare_total_nanos
    }
}

// ########################################
//
//  OffsetUtility trait implementation
//
// ########################################

impl OffsetUtilities for Time {
    fn set_offset_hms(&self, hour: i32, minute: u32, second: u32) -> Result<Self, AstrolabeError> {
        let mut seconds = time_to_day_seconds(hour.unsigned_abs(), minute, second)? as i32;
        seconds = if hour.is_negative() {
            -seconds
        } else {
            seconds
        };

        Ok(self.set_offset(seconds).unwrap())
    }

    fn as_offset_hms(&self, hour: i32, minute: u32, second: u32) -> Result<Self, AstrolabeError> {
        let mut offset_secs = time_to_day_seconds(hour.unsigned_abs(), minute, second)? as i32;
        offset_secs = if hour.is_negative() {
            -offset_secs
        } else {
            offset_secs
        };

        let new_seconds = remove_offset_from_nanos(self.nanoseconds, offset_secs);

        Ok(Self::from_nanos(new_seconds)
            .unwrap()
            .set_offset(offset_secs)
            .unwrap())
    }

    fn get_offset_hms(&self) -> (i32, u32, u32) {
        let hour = self.offset / SECS_PER_HOUR as i32;
        let minute = self.offset % SECS_PER_HOUR as i32 / SECS_PER_MINUTE as i32;
        let second = self.offset % SECS_PER_MINUTE as i32;

        (hour, minute.unsigned_abs(), second.unsigned_abs())
    }

    fn set_offset(&self, seconds: i32) -> Result<Self, AstrolabeError> {
        if seconds <= -(SECS_PER_DAY as i32) || seconds >= SECS_PER_DAY as i32 {
            return Err(create_simple_oor(
                "seconds",
                -(SECS_PER_DAY as i128) + 1,
                SECS_PER_DAY as i128 - 1,
                seconds as i128,
            ));
        }

        Ok(Self {
            nanoseconds: self.nanoseconds,
            offset: seconds,
        })
    }

    fn as_offset(&self, seconds: i32) -> Result<Self, AstrolabeError> {
        let new_seconds = remove_offset_from_nanos(self.nanoseconds, seconds);
        Self::from_nanos(new_seconds).unwrap().set_offset(seconds)
    }

    fn get_offset(&self) -> i32 {
        self.offset
    }
}

// ########################################
//
//  Standard trait implementations
//
// ########################################

impl From<&Time> for Time {
    fn from(time: &Time) -> Self {
        Self {
            nanoseconds: time.nanoseconds,
            offset: time.offset,
        }
    }
}

impl From<DateTime> for Time {
    fn from(value: DateTime) -> Self {
        Self {
            nanoseconds: (value.as_nanos() % NANOS_PER_DAY as i128) as u64,
            offset: value.get_offset(),
        }
    }
}
impl From<&DateTime> for Time {
    fn from(value: &DateTime) -> Self {
        Self {
            nanoseconds: (value.as_nanos() % NANOS_PER_DAY as i128) as u64,
            offset: value.get_offset(),
        }
    }
}

impl Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.format("HH:mm:ss"))
    }
}

impl PartialEq for Time {
    fn eq(&self, rhs: &Self) -> bool {
        self.as_nanos() == rhs.as_nanos()
    }
}
impl PartialOrd for Time {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.nanoseconds.partial_cmp(&other.nanoseconds)
    }
}

impl Ord for Time {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.nanoseconds.cmp(&other.nanoseconds)
    }
}

impl FromStr for Time {
    type Err = AstrolabeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Time::parse(s, "HH:mm:ss")
    }
}

impl Add for Time {
    type Output = Time;

    fn add(self, rhs: Self) -> Self::Output {
        Time {
            nanoseconds: self.nanoseconds + rhs.nanoseconds,
            offset: self.offset,
        }
    }
}
impl AddAssign for Time {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Time {
    type Output = Time;

    fn sub(self, rhs: Self) -> Self::Output {
        Time {
            nanoseconds: self.nanoseconds - rhs.nanoseconds,
            offset: self.offset,
        }
    }
}
impl SubAssign for Time {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Add<Duration> for Time {
    type Output = Self;

    fn add(self, rhs: Duration) -> Self::Output {
        let nanos = self.as_nanos() + rhs.as_nanos() as u64;
        Self::from_nanos(nanos).unwrap()
    }
}
impl AddAssign<Duration> for Time {
    fn add_assign(&mut self, rhs: Duration) {
        *self = *self + rhs;
    }
}

impl Sub<Duration> for Time {
    type Output = Self;

    fn sub(self, rhs: Duration) -> Self::Output {
        let nanos = self.as_nanos() - rhs.as_nanos() as u64;
        Self::from_nanos(nanos).unwrap()
    }
}
impl SubAssign<Duration> for Time {
    fn sub_assign(&mut self, rhs: Duration) {
        *self = *self - rhs;
    }
}

// ########################################
//
//  Serde implementations
//
// ########################################

#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
mod serde {
    use crate::Time;
    use serde::de;
    use serde::ser;
    use std::fmt;

    /// Serialize a [`Time`] instance as `HH:mm:ss`.
    impl ser::Serialize for Time {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ser::Serializer,
        {
            serializer.serialize_str(&self.format("HH:mm:ss"))
        }
    }

    struct TimeVisitor;

    impl<'de> de::Visitor<'de> for TimeVisitor {
        type Value = Time;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a formatted date string in the format `HH:mm:ss`")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            value.parse().map_err(E::custom)
        }
    }

    /// Deserialize a `HH:mm:ss` formatted string into a [`Time`] instance.
    impl<'de> de::Deserialize<'de> for Time {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            deserializer.deserialize_str(TimeVisitor)
        }
    }
}
