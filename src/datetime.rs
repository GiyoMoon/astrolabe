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
    Centis = 2,
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
    /// **Note**: When used in the [`DateTime::add`] or [`DateTime::sub`] functions, this unit adds or removes calendar months, not 30 days.
    ///
    /// # Example
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
    Centis,
    #[allow(missing_docs)]
    Millis,
    #[allow(missing_docs)]
    Micros,
    #[allow(missing_docs)]
    Nanos,
}

/// Used to define if an offset is `UTC+` or `UTC-` (eastern or western hemisphere)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Offset {
    /// Offset in the eastern hemisphere (`UTC±00:00 - UTC+23:59:59`)
    East,
    /// Offset in the western hemisphere (`UTC±00:00 - UTC-23:59:59`)
    West,
}

/// Wrapper around [`std::time::SystemTime`] which implements formatting and manipulation functions
#[derive(Debug)]
pub struct DateTime(SystemTime, i64);

impl DateTime {
    /// Creates a new [`DateTime`] instance with [`SystemTime::now()`]
    pub fn now() -> Self {
        DateTime(SystemTime::now(), 0)
    }

    /// Creates a new [`DateTime`] instance from year, month and day (day of month)
    ///
    /// Returns an [`OutOfRange`](DateTimeError::OutOfRange) error if the provided date is invalid or before the year `1970`
    ///
    /// # Example
    /// ```rust
    /// use astrolabe::DateTime;
    ///
    /// let date_time = DateTime::from_ymd(1970, 1, 1).unwrap();
    /// assert_eq!(0, date_time.timestamp());
    /// ```
    pub fn from_ymd(year: u64, month: u64, day: u64) -> Result<Self, DateTimeError> {
        let days = date_to_days(year, month, day)?;

        Ok(DateTime(UNIX_EPOCH + Duration::new(days * 86400, 0), 0))
    }

    /// Creates a new [`DateTime`] instance from year, month, day (day of month), hours, minutes and seconds
    ///
    /// Returns an [`OutOfRange`](DateTimeError::OutOfRange) error if the provided date is invalid or before the year `1970`
    ///
    /// # Example
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
        if hour > 23 || min > 59 || sec > 59 {
            return Err(DateTimeError::OutOfRange);
        }

        let days = date_to_days(year, month, day)?;
        let day_seconds = hour * 3600 + min * 60 + sec;

