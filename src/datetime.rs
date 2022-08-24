use crate::{
    errors::{
        invalid_format::create_invalid_format,
        out_of_range::{create_custom_oor, create_simple_oor},
        AstrolabeError,
    },
    shared::{DAYS_TO_1970, DAYS_TO_1970_I64, NANOS_PER_SEC, SECS_PER_DAY, SECS_PER_DAY_U64},
    util::{
        convert::{
            add_offset_to_dn, date_to_days, days_nanos_to_nanos, days_nanos_to_secs, days_to_date,
            dtu_to_du, dtu_to_tu, nanos_to_days_nanos, nanos_to_unit, remove_offset_from_dn,
            secs_to_days_nanos, time_to_day_seconds,
        },
        format::{format_part, parse_format_string},
        manipulation::{apply_date_unit, apply_time_unit, set_date_unit, set_time_unit},
        parse::parse_offset,
    },
    Date, Offset, Precision, Time,
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
    /// ```rust
    /// # use astrolabe::{DateTime, DateTimeUnit};
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
pub struct DateTime {
    days: i32,
    nanoseconds: u64,
    offset: i32,
}

impl DateTime {
    /// Creates a new [`DateTime`] instance with [`SystemTime::now()`].
    ///
    /// ```rust
    /// # use astrolabe::{DateTime, DateTimeUnit};
    /// let date_time = DateTime::now();
    /// assert!(2021 < date_time.get(DateTimeUnit::Year));
    /// ```
    pub fn now() -> Self {
        let duration = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");

        let days = duration.as_secs() / SECS_PER_DAY_U64 + DAYS_TO_1970;
        let nanoseconds =
            duration.as_secs() % SECS_PER_DAY_U64 * NANOS_PER_SEC + duration.subsec_nanos() as u64;

        Self {
            days: days as i32,
            nanoseconds,
            offset: 0,
        }
    }

    /// Creates a new [`DateTime`] instance from year, month, day (day of month), hour, minute and seconds.
    ///
    /// Returns an [`OutOfRange`](AstrolabeError::OutOfRange) error if the provided values are invalid.
    ///
    /// ```rust
    /// # use astrolabe::{DateTime, DateTimeUnit};
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
        Ok(Self {
            days,
            nanoseconds: seconds * NANOS_PER_SEC,
            offset: 0,
        })
    }

    /// Creates a new [`DateTime`] instance from year, month and day (day of month).
    ///
    /// Returns an [`OutOfRange`](AstrolabeError::OutOfRange) error if the provided values are invalid.
    ///
    /// ```rust
    /// # use astrolabe::DateTime;
    /// let date_time = DateTime::from_ymd(2022, 05, 02).unwrap();
    /// assert_eq!("2022/05/02", date_time.format("yyyy/MM/dd"));
    /// ```
    pub fn from_ymd(year: i32, month: u32, day: u32) -> Result<Self, AstrolabeError> {
        let days = date_to_days(year, month, day)?;

        Ok(Self {
            days,
            nanoseconds: 0,
            offset: 0,
        })
    }

    /// Creates a new [`DateTime`] instance from hour, minute and seconds.
    ///
    /// Returns an [`OutOfRange`](AstrolabeError::OutOfRange) error if the provided values are invalid.
    ///
    /// ```rust
    /// # use astrolabe::DateTime;
    /// let date_time = DateTime::from_hms(12, 32, 12).unwrap();
    /// assert_eq!("0001/01/01 12:32:12", date_time.format("yyyy/MM/dd HH:mm:ss"));
    /// ```
    pub fn from_hms(hour: u32, minute: u32, second: u32) -> Result<Self, AstrolabeError> {
        let seconds = time_to_day_seconds(hour, minute, second)? as u64;

        Ok(Self {
            days: 0,
            nanoseconds: seconds * NANOS_PER_SEC,
            offset: 0,
        })
    }

    /// Creates a new [`DateTime`] instance from a unix timestamp (non-leap seconds since January 1, 1970 00:00:00 UTC).
    ///
    /// Returns an [`OutOfRange`](AstrolabeError::OutOfRange) error if the provided timestamp would result in an out of range date.
    ///
    /// ```rust
    /// # use astrolabe::DateTime;
    /// let date_time = DateTime::from_timestamp(0).unwrap();
    /// assert_eq!("1970/01/01 00:00:00", date_time.format("yyyy/MM/dd HH:mm:ss"));
    /// ```
    pub fn from_timestamp(timestamp: i64) -> Result<Self, AstrolabeError> {
        Self::from_seconds(timestamp + DAYS_TO_1970_I64 * SECS_PER_DAY_U64 as i64)
    }

    /// Creates a new [`DateTime`] instance from days.
    ///
    /// ```rust
    /// # use astrolabe::DateTime;
    /// let date_time = DateTime::from_days(738276);
    /// assert_eq!("2022/05/02", date_time.format("yyyy/MM/dd"));;
    /// ```
    pub fn from_days(days: i32) -> Self {
        Self {
            days,
            nanoseconds: 0,
            offset: 0,
        }
    }

    /// Creates a new [`DateTime`] instance from seconds.
    ///
    /// Returns an [`OutOfRange`](AstrolabeError::OutOfRange) error if the provided seconds would result in an out of range date.
    ///
    /// ```rust
    /// # use astrolabe::DateTime;
    /// let date_time = DateTime::from_seconds(86400).unwrap();
    /// assert_eq!("0001/01/02", date_time.format("yyyy/MM/dd"));
    /// ```
    pub fn from_seconds(seconds: i64) -> Result<Self, AstrolabeError> {
        let (days, nanoseconds) = secs_to_days_nanos(seconds)?;

        Ok(Self {
            days,
            nanoseconds,
            offset: 0,
        })
    }

    /// Creates a new [`DateTime`] instance from nanoseconds.
    ///
    /// Returns an [`OutOfRange`](AstrolabeError::OutOfRange) error if the provided nanoseconds would result in an out of range date.
    ///
    /// ```rust
    /// # use astrolabe::DateTime;
    /// let date_time = DateTime::from_nanoseconds(86_400_000_000_000).unwrap();
    /// assert_eq!("0001/01/02", date_time.format("yyyy/MM/dd"));
    /// ```
    pub fn from_nanoseconds(nanoseconds: i128) -> Result<Self, AstrolabeError> {
        let (days, nanoseconds) = nanos_to_days_nanos(nanoseconds)?;

        Ok(Self {
            days,
            nanoseconds,
            offset: 0,
        })
    }

    /// Creates a new [`DateTime`] instance from an RFC3339 timestamp string.
    ///
    /// ```rust
    /// # use astrolabe::DateTime;
    /// let date_time = DateTime::parse_rfc3339("2022-05-02T15:30:20Z").unwrap();
    /// assert_eq!("2022/05/02 15:30:20", date_time.format("yyyy/MM/dd HH:mm:ss"));
    /// ```
    pub fn parse_rfc3339(string: &str) -> Result<Self, AstrolabeError> {
        if string.len() < 20 {
            return Err(create_invalid_format(
                "RFC3339 string cannot be shorter than 20 chars",
            ));
        }

        let year = string[0..4]
            .parse::<i32>()
            .map_err(|_| create_invalid_format("Failed parsing year from RFC3339 string"))?;
        let month = string[5..7]
            .parse::<u32>()
            .map_err(|_| create_invalid_format("Failed parsing month from RFC3339 string"))?;
        let day = string[8..10]
            .parse::<u32>()
            .map_err(|_| create_invalid_format("Failed parsing day from RFC3339 string"))?;
        let hour = string[11..13]
            .parse::<u32>()
            .map_err(|_| create_invalid_format("Failed parsing hour from RFC3339 string"))?;
        let min = string[14..16]
            .parse::<u32>()
            .map_err(|_| create_invalid_format("Failed parsing minute from RFC3339 string"))?;
        let sec = string[17..19]
            .parse::<u32>()
            .map_err(|_| create_invalid_format("Failed parsing second from RFC3339 string"))?;

        let (nanos, offset) = if string.chars().nth(19).unwrap() == '.' {
            let nanos_string = string[20..]
                .chars()
                .take_while(|&char| char != 'Z' && char != '+' && char != '-')
                .collect::<String>();
            let nanos = nanos_string.parse::<u64>().map_err(|_| {
                create_invalid_format("Failed parsing subseconds from RFC3339 string")
            })? * (1000000000 / 10_u64.pow(nanos_string.len() as u32));

            let offset_substring = string[20..]
                .chars()
                .position(|char| char == 'Z' || char == '+' || char == '-')
                .ok_or_else(|| {
                    create_invalid_format("Failed parsing offset from RFC3339 string")
                })?;
            let offset = parse_offset(&string[20 + offset_substring..])?;

            (nanos, offset)
        } else {
            let offset = parse_offset(&string[19..])?;
            (0, offset)
        };

        let days = date_to_days(year, month, day)?;
        let seconds = time_to_day_seconds(hour, min, sec)? as u64;

        Self {
            days,
            nanoseconds: seconds * NANOS_PER_SEC + nanos,
            offset: 0,
        }
        .as_offset(offset)
    }

    /// Creates a new [`DateTime`] with the specified amount of nanoseconds set as clock time.
    ///
    /// Returns an [`OutOfRange`](AstrolabeError::OutOfRange) error if the provided nanoseconds are invalid (over `86_399_999_999_999`)
    ///
    /// ```rust
    /// # use astrolabe::{DateTime, DateTimeUnit};
    /// let date_time = DateTime::from_days(738276).set_time(3_600_000_000_000).unwrap();
    /// assert_eq!("2022/05/02 01:00:00", date_time.format("yyyy/MM/dd HH:mm:ss"));
    /// ```
    pub fn set_time(&self, nanoseconds: u64) -> Result<Self, AstrolabeError> {
        if nanoseconds >= SECS_PER_DAY_U64 * NANOS_PER_SEC {
            return Err(create_simple_oor(
                "nanoseconds",
                0,
                (SECS_PER_DAY_U64 * NANOS_PER_SEC - 1) as i128,
                nanoseconds as i128,
            ));
        }
        Ok(Self {
            days: self.days,
            nanoseconds,
            offset: 0,
        })
    }

    /// Returns the date.
    ///
    /// ```rust
    /// # use astrolabe::{DateTime, DateTimeUnit};
    /// let date_time = DateTime::from_days(123);
    /// let date = date_time.date();
    /// assert_eq!(123, date.as_days());
    /// ```
    pub fn date(&self) -> Date {
        Date::from_days(self.days)
    }

    /// Returns the clock time.
    ///
    /// ```rust
    /// # use astrolabe::{DateTime, DateTimeUnit};
    /// let date_time = DateTime::from_days(0).set_time(3_600_000_000_000).unwrap();
    /// let time = date_time.time();
    /// assert_eq!(3_600_000_000_000, time.as_nanoseconds());
    /// ```
    pub fn time(&self) -> Time {
        Time::from_nanoseconds(self.nanoseconds)
            .unwrap()
            .set_offset(self.offset)
            .unwrap()
    }

    /// Returns the number of days since January 1, 0001 00:00:00 UTC. (Negative if date is before)
    ///
    /// ```rust
    /// # use astrolabe::DateTime;
    /// let date_time = DateTime::from_ymd(1, 1, 2).unwrap();
    /// assert_eq!(1, date_time.as_days());
    /// ```
    pub fn as_days(&self) -> i32 {
        self.days
    }

    /// Returns the number of seconds since January 1, 0001 00:00:00 UTC. (Negative if date is before)
    ///
    /// ```rust
    /// # use astrolabe::DateTime;
    /// let date_time = DateTime::from_ymd(1, 1, 2).unwrap();
    /// assert_eq!(86400, date_time.as_seconds());
    /// ```
    pub fn as_seconds(&self) -> i64 {
        days_nanos_to_secs(self.days, self.nanoseconds)
    }

    /// Returns the number of nanoseconds since January 1, 0001 00:00:00 UTC. (Negative if date is before)
    ///
    /// ```rust
    /// # use astrolabe::DateTime;
    /// let date_time = DateTime::from_ymd(1, 1, 2).unwrap();
    /// assert_eq!(86_400_000_000_000, date_time.as_nanoseconds());
    /// ```
    pub fn as_nanoseconds(&self) -> i128 {
        days_nanos_to_nanos(self.days, self.nanoseconds)
    }

    /// Returns the number of non-leap seconds since January 1, 1970 00:00:00 UTC. (Negative if date is before)
    ///
    /// ```rust
    /// # use astrolabe::DateTime;
    /// let date_time = DateTime::from_ymd(2000, 1, 1).unwrap();
    /// assert_eq!(946_684_800, date_time.timestamp());
    /// ```
    pub fn timestamp(&self) -> i64 {
        self.as_seconds() - DAYS_TO_1970 as i64 * SECS_PER_DAY_U64 as i64
    }

    /// Returns the number of seconds between two [`DateTime`] instances.
    ///
    /// ```rust
    /// # use astrolabe::DateTime;
    /// let date_time1 = DateTime::from_ymd(1970, 1, 1).unwrap();
    /// let date_time2 = DateTime::from_ymd(1970, 1, 2).unwrap();
    /// assert_eq!(86400, date_time1.between(&date_time2));
    /// assert_eq!(86400, date_time2.between(&date_time1));
    /// ```
    pub fn between(&self, compare: &Self) -> u64 {
        (self.as_seconds() - compare.as_seconds()).unsigned_abs()
    }

    /// Get a specific [`DateTimeUnit`].
    ///
    /// ```rust
    /// # use astrolabe::{DateTime, DateTimeUnit};
    /// let date_time = DateTime::from_ymdhms(2022, 5, 2, 12, 32, 1).unwrap();
    /// assert_eq!(2022, date_time.get(DateTimeUnit::Year));
    /// assert_eq!(5, date_time.get(DateTimeUnit::Month));
    /// assert_eq!(32, date_time.get(DateTimeUnit::Min));
    /// ```
    pub fn get(&self, unit: DateTimeUnit) -> i64 {
        let (days, nanoseconds) = add_offset_to_dn(self.days, self.nanoseconds, self.offset);
        match unit {
            DateTimeUnit::Year => days_to_date(days).0 as i64,
            DateTimeUnit::Month => days_to_date(days).1 as i64,
            DateTimeUnit::Day => days_to_date(days).2 as i64,
            _ => nanos_to_unit(nanoseconds, dtu_to_tu(unit)) as i64,
        }
    }

    /// Creates a new [`DateTime`] instance with a specified amount of time applied (added or subtracted).
    ///
    /// **Note**: When using [`DateTimeUnit::Month`], it adds calendar months and not 30 days. See it's [documentation](DateTimeUnit::Month) for examples.
    ///
    /// Returns an [`OutOfRange`](AstrolabeError::OutOfRange) error if the provided value would result in an out of range date.
    ///
    /// ```rust
    /// # use astrolabe::{DateTime, DateTimeUnit};
    /// let date_time = DateTime::from_ymdhms(1970, 1, 1, 12, 32, 1).unwrap();
    /// let applied = date_time.apply(1, DateTimeUnit::Day).unwrap();
    /// assert_eq!("1970-01-01 12:32:01", date_time.format("yyyy-MM-dd HH:mm:ss"));
    /// assert_eq!("1970-01-02 12:32:01", applied.format("yyyy-MM-dd HH:mm:ss"));
    /// let applied_2 = applied.apply(-1, DateTimeUnit::Hour).unwrap();
    /// assert_eq!("1970-01-02 11:32:01", applied_2.format("yyyy-MM-dd HH:mm:ss"));
    /// ```
    pub fn apply(&self, amount: i64, unit: DateTimeUnit) -> Result<Self, AstrolabeError> {
        Ok(match unit {
            DateTimeUnit::Year | DateTimeUnit::Month | DateTimeUnit::Day => Self {
                days: apply_date_unit(self.days, amount, dtu_to_du(unit))?,
                nanoseconds: self.nanoseconds,
                offset: self.offset,
            },
            _ => Self::from_nanoseconds(apply_time_unit(
                self.as_nanoseconds(),
                amount,
                dtu_to_tu(unit),
            ))?
            .set_offset(self.offset)
            .unwrap(),
        })
    }

    /// Creates a new [`DateTime`] instance with a specific [`DateTimeUnit`] set to the provided value.
    ///
    /// Returns an [`OutOfRange`](AstrolabeError::OutOfRange) error if the provided value is invalid or out of range.
    ///
    /// ```rust
    /// # use astrolabe::{DateTime, DateTimeUnit};
    /// let date_time = DateTime::from_ymdhms(2022, 5, 2, 12, 32, 1).unwrap();
    /// assert_eq!(2000, date_time.set(2000, DateTimeUnit::Year).unwrap().get(DateTimeUnit::Year));
    /// assert_eq!(10, date_time.set(10, DateTimeUnit::Min).unwrap().get(DateTimeUnit::Min));
    /// ```
    pub fn set(&self, value: i32, unit: DateTimeUnit) -> Result<Self, AstrolabeError> {
        let (days, nanoseconds) = add_offset_to_dn(self.days, self.nanoseconds, self.offset);
        Ok(match unit {
            DateTimeUnit::Year | DateTimeUnit::Month | DateTimeUnit::Day => {
                let new_days = set_date_unit(self.days, value, dtu_to_du(unit))?;
                let (days, nanoseconds) = remove_offset_from_dn(new_days, nanoseconds, self.offset);
                Self {
                    days,
                    nanoseconds,
                    offset: self.offset,
                }
            }
            _ => {
                if value.is_negative() {
                    return Err(create_custom_oor(format!(
                        "Value cannot be negative because unit is \"{:?}\"",
                        unit
                    )));
                }
                let new_nanoseconds =
                    set_time_unit(self.nanoseconds, value.unsigned_abs(), dtu_to_tu(unit))?;
                let (days, nanoseconds) = remove_offset_from_dn(days, new_nanoseconds, self.offset);
                Self {
                    days,
                    nanoseconds,
                    offset: self.offset,
                }
            }
        })
    }

    /// Format as an RFC3339 timestamp (`2022-05-02T15:30:20Z`).
    ///
    /// Use the [`Precision`] enum to specify decimal places after seconds:
    /// * [`Precision::Seconds`] -> `2022-05-02T15:30:20Z`
    /// * [`Precision::Centis`] -> `2022-05-02T15:30:20.00Z`
    /// * [`Precision::Millis`] -> `2022-05-02T15:30:20.000Z`
    /// * [`Precision::Micros`] -> `2022-05-02T15:30:20.000000Z`
    /// * [`Precision::Nanos`] -> `2022-05-02T15:30:20.000000000Z`
    ///
    /// ```rust
    /// # use astrolabe::{DateTime, Precision};
    /// let date_time = DateTime::from_ymdhms(2022, 5, 2, 15, 30, 20).unwrap();
    /// assert_eq!("2022-05-02T15:30:20Z", date_time.format_rfc3339(Precision::Seconds));
    /// // Equivalent to:
    /// assert_eq!("2022-05-02T15:30:20Z", date_time.format("yyyy-MM-ddTHH:mm:ssXXX"));
    /// ```
    pub fn format_rfc3339(&self, precision: Precision) -> String {
        match precision {
            Precision::Seconds => self.format("yyyy-MM-ddTHH:mm:ssXXX"),
            Precision::Centis => self.format("yyyy-MM-ddTHH:mm:ss.nnXXX"),
            Precision::Millis => self.format("yyyy-MM-ddTHH:mm:ss.nnnXXX"),
            Precision::Micros => self.format("yyyy-MM-ddTHH:mm:ss.nnnnXXX"),
            Precision::Nanos => self.format("yyyy-MM-ddTHH:mm:ss.nnnnnXXX"),
        }
    }

    /// Formatting with format strings based on [Unicode Date Field Symbols](https://www.unicode.org/reports/tr35/tr35-dates.html#Date_Field_Symbol_Table).
    ///
    /// Please note that not all symbols are implemented. If you need something that is not implemented, please open an issue on [GitHub](https://github.com/GiyoMoon/astrolabe/issues) describing your need.
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
    /// | subsecond values           | n        | 1, 9                           | Deciseconds                              |
    /// |                            | nn       | 01, 99                         | Centiseconds                             |
    /// |                            | nnn      | 001, 999                       | Milliseconds, *                          |
    /// |                            | nnnn     | 000001, 999999                 | Microseconds                             |
    /// |                            | nnnnn    | 000000001, 999999999           | Nanoseconds                              |
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
    /// If the sequence is longer than listed in the table, the output will be the same as the default pattern for this unit (marked with `*`).
    ///
    /// Surround any character with apostrophes (`'`) to escape them.
    /// If you want escape `'`, write `''`.
    ///
    /// ```rust
    /// # use astrolabe::DateTime;
    /// let date_time = DateTime::from_ymdhms(2022, 5, 2, 12, 32, 1).unwrap();
    /// assert_eq!("2022/05/02 12:32:01", date_time.format("yyyy/MM/dd HH:mm:ss"));
    /// // Escape characters
    /// assert_eq!("2022/MM/dd 12:32:01", date_time.format("yyyy/'MM/dd' HH:mm:ss"));
    /// assert_eq!("2022/'05/02' 12:32:01", date_time.format("yyyy/''MM/dd'' HH:mm:ss"));
    /// ```
    ///
    pub fn format(&self, format: &str) -> String {
        let parts = parse_format_string(format);
        let (days, nanoseconds) = add_offset_to_dn(self.days, self.nanoseconds, self.offset);

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

                format_part(part, days, nanoseconds, self.offset)
                    .chars()
                    .collect::<Vec<char>>()
            })
            .collect::<String>()
    }

    /// Creates a new [`DateTime`] instance with a given timezone offset defined as time units (hour, minute and second). Offset can range anywhere from `UTC-23:59:59` to `UTC+23:59:59`.
    ///
    /// The offset affects all format functions and the [`get`](DateTime::get) and [`set`](DateTime::set) functions but does not change the datetime itself which always represents UTC.
    ///
    /// Returns an [`OutOfRange`](AstrolabeError::OutOfRange) error if the provided offset is either not between `UTC-23:59:59` and `UTC+23:59:59` or if it would lead to an out of range date.
    ///
    /// ```rust
    /// # use astrolabe::DateTime;
    /// let date_time = DateTime::from_ymdhms(2022, 5, 2, 12, 32, 1).unwrap();
    /// // Set offset to UTC+2
    /// let with_offset = date_time.set_offset(7200).unwrap();
    /// assert_eq!("2022/05/02 14:32:01", with_offset.format("yyyy/MM/dd HH:mm:ss"));
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

        self.set_offset(seconds)
    }

    /// Creates a new [`DateTime`] instance with a given timezone offset defined as seconds. Offset can range anywhere from `UTC-23:59:59` to `UTC+23:59:59`.
    ///
    /// The offset affects all format functions and the [`get`](DateTime::get) and [`set`](DateTime::set) functions but does not change the datetime itself which always represents UTC.
    ///
    /// Returns an [`OutOfRange`](AstrolabeError::OutOfRange) error if the provided offset is either not between `UTC-23:59:59` and `UTC+23:59:59` or if it would lead to an out of range date.
    ///
    /// ```rust
    /// # use astrolabe::DateTime;
    /// let date_time = DateTime::from_ymdhms(2022, 5, 2, 12, 32, 1).unwrap();
    /// // Set offset to UTC+2
    /// let with_offset = date_time.set_offset(7200).unwrap();
    /// assert_eq!("2022/05/02 14:32:01", with_offset.format("yyyy/MM/dd HH:mm:ss"));
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

        let offset_days = (self.as_seconds() + seconds as i64) / SECS_PER_DAY_U64 as i64;
        let offset_nanos = (self.nanoseconds / NANOS_PER_SEC) as i64 + seconds as i64;
        if offset_days < i32::MIN as i64
            || offset_days > i32::MAX as i64
            || (offset_days == i32::MIN as i64 && offset_nanos.is_negative())
        {
            return Err(create_custom_oor(
                "Offset would result in an out of range date".to_string(),
            ));
        }

        Ok(Self {
            days: self.days,
            nanoseconds: self.nanoseconds,
            offset: seconds as i32,
        })
    }

    /// Creates a new [`DateTime`] instance, assuming the current instance has the provided offset applied. The new instance will have the specified offset and the datetime itself will be converted to `UTC`.
    ///
    /// The offset affects all format functions and the [`get`](DateTime::get) and [`set`](DateTime::set) functions but does not change the datetime itself which always represents UTC.
    ///
    /// Returns an [`OutOfRange`](AstrolabeError::OutOfRange) error if the provided offset is either not between `UTC-23:59:59` and `UTC+23:59:59` or if it would lead to an out of range date.
    ///
    /// ```rust
    /// # use astrolabe::{DateTime, Offset};
    /// let date_time = DateTime::from_ymdhms(2022, 5, 2, 12, 32, 1).unwrap();
    /// // Set offset to UTC+2
    /// let with_offset = date_time.as_offset_time(2, 0, 0, Offset::East).unwrap();
    /// assert_eq!("2022/05/02 12:32:01", with_offset.format("yyyy/MM/dd HH:mm:ss"));
    /// ```
    pub fn as_offset_time(
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

        let new_nanos = self.as_nanoseconds() - seconds as i128 * NANOS_PER_SEC as i128;

        Ok(Self::from_nanoseconds(new_nanos)?
            .set_offset(seconds)
            .unwrap())
    }

    /// Creates a new [`DateTime`] instance, assuming the current instance has the provided offset applied. The new instance will have the specified offset and the datetime itself will be converted to `UTC`.
    ///
    /// The offset affects all format functions and the [`get`](DateTime::get) and [`set`](DateTime::set) functions but does not change the datetime itself which always represents UTC.
    ///
    /// Returns an [`OutOfRange`](AstrolabeError::OutOfRange) error if the provided offset is either not between `UTC-23:59:59` and `UTC+23:59:59` or if it would lead to an out of range date.
    ///
    /// ```rust
    /// # use astrolabe::DateTime;
    /// let date_time = DateTime::from_ymdhms(2022, 5, 2, 12, 32, 1).unwrap();
    /// // Set offset to UTC+2
    /// let with_offset = date_time.as_offset(7200).unwrap();
    /// assert_eq!("2022/05/02 12:32:01", with_offset.format("yyyy/MM/dd HH:mm:ss"));
    /// ```
    pub fn as_offset(&self, seconds: i32) -> Result<Self, AstrolabeError> {
        let new_nanos = self.as_nanoseconds() - seconds as i128 * NANOS_PER_SEC as i128;
        Self::from_nanoseconds(new_nanos)?.set_offset(seconds)
    }

    /// Returns the set offset in seconds.
    ///
    /// ```rust
    /// # use astrolabe::DateTime;
    /// let date_time = DateTime::now().set_offset(3600).unwrap();
    /// assert_eq!(3600, date_time.get_offset());
    /// ```
    pub fn get_offset(&self) -> i32 {
        self.offset
    }
}

impl From<&DateTime> for DateTime {
    fn from(date_time: &DateTime) -> Self {
        Self {
            days: date_time.days,
            nanoseconds: date_time.nanoseconds,
            offset: date_time.offset,
        }
    }
}

impl Display for DateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.format("yyyy/MM/dd HH:mm:ss"))
    }
}