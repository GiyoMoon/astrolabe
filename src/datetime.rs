use crate::util::convert::{date_to_days, ts_to_d_units, ts_to_t_units};
use crate::util::format::format_part;
use crate::util::format::zero_padded;
use crate::util::manipulation::{apply_unit, ApplyType};
use std::cmp::Ordering;
use std::ops::{Add, Sub};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Error parsing or formatting [`DateTime`] struct
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DateTimeError {
    /// Failed parsing the provided format string
    InvalidFormat,
    /// Numeric component is out of range
    OutOfRange,
}

/// Used for specifing the precision for RFC3339 timestamps
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Precision {
    /// Only seconds -> `2022-05-02T15:30:20Z`
    Seconds = 0,
    /// 2 decimal places -> `2022-05-02T15:30:20.00Z`
    Centi = 2,
    /// 3 decimal places -> `2022-05-02T15:30:20.000Z`
    Millis = 3,
    /// 6 decimal places -> `2022-05-02T15:30:20.000000Z`
    Micros = 6,
    /// 9 decimal places -> `2022-05-02T15:30:20.000000000Z`
    Nanos = 9,
}

/// Time units for functions like [`DateTime::add`]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Unit {
    #[allow(missing_docs)]
    Year,
    /// Note: Adds or removes calendar months, not 30 days.
    ///
    /// # Example
    ///
    /// ```rust
    /// use astrolabe::{DateTime, Unit};
    ///
    /// let date_time = DateTime::from_ymd(1970, 1, 31).unwrap();
    /// assert_eq!("1970-02-28", date_time.add(1, Unit::Month).format("yyyy-MM-dd").unwrap());
    /// assert_eq!("1970-03-31", date_time.add(2, Unit::Month).format("yyyy-MM-dd").unwrap());
    /// assert_eq!("1970-04-30", date_time.add(3, Unit::Month).format("yyyy-MM-dd").unwrap());
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
    Milli,
    #[allow(missing_docs)]
    Micro,
    #[allow(missing_docs)]
    Nano,
}

/// Wrapper around [`std::time::SystemTime`] which implements formatting and manipulation functions
#[derive(Debug)]
pub struct DateTime(SystemTime);

impl DateTime {
    /// Creates a new [`DateTime`] struct with [`SystemTime::now()`]
    pub fn now() -> Self {
        DateTime(SystemTime::now())
    }

    /// Creates a new [`DateTime`] struct from year, month and day (day of month)
    ///
    /// Throws an [`OutOfRange`](DateTimeError::OutOfRange) error if the provided date is invalid or before the year `1970`
    ///
    /// # Example
    ///
    /// ```rust
    /// use astrolabe::DateTime;
    ///
    /// let date_time = DateTime::from_ymd(1970, 1, 1).unwrap();
    /// assert_eq!(0, date_time.timestamp());
    /// ```
    pub fn from_ymd(year: u64, month: u64, day: u64) -> Result<Self, DateTimeError> {
        if year < 1970 {
            return Err(DateTimeError::OutOfRange);
        }

        let days = date_to_days(year, month, day)?;

        Ok(DateTime(UNIX_EPOCH + Duration::new(days * 86400, 0)))
    }

    /// Creates a new [`DateTime`] struct from year, month, day (day of month), hours, minutes and seconds
    ///
    /// Throws an [`OutOfRange`](DateTimeError::OutOfRange) error if the provided date is invalid or before the year `1970`
    ///
    /// # Example
    ///
    /// ```rust
    /// use astrolabe::DateTime;
    ///
    /// let date_time = DateTime::from_ymdhms(1970, 1, 1, 0, 0, 0).unwrap();
    /// assert_eq!(0, date_time.timestamp());
    /// ```
    pub fn from_ymdhms(
        year: u64,
        month: u64,
        day: u64,
        hour: u64,
        min: u64,
        sec: u64,
    ) -> Result<Self, DateTimeError> {
        if year < 1970 || hour > 23 || min > 59 || sec > 59 {
            return Err(DateTimeError::OutOfRange);
        }

        let days = date_to_days(year, month, day)?;
        let day_seconds = hour * 3600 + min * 60 + sec;

        Ok(DateTime(
            UNIX_EPOCH + Duration::new(days * 86400 + day_seconds, 0),
        ))
    }

