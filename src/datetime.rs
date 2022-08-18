use crate::{
    shared::{DAYS_TO_1970, DAYS_TO_1970_I64, NANOS_PER_DAY, NANOS_PER_SEC, SECS_PER_DAY_U64},
    util::{
        convert::{
            date_to_days, days_to_date, dtu_to_du, dtu_to_tu, nanos_to_unit, time_to_day_seconds,
        },
        format::{format_part, parse_format_string},
        manipulation::{apply_date_unit, apply_time_unit, set_date_unit, set_time_unit},
    },
    AstrolabeError,
};
use std::{
    fmt::Display,
    time::{SystemTime, UNIX_EPOCH},
};

/// Date and time units for functions like [`DateTime::get`] or [`DateTime::apply`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DateTimeUnit {
    #[allow(missing_docs)]
    Year,
    /// **Note**: When used in the [`DateTime::apply`] function, this unit adds or removes calendar months, not 30 days.
    ///
    /// # Example
    /// ```rust
    /// use astrolabe::{DateTime, DateTimeUnit};
    ///
    /// let date_time = DateTime::from_ymd(1970, 1, 31).unwrap();
    /// assert_eq!("1970-02-28", date_time.apply(1, DateTimeUnit::Month).unwrap().format("yyyy-MM-dd"));
    /// assert_eq!("1970-03-31", date_time.apply(2, DateTimeUnit::Month).unwrap().format("yyyy-MM-dd"));
    /// assert_eq!("1970-04-30", date_time.apply(3, DateTimeUnit::Month).unwrap().format("yyyy-MM-dd"));
    /// ```
    Month,
    #[allow(missing_docs)]
    Day,
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

/// Combined date and time.
/// Date is in the proleptic Gregorian calendar and clock time is with nanosecond precision.
///
/// Date ranges from `30. June -5879611` to `12. July 5879611`. Please note that year 0 does not exist. After year -1 follows year 1.
#[derive(Debug, Default)]
pub struct DateTime(i32, u64);

impl DateTime {
    /// Creates a new [`DateTime`] instance with [`SystemTime::now()`].
    ///
    /// # Example
    /// ```rust
    /// use astrolabe::{DateTime, DateTimeUnit};
    ///
    /// let date_time = DateTime::now();
    /// assert!(2021 < date_time.get(DateTimeUnit::Year));
    /// ```
    pub fn now() -> Self {
        let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

        let days = duration.as_secs() / SECS_PER_DAY_U64 + DAYS_TO_1970;
        let nanos =
            duration.as_secs() % SECS_PER_DAY_U64 * NANOS_PER_SEC + duration.subsec_nanos() as u64;

        DateTime(days as i32, nanos)
    }

    /// Creates a new [`DateTime`] instance from days.
    ///
    /// # Example
    /// ```rust
    /// use astrolabe::DateTime;
    ///
    /// let date_time = DateTime::from_days(738276);
    /// assert_eq!("2022/05/02", date_time.format("yyyy/MM/dd"));;
    /// ```
    pub fn from_days(days: i32) -> Self {
        DateTime(days, 0)
    }

    /// Creates a new [`DateTime`] instance from seconds.
    ///
    /// Returns an [`OutOfRange`](AstrolabeError::OutOfRange) error if the provided seconds would result in an out of range datetime.
    ///
    /// # Example
    /// ```rust
    /// use astrolabe::DateTime;
    ///
    /// let date_time = DateTime::from_seconds(86400).unwrap();
    /// assert_eq!("0001/01/02", date_time.format("yyyy/MM/dd"));
    /// ```
    pub fn from_seconds(seconds: i64) -> Result<Self, AstrolabeError> {
        let days = (seconds / SECS_PER_DAY_U64 as i64)
            .try_into()
            .map_err(|_| AstrolabeError::OutOfRange)?;
        let day_seconds = if seconds.is_negative() {
            SECS_PER_DAY_U64 - seconds.unsigned_abs() % SECS_PER_DAY_U64
        } else {
            seconds.unsigned_abs() % SECS_PER_DAY_U64
        };

        Ok(DateTime(days, day_seconds * NANOS_PER_SEC))
    }

