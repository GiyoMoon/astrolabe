use crate::{
    shared::{NANOS_PER_SEC, SECS_PER_DAY, SECS_PER_DAY_U64},
    util::{
        convert::{nanos_as_unit, nanos_to_unit, time_to_day_seconds},
        format::{format_time_part, parse_format_string},
        manipulation::{apply_time_unit, set_time_unit},
    },
    AstrolabeError,
};
use std::time::{SystemTime, UNIX_EPOCH};

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
#[derive(Debug, Default)]
pub struct Time(u64);

impl Time {
    /// Creates a new [`Time`] instance with [`SystemTime::now()`].
    ///
    /// # Example
    /// ```rust
    /// use astrolabe::{Time, TimeUnit};
    ///
    /// let time = Time::now();
    /// assert!(24 > time.get(TimeUnit::Hour));
    /// ```
    pub fn now() -> Self {
        let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let nanos =
            duration.as_secs() % SECS_PER_DAY_U64 * NANOS_PER_SEC + duration.subsec_nanos() as u64;
        Time(nanos)
    }

    /// Creates a new [`Time`] instance from hour, minute and seconds.
    ///
    /// Returns an [`OutOfRange`](AstrolabeError::OutOfRange) error if the provided time is invalid.
    ///
    /// # Example
    /// ```rust
    /// use astrolabe::Time;
    ///
    /// let time = Time::from_hms(12, 32, 01).unwrap();
    /// assert_eq!("12:32:01", time.format("HH:mm:ss").unwrap());
    /// ```
    pub fn from_hms(hour: u32, min: u32, sec: u32) -> Result<Self, AstrolabeError> {
        let seconds = time_to_day_seconds(hour, min, sec)? as u64;

        Ok(Time(seconds * NANOS_PER_SEC))
    }

    /// Creates a new [`Time`] instance from seconds.
    ///
    /// Returns an [`OutOfRange`](AstrolabeError::OutOfRange) error if the provided seconds are invalid (over `86399`)
    ///
    /// # Example
    /// ```rust
    /// use astrolabe::Time;
    ///
    /// let time = Time::from_seconds(1_234).unwrap();
    /// assert_eq!("00:20:34", time.format("HH:mm:ss").unwrap());
    /// ```
    pub fn from_seconds(seconds: u32) -> Result<Self, AstrolabeError> {
        if seconds > SECS_PER_DAY - 1 {
            return Err(AstrolabeError::OutOfRange);
        }
        Ok(Time(seconds as u64 * NANOS_PER_SEC))
    }

    /// Creates a new [`Time`] instance from seconds.
    ///
    /// Returns an [`OutOfRange`](AstrolabeError::OutOfRange) error if the provided nano seconds are invalid (over `86_399_999_999_999`)
    ///
    /// # Example
    /// ```rust
    /// use astrolabe::{Time, TimeUnit};
    ///
    /// let time = Time::from_nanoseconds(1_234).unwrap();
    /// assert_eq!(1_234, time.as_nanoseconds());
    /// ```
    pub fn from_nanoseconds(nanoseconds: u64) -> Result<Self, AstrolabeError> {
        if nanoseconds > SECS_PER_DAY_U64 * NANOS_PER_SEC - 1 {
            return Err(AstrolabeError::OutOfRange);
        }
        Ok(Time(nanoseconds))
    }

    /// Returns the number of nano seconds between two [`Time`] instances.
    ///
    /// # Example
    /// ```rust
    /// use astrolabe::Time;
    ///
    /// let time = Time::from_hms(12, 0, 0).unwrap();
    /// let time_2 = Time::from_hms(12, 0, 1).unwrap();
    /// assert_eq!(1_000_000_000, time.between(&time_2));
    /// assert_eq!(1_000_000_000, time_2.between(&time));
    /// ```
    pub fn between(&self, compare: &Time) -> u64 {
        (self.0 as i64 - compare.0 as i64).unsigned_abs()
    }

    /// Returns the time as seconds
    ///
    /// # Example
    /// ```rust
    /// use astrolabe::{Time, TimeUnit};
    ///
    /// let time = Time::from_hms(12, 12, 12).unwrap();
    /// assert_eq!(43932, time.as_seconds());
    /// ```
    pub fn as_seconds(&self) -> u64 {
        self.0 / NANOS_PER_SEC
    }

    /// Returns the time as nano seconds
    ///
    /// # Example
    /// ```rust
    /// use astrolabe::{Time, TimeUnit};
    ///
    /// let time = Time::from_hms(12, 12, 12).unwrap();
    /// assert_eq!(43_932_000_000_000, time.as_nanoseconds());
    /// ```
    pub fn as_nanoseconds(&self) -> u64 {
        self.0
    }

    /// Returns the time as the specified [`TimeUnit`]
    ///
    /// # Example
    /// ```rust
    /// use astrolabe::{Time, TimeUnit};
    ///
    /// let time = Time::from_hms(12, 12, 12).unwrap();
    /// assert_eq!(732, time.as_unit(TimeUnit::Min));
    /// ```
    pub fn as_unit(&self, unit: TimeUnit) -> u64 {
        nanos_as_unit(self.0, unit)
    }