    /// Returns the duration since January 1, 1970 00:00:00 UTC
    ///
    /// # Example
    ///
    /// ```rust
    /// use astrolabe::DateTime;
    ///
    /// let date_time = DateTime::from_ymd(1970, 1, 1).unwrap();
    /// assert_eq!(0, date_time.duration().as_secs());
    /// ```
    pub fn duration(&self) -> Duration {
        // Using unwrap because it's safe to assume that the duration is valid
        self.0.duration_since(UNIX_EPOCH).unwrap()
    }

    /// Returns the number of non-leap seconds since January 1, 1970 00:00:00 UTC
    ///
    /// # Example
    ///
    /// ```rust
    /// use astrolabe::DateTime;
    ///
    /// let date_time = DateTime::from_ymd(1970, 1, 1).unwrap();
    /// assert_eq!(0, date_time.timestamp());
    /// ```
    pub fn timestamp(&self) -> u64 {
        self.duration().as_secs()
    }

    /// Returns the duration between two [`DateTime`] structs
    ///
    /// # Example
    ///
    /// ```rust
    /// use astrolabe::DateTime;
    ///
    /// let date_time = DateTime::from_ymd(1970, 1, 1).unwrap();
    /// let date_time_2 = DateTime::from_ymd(1970, 1, 2).unwrap();
    /// assert_eq!(86400, date_time.between(&date_time_2).as_secs());
    /// assert_eq!(86400, date_time_2.between(&date_time).as_secs());
    /// ```
    pub fn between(&self, compare: &DateTime) -> Duration {
        // Using unwrap because it's safe to assume that the duration is valid
        if self.0.cmp(&compare.0) == Ordering::Greater {
            self.0.duration_since(compare.0).unwrap()
        } else {
            compare.0.duration_since(self.0).unwrap()
        }
    }

    /// Adds [`Duration`] to [`DateTime`]
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::time::Duration;
    /// use astrolabe::DateTime;
    ///
    /// let date_time = DateTime::from_ymd(1970, 1, 1).unwrap();
    /// let added = date_time.add_dur(Duration::new(86400, 0));
    /// assert_eq!(0, date_time.timestamp());
    /// assert_eq!(86400, added.timestamp());
    /// ```
    pub fn add_dur(&self, duration: Duration) -> Self {
        DateTime(self.0.add(duration))
    }

    /// Removes [`Duration`] from [`DateTime`]
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::time::Duration;
    /// use astrolabe::DateTime;
    ///
    /// let date_time = DateTime::from_ymd(1970, 1, 2).unwrap();
    /// let removed = date_time.remove_dur(Duration::new(86400, 0));
    /// assert_eq!(86400, date_time.timestamp());
    /// assert_eq!(0, removed.timestamp());
    /// ```
    pub fn remove_dur(&self, duration: Duration) -> Self {
        DateTime(self.0.sub(duration))
    }

    /// Adds a specified amount of time to [`DateTime`]
    ///
    /// **Note**: When using [`Unit::Month`], it adds calendar months and not 30 days. See it's [documentation](Unit::Month) for examples.
    ///
    ///
    /// # Example
    ///
    /// ```rust
    /// use astrolabe::{DateTime, Unit};
    ///
    /// let date_time = DateTime::from_ymd(1970, 1, 1).unwrap();
    /// let added = date_time.add(1, Unit::Day);
    /// assert_eq!("1970-01-01", date_time.format("yyyy-MM-dd").unwrap());
    /// assert_eq!("1970-01-02", added.format("yyyy-MM-dd").unwrap());
    /// ```
    pub fn add(&self, amount: u64, unit: Unit) -> Self {
        apply_unit(self, amount, unit, ApplyType::Add)
    }