    /// Creates a new [`DateTime`] instance from nanoseconds.
    ///
    /// Returns an [`OutOfRange`](AstrolabeError::OutOfRange) error if the provided nanoseconds would result in an out of range datetime.
    ///
    /// # Example
    /// ```rust
    /// use astrolabe::DateTime;
    ///
    /// let date_time = DateTime::from_nanoseconds(86_400_000_000_000).unwrap();
    /// assert_eq!("0001/01/02", date_time.format("yyyy/MM/dd"));
    /// ```
    pub fn from_nanoseconds(nanoseconds: i128) -> Result<Self, AstrolabeError> {
        let days = (nanoseconds / NANOS_PER_DAY as i128)
            .try_into()
            .map_err(|_| AstrolabeError::OutOfRange)?;
        let day_nanos = if nanoseconds.is_negative() {
            NANOS_PER_DAY - (nanoseconds.unsigned_abs() % NANOS_PER_DAY as u128) as u64
        } else {
            (nanoseconds.unsigned_abs() % NANOS_PER_DAY as u128) as u64
        };

        Ok(DateTime(days, day_nanos))
    }

    /// Creates a new [`DateTime`] instance from year, month and day (day of month).
    ///
    /// Returns an [`OutOfRange`](AstrolabeError::OutOfRange) error if the provided values are invalid.
    ///
    /// # Example
    /// ```rust
    /// use astrolabe::DateTime;
    ///
    /// let date_time = DateTime::from_ymd(2022, 05, 02).unwrap();
    /// assert_eq!("2022/05/02", date_time.format("yyyy/MM/dd"));
    /// ```
    pub fn from_ymd(year: i32, month: u32, day: u32) -> Result<Self, AstrolabeError> {
        let days = date_to_days(year, month, day)?;

        Ok(DateTime(days, 0))
    }

    /// Creates a new [`DateTime`] instance from hour, minute and seconds.
    ///
    /// Returns an [`OutOfRange`](AstrolabeError::OutOfRange) error if the provided values are invalid.
    ///
    /// # Example
    /// ```rust
    /// use astrolabe::DateTime;
    ///
    /// let date_time = DateTime::from_hms(12, 32, 12).unwrap();
    /// assert_eq!("0001/01/01 12:32:12", date_time.format("yyyy/MM/dd HH:mm:ss"));
    /// ```
    pub fn from_hms(hour: u32, minute: u32, second: u32) -> Result<Self, AstrolabeError> {
        let seconds = time_to_day_seconds(hour, minute, second)? as u64;

        Ok(DateTime(0, seconds * NANOS_PER_SEC))
    }

    /// Creates a new [`DateTime`] instance from year, month, day (day of month), hour, minute and seconds.
    ///
    /// Returns an [`OutOfRange`](AstrolabeError::OutOfRange) error if the provided values are invalid.
    ///
    /// # Example
    /// ```rust
    /// use astrolabe::{DateTime, DateTimeUnit};
    ///
    /// let date_time = DateTime::from_ymdhms(2022, 05, 02, 12, 32, 1).unwrap();
    /// assert_eq!("2022/05/02 12:32:01", date_time.format("yyyy/MM/dd HH:mm:ss"));
    /// ```
    pub fn from_ymdhms(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        min: u32,
        sec: u32,
    ) -> Result<Self, AstrolabeError> {
        let days = date_to_days(year, month, day)?;
        let seconds = time_to_day_seconds(hour, min, sec)? as u64;

        Ok(DateTime(days, seconds * NANOS_PER_SEC))
    }

    /// Creates a new [`DateTime`] instance from a unix timestamp (non-leap seconds since January 1, 1970 00:00:00 UTC).
    ///
    /// Returns an [`OutOfRange`](AstrolabeError::OutOfRange) error if the provided timestamp would result in an out of range date.
    ///
    /// # Example
    /// ```rust
    /// use astrolabe::DateTime;
    ///
    /// let date_time = DateTime::from_timestamp(0).unwrap();
    /// assert_eq!("1970/01/01 00:00:00", date_time.format("yyyy/MM/dd HH:mm:ss"));
    /// ```
    pub fn from_timestamp(timestamp: i64) -> Result<Self, AstrolabeError> {
        DateTime::from_seconds(timestamp + DAYS_TO_1970_I64 * SECS_PER_DAY_U64 as i64)
    }