        Ok(DateTime(
            UNIX_EPOCH + Duration::new(days * 86400 + day_seconds, 0),
            0,
        ))
    }

    /// Creates a new [`DateTime`] instance from a unix timestamp (non-leap seconds since January 1, 1970 00:00:00 UTC)
    ///
    /// # Example
    /// ```rust
    /// use astrolabe::DateTime;
    ///
    /// let date_time = DateTime::from_timestamp(0);
    /// assert_eq!(0, date_time.timestamp());
    /// assert_eq!("1970-01-01", date_time.format("yyyy-MM-dd").unwrap());
    /// ```
    pub fn from_timestamp(timestamp: u64) -> Self {
        DateTime(UNIX_EPOCH + Duration::new(timestamp, 0), 0)
    }

    /// Returns the duration since January 1, 1970 00:00:00 UTC
    ///
    /// # Example
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
    /// ```rust
    /// use astrolabe::DateTime;
    ///
    /// let date_time = DateTime::from_ymd(1970, 1, 1).unwrap();
    /// assert_eq!(0, date_time.timestamp());
    /// ```
    pub fn timestamp(&self) -> u64 {
        self.duration().as_secs()
    }

    /// Returns the duration between two [`DateTime`] instances
    ///
    /// # Example
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

    /// Create a new [`DateTime`] instance with the [`Duration`] added.
    ///
    /// # Example
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
        DateTime(self.0.add(duration), 0)
    }

    /// Create a new [`DateTime`] instance with the [`Duration`] subtracted.
    ///
    /// # Example
    /// ```rust
    /// use std::time::Duration;
    /// use astrolabe::DateTime;
    ///
    /// let date_time = DateTime::from_ymd(1970, 1, 2).unwrap();
    /// let removed = date_time.sub_dur(Duration::new(86400, 0));
    /// assert_eq!(86400, date_time.timestamp());
    /// assert_eq!(0, removed.timestamp());
    /// ```
    pub fn sub_dur(&self, duration: Duration) -> Self {
        DateTime(self.0.sub(duration), 0)
    }

    /// Creates a new [`DateTime`] instance with a specified amount of time added.
    ///
    /// **Note**: When using [`Unit::Month`], it adds calendar months and not 30 days. See it's [documentation](Unit::Month) for examples.
    ///
    ///
    /// # Example
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

    /// Creates a new [`DateTime`] instance with a specified amount of time subtracted.
    ///
    /// **Note**: When using [`Unit::Month`], it removes calendar months and not 30 days. See it's [documentation](Unit::Month) for examples.
    ///
    /// # Example
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

    /// Get a specific [`Unit`].
    ///
    /// # Example
    /// ```rust
    /// use astrolabe::{DateTime, Unit};
    ///
    /// let date_time = DateTime::from_ymdhms(2022, 5, 2, 15, 30, 20).unwrap();
    /// assert_eq!(2022, date_time.get(Unit::Year));
    /// assert_eq!(2, date_time.get(Unit::Day));
    /// assert_eq!(20, date_time.get(Unit::Sec));
    /// ```
    pub fn get(&self, unit: Unit) -> u64 {
        match unit {
            Unit::Year => ts_to_d_units(self.timestamp()).0,
            Unit::Month => ts_to_d_units(self.timestamp()).1,
            Unit::Day => ts_to_d_units(self.timestamp()).2,
            Unit::Hour => ts_to_t_units(self.timestamp()).0,
            Unit::Min => ts_to_t_units(self.timestamp()).1,
            Unit::Sec => ts_to_t_units(self.timestamp()).2,
            Unit::Centis => (self.duration().subsec_millis() / 10) as u64,
            Unit::Millis => self.duration().subsec_millis() as u64,
            Unit::Micros => self.duration().subsec_micros() as u64,
            Unit::Nanos => self.duration().subsec_nanos() as u64,
        }
    }

    /// Creates a new [`DateTime`] instance with a specific [`Unit`] set to the provided value.
    ///
    /// Returns an [`OutOfRange`](DateTimeError::OutOfRange) error if the provided value is invalid or the year is before `1970`
    ///
    /// # Example
    /// ```rust
    /// use astrolabe::{DateTime, Unit};
    ///
    /// let date_time = DateTime::from_ymdhms(2022, 5, 2, 15, 30, 20).unwrap();
    /// assert_eq!(2000, date_time.set(2000, Unit::Year).unwrap().get(Unit::Year));
    /// assert_eq!(10, date_time.set(10, Unit::Hour).unwrap().get(Unit::Hour));
    /// ```
    pub fn set(&self, value: u64, unit: Unit) -> Result<DateTime, DateTimeError> {
        Ok(match unit {
            Unit::Year => {
                let timestamp = self.timestamp();
                let (_, month, day) = ts_to_d_units(timestamp);
                let days = date_to_days(value, month, day)?;
                let day_seconds = timestamp % 86400;
                DateTime(
                    UNIX_EPOCH
                        + Duration::new(days * 86400 + day_seconds, self.duration().subsec_nanos()),
                    0,
                )
            }
            Unit::Month => {
                let timestamp = self.timestamp();
                let (year, _, day) = ts_to_d_units(timestamp);
                let days = date_to_days(year, value, day)?;
                let day_seconds = timestamp % 86400;
                DateTime(
                    UNIX_EPOCH
                        + Duration::new(days * 86400 + day_seconds, self.duration().subsec_nanos()),
                    0,
                )
            }
            Unit::Day => {
                let timestamp = self.timestamp();
                let (year, month, _) = ts_to_d_units(timestamp);
                let days = date_to_days(year, month, value)?;
                let day_seconds = timestamp % 86400;
                DateTime(
                    UNIX_EPOCH
                        + Duration::new(days * 86400 + day_seconds, self.duration().subsec_nanos()),
                    0,
                )
            }
            Unit::Hour => {
                if value > 23 {
                    return Err(DateTimeError::OutOfRange);
                }
                let timestamp = self.timestamp();
                let (_, min, sec) = ts_to_t_units(timestamp);
                let days = timestamp / 86400;
                let day_seconds = value * 3600 + min * 60 + sec;
                DateTime(
                    UNIX_EPOCH
                        + Duration::new(days * 86400 + day_seconds, self.duration().subsec_nanos()),
                    0,
                )
            }
            Unit::Min => {
                if value > 59 {
                    return Err(DateTimeError::OutOfRange);
                }
                let timestamp = self.timestamp();
                let (hour, _, sec) = ts_to_t_units(timestamp);
                let days = timestamp / 86400;
                let day_seconds = hour * 3600 + value * 60 + sec;
                DateTime(
                    UNIX_EPOCH
                        + Duration::new(days * 86400 + day_seconds, self.duration().subsec_nanos()),
                    0,
                )
            }
            Unit::Sec => {
                if value > 59 {
                    return Err(DateTimeError::OutOfRange);
                }
                let timestamp = self.timestamp();
                let (hour, min, _) = ts_to_t_units(timestamp);
                let days = timestamp / 86400;
                let day_seconds = hour * 3600 + min * 60 + value;
                DateTime(
                    UNIX_EPOCH
                        + Duration::new(days * 86400 + day_seconds, self.duration().subsec_nanos()),
                    0,
                )
            }
            Unit::Centis => {
                if value > 99 {
                    return Err(DateTimeError::OutOfRange);
                }
                DateTime::from(Duration::new(
                    self.timestamp(),
                    value as u32 * 10000000 + self.duration().subsec_nanos() % 10000000,
                ))
            }
            Unit::Millis => {
                if value > 999 {
                    return Err(DateTimeError::OutOfRange);
                }
                DateTime::from(Duration::new(
                    self.timestamp(),
                    value as u32 * 1000000 + self.duration().subsec_nanos() % 1000000,
                ))
            }
            Unit::Micros => {
                if value > 999999 {
                    return Err(DateTimeError::OutOfRange);
                }
                DateTime::from(Duration::new(
                    self.timestamp(),
                    value as u32 * 1000 + self.duration().subsec_nanos() % 1000,
                ))
            }
            Unit::Nanos => {
                if value > 999999999 {
                    return Err(DateTimeError::OutOfRange);
                }
                DateTime::from(Duration::new(self.timestamp(), value as u32))
            }
        })
    }

    /// Creates a new [`DateTime`] instance with a given timezone offset defined with time units. Offset can range anywhere from `UTC-23:59:59` to `UTC+23:59:59`.
    ///
    /// The offset affects all format functions but does not change the timestamp which always represents `UTC`.
    ///
    /// Returns an [`OutOfRange`](DateTimeError::OutOfRange) error if the provided time units are invalid or if the offset would lead to an invalid date (any date before the year 1970).
    ///
    /// # Example
    /// ```rust
    /// use astrolabe::{DateTime, Offset};
    ///
    /// let date_time = DateTime::from_ymdhms(2022, 5, 2, 15, 30, 20).unwrap();
    /// // Set offset to UTC+2
    /// let with_offset = date_time.set_offset_time(2, 0, 0, Offset::East).unwrap();
    /// assert_eq!("17:30", with_offset.format("HH:mm").unwrap())
    /// ```
    pub fn set_offset_time(
        &self,
        hour: u64,
        min: u64,
        sec: u64,
        offset: Offset,
    ) -> Result<DateTime, DateTimeError> {
        if hour > 23 || min > 59 || sec > 59 {
            return Err(DateTimeError::OutOfRange);
        }

        let mut hour = hour as i64;
        let mut min = min as i64;
        let mut sec = sec as i64;
        if offset == Offset::West {
            hour *= -1;
            min *= -1;
            sec *= -1;
        }

        let offset = hour * 3600 + min * 60 + sec;
        self.set_offset(offset)
    }

    /// Creates a new [`DateTime`] instance with a given timezone offset defined as seconds.
    ///
    /// Returns an [`OutOfRange`](DateTimeError::OutOfRange) error if the provided offset is either not between `UTC-23:59:59` and `UTC-23:59:59` or if it would lead to an invalid date (any date before the year 1970).
    ///
    /// # Example
    /// ```rust
    /// use astrolabe::DateTime;
    ///
    /// let date_time = DateTime::from_ymdhms(2022, 5, 2, 15, 30, 20).unwrap();
    /// // Set offset to UTC+2
    /// let with_offset = date_time.set_offset(7200).unwrap();
    /// assert_eq!("17:30", with_offset.format("HH:mm").unwrap())
    /// ```
    pub fn set_offset(&self, secs: i64) -> Result<DateTime, DateTimeError> {
        if secs < -86399 || secs > 86399 || secs < 0 && self.timestamp() < -secs as u64 {
            return Err(DateTimeError::OutOfRange);
        }

        Ok(DateTime(self.0, secs))
    }

    /// Creates a new [`DateTime`] instance, assuming the current timestamp has the provided offset applied.
    /// The new instance will have the specified offset and the timestamp itself will be converted to `UTC`.
    ///
    /// Returns an [`OutOfRange`](DateTimeError::OutOfRange) error if the provided time units are invalid or if the offset would lead to an invalid date (any date before the year 1970).
    ///
    /// # Example
    /// ```rust
    /// use astrolabe::{DateTime, Offset};
    ///
    /// let date_time = DateTime::from_ymd(1970, 1, 2).unwrap();
    /// // Set offset to UTC+2
    /// let with_offset = date_time.as_offset_time(2, 0, 0, Offset::West).unwrap();
    /// assert_eq!(93600, with_offset.timestamp());
    /// assert_eq!("00:00", with_offset.format("HH:mm").unwrap())
    /// ```
    pub fn as_offset_time(
        &self,
        hour: u64,
        min: u64,
        sec: u64,
        offset: Offset,
    ) -> Result<DateTime, DateTimeError> {
        if hour > 23 || min > 59 || sec > 59 {
            return Err(DateTimeError::OutOfRange);
        }

        let mut hour = hour as i64;
        let mut min = min as i64;
        let mut sec = sec as i64;
        if offset == Offset::West {
            hour *= -1;
            min *= -1;
            sec *= -1;
        }

        let offset = hour * 3600 + min * 60 + sec;
        self.as_offset(offset)
    }

    /// Creates a new [`DateTime`] instance, assuming the current timestamp has the provided offset applied.
    /// The new instance will have the specified offset and the timestamp itself will be converted to `UTC`.
    ///
    /// Returns an [`OutOfRange`](DateTimeError::OutOfRange) error if the provided offset is either not between `UTC-23:59:59` and `UTC-23:59:59` or if it would lead to an invalid date (any date before the year 1970).
    ///
    /// # Example
    /// ```rust
    /// use astrolabe::{DateTime, Offset};
    ///
    /// let date_time = DateTime::from_ymd(1970, 1, 2).unwrap();
    /// // Set offset to UTC+2
    /// let with_offset = date_time.as_offset(-7200).unwrap();
    /// assert_eq!(93600, with_offset.timestamp());
    /// assert_eq!("00:00", with_offset.format("HH:mm").unwrap())
    /// ```
    pub fn as_offset(&self, secs: i64) -> Result<DateTime, DateTimeError> {
        if secs > 0 && self.timestamp() < secs as u64 {
            return Err(DateTimeError::OutOfRange);
        }
        let new_timestamp = if secs < 0 {
            self.timestamp() + -secs as u64
        } else {
            self.timestamp() - secs as u64
        };

        Ok(DateTime(UNIX_EPOCH + Duration::new(new_timestamp, 0), secs))
    }

    /// Returns the offset as seconds.
    ///
    /// # Example
    /// ```rust
    /// use astrolabe::DateTime;
    ///
    /// let date_time = DateTime::now().set_offset(3600).unwrap();
    /// assert_eq!(3600, date_time.get_offset());
    /// ```
    pub fn get_offset(&self) -> i64 {
        self.1
    }

    /// Format as an RFC3339 timestamp (`2022-05-02T15:30:20Z`).
    ///
    /// The set offset will be considered in this function (Default is `UTC`).
    ///
    /// Use the [`Precision`] enum to specify decimal places after seconds:
    /// * [`Precision::Seconds`] -> `2022-05-02T15:30:20Z`
    /// * [`Precision::Centis`] -> `2022-05-02T15:30:20.00Z`
    /// * [`Precision::Millis`] -> `2022-05-02T15:30:20.000Z`
    /// * [`Precision::Micros`] -> `2022-05-02T15:30:20.000000Z`
    /// * [`Precision::Nanos`] -> `2022-05-02T15:30:20.000000000Z`
    ///
    /// # Example
    /// ```rust
    /// use astrolabe::{DateTime, Precision};
    ///
    /// let date_time = DateTime::from_ymdhms(2022, 5, 2, 15, 30, 20).unwrap();
    /// assert_eq!("2022-05-02T15:30:20Z", date_time.format_rfc3339(Precision::Seconds));
    /// ```
    pub fn format_rfc3339(&self, precision: Precision) -> String {
        let timestamp = self.timestamp_with_offset();

        let (year, month, day) = ts_to_d_units(timestamp);
        let (hour, min, sec) = ts_to_t_units(timestamp);
        let nanos = self.duration().subsec_nanos() as u64;

        format!(
            "{}-{}-{}T{}:{}:{}{}{}",
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
            },
            match self.1 {
                0 => "Z".to_string(),
                _ => {
                    let hour = self.1.abs() / 3600;
                    let minute = self.1.abs() % 3600 / 60;
                    let prefix = if self.1 < 0 { "-" } else { "+" };
                    format!(
                        "{}{}:{}",
                        prefix,
                        zero_padded(hour as u64, 2),
                        zero_padded(minute as u64, 2),
                    )
                }
            }
        )
    }

    /// Formatting with format strings based on [Unicode Date Field Symbols](https://www.unicode.org/reports/tr35/tr35-dates.html#Date_Field_Symbol_Table)
    ///
    /// The set offset will be considered in this function (Default is `UTC`).
    ///
    /// Returns an [`InvalidFormat`](DateTimeError::InvalidFormat`) error if the provided format string can't be parsed
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
    /// |                            | h        | 1, 24                          | [1-24]                                   |
    /// |                            | hh       | 01, 24                         | *                                        |
    /// | minute                     | m        | 0, 59                          |                                          |
    /// |                            | mm       | 00, 59                         | *                                        |
    /// | second                     | s        | 0, 59                          |                                          |
    /// |                            | ss       | 00, 59                         | *                                        |
    /// | zone                       | X        | -08, +0530, Z                  |                                          |
    /// |                            | XX       | -0800, Z                       |                                          |
    /// |                            | XXX      | -08:00, Z                      | *                                        |
    /// |                            | XXXX     | -0800, -075258, Z              |                                          |
    /// |                            | XXXXX    | -08:00, -07:52:58, Z           |                                          |
    /// |                            | x        | -08, +0530, +00                | Like X but without Z                     |
    /// |                            | xx       | -0800, +0000                   |                                          |
    /// |                            | xxx      | -08:00, +00:00                 | *                                        |
    /// |                            | xxxx     | -0800, -075258, +0000          |                                          |
    /// |                            | xxxxx    | -08:00, -07:52:58, +00:00      |                                          |
    ///
    /// `*` = Default
    ///
    /// If the sequence is longer than listed in the table, the output will be the same as the default pattern for this unit (marked with `*`)
    ///
    /// Surround any character with apostrophes (`'`) to escape them.
    /// If you want escape `'`, write `''`
    ///
    /// # Example
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

        let timestamp = self.timestamp_with_offset();
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

                Ok(format_part(part, timestamp, self.1)?
                    .chars()
                    .collect::<Vec<char>>())
            })
            .flat_map(|result| match result {
                Ok(vec) => vec.into_iter().map(Ok).collect(),
                Err(er) => vec![Err(er)],
            })
            .collect::<Result<String, DateTimeError>>()
    }

    fn timestamp_with_offset(&self) -> u64 {
        let mut timestamp = self.timestamp();
        if self.1 < 0 {
            timestamp -= -self.1 as u64;
        } else {
            timestamp += self.1 as u64;
        }
        timestamp
    }
}

impl From<SystemTime> for DateTime {
    fn from(time: SystemTime) -> Self {
        DateTime(time, 0)
    }
}

impl From<Duration> for DateTime {
    fn from(duration: Duration) -> Self {
        DateTime(UNIX_EPOCH + duration, 0)
    }
}

impl From<DateTime> for SystemTime {
    fn from(date_time: DateTime) -> Self {
        date_time.0
    }
}

impl From<&DateTime> for SystemTime {
    fn from(date_time: &DateTime) -> Self {
        date_time.0
    }
}

impl Default for DateTime {
    fn default() -> Self {
        Self::from_timestamp(0)
    }
}