    /// Removes a specified amount of time to [`DateTime`]
    ///
    /// **Note**: When using [`Unit::Month`], it removes calendar months and not 30 days. See it's [documentation](Unit::Month) for examples.
    ///
    /// # Example
    ///
    /// ```rust
    /// use astrolabe::{DateTime, Unit};
    ///
    /// let date_time = DateTime::from_ymd(1970, 1, 2).unwrap();
    /// let removed = date_time.sub(1, Unit::Day);
    /// assert_eq!("1970-01-02", date_time.format("yyyy-MM-dd").unwrap());
    /// assert_eq!("1970-01-01", removed.format("yyyy-MM-dd").unwrap());
    /// ```
    pub fn sub(&self, amount: u64, unit: Unit) -> Self {
        apply_unit(self, amount, unit, ApplyType::Sub)
    }

    /// Format as an RFC3339 timestamp
    /// (`2022-05-02T15:30:20Z`)
    ///
    /// Use the [`Precision`] enum to specify decimal places after seconds:
    /// * [`Precision::Seconds`] -> `2022-05-02T15:30:20Z`
    /// * [`Precision::Centi`] -> `2022-05-02T15:30:20.00Z`
    /// * [`Precision::Millis`] -> `2022-05-02T15:30:20.000Z`
    /// * [`Precision::Micros`] -> `2022-05-02T15:30:20.000000Z`
    /// * [`Precision::Nanos`] -> `2022-05-02T15:30:20.000000000Z`
    ///
    /// # Example
    ///
    /// ```rust
    /// use astrolabe::{DateTime, Precision};
    ///
    /// let date_time = DateTime::from_ymdhms(2022, 5, 2, 15, 30, 20).unwrap();
    /// assert_eq!("2022-05-02T15:30:20Z", date_time.format_rfc3339(Precision::Seconds));
    /// ```
    pub fn format_rfc3339(&self, precision: Precision) -> String {
        let timestamp = self.timestamp();
        let (year, month, day) = ts_to_d_units(timestamp);
        let (hour, min, sec) = ts_to_t_units(timestamp);
        let nanos = self.duration().subsec_nanos() as u64;

        format!(
            "{}-{}-{}T{}:{}:{}{}Z",
            zero_padded(year, 4),
            zero_padded(month, 2),
            zero_padded(day, 2),
            zero_padded(hour, 2),
            zero_padded(min, 2),
            zero_padded(sec, 2),
            match precision {
                Precision::Seconds => "".to_string(),
                _ => {
                    let length = precision as usize;
                    format!(".{}", &zero_padded(nanos, length)[..length])
                }
            }
        )
    }