    /// Creates a new [`DateTime`] with the specified amount of nanoseconds set as clock time.
    ///
    /// Returns an [`OutOfRange`](AstrolabeError::OutOfRange) error if the provided nanoseconds are invalid (over `86_399_999_999_999`)
    ///
    /// # Example
    /// ```rust
    /// use astrolabe::{DateTime, DateTimeUnit};
    ///
    /// let date_time = DateTime::from_days(738276).set_time(3_600_000_000_000).unwrap();
    /// assert_eq!("2022/05/02 01:00:00", date_time.format("yyyy/MM/dd HH:mm:ss"));
    /// ```
    pub fn set_time(&self, nanoseconds: u64) -> Result<Self, AstrolabeError> {
        if nanoseconds > SECS_PER_DAY_U64 * NANOS_PER_SEC - 1 {
            return Err(AstrolabeError::OutOfRange);
        }
        Ok(DateTime(self.0, nanoseconds))
    }

    /// Returns the clock time in nanoseconds.
    ///
    /// # Example
    /// ```rust
    /// use astrolabe::{DateTime, DateTimeUnit};
    ///
    /// let date_time = DateTime::from_days(0).set_time(3_600_000_000_000).unwrap();
    /// assert_eq!(3_600_000_000_000, date_time.get_time());
    /// ```
    pub fn get_time(&self) -> u64 {
        self.1
    }

    /// Returns the number of days since January 1, 0001 00:00:00 UTC. (Negative if date is before)
    ///
    /// # Example
    /// ```rust
    /// use astrolabe::DateTime;
    ///
    /// let date_time = DateTime::from_ymd(1, 1, 2).unwrap();
    /// assert_eq!(1, date_time.as_days());
    /// ```
    pub fn as_days(&self) -> i32 {
        self.0
    }

    /// Returns the number of seconds since January 1, 0001 00:00:00 UTC. (Negative if date is before)
    ///
    /// # Example
    /// ```rust
    /// use astrolabe::DateTime;
    ///
    /// let date_time = DateTime::from_ymd(1, 1, 2).unwrap();
    /// assert_eq!(86400, date_time.as_seconds());
    /// ```
    pub fn as_seconds(&self) -> i64 {
        let day_seconds = if self.0.is_negative() {
            -(((NANOS_PER_DAY - self.1) / NANOS_PER_SEC) as i64)
        } else {
            (self.1 / NANOS_PER_SEC) as i64
        };
        self.0 as i64 * SECS_PER_DAY_U64 as i64 + day_seconds
    }

    /// Returns the number of nanoseconds since January 1, 0001 00:00:00 UTC. (Negative if date is before)
    ///
    /// # Example
    /// ```rust
    /// use astrolabe::DateTime;
    ///
    /// let date_time = DateTime::from_ymd(1, 1, 2).unwrap();
    /// assert_eq!(86_400_000_000_000, date_time.as_nanoseconds());
    /// ```
    pub fn as_nanoseconds(&self) -> i128 {
        let day_nanoseconds = if self.0.is_negative() {
            -(NANOS_PER_DAY as i128 - self.1 as i128)
        } else {
            self.1 as i128
        };
        self.0 as i128 * NANOS_PER_DAY as i128 + day_nanoseconds
    }

    /// Returns the number of non-leap seconds since January 1, 1970 00:00:00 UTC. (Negative if date is before)
    ///
    /// # Example
    /// ```rust
    /// use astrolabe::DateTime;
    ///
    /// let date_time = DateTime::from_ymd(2000, 1, 1).unwrap();
    /// assert_eq!(946_684_800, date_time.timestamp());
    /// ```
    pub fn timestamp(&self) -> i64 {
        self.as_seconds() - DAYS_TO_1970 as i64 * SECS_PER_DAY_U64 as i64
    }