    /// Get a specific [`TimeUnit`].
    ///
    /// # Example
    /// ```rust
    /// use astrolabe::{Time, TimeUnit};
    ///
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
        nanos_to_unit(self.0, unit)
    }

    /// Creates a new [`Time`] instance with a specified amount of time applied (added or subtracted).
    ///
    /// Returns an [`OutOfRange`](AstrolabeError::OutOfRange) error if the provided value would result in an out of range time.
    ///
    /// # Example
    /// ```rust
    /// use astrolabe::{Time, TimeUnit};
    ///
    /// let time = Time::from_hms(12, 32, 15).unwrap();
    /// let applied = time.apply(1, TimeUnit::Hour).unwrap();
    /// assert_eq!("12:32:15", time.format("HH:mm:ss").unwrap());
    /// assert_eq!("13:32:15", applied.format("HH:mm:ss").unwrap());
    /// let applied_2 = applied.apply(-1, TimeUnit::Hour).unwrap();
    /// assert_eq!("12:32:15", applied_2.format("HH:mm:ss").unwrap());
    /// ```
    pub fn apply(&self, amount: i64, unit: TimeUnit) -> Result<Time, AstrolabeError> {
        Time::from_nanoseconds(apply_time_unit(self.0 as i128, amount, unit)? as u64)
    }

    /// Creates a new [`Time`] instance with a specific [`TimeUnit`] set to the provided value.
    ///
    /// Returns an [`OutOfRange`](AstrolabeError::OutOfRange) error if the provided value is invalid or out of range.
    ///
    /// # Example
    /// ```rust
    /// use astrolabe::{Time, TimeUnit};
    ///
    /// let time = Time::from_hms(12, 32, 15).unwrap();
    /// assert_eq!(15, time.set(15, TimeUnit::Hour).unwrap().get(TimeUnit::Hour));
    /// assert_eq!(10, time.set(10, TimeUnit::Min).unwrap().get(TimeUnit::Min));
    /// ```
    pub fn set(&self, value: u32, unit: TimeUnit) -> Result<Time, AstrolabeError> {
        Ok(Time(set_time_unit(self.0, value, unit)?))
    }

    /// Formatting with format strings based on [Unicode Date Field Symbols](https://www.unicode.org/reports/tr35/tr35-dates.html#Date_Field_Symbol_Table).
    ///
    /// Returns an [`InvalidFormat`](AstrolabeError::InvalidFormat`) error if the provided format string can't be parsed.
    ///
    /// # Available Symbols:
    ///
    /// | Field Type                 | Pattern  | Examples                       | Hint                                     |
    /// | -------------------------- | -------- | ------------------------------ | ---------------------------------------- |
    /// | AM, PM                     | a..aa    | AM, PM                         |                                          |
    /// |                            | aaa      | am, pm                         | *                                        |
    /// |                            | aaaa     | a.m., p.m.                     |                                          |
    /// |                            | aaaaa    | a, p                           |                                          |
    /// | AM, PM,<br/>noon, midnight | b..bb    | AM, PM,<br/>noon, midnight     |                                          |
    /// |                            | bbb      | am, pm,<br/>noon, midnight     | *                                        |
    /// |                            | bbbb     | a.m., p.m.,<br/>noon, midnight |                                          |
    /// |                            | bbbbb    | a, p, n, mi                    |                                          |
    /// | hour                       | h        | 1, 12                          | [1-12]                                   |
    /// |                            | hh       | 01, 12                         | *                                        |
    /// |                            | H        | 0, 23                          | [0-23]                                   |
    /// |                            | HH       | 00, 23                         | *                                        |
    /// |                            | K        | 0, 11                          | [0-11]                                   |
    /// |                            | KK       | 00, 11                         | *                                        |
    /// |                            | k        | 1, 24                          | [1-24]                                   |
    /// |                            | kk       | 01, 24                         | *                                        |
    /// | minute                     | m        | 0, 59                          |                                          |
    /// |                            | mm       | 00, 59                         | *                                        |
    /// | second                     | s        | 0, 59                          |                                          |
    /// |                            | ss       | 00, 59                         | *                                        |
    ///
    /// `*` = Default
    ///
    /// If the sequence is longer than listed in the table, the output will be the same as the default pattern for this unit (marked with `*`).
    ///
    /// Surround any character with apostrophes (`'`) to escape them.
    /// If you want escape `'`, write `''`.
    ///
    /// # Example
    /// ```rust
    /// use astrolabe::Time;
    ///
    /// let time = Time::from_hms(12, 32, 1).unwrap();
    /// assert_eq!("12:32:01", time.format("HH:mm:ss").unwrap());
    /// // Escape characters
    /// assert_eq!("12:mm:ss", time.format("HH:'mm:ss'").unwrap());
    /// assert_eq!("12:'32:01'", time.format("HH:''mm:ss''").unwrap());
    /// ```
    ///
    pub fn format(&self, format: &str) -> Result<String, AstrolabeError> {
        let parts = parse_format_string(format)?;
        let nanoseconds = self.as_nanoseconds();
        parts
            .iter()
            .map(|part| -> Result<Vec<char>, AstrolabeError> {
                // Escaped apostrophes
                if part.starts_with('\u{0000}') {
                    return Ok(part.replace('\u{0000}', "'").chars().collect::<Vec<char>>());
                }

                // Escape parts starting with apostrophe
                if part.starts_with('\'') {
                    let part = part.replace('\u{0000}', "'");
                    return Ok(
                        part[1..part.len() - if part.ends_with('\'') { 1 } else { 0 }]
                            .chars()
                            .collect::<Vec<char>>(),
                    );
                }

                Ok(format_time_part(part, nanoseconds)?
                    .chars()
                    .collect::<Vec<char>>())
            })
            .flat_map(|result| match result {
                Ok(vec) => vec.into_iter().map(Ok).collect(),
                Err(er) => vec![Err(er)],
            })
            .collect::<Result<String, AstrolabeError>>()
    }
}

impl From<&Time> for Time {
    fn from(time: &Time) -> Self {
        Self(time.0)
    }
}