    /// Formatting with format strings based on [Unicode Date Field Symbols](https://www.unicode.org/reports/tr35/tr35-dates.html#Date_Field_Symbol_Table)
    ///
    /// Throws an [`InvalidFormat`](DateTimeError::InvalidFormat`) error if the provided format string can't be parsed
    ///
    /// # Available Symbols:
    ///
    /// | Field Type | Pattern  | Examples                      | Hint                                     |
    /// | ---------- | -------- | ----------------------------- | ---------------------------------------- |
    /// | era        | G..GGG   | AD                            |                                          |
    /// |            | GGGG     | Anno Domini                   | *                                        |
    /// |            | GGGGG    | A                             |                                          |
    /// | year       | y        | 2, 20, 201, 2017, 20173       |                                          |
    /// |            | yy       | 02, 20, 01, 17, 73            |                                          |
    /// |            | yyy      | 002, 020, 201, 2017, 20173    |                                          |
    /// |            | yyyy     | 0002, 0020, 0201, 2017, 20173 |                                          |
    /// |            | yyyyy+   | ...                           | Unlimited length,<br/>padded with zeros. |
    /// | quarter    | q        | 2                             | *                                        |
    /// |            | qq       | 02                            |                                          |
    /// |            | qqq      | Q2                            |                                          |
    /// |            | qqqq     | 2nd quarter                   |                                          |
    /// |            | qqqqq    | 2                             |                                          |
    /// | month      | M        | 9, 12                         |                                          |
    /// |            | MM       | 09, 12                        |                                          |
    /// |            | MMM      | Sep                           |                                          |
    /// |            | MMMM     | September                     | *                                        |
    /// |            | MMMMM    | S                             |                                          |
    /// | days       | d        | 1                             | Day of month                             |
    /// |            | dd       | 01                            | *                                        |
    /// |            | D        | 1, 24 135                     | Day of year, *                           |
    /// |            | DD       | 01, 24, 135                   |                                          |
    /// |            | DDD      | 001, 024, 135                 |                                          |
    /// | week day   | e        | 3                             | 1-7, 1 is Sunday, *                      |
    /// |            | ee       | 03                            | 1-7, 1 is Sunday                         |
    /// |            | eee      | Tue                           |                                          |
    /// |            | eeee     | Tuesday                       |                                          |
    /// |            | eeeee    | T                             |                                          |
    /// |            | eeeeee   | Tu                            |                                          |
    /// |            | eeeeeee  | 2                             | 1-7, 1 is Monday                         |
    /// |            | eeeeeeee | 02                            | 1-7, 1 is Monday                         |
    /// | hour       | h        | 1, 12                         | [1-12]                                   |
    /// |            | hh       | 01, 12                        | *                                        |
    /// |            | H        | 0, 23                         | [0-23]                                   |
    /// |            | HH       | 00, 23                        | *                                        |
    /// |            | K        | 0, 11                         | [0-11]                                   |
    /// |            | KK       | 00, 11                        | *                                        |
    /// |            | h        | 1, 24                         | [1-24]                                   |
    /// |            | hh       | 01, 24                        | *                                        |
    /// | minute     | m        | 0, 59                         |                                          |
    /// |            | mm       | 00, 59                        | *                                        |
    /// | second     | s        | 0, 59                         |                                          |
    /// |            | ss       | 00, 59                        | *                                        |
    ///
    /// `*` = Default
    ///
    /// If the sequence is longer than listed in the table, the output will be the same as the default pattern for this unit (marked with `*`)
    ///
    /// Surround any character with apostrophes (`'`) to escape them.
    /// If you want escape `'`, write `''`
    ///
    /// # Example
    ///
    /// ```rust
    /// use astrolabe::DateTime;
    ///
    /// let date_time = DateTime::from_ymdhms(1970, 1, 1, 0, 0, 0).unwrap();
    /// assert_eq!("01/01/1970 00:00:00", date_time.format("MM/dd/yyyy HH:mm:ss").unwrap());
    /// // Escape characters
    /// assert_eq!("MM/dd/1970 00:00:00", date_time.format("'MM/dd'/yyyy HH:mm:ss").unwrap());
    /// assert_eq!("'01/01/1970' 00:00:00", date_time.format("''MM/dd/yyyy'' HH:mm:ss").unwrap());
    /// ```
    ///
    pub fn format(&self, format: &str) -> Result<String, DateTimeError> {
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
                            .ok_or(DateTimeError::InvalidFormat)?
                            .push(char);
                    }
                    currently_escaped = !currently_escaped;
                }
                _ => {
                    if (currently_escaped
                        || parts.last().unwrap_or(&"".to_string()).starts_with(char))
                        && parts.last().is_some()
                    {
                        parts
                            .last_mut()
                            .ok_or(DateTimeError::InvalidFormat)?
                            .push(char);
                    } else {
                        parts.push(char.to_string());
                    }
                }
            };
        }

        let timestamp = self.timestamp();
        parts
            .iter()
            .map(|part| -> Result<Vec<char>, DateTimeError> {
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

                Ok(format_part(part, timestamp)?.chars().collect::<Vec<char>>())
            })
            .flat_map(|result| match result {
                Ok(vec) => vec.into_iter().map(Ok).collect(),
                Err(er) => vec![Err(er)],
            })
            .collect::<Result<String, DateTimeError>>()
    }
}

impl From<SystemTime> for DateTime {
    fn from(time: SystemTime) -> DateTime {
        DateTime(time)
    }
}

impl From<DateTime> for SystemTime {
    fn from(date_time: DateTime) -> SystemTime {
        date_time.0
    }
}

impl From<&DateTime> for SystemTime {
    fn from(date_time: &DateTime) -> SystemTime {
        date_time.0
    }
}
