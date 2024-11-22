use crate::{
    errors::{out_of_range::create_simple_oor, AstrolabeError},
    util::{
        constants::{
            NANOS_PER_DAY, NANOS_PER_SEC, SECS_PER_DAY, SECS_PER_DAY_U64, SECS_PER_HOUR_U64,
            SECS_PER_MINUTE_U64,
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
                nanos_to_time, since_i32, since_i64, time_to_day_seconds,
            },
            manipulate::{
                add_hours, add_micros, add_millis, add_minutes, add_seconds,
                clear_nanos_until_micro, clear_nanos_until_milli, clear_nanos_until_minute,
                clear_nanos_until_nanos, clear_nanos_until_second, set_hour, set_micro, set_milli,
                set_minute, set_nano, set_second, sub_hours, sub_micros, sub_millis, sub_minutes,
                sub_seconds,
            },
        },
    },
    DateTime, Offset, OffsetUtilities, TimeUtilities,
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
/// [`OffsetUtilities`](#impl-OffsetUtilities-for-Time) implements methods for setting and getting the offset.
#[derive(Debug, Default, Clone, Copy, Eq)]
pub struct Time {
    pub(crate) nanoseconds: u64,
    pub(crate) offset: Offset,
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
            offset: Offset::default(),
        }
    }

    /// Creates a new [`Time`] instance with [`SystemTime::now()`] with the local timezone as the offset.
    ///
    /// ```rust
    /// # use astrolabe::{Time, Offset, OffsetUtilities};
    /// let time = Time::now_local();
    /// println!("{}", time);
    /// assert_eq!(time.get_offset(), Offset::Local);
    /// ```
    pub fn now_local() -> Self {
        Self::now().set_offset(Offset::Local)
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
            offset: Offset::default(),
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
            offset: Offset::default(),
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

    /// Creates a new [`Time`] instance from nanoseconds.
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
            offset: Offset::default(),
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
            Self::from_nanos(nanoseconds)?.as_offset(Offset::from_seconds(offset)?)
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
        let offset_seconds = self.offset.resolve();

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
                    add_offset_to_nanos(self.nanoseconds, offset_seconds),
                    offset_seconds,
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
        let nanos = add_offset_to_nanos(self.nanoseconds, self.offset.resolve());

        nanos_to_time(nanos).0
    }

    fn minute(&self) -> u32 {
        let nanos = add_offset_to_nanos(self.nanoseconds, self.offset.resolve());

        nanos_to_time(nanos).1
    }

    fn second(&self) -> u32 {
        let nanos = add_offset_to_nanos(self.nanoseconds, self.offset.resolve());

        nanos_to_time(nanos).2
    }

    fn milli(&self) -> u32 {
        let nanos = add_offset_to_nanos(self.nanoseconds, self.offset.resolve());

        nanos_to_subsecond(nanos).0
    }

    fn micro(&self) -> u32 {
        let nanos = add_offset_to_nanos(self.nanoseconds, self.offset.resolve());

        nanos_to_subsecond(nanos).1
    }

    fn nano(&self) -> u32 {
        let nanos = add_offset_to_nanos(self.nanoseconds, self.offset.resolve());

        nanos_to_subsecond(nanos).2
    }

    fn set_hour(&self, hour: u32) -> Result<Self, AstrolabeError> {
        let offset_seconds = self.offset.resolve();

        let nanos = add_offset_to_nanos(self.nanoseconds, offset_seconds);

        let new_nanos = set_hour(nanos, hour)?;

        let new_nanos = remove_offset_from_nanos(new_nanos, offset_seconds);

        Ok(Self {
            nanoseconds: new_nanos,
            offset: self.offset,
        })
    }

    fn set_minute(&self, minute: u32) -> Result<Self, AstrolabeError> {
        let offset_seconds = self.offset.resolve();

        let nanos = add_offset_to_nanos(self.nanoseconds, offset_seconds);

        let new_nanos = set_minute(nanos, minute)?;

        let new_nanos = remove_offset_from_nanos(new_nanos, offset_seconds);

        Ok(Self {
            nanoseconds: new_nanos,
            offset: self.offset,
        })
    }

    fn set_second(&self, second: u32) -> Result<Self, AstrolabeError> {
        let offset_seconds = self.offset.resolve();

        let nanos = add_offset_to_nanos(self.nanoseconds, offset_seconds);

        let new_nanos = set_second(nanos, second)?;

        let new_nanos = remove_offset_from_nanos(new_nanos, offset_seconds);

        Ok(Self {
            nanoseconds: new_nanos,
            offset: self.offset,
        })
    }

    fn set_milli(&self, milli: u32) -> Result<Self, AstrolabeError> {
        let offset_seconds = self.offset.resolve();

        let nanos = add_offset_to_nanos(self.nanoseconds, offset_seconds);

        let new_nanos = set_milli(nanos, milli)?;

        let new_nanos = remove_offset_from_nanos(new_nanos, offset_seconds);

        Ok(Self {
            nanoseconds: new_nanos,
            offset: self.offset,
        })
    }

    fn set_micro(&self, micro: u32) -> Result<Self, AstrolabeError> {
        let offset_seconds = self.offset.resolve();

        let nanos = add_offset_to_nanos(self.nanoseconds, offset_seconds);

        let new_nanos = set_micro(nanos, micro)?;

        let new_nanos = remove_offset_from_nanos(new_nanos, offset_seconds);

        Ok(Self {
            nanoseconds: new_nanos,
            offset: self.offset,
        })
    }

    fn set_nano(&self, nano: u32) -> Result<Self, AstrolabeError> {
        let offset_seconds = self.offset.resolve();

        let nanos = add_offset_to_nanos(self.nanoseconds, offset_seconds);

        let new_nanos = set_nano(nanos, nano)?;

        let new_nanos = remove_offset_from_nanos(new_nanos, offset_seconds);

        Ok(Self {
            nanoseconds: new_nanos,
            offset: self.offset,
        })
    }

    /// Wraps around from `23:59:59` to `00:00:00`
    fn add_hours(&self, hours: u32) -> Self {
        Self {
            nanoseconds: add_hours(self.nanoseconds, hours) % (SECS_PER_DAY_U64 * NANOS_PER_SEC),
            offset: self.offset,
        }
    }

    /// Wraps around from `23:59:59` to `00:00:00`
    fn add_minutes(&self, minutes: u32) -> Self {
        Self {
            nanoseconds: add_minutes(self.nanoseconds, minutes)
                % (SECS_PER_DAY_U64 * NANOS_PER_SEC),
            offset: self.offset,
        }
    }

    /// Wraps around from `23:59:59` to `00:00:00`
    fn add_seconds(&self, seconds: u32) -> Self {
        Self {
            nanoseconds: add_seconds(self.nanoseconds, seconds)
                % (SECS_PER_DAY_U64 * NANOS_PER_SEC),
            offset: self.offset,
        }
    }

    /// Wraps around from `23:59:59` to `00:00:00`
    fn add_millis(&self, millis: u32) -> Self {
        Self {
            nanoseconds: add_millis(self.nanoseconds, millis) % (SECS_PER_DAY_U64 * NANOS_PER_SEC),
            offset: self.offset,
        }
    }

    /// Wraps around from `23:59:59` to `00:00:00`
    fn add_micros(&self, micros: u32) -> Self {
        Self {
            nanoseconds: add_micros(self.nanoseconds, micros) % (SECS_PER_DAY_U64 * NANOS_PER_SEC),
            offset: self.offset,
        }
    }

    /// Wraps around from `23:59:59` to `00:00:00`
    fn add_nanos(&self, nanos: u32) -> Self {
        Self {
            nanoseconds: (self.nanoseconds + nanos as u64) % (SECS_PER_DAY_U64 * NANOS_PER_SEC),
            offset: self.offset,
        }
    }

    /// Wraps around from `00:00:00` to `23:59:59`
    fn sub_hours(&self, hours: u32) -> Self {
        let new_nanos = sub_hours(self.nanoseconds as i64, hours);
        let rhs = SECS_PER_DAY_U64 as i64 * NANOS_PER_SEC as i64;
        Self {
            nanoseconds: new_nanos.rem_euclid(rhs) as u64,
            offset: self.offset,
        }
    }

    /// Wraps around from `00:00:00` to `23:59:59`
    fn sub_minutes(&self, minutes: u32) -> Self {
        let new_nanos = sub_minutes(self.nanoseconds as i64, minutes);
        let rhs = SECS_PER_DAY_U64 as i64 * NANOS_PER_SEC as i64;
        Self {
            nanoseconds: new_nanos.rem_euclid(rhs) as u64,
            offset: self.offset,
        }
    }

    /// Wraps around from `00:00:00` to `23:59:59`
    fn sub_seconds(&self, seconds: u32) -> Self {
        let new_nanos = sub_seconds(self.nanoseconds as i64, seconds);
        let rhs = SECS_PER_DAY_U64 as i64 * NANOS_PER_SEC as i64;
        Self {
            nanoseconds: new_nanos.rem_euclid(rhs) as u64,
            offset: self.offset,
        }
    }

    /// Wraps around from `00:00:00` to `23:59:59`
    fn sub_millis(&self, millis: u32) -> Self {
        let new_nanos = sub_millis(self.nanoseconds as i64, millis);
        let rhs = SECS_PER_DAY_U64 as i64 * NANOS_PER_SEC as i64;
        Self {
            nanoseconds: new_nanos.rem_euclid(rhs) as u64,
            offset: self.offset,
        }
    }

    /// Wraps around from `00:00:00` to `23:59:59`
    fn sub_micros(&self, micros: u32) -> Self {
        let new_nanos = sub_micros(self.nanoseconds as i64, micros);
        let rhs = SECS_PER_DAY_U64 as i64 * NANOS_PER_SEC as i64;
        Self {
            nanoseconds: new_nanos.rem_euclid(rhs) as u64,
            offset: self.offset,
        }
    }

    /// Wraps around from `00:00:00` to `23:59:59`
    fn sub_nanos(&self, nanos: u32) -> Self {
        let new_nanos = self.nanoseconds as i64 - nanos as i64;
        let rhs = SECS_PER_DAY_U64 as i64 * NANOS_PER_SEC as i64;
        Self {
            nanoseconds: new_nanos.rem_euclid(rhs) as u64,
            offset: self.offset,
        }
    }

    fn clear_until_hour(&self) -> Self {
        let nanoseconds = remove_offset_from_nanos(0, self.offset.resolve());
        Self {
            nanoseconds,
            offset: self.offset,
        }
    }

    fn clear_until_minute(&self) -> Self {
        let offset_seconds = self.offset.resolve();

        let nanoseconds = add_offset_to_nanos(self.nanoseconds, offset_seconds);
        let nanoseconds =
            remove_offset_from_nanos(clear_nanos_until_minute(nanoseconds), offset_seconds);
        Self {
            nanoseconds,
            offset: self.offset,
        }
    }

    fn clear_until_second(&self) -> Self {
        let offset_seconds = self.offset.resolve();

        let nanoseconds = add_offset_to_nanos(self.nanoseconds, offset_seconds);
        let nanoseconds =
            remove_offset_from_nanos(clear_nanos_until_second(nanoseconds), offset_seconds);
        Self {
            nanoseconds,
            offset: self.offset,
        }
    }

    fn clear_until_milli(&self) -> Self {
        let offset_seconds = self.offset.resolve();

        let nanoseconds = add_offset_to_nanos(self.nanoseconds, offset_seconds);
        let nanoseconds =
            remove_offset_from_nanos(clear_nanos_until_milli(nanoseconds), offset_seconds);
        Self {
            nanoseconds,
            offset: self.offset,
        }
    }

    fn clear_until_micro(&self) -> Self {
        let offset_seconds = self.offset.resolve();

        let nanoseconds = add_offset_to_nanos(self.nanoseconds, offset_seconds);
        let nanoseconds =
            remove_offset_from_nanos(clear_nanos_until_micro(nanoseconds), offset_seconds);
        Self {
            nanoseconds,
            offset: self.offset,
        }
    }

    fn clear_until_nano(&self) -> Self {
        let offset_seconds = self.offset.resolve();

        let nanoseconds = add_offset_to_nanos(self.nanoseconds, offset_seconds);
        let nanoseconds =
            remove_offset_from_nanos(clear_nanos_until_nanos(nanoseconds), offset_seconds);
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

        since_i32(
            self_total_hours,
            self_subhour_nanos,
            compare_total_hours,
            compare_subhour_nanos,
        )
    }

    fn minutes_since(&self, compare: &Self) -> Self::SubDayReturn {
        let self_total_minutes = days_nanos_to_minutes(0, self.nanoseconds) as i32;
        let self_subminute_nanos = nanos_to_subminute_nanos(self.nanoseconds);

        let compare_total_minutes = days_nanos_to_minutes(0, compare.nanoseconds) as i32;
        let compare_subminute_nanos = nanos_to_subminute_nanos(compare.nanoseconds);

        since_i32(
            self_total_minutes,
            self_subminute_nanos,
            compare_total_minutes,
            compare_subminute_nanos,
        )
    }

    fn seconds_since(&self, compare: &Self) -> Self::SubDayReturn {
        let self_total_seconds = days_nanos_to_seconds(0, self.nanoseconds) as i32;
        let self_subsecond_nanos = nanos_to_subsecond_nanos(self.nanoseconds);

        let compare_total_seconds = days_nanos_to_seconds(0, compare.nanoseconds) as i32;
        let compare_subsecond_nanos = nanos_to_subsecond_nanos(compare.nanoseconds);

        since_i32(
            self_total_seconds,
            self_subsecond_nanos,
            compare_total_seconds,
            compare_subsecond_nanos,
        )
    }

    type SubSecReturn = i64;

    fn millis_since(&self, compare: &Self) -> Self::SubSecReturn {
        let self_total_millis = days_nanos_to_millis(0, self.nanoseconds) as i64;
        let self_submilli_nanos = nanos_to_submilli_nanos(self.nanoseconds);

        let compare_total_millis = days_nanos_to_millis(0, compare.nanoseconds) as i64;
        let compare_submilli_nanos = nanos_to_submilli_nanos(compare.nanoseconds);

        since_i64(
            self_total_millis,
            self_submilli_nanos,
            compare_total_millis,
            compare_submilli_nanos,
        )
    }

    fn micros_since(&self, compare: &Self) -> Self::SubSecReturn {
        let self_total_micros = days_nanos_to_micros(0, self.nanoseconds) as i64;
        let self_submicro_nanos = nanos_to_submicro_nanos(self.nanoseconds);

        let compare_total_micros = days_nanos_to_micros(0, compare.nanoseconds) as i64;
        let compare_submicro_nanos = nanos_to_submicro_nanos(compare.nanoseconds);

        since_i64(
            self_total_micros,
            self_submicro_nanos,
            compare_total_micros,
            compare_submicro_nanos,
        )
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
    fn set_offset(&self, offset: Offset) -> Self {
        Self {
            nanoseconds: self.nanoseconds,
            offset,
        }
    }

    fn as_offset(&self, offset: Offset) -> Self {
        let new_seconds = remove_offset_from_nanos(self.nanoseconds, offset.resolve());
        Self::from_nanos(new_seconds).unwrap().set_offset(offset)
    }

    fn get_offset(&self) -> Offset {
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
        Some(self.cmp(other))
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
        Self::parse(s, "HH:mm:ss")
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