    /// Returns the number of seconds between two [`DateTime`] instances.
    ///
    /// # Example
    /// ```rust
    /// use astrolabe::DateTime;
    ///
    /// let date_time1 = DateTime::from_ymd(1970, 1, 1).unwrap();
    /// let date_time2 = DateTime::from_ymd(1970, 1, 2).unwrap();
    /// assert_eq!(86400, date_time1.between(&date_time2));
    /// assert_eq!(86400, date_time2.between(&date_time1));
    /// ```
    pub fn between(&self, compare: &DateTime) -> u64 {
        (self.as_seconds() - compare.as_seconds()).unsigned_abs()
    }

    /// Get a specific [`DateTimeUnit`].
    ///
    /// # Example
    /// ```rust
    /// use astrolabe::{DateTime, DateTimeUnit};
    ///
    /// let date_time = DateTime::from_ymdhms(2022, 5, 2, 12, 32, 1).unwrap();
    /// assert_eq!(2022, date_time.get(DateTimeUnit::Year));
    /// assert_eq!(5, date_time.get(DateTimeUnit::Month));
    /// assert_eq!(32, date_time.get(DateTimeUnit::Min));
    /// ```
    pub fn get(&self, unit: DateTimeUnit) -> i64 {
        match unit {
            DateTimeUnit::Year => days_to_date(self.0).0 as i64,
            DateTimeUnit::Month => days_to_date(self.0).1 as i64,
            DateTimeUnit::Day => days_to_date(self.0).2 as i64,
            _ => nanos_to_unit(self.1, dtu_to_tu(unit)) as i64,
        }
    }

    /// Creates a new [`DateTime`] instance with a specified amount of time applied (added or subtracted).
    ///
    /// **Note**: When using [`DateTimeUnit::Month`], it adds calendar months and not 30 days. See it's [documentation](DateTimeUnit::Month) for examples.
    ///
    /// Returns an [`OutOfRange`](AstrolabeError::OutOfRange) error if the provided value would result in an out of range date.
    ///
    /// # Example
    /// ```rust
    /// use astrolabe::{DateTime, DateTimeUnit};
    ///
    /// let date_time = DateTime::from_ymdhms(1970, 1, 1, 12, 32, 1).unwrap();
    /// let applied = date_time.apply(1, DateTimeUnit::Day).unwrap();
    /// assert_eq!("1970-01-01 12:32:01", date_time.format("yyyy-MM-dd HH:mm:ss"));
    /// assert_eq!("1970-01-02 12:32:01", applied.format("yyyy-MM-dd HH:mm:ss"));
    /// let applied_2 = applied.apply(-1, DateTimeUnit::Hour).unwrap();
    /// assert_eq!("1970-01-02 11:32:01", applied_2.format("yyyy-MM-dd HH:mm:ss"));
    /// ```
    pub fn apply(&self, amount: i64, unit: DateTimeUnit) -> Result<DateTime, AstrolabeError> {
        Ok(match unit {
            DateTimeUnit::Year | DateTimeUnit::Month | DateTimeUnit::Day => {
                DateTime(apply_date_unit(self.0, amount, dtu_to_du(unit))?, self.1)
            }
            _ => DateTime::from_nanoseconds(apply_time_unit(
                self.as_nanoseconds(),
                amount,
                dtu_to_tu(unit),
            ))?,
        })
    }

    /// Creates a new [`DateTime`] instance with a specific [`DateTimeUnit`] set to the provided value.
    ///
    /// Returns an [`OutOfRange`](AstrolabeError::OutOfRange) error if the provided value is invalid or out of range.
    ///
    /// # Example
    /// ```rust
    /// use astrolabe::{DateTime, DateTimeUnit};
    ///
    /// let date_time = DateTime::from_ymdhms(2022, 5, 2, 12, 32, 1).unwrap();
    /// assert_eq!(2000, date_time.set(2000, DateTimeUnit::Year).unwrap().get(DateTimeUnit::Year));
    /// assert_eq!(10, date_time.set(10, DateTimeUnit::Min).unwrap().get(DateTimeUnit::Min));
    /// ```
    pub fn set(&self, value: i32, unit: DateTimeUnit) -> Result<DateTime, AstrolabeError> {
        Ok(match unit {
            DateTimeUnit::Year | DateTimeUnit::Month | DateTimeUnit::Day => {
                DateTime(set_date_unit(self.0, value, dtu_to_du(unit))?, self.1)
            }
            _ => {
                if value.is_negative() {
                    return Err(AstrolabeError::OutOfRange);
                }
                DateTime(
                    self.0,
                    set_time_unit(self.1, value.unsigned_abs(), dtu_to_tu(unit))?,
                )
            }
        })
    }

