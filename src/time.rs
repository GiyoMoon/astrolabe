use crate::{
    errors::{
        out_of_range::{create_custom_oor, create_simple_oor},
        AstrolabeError,
    },
    shared::{
        NANOS_PER_SEC, SECS_PER_DAY, SECS_PER_DAY_U64, SECS_PER_HOUR_U64, SECS_PER_MINUTE_U64,
    },
    util::{
        convert::{
            add_offset_to_nanos, nanos_to_time, nanos_to_unit, remove_offset_from_nanos,
            time_to_day_seconds,
        },
        format::format_time_part,
        manipulation::{apply_time_unit, set_time_unit},
        parse::{parse_format_string, parse_time_part, ParseUnit, ParsedTime, Period},
    },
    DateTime, DateTimeUnit, Offset,
};
use std::{
    cmp,
    fmt::Display,
    str::FromStr,
    time::{SystemTime, UNIX_EPOCH},
};

/// Time units for functions like [`Time::get`] or [`Time::apply`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TimeUnit {
    #[allow(missing_docs)]
    Hour,
    #[allow(missing_docs)]
    Min,
    #[allow(missing_docs)]
    Sec,
    #[allow(missing_docs)]
    Centis,
    #[allow(missing_docs)]
    Millis,
    #[allow(missing_docs)]
    Micros,
    #[allow(missing_docs)]
    Nanos,
}

/// Clock time with nanosecond precision.
#[derive(Debug, Default, Copy, Clone, Eq)]
pub struct Time {
    nanoseconds: u64,
    offset: i32,
}

impl Time {
    /// Creates a new [`Time`] instance with [`SystemTime::now()`].
    ///
    /// ```rust
    /// # use astrolabe::{Time, TimeUnit};
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
    pub fn from_hms(hour: u32, min: u32, sec: u32) -> Result<Self, AstrolabeError> {
        let seconds = time_to_day_seconds(hour, min, sec)? as u64;

        Ok(Self {
            nanoseconds: seconds * NANOS_PER_SEC,
            offset: 0,
        })
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

    /// Creates a new [`Time`] instance from seconds.
    ///
    /// Returns an [`OutOfRange`](AstrolabeError::OutOfRange) error if the provided nanoseconds are invalid (over `86_399_999_999_999`)
    ///
    /// ```rust
    /// # use astrolabe::{Time, TimeUnit};
    /// let time = Time::from_nanoseconds(1_234).unwrap();
    /// assert_eq!(1_234, time.as_nanoseconds());
    /// ```
    pub fn from_nanoseconds(nanoseconds: u64) -> Result<Self, AstrolabeError> {
        if nanoseconds >= SECS_PER_DAY_U64 * NANOS_PER_SEC {
            return Err(create_simple_oor(
                "nanoseconds",
                0,
                SECS_PER_DAY_U64 as i128 * NANOS_PER_SEC as i128 - 1,
                nanoseconds as i128,
            ));
        }
        Ok(Self {
            nanoseconds,
            offset: 0,
        })
    }

    /// Returns the time as seconds.
    ///
    /// ```rust
    /// # use astrolabe::{Time, TimeUnit};
    /// let time = Time::from_hms(12, 12, 12).unwrap();
    /// assert_eq!(43932, time.as_seconds());
    /// ```
    pub fn as_seconds(&self) -> u64 {
        self.nanoseconds / NANOS_PER_SEC
    }

    /// Returns the time as nanoseconds.
    ///
    /// ```rust
    /// # use astrolabe::{Time, TimeUnit};
    /// let time = Time::from_hms(12, 12, 12).unwrap();
    /// assert_eq!(43_932_000_000_000, time.as_nanoseconds());
    /// ```
    pub fn as_nanoseconds(&self) -> u64 {
        self.nanoseconds
    }

    /// Returns the number of nanoseconds between two [`Time`] instances.
    ///
    /// ```rust
    /// # use astrolabe::Time;
    /// let time = Time::from_hms(12, 0, 0).unwrap();
    /// let time_2 = Time::from_hms(12, 0, 1).unwrap();
    /// assert_eq!(1_000_000_000, time.between(&time_2));
    /// assert_eq!(1_000_000_000, time_2.between(&time));
    /// ```
    pub fn between(&self, compare: &Self) -> u64 {
        (self.nanoseconds as i64 - compare.nanoseconds as i64).unsigned_abs()
    }

    /// Get a specific [`TimeUnit`].
    ///
    /// ```rust
    /// # use astrolabe::{Time, TimeUnit};
    /// let time = Time::from_hms(12, 32, 15).unwrap();
    /// assert_eq!(12, time.get(TimeUnit::Hour));
    /// assert_eq!(32, time.get(TimeUnit::Min));
    /// assert_eq!(15, time.get(TimeUnit::Sec));
    ///
    /// let time = Time::from_nanoseconds(1_123_456_789).unwrap();
    /// assert_eq!(12, time.get(TimeUnit::Centis));
    /// assert_eq!(123, time.get(TimeUnit::Millis));
    /// assert_eq!(123_456, time.get(TimeUnit::Micros));
    /// assert_eq!(123_456_789, time.get(TimeUnit::Nanos));
    /// ```
    pub fn get(&self, unit: TimeUnit) -> u64 {
        nanos_to_unit(add_offset_to_nanos(self.nanoseconds, self.offset), unit)
    }

    /// Creates a new [`Time`] instance with a specific [`TimeUnit`] set to the provided value.
    ///
    /// Returns an [`OutOfRange`](AstrolabeError::OutOfRange) error if the provided value is invalid or out of range.
    ///
    /// ```rust
    /// # use astrolabe::{Time, TimeUnit};
    /// let mut time = Time::from_hms(12, 32, 15).unwrap();
    /// time = time.set(15, TimeUnit::Hour).unwrap();
    /// time = time.set(10, TimeUnit::Min).unwrap();
    /// assert_eq!("15:10:15", time.format("HH:mm:ss"));
    /// ```
    pub fn set(&self, value: u32, unit: TimeUnit) -> Result<Self, AstrolabeError> {
        Ok(Self {
            nanoseconds: remove_offset_from_nanos(
                set_time_unit(
                    add_offset_to_nanos(self.nanoseconds, self.offset),
                    value,
                    unit,
                )?,
                self.offset,
            ),
            offset: self.offset,
        })
    }

    /// Creates a new [`Time`] instance with a specified amount of time applied (added or subtracted).
    ///
    /// Returns an [`OutOfRange`](AstrolabeError::OutOfRange) error if the provided value would result in an out of range time.
    ///
    /// ```rust
    /// # use astrolabe::{Time, TimeUnit};
    /// let time = Time::from_hms(12, 32, 15).unwrap();
    ///
    /// let applied = time.apply(1, TimeUnit::Hour).unwrap();
    /// assert_eq!("12:32:15", time.format("HH:mm:ss"));
    /// assert_eq!("13:32:15", applied.format("HH:mm:ss"));
    ///
    /// let applied_2 = applied.apply(-1, TimeUnit::Hour).unwrap();
    /// assert_eq!("12:32:15", applied_2.format("HH:mm:ss"));
    /// ```
    pub fn apply(&self, amount: i64, unit: TimeUnit) -> Result<Self, AstrolabeError> {
        Ok(Self::from_nanoseconds(
            apply_time_unit(self.nanoseconds as i128, amount, unit)
                .try_into()
                .map_err(|_| {
                    create_custom_oor(format!(
                        "Appling {} would result into an out of range time",
                        amount
                    ))
                })?,
        )?
        .set_offset(self.offset)
        .unwrap())
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
                    ParseUnit::Min => time.min = Some(parsed_part.value as u64),
                    ParseUnit::Sec => time.sec = Some(parsed_part.value as u64),
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
        nanoseconds += time.min.unwrap_or(0) * SECS_PER_MINUTE_U64 * NANOS_PER_SEC;
        nanoseconds += time.sec.unwrap_or(0) * NANOS_PER_SEC;
        nanoseconds += time.decis.unwrap_or(0) * 100_000_000;
        nanoseconds += time.centis.unwrap_or(0) * 10_000_000;
        nanoseconds += time.millis.unwrap_or(0) * 1_000_000;
        nanoseconds += time.micros.unwrap_or(0) * 1_000;
        nanoseconds += time.nanos.unwrap_or(0);

        Ok(if let Some(offset) = time.offset {
            Time::from_nanoseconds(nanoseconds)?.set_offset(offset)?
        } else {
            Time::from_nanoseconds(nanoseconds)?
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

    /// Creates an new [`Time`] struct with the values cleared until the given unit. Is inclusive.
    ///
    /// ```rust
    /// # use astrolabe::{Time, TimeUnit};
    /// let time = Time::from_hms(12, 32, 1).unwrap();
    /// assert_eq!("12:32:00", time.clear_until(TimeUnit::Sec).format("HH:mm:ss"));
    /// assert_eq!("12:00:00", time.clear_until(TimeUnit::Min).format("HH:mm:ss"));
    /// ```
    pub fn clear_until(&self, unit: TimeUnit) -> Self {
        match unit {
            TimeUnit::Hour => Time::default(),
            TimeUnit::Min => {
                let (hour, _, _) = nanos_to_time(self.nanoseconds);
                // Using unwrap because it's safe to assume that the provided values are valid
                Time::from_hms(hour, 0, 0).unwrap()
            }
            TimeUnit::Sec => {
                let (hour, min, _) = nanos_to_time(self.nanoseconds);
                // Using unwrap because it's safe to assume that the provided values are valid
                Time::from_hms(hour, min, 0).unwrap()
            }
            TimeUnit::Centis => {
                let (hour, min, sec) = nanos_to_time(self.nanoseconds);
                // Using unwrap because it's safe to assume that the provided values are valid
                Time::from_hms(hour, min, sec).unwrap()
            }
            TimeUnit::Millis => {
                // Using unwrap because it's safe to assume that the provided values are valid
                Time::from_nanoseconds(self.as_nanoseconds() / 10_000_000 * 10_000_000).unwrap()
            }
            TimeUnit::Micros => {
                // Using unwrap because it's safe to assume that the provided values are valid
                Time::from_nanoseconds(self.as_nanoseconds() / 1_000_000 * 1_000_000).unwrap()
            }
            TimeUnit::Nanos => {
                // Using unwrap because it's safe to assume that the provided values are valid
                Time::from_nanoseconds(self.as_nanoseconds() / 1_000 * 1_000).unwrap()
            }
        }
    }

    /// Creates a new [`Time`] instance with a given timezone offset defined as time units (hour, minute and second). Offset can range anywhere from `UTC-23:59:59` to `UTC+23:59:59`.
    ///
    /// The offset affects all format functions and the [`get`](Time::get) and [`set`](Time::set) functions but does not change the time itself which always represents UTC.
    ///
    /// Returns an [`OutOfRange`](AstrolabeError::OutOfRange) error if the provided offset is not between `UTC-23:59:59` and `UTC+23:59:59`.
    ///
    /// ```rust
    /// # use astrolabe::{Time, Offset};
    /// let time = Time::from_hms(12, 32, 1).unwrap();
    /// // Set offset to UTC+2
    /// let with_offset = time.set_offset_time(2, 0, 0, Offset::East).unwrap();
    /// assert_eq!("14:32:01", with_offset.format("HH:mm:ss"));
    /// ```
    pub fn set_offset_time(
        &self,
        hour: u32,
        minute: u32,
        second: u32,
        offset: Offset,
    ) -> Result<Self, AstrolabeError> {
        let mut seconds = time_to_day_seconds(hour, minute, second)? as i32;
        seconds = if offset == Offset::West {
            -seconds
        } else {
            seconds
        };

        Ok(self.set_offset(seconds).unwrap())
    }

    /// Creates a new [`Time`] instance with a given timezone offset defined as seconds. Offset can range anywhere from `UTC-23:59:59` to `UTC+23:59:59`.
    ///
    /// The offset affects all format functions and the [`get`](Time::get) and [`set`](Time::set) functions but does not change the time itself which always represents UTC.
    ///
    /// Returns an [`OutOfRange`](AstrolabeError::OutOfRange) error if the provided offset is not between `UTC-23:59:59` and `UTC+23:59:59`.
    ///
    /// ```rust
    /// # use astrolabe::Time;
    /// let time = Time::from_hms(12, 32, 1).unwrap();
    /// // Set offset to UTC+2
    /// let with_offset = time.set_offset(7200).unwrap();
    /// assert_eq!("14:32:01", with_offset.format("HH:mm:ss"));
    /// ```
    pub fn set_offset(&self, seconds: i32) -> Result<Self, AstrolabeError> {
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

    /// Creates a new [`Time`] instance, assuming the current instance has the provided offset applied. The new instance will have the specified offset and the time itself will be converted to `UTC`.
    ///
    /// The offset affects all format functions and the [`get`](Time::get) and [`set`](Time::set) functions but does not change the time itself which always represents UTC.
    ///
    /// Returns an [`OutOfRange`](AstrolabeError::OutOfRange) error if the provided offset is not between `UTC-23:59:59` and `UTC+23:59:59`.
    ///
    /// ```rust
    /// # use astrolabe::{Time, Offset};
    /// let time = Time::from_hms(12, 32, 1).unwrap();
    /// // Set offset to UTC+2
    /// let with_offset = time.as_offset_time(2, 0, 0, Offset::East).unwrap();
    /// assert_eq!("12:32:01", with_offset.format("HH:mm:ss"));
    /// ```
    pub fn as_offset_time(
        &self,
        hour: u32,
        minute: u32,
        second: u32,
        offset: Offset,
    ) -> Result<Self, AstrolabeError> {
        let mut offset_secs = time_to_day_seconds(hour, minute, second)? as i32;
        offset_secs = if offset == Offset::West {
            -offset_secs
        } else {
            offset_secs
        };

        let new_seconds = remove_offset_from_nanos(self.nanoseconds, offset_secs);

        Ok(Self::from_nanoseconds(new_seconds)
            .unwrap()
            .set_offset(offset_secs)
            .unwrap())
    }

    /// Creates a new [`Time`] instance, assuming the current instance has the provided offset applied. The new instance will have the specified offset and the time itself will be converted to `UTC`.
    ///
    /// The offset affects all format functions and the [`get`](Time::get) and [`set`](Time::set) functions but does not change the time itself which always represents UTC.
    ///
    /// Returns an [`OutOfRange`](AstrolabeError::OutOfRange) error if the provided offset is not between `UTC-23:59:59` and `UTC+23:59:59`.
    ///
    /// ```rust
    /// # use astrolabe::Time;
    /// let time = Time::from_hms(12, 32, 1).unwrap();
    /// // Set offset to UTC+2
    /// let with_offset = time.as_offset(7200).unwrap();
    /// assert_eq!("12:32:01", with_offset.format("HH:mm:ss"));
    /// ```
    pub fn as_offset(&self, seconds: i32) -> Result<Self, AstrolabeError> {
        let new_seconds = remove_offset_from_nanos(self.nanoseconds, seconds);
        Self::from_nanoseconds(new_seconds)
            .unwrap()
            .set_offset(seconds)
    }

    /// Returns the set offset in seconds.
    ///
    /// ```rust
    /// # use astrolabe::Time;
    /// let time = Time::now().set_offset(3600).unwrap();
    /// assert_eq!(3600, time.get_offset());
    /// ```
    pub fn get_offset(&self) -> i32 {
        self.offset
    }
}

impl From<&Time> for Time {
    fn from(time: &Time) -> Self {
        Self {
            nanoseconds: time.nanoseconds,
            offset: time.offset,
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
        self.as_nanoseconds() == rhs.as_nanoseconds()
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

impl From<DateTime> for Time {
    fn from(value: DateTime) -> Self {
        Self {
            nanoseconds: value.get(DateTimeUnit::Nanos) as u64,
            offset: value.get_offset(),
        }
    }
}

impl From<&DateTime> for Time {
    fn from(value: &DateTime) -> Self {
        Self {
            nanoseconds: value.get(DateTimeUnit::Nanos) as u64,
            offset: value.get_offset(),
        }
    }
}

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