    /// Formatting with format strings based on [Unicode Date Field Symbols](https://www.unicode.org/reports/tr35/tr35-dates.html#Date_Field_Symbol_Table).
    ///
    /// # Available Symbols:
    ///
    /// | Field Type                 | Pattern  | Examples                       | Hint                                     |
    /// | -------------------------- | -------- | ------------------------------ | ---------------------------------------- |
    /// | era                        | G..GGG   | AD                             |                                          |
    /// |                            | GGGG     | Anno Domini                    | *                                        |
    /// |                            | GGGGG    | A                              |                                          |
    /// | year                       | y        | 2, 20, 201, 2017, 20173        |                                          |
    /// |                            | yy       | 02, 20, 01, 17, 73             |                                          |
    /// |                            | yyy      | 002, 020, 201, 2017, 20173     |                                          |
    /// |                            | yyyy     | 0002, 0020, 0201, 2017, 20173  |                                          |
    /// |                            | yyyyy+   | ...                            | Unlimited length,<br/>padded with zeros. |
    /// | quarter                    | q        | 2                              | *                                        |
    /// |                            | qq       | 02                             |                                          |
    /// |                            | qqq      | Q2                             |                                          |
    /// |                            | qqqq     | 2nd quarter                    |                                          |
    /// |                            | qqqqq    | 2                              |                                          |
    /// | month                      | M        | 9, 12                          |                                          |
    /// |                            | MM       | 09, 12                         |                                          |
    /// |                            | MMM      | Sep                            |                                          |
    /// |                            | MMMM     | September                      | *                                        |
    /// |                            | MMMMM    | S                              |                                          |
    /// | week                       | w        | 8, 27                          | Week of year                             |
    /// |                            | ww       | 08, 27                         | *                                        |
    /// | days                       | d        | 1                              | Day of month                             |
    /// |                            | dd       | 01                             | *                                        |
    /// |                            | D        | 1, 24 135                      | Day of year, *                           |
    /// |                            | DD       | 01, 24, 135                    |                                          |
    /// |                            | DDD      | 001, 024, 135                  |                                          |
    /// | week day                   | e        | 3                              | 1-7, 1 is Sunday, *                      |
    /// |                            | ee       | 03                             | 1-7, 1 is Sunday                         |
    /// |                            | eee      | Tue                            |                                          |
    /// |                            | eeee     | Tuesday                        |                                          |
    /// |                            | eeeee    | T                              |                                          |
    /// |                            | eeeeee   | Tu                             |                                          |
    /// |                            | eeeeeee  | 2                              | 1-7, 1 is Monday                         |
    /// |                            | eeeeeeee | 02                             | 1-7, 1 is Monday                         |
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
    /// use astrolabe::DateTime;
    ///
    /// let date_time = DateTime::from_ymdhms(2022, 5, 2, 12, 32, 1).unwrap();
    /// assert_eq!("2022/05/02 12:32:01", date_time.format("yyyy/MM/dd HH:mm:ss"));
    /// // Escape characters
    /// assert_eq!("2022/MM/dd 12:32:01", date_time.format("yyyy/'MM/dd' HH:mm:ss"));
    /// assert_eq!("2022/'05/02' 12:32:01", date_time.format("yyyy/''MM/dd'' HH:mm:ss"));
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
                    return part[1..part.len() - if part.ends_with('\'') { 1 } else { 0 }]
                        .chars()
                        .collect::<Vec<char>>();
                }

                format_part(part, self.0, self.1)
                    .chars()
                    .collect::<Vec<char>>()
            })
            .collect::<String>()
    }
}

impl From<&DateTime> for DateTime {
    fn from(date_time: &DateTime) -> Self {
        Self(date_time.0, date_time.1)
    }
}

impl Display for DateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.format("yyyy/MM/dd HH:mm:ss"))
    }
}
