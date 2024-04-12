use crate::offset::Offset;
use crate::util::constants::DAYS_TO_1970;
use crate::{
    errors::{invalid_format::create_invalid_format, AstrolabeError},
    util::{
        constants::{
            DAYS_TO_1970_I64, NANOS_PER_DAY, NANOS_PER_SEC, SECS_PER_DAY_U64, SECS_PER_HOUR_U64,
            SECS_PER_MINUTE_U64,
        },
        date::{
            convert::{
                date_to_days, days_to_date, days_to_doy, days_to_wday, months_between,
                year_doy_to_days, years_between,
            },
            manipulate::{
                add_days, add_months, add_years, set_day, set_day_of_year, set_month, set_year,
                sub_days, sub_months, sub_years,
            },
        },
        format::format_part,
        offset::{add_offset_to_dn, remove_offset_from_dn},
        parse::{
            parse_format_string, parse_offset, parse_part, ParseUnit, ParsedDate, ParsedTime,
            Period,
        },
        time::{
            convert::{
                days_nanos_to_hours, days_nanos_to_micros, days_nanos_to_millis,
                days_nanos_to_minutes, days_nanos_to_nanos, days_nanos_to_seconds,
                days_nanos_to_secs, nanos_to_days_nanos, nanos_to_subhour_nanos,
                nanos_to_submicro_nanos, nanos_to_submilli_nanos, nanos_to_subminute_nanos,
                nanos_to_subsecond, nanos_to_subsecond_nanos, nanos_to_time, secs_to_days_nanos,
                since_i128, since_i64, time_to_day_seconds,
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
    Date, DateUtilities, OffsetUtilities, Precision, Time, TimeUtilities,
};
use std::time::{SystemTime, UNIX_EPOCH};
use std::{
    cmp,
    fmt::Display,
    ops::{Add, AddAssign, Sub, SubAssign},
    str::FromStr,
    time::Duration,
};

/// Combined date and time.
/// Date is in the proleptic Gregorian calendar and clock time is with nanosecond precision.
///
/// See the [`DateUtilites`](#impl-DateUtilities-for-DateTime) and [`TimeUtilities`](#impl-TimeUtilities-for-DateTime) implementations for get, set and manipulation methods.
///
/// [`OffsetUtilities`](#impl-OffsetUtilities-for-DateTime) implements methods for setting and getting the offset.
///
/// Range: `30. June -5879611 00:00:00`..=`12. July 5879611 23:59:59`. Please note that year 0 does not exist. After year -1 follows year 1.
#[derive(Debug, Default, Clone, Copy, Eq)]
pub struct DateTime {
    pub(crate) days: i32,
    pub(crate) nanoseconds: u64,
    pub(crate) offset: Offset,
}

impl DateTime {
    /// Creates a new [`DateTime`] instance with [`SystemTime::now()`].
    ///
    /// ```rust
    /// # use astrolabe::{DateTime, DateUtilities};
    /// let date_time = DateTime::now();
    /// assert!(2021 < date_time.year());
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
            offset: Offset::default(),
        }
    }

    /// Creates a new [`DateTime`] instance with [`SystemTime::now()`] with the local timezone as the offset.
    ///
    /// ```rust
    /// # use astrolabe::{DateTime, DateUtilities, Offset, OffsetUtilities};
    /// let date_time = DateTime::now_local();
    /// assert!(2021 < date_time.year());
    /// assert_eq!(date_time.get_offset(), Offset::Local);
    /// ```
    pub fn now_local() -> Self {
        Self::now().set_offset(Offset::Local)
    }

    /// Creates a new [`DateTime`] instance from year, month, day (day of month), hour, minute and seconds.
    ///
    /// Returns an [`OutOfRange`](AstrolabeError::OutOfRange) error if the provided values are invalid.
    ///
    /// ```rust
    /// # use astrolabe::DateTime;
    /// let date_time = DateTime::from_ymdhms(2022, 05, 02, 12, 32, 1).unwrap();
    /// assert_eq!("2022/05/02 12:32:01", date_time.format("yyyy/MM/dd HH:mm:ss"));
    /// ```
    pub fn from_ymdhms(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        minute: u32,
        second: u32,
    ) -> Result<Self, AstrolabeError> {
        let days = date_to_days(year, month, day)?;
        let seconds = time_to_day_seconds(hour, minute, second)? as u64;
        Ok(Self {
            days,
            nanoseconds: seconds * NANOS_PER_SEC,
            offset: Offset::default(),
        })
    }

    /// Returns the DateTime as year, month, day (day of month), hour, minute and seconds.
    ///
    /// ```rust
    /// # use astrolabe::DateTime;
    /// let date_time = DateTime::from_ymdhms(2022, 05, 02, 12, 32, 1).unwrap();
    /// let (year, month, day, hour, minute, second) = date_time.as_ymdhms();
    /// assert_eq!(2022, year);
    /// assert_eq!(5, month);
    /// assert_eq!(2, day);
    /// assert_eq!(12, hour);
    /// assert_eq!(32, minute);
    /// assert_eq!(1, second);
    /// ```
    pub fn as_ymdhms(&self) -> (i32, u32, u32, u32, u32, u32) {
        let (year, month, day) = self.as_ymd();
        let (hour, minute, second) = self.as_hms();
        (year, month, day, hour, minute, second)
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
            offset: Offset::default(),
        })
    }

    /// Returns the DateTime as year, month and day (day of month).
    ///
    /// ```rust
    /// # use astrolabe::DateTime;
    /// let date_time = DateTime::from_ymd(2022, 05, 02).unwrap();
    /// let (year, month, day) = date_time.as_ymd();
    /// assert_eq!(2022, year);
    /// assert_eq!(5, month);
    /// assert_eq!(2, day);
    /// ```
    pub fn as_ymd(&self) -> (i32, u32, u32) {
        days_to_date(self.days)
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
            offset: Offset::default(),
        })
    }

    /// Returns the DateTime as hour, minute and seconds.
    ///
    /// ```rust
    /// # use astrolabe::DateTime;
    /// let date_time = DateTime::from_hms(12, 12, 12).unwrap();
    /// let (hour, minute, second) = date_time.as_hms();
    /// assert_eq!(12, hour);
    /// assert_eq!(12, minute);
    /// assert_eq!(12, second);
    /// ```
    pub fn as_hms(&self) -> (u32, u32, u32) {
        let seconds = self.nanoseconds / NANOS_PER_SEC;

        let hour = seconds / 3600;
        let minute = (seconds % 3600) / 60;
        let second = seconds % 60;

        (hour as u32, minute as u32, second as u32)
    }

    /// Creates a new [`DateTime`] with the specified time.
    ///
    /// ```rust
    /// # use astrolabe::{DateTime, Time};
    /// let time = Time::from_hms(12, 32, 1).unwrap();
    /// let date_time = DateTime::from_ymd(2022, 5, 2).unwrap().set_time(time);
    /// assert_eq!("2022/05/02 12:32:01", date_time.format("yyyy/MM/dd HH:mm:ss"));
    /// ```
    pub fn set_time(&self, time: Time) -> Self {
        Self {
            days: self.days,
            nanoseconds: time.as_nanos(),
            offset: self.offset,
        }
    }

    /// Creates a new [`DateTime`] instance from an RFC 3339 timestamp string.
    ///
    /// ```rust
    /// # use astrolabe::DateTime;
    /// let date_time = DateTime::parse_rfc3339("2022-05-02T15:30:20Z").unwrap();
    /// assert_eq!("2022/05/02 15:30:20", date_time.format("yyyy/MM/dd HH:mm:ss"));
    /// ```
    pub fn parse_rfc3339(string: &str) -> Result<Self, AstrolabeError> {
        if string.len() < 20 {
            return Err(create_invalid_format(
                "RFC 3339 string cannot be shorter than 20 chars".to_string(),
            ));
        }

        let year = string[0..4].parse::<i32>().map_err(|_| {
            create_invalid_format("Failed parsing year from RFC 3339 string".to_string())
        })?;
        let month = string[5..7].parse::<u32>().map_err(|_| {
            create_invalid_format("Failed parsing month from RFC 3339 string".to_string())
        })?;
        let day = string[8..10].parse::<u32>().map_err(|_| {
            create_invalid_format("Failed parsing day from RFC 3339 string".to_string())
        })?;
        let hour = string[11..13].parse::<u32>().map_err(|_| {
            create_invalid_format("Failed parsing hour from RFC 3339 string".to_string())
        })?;
        let minute = string[14..16].parse::<u32>().map_err(|_| {
            create_invalid_format("Failed parsing minute from RFC 3339 string".to_string())
        })?;
        let second = string[17..19].parse::<u32>().map_err(|_| {
            create_invalid_format("Failed parsing second from RFC 3339 string".to_string())
        })?;

        let (nanos, offset) = if string.chars().nth(19).unwrap() == '.' {
            let nanos_string = string[20..]
                .chars()
                .take_while(|&char| char != 'Z' && char != '+' && char != '-')
                .collect::<String>();
            let nanos = nanos_string.parse::<u64>().map_err(|_| {
                create_invalid_format("Failed parsing subseconds from RFC 3339 string".to_string())
            })? * (1000000000 / 10_u64.pow(nanos_string.len() as u32));

            let offset_substring = string[20..]
                .chars()
                .position(|char| char == 'Z' || char == '+' || char == '-')
                .ok_or_else(|| {
                    create_invalid_format("Failed parsing offset from RFC 3339 string".to_string())
                })?;
            let offset = parse_offset(&string[20 + offset_substring..])?;

            (nanos, offset)
        } else {
            let offset = parse_offset(&string[19..])?;
            (0, offset)
        };

        let days = date_to_days(year, month, day)?;
        let seconds = time_to_day_seconds(hour, minute, second)? as u64;

        Ok(Self {
            days,
            nanoseconds: seconds * NANOS_PER_SEC + nanos,
            offset: Offset::default(),
        }
        .as_offset(Offset::Fixed(offset)))
    }

    /// Format as an RFC 3339 timestamp (`2022-05-02T15:30:20Z`).
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

    /// Parses a string with a given format and creates a new [`DateTime`] instance from it. See [`DateTime::format`] for a list of available symbols.
    ///
    /// Returns an [`InvalidFormat`](AstrolabeError::InvalidFormat) error if the given string could not be parsed with the given format.
    ///
    /// ```rust
    /// # use astrolabe::DateTime;
    /// let date_time = DateTime::parse("2022-05-02 12:32:01", "yyyy-MM-dd HH:mm:ss").unwrap();
    /// assert_eq!("2022/05/02 12:32:01", date_time.format("yyyy/MM/dd HH:mm:ss"));
    /// ```
    pub fn parse(string: &str, format: &str) -> Result<Self, AstrolabeError> {
        let parts = parse_format_string(format);

        let mut date = ParsedDate::default();
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

            let parsed_part = parse_part(&part, &mut string)?;
            if let Some(parsed_part) = parsed_part {
                match parsed_part.unit {
                    ParseUnit::Year => date.year = Some(parsed_part.value as i32),
                    ParseUnit::Month => date.month = Some(parsed_part.value as u32),
                    ParseUnit::DayOfMonth => date.day_of_month = Some(parsed_part.value as u32),
                    ParseUnit::DayOfYear => date.day_of_year = Some(parsed_part.value as u32),
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
                    ParseUnit::Offset => time.offset = Some(parsed_part.value as i32),
                };
            };
        }

        // Use day of year if present, otherwise use month + day of month
        let mut date_time = if date.day_of_year.is_some() {
            let days = year_doy_to_days(date.year.unwrap_or(1), date.day_of_year.unwrap(), false)?;
            Self {
                days,
                ..Default::default()
            }
        } else {
            Self::from_ymd(
                date.year.unwrap_or(1),
                date.month.unwrap_or(1),
                date.day_of_month.unwrap_or(1),
            )?
        };

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

        date_time = date_time.set_time(Time::from_nanos(nanoseconds)?);

        if let Some(offset) = time.offset {
            date_time = date_time.as_offset(Offset::from_seconds(offset)?);
        }

        Ok(date_time)
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
    /// |                            | D        | 1, 24, 135                     | Day of year, *                           |
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
        let offset_seconds = self.offset.resolve();
        let parts = parse_format_string(format);
        let (days, nanoseconds) = add_offset_to_dn(self.days, self.nanoseconds, offset_seconds);

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

                format_part(part, days, nanoseconds, offset_seconds)
                    .chars()
                    .collect::<Vec<char>>()
            })
            .collect::<String>()
    }

    /// Returns the duration between the provided DateTime.
    pub fn duration_between(&self, compare: &Self) -> Duration {
        let lower = cmp::min(self, compare);
        let upper = cmp::max(self, compare);

        let mut days = upper.days as i64 - lower.days as i64;
        let mut nanos = upper.nanoseconds as i64 - lower.nanoseconds as i64;
        if lower.nanoseconds > upper.nanoseconds {
            days -= 1;
            nanos += NANOS_PER_DAY as i64;
        };

        let days_duration = Duration::from_secs(days.unsigned_abs() * SECS_PER_DAY_U64);
        let nanos_duration = Duration::from_nanos(nanos.unsigned_abs());
        days_duration + nanos_duration
    }
}

// ########################################
//
//  DateUtility trait implementation
//
// ########################################

impl DateUtilities for DateTime {
    fn year(&self) -> i32 {
        let days = add_offset_to_dn(self.days, self.nanoseconds, self.offset.resolve()).0;

        days_to_date(days).0
    }

    fn month(&self) -> u32 {
        let days = add_offset_to_dn(self.days, self.nanoseconds, self.offset.resolve()).0;

        days_to_date(days).1
    }

    fn day(&self) -> u32 {
        let days = add_offset_to_dn(self.days, self.nanoseconds, self.offset.resolve()).0;

        days_to_date(days).2
    }

    fn day_of_year(&self) -> u32 {
        let days = add_offset_to_dn(self.days, self.nanoseconds, self.offset.resolve()).0;

        days_to_doy(days)
    }

    fn weekday(&self) -> u8 {
        let days = add_offset_to_dn(self.days, self.nanoseconds, self.offset.resolve()).0;

        days_to_wday(days, false) as u8
    }

    fn from_timestamp(timestamp: i64) -> Self {
        let date_time = Self::from_seconds(timestamp + DAYS_TO_1970_I64 * SECS_PER_DAY_U64 as i64);
        match date_time {
            Ok(date_time) => date_time,
            Err(e) => panic!("{}", e),
        }
    }

    fn timestamp(&self) -> i64 {
        self.as_seconds() - DAYS_TO_1970_I64 * SECS_PER_DAY_U64 as i64
    }

    fn set_year(&self, year: i32) -> Result<Self, AstrolabeError> {
        let offset_seconds = self.offset.resolve();
        let (days, nanoseconds) = add_offset_to_dn(self.days, self.nanoseconds, offset_seconds);

        let new_days = set_year(days, year)?;

        Ok(Self {
            days: remove_offset_from_dn(new_days, nanoseconds, offset_seconds).0,
            nanoseconds: self.nanoseconds,
            offset: self.offset,
        })
    }

    fn set_month(&self, month: u32) -> Result<Self, AstrolabeError> {
        let offset_seconds = self.offset.resolve();
        let (days, nanoseconds) = add_offset_to_dn(self.days, self.nanoseconds, offset_seconds);

        let new_days = set_month(days, month)?;

        Ok(Self {
            days: remove_offset_from_dn(new_days, nanoseconds, offset_seconds).0,
            nanoseconds: self.nanoseconds,
            offset: self.offset,
        })
    }

    fn set_day(&self, day: u32) -> Result<Self, AstrolabeError> {
        let offset_seconds = self.offset.resolve();
        let (days, nanoseconds) = add_offset_to_dn(self.days, self.nanoseconds, offset_seconds);

        let new_days = set_day(days, day)?;

        Ok(Self {
            days: remove_offset_from_dn(new_days, nanoseconds, offset_seconds).0,
            nanoseconds: self.nanoseconds,
            offset: self.offset,
        })
    }

    fn set_day_of_year(&self, day_of_year: u32) -> Result<Self, AstrolabeError> {
        let offset_seconds = self.offset.resolve();
        let (days, nanoseconds) = add_offset_to_dn(self.days, self.nanoseconds, offset_seconds);

        let new_days = set_day_of_year(days, day_of_year)?;

        Ok(Self {
            days: remove_offset_from_dn(new_days, nanoseconds, offset_seconds).0,
            nanoseconds: self.nanoseconds,
            offset: self.offset,
        })
    }

    fn add_years(&self, years: u32) -> Self {
        let new_days = add_years(self.days, years);

        let new_days = match new_days {
            Ok(new_days) => new_days,
            Err(e) => panic!("{}", e),
        };

        Self {
            days: new_days,
            nanoseconds: self.nanoseconds,
            offset: self.offset,
        }
    }

    fn add_months(&self, months: u32) -> Self {
        let new_days = add_months(self.days, months);

        let new_days = match new_days {
            Ok(new_days) => new_days,
            Err(e) => panic!("{}", e),
        };

        Self {
            days: new_days,
            nanoseconds: self.nanoseconds,
            offset: self.offset,
        }
    }

    fn add_days(&self, days: u32) -> Self {
        let new_days = add_days(self.days, days);

        let new_days = match new_days {
            Ok(new_days) => new_days,
            Err(e) => panic!("{}", e),
        };

        Self {
            days: new_days,
            nanoseconds: self.nanoseconds,
            offset: self.offset,
        }
    }

    fn sub_years(&self, years: u32) -> Self {
        let new_days = sub_years(self.days, years);

        let new_days = match new_days {
            Ok(new_days) => new_days,
            Err(e) => panic!("{}", e),
        };

        Self {
            days: new_days,
            nanoseconds: self.nanoseconds,
            offset: self.offset,
        }
    }

    fn sub_months(&self, months: u32) -> Self {
        let new_days = sub_months(self.days, months);

        let new_days = match new_days {
            Ok(new_days) => new_days,
            Err(e) => panic!("{}", e),
        };

        Self {
            days: new_days,
            nanoseconds: self.nanoseconds,
            offset: self.offset,
        }
    }

    fn sub_days(&self, days: u32) -> Self {
        let new_days = sub_days(self.days, days);

        let new_days = match new_days {
            Ok(new_days) => new_days,
            Err(e) => panic!("{}", e),
        };

        Self {
            days: new_days,
            nanoseconds: self.nanoseconds,
            offset: self.offset,
        }
    }

    fn clear_until_year(&self) -> Self {
        Self {
            offset: self.offset,
            ..Default::default()
        }
    }

    fn clear_until_month(&self) -> Self {
        let year = days_to_date(self.days).0;
        let new_days = date_to_days(year, 1, 1).unwrap();
        Self {
            days: new_days,
            offset: self.offset,
            ..Default::default()
        }
    }

    fn clear_until_day(&self) -> Self {
        let (year, month, _) = days_to_date(self.days);
        let new_days = date_to_days(year, month, 1).unwrap();
        Self {
            days: new_days,
            offset: self.offset,
            ..Default::default()
        }
    }

    fn years_since(&self, compare: &Self) -> i32 {
        years_between(
            self.days,
            self.nanoseconds,
            compare.days,
            compare.nanoseconds,
        )
    }

    fn months_since(&self, compare: &Self) -> i32 {
        months_between(
            self.days,
            self.nanoseconds,
            compare.days,
            compare.nanoseconds,
        )
    }

    fn days_since(&self, compare: &Self) -> i64 {
        let extra_day = if self.days > compare.days && self.nanoseconds < compare.nanoseconds {
            -1
        } else if self.days < compare.days && self.nanoseconds > compare.nanoseconds {
            1
        } else {
            0
        };

        self.days as i64 - compare.days as i64 + extra_day
    }
}

// ########################################
//
//  TimeUtility trait implementation
//
// ########################################

impl TimeUtilities for DateTime {
    fn hour(&self) -> u32 {
        let nanoseconds = add_offset_to_dn(self.days, self.nanoseconds, self.offset.resolve()).1;

        nanos_to_time(nanoseconds).0
    }

    fn minute(&self) -> u32 {
        let nanoseconds = add_offset_to_dn(self.days, self.nanoseconds, self.offset.resolve()).1;

        nanos_to_time(nanoseconds).1
    }

    fn second(&self) -> u32 {
        let nanoseconds = add_offset_to_dn(self.days, self.nanoseconds, self.offset.resolve()).1;

        nanos_to_time(nanoseconds).2
    }

    fn milli(&self) -> u32 {
        let nanoseconds = add_offset_to_dn(self.days, self.nanoseconds, self.offset.resolve()).1;

        nanos_to_subsecond(nanoseconds).0
    }

    fn micro(&self) -> u32 {
        let nanoseconds = add_offset_to_dn(self.days, self.nanoseconds, self.offset.resolve()).1;

        nanos_to_subsecond(nanoseconds).1
    }

    fn nano(&self) -> u32 {
        let nanoseconds = add_offset_to_dn(self.days, self.nanoseconds, self.offset.resolve()).1;

        nanos_to_subsecond(nanoseconds).2
    }

    fn set_hour(&self, hour: u32) -> Result<Self, AstrolabeError> {
        let offset_seconds = self.offset.resolve();

        let (days, nanos) = add_offset_to_dn(self.days, self.nanoseconds, offset_seconds);

        let new_nanos = set_hour(nanos, hour)?;

        let (new_days, new_nanos) = remove_offset_from_dn(days, new_nanos, offset_seconds);

        Ok(Self {
            days: new_days,
            nanoseconds: new_nanos,
            offset: self.offset,
        })
    }

    fn set_minute(&self, minute: u32) -> Result<Self, AstrolabeError> {
        let offset_seconds = self.offset.resolve();

        let (days, nanos) = add_offset_to_dn(self.days, self.nanoseconds, offset_seconds);

        let new_nanos = set_minute(nanos, minute)?;

        let (new_days, new_nanos) = remove_offset_from_dn(days, new_nanos, offset_seconds);

        Ok(Self {
            days: new_days,
            nanoseconds: new_nanos,
            offset: self.offset,
        })
    }

    fn set_second(&self, second: u32) -> Result<Self, AstrolabeError> {
        let offset_seconds = self.offset.resolve();

        let (days, nanos) = add_offset_to_dn(self.days, self.nanoseconds, offset_seconds);

        let new_nanos = set_second(nanos, second)?;

        let (new_days, new_nanos) = remove_offset_from_dn(days, new_nanos, offset_seconds);

        Ok(Self {
            days: new_days,
            nanoseconds: new_nanos,
            offset: self.offset,
        })
    }

    fn set_milli(&self, milli: u32) -> Result<Self, AstrolabeError> {
        let offset_seconds = self.offset.resolve();

        let (days, nanos) = add_offset_to_dn(self.days, self.nanoseconds, offset_seconds);

        let new_nanos = set_milli(nanos, milli)?;

        let (new_days, new_nanos) = remove_offset_from_dn(days, new_nanos, offset_seconds);

        Ok(Self {
            days: new_days,
            nanoseconds: new_nanos,
            offset: self.offset,
        })
    }

    fn set_micro(&self, micro: u32) -> Result<Self, AstrolabeError> {
        let offset_seconds = self.offset.resolve();

        let (days, nanos) = add_offset_to_dn(self.days, self.nanoseconds, offset_seconds);

        let new_nanos = set_micro(nanos, micro)?;

        let (new_days, new_nanos) = remove_offset_from_dn(days, new_nanos, offset_seconds);

        Ok(Self {
            days: new_days,
            nanoseconds: new_nanos,
            offset: self.offset,
        })
    }

    fn set_nano(&self, nano: u32) -> Result<Self, AstrolabeError> {
        let offset_seconds = self.offset.resolve();

        let (days, nanos) = add_offset_to_dn(self.days, self.nanoseconds, offset_seconds);

        let new_nanos = set_nano(nanos, nano)?;

        let (new_days, new_nanos) = remove_offset_from_dn(days, new_nanos, offset_seconds);

        Ok(Self {
            days: new_days,
            nanoseconds: new_nanos,
            offset: self.offset,
        })
    }

    /// Panics if the provided value would result in an out of range datetime.
    fn add_hours(&self, hours: u32) -> Self {
        let total_nanos =
            self.days as i128 * NANOS_PER_DAY as i128 + add_hours(self.nanoseconds, hours) as i128;

        let (days, nanoseconds) = nanos_to_days_nanos(total_nanos).unwrap_or_else(|_| {
            panic!(
                "Adding {} hours would result into an out of range datetime",
                hours
            )
        });

        Self {
            days,
            nanoseconds,
            offset: self.offset,
        }
    }

    /// Panics if the provided value would result in an out of range datetime.
    fn add_minutes(&self, minutes: u32) -> Self {
        let total_nanos = self.days as i128 * NANOS_PER_DAY as i128
            + add_minutes(self.nanoseconds, minutes) as i128;

        let (days, nanoseconds) = nanos_to_days_nanos(total_nanos).unwrap_or_else(|_| {
            panic!(
                "Adding {} minutes would result into an out of range datetime",
                minutes
            )
        });

        Self {
            days,
            nanoseconds,
            offset: self.offset,
        }
    }

    /// Panics if the provided value would result in an out of range datetime.
    fn add_seconds(&self, seconds: u32) -> Self {
        let total_nanos = self.days as i128 * NANOS_PER_DAY as i128
            + add_seconds(self.nanoseconds, seconds) as i128;

        let (days, nanoseconds) = nanos_to_days_nanos(total_nanos).unwrap_or_else(|_| {
            panic!(
                "Adding {} seconds would result into an out of range datetime",
                seconds
            )
        });

        Self {
            days,
            nanoseconds,
            offset: self.offset,
        }
    }

    fn add_millis(&self, millis: u32) -> Self {
        let total_nanos = self.days as i128 * NANOS_PER_DAY as i128
            + add_millis(self.nanoseconds, millis) as i128;

        let (days, nanoseconds) = nanos_to_days_nanos(total_nanos).unwrap_or_else(|_| {
            panic!(
                "Adding {} milliseconds would result into an out of range datetime",
                millis
            )
        });

        Self {
            days,
            nanoseconds,
            offset: self.offset,
        }
    }

    fn add_micros(&self, micros: u32) -> Self {
        let total_nanos = self.days as i128 * NANOS_PER_DAY as i128
            + add_micros(self.nanoseconds, micros) as i128;

        let (days, nanoseconds) = nanos_to_days_nanos(total_nanos).unwrap_or_else(|_| {
            panic!(
                "Adding {} microseconds would result into an out of range datetime",
                micros
            )
        });

        Self {
            days,
            nanoseconds,
            offset: self.offset,
        }
    }

    fn add_nanos(&self, nanos: u32) -> Self {
        let total_nanos =
            self.days as i128 * NANOS_PER_DAY as i128 + self.nanoseconds as i128 + nanos as i128;

        let (days, nanoseconds) = nanos_to_days_nanos(total_nanos).unwrap_or_else(|_| {
            panic!(
                "Adding {} nanoseconds would result into an out of range datetime",
                nanos
            )
        });

        Self {
            days,
            nanoseconds,
            offset: self.offset,
        }
    }

    fn sub_hours(&self, hours: u32) -> Self {
        let total_nanos = self.days as i128 * NANOS_PER_DAY as i128
            + sub_hours(self.nanoseconds as i64, hours) as i128;

        let (days, nanoseconds) = nanos_to_days_nanos(total_nanos).unwrap_or_else(|_| {
            panic!(
                "Subtracting {} hours would result into an out of range datetime",
                hours
            )
        });

        Self {
            days,
            nanoseconds,
            offset: self.offset,
        }
    }

    fn sub_minutes(&self, minutes: u32) -> Self {
        let total_nanos = self.days as i128 * NANOS_PER_DAY as i128
            + sub_minutes(self.nanoseconds as i64, minutes) as i128;

        let (days, nanoseconds) = nanos_to_days_nanos(total_nanos).unwrap_or_else(|_| {
            panic!(
                "Subtracting {} minutes would result into an out of range datetime",
                minutes
            )
        });

        Self {
            days,
            nanoseconds,
            offset: self.offset,
        }
    }

    fn sub_seconds(&self, seconds: u32) -> Self {
        let total_nanos = self.days as i128 * NANOS_PER_DAY as i128
            + sub_seconds(self.nanoseconds as i64, seconds) as i128;

        let (days, nanoseconds) = nanos_to_days_nanos(total_nanos).unwrap_or_else(|_| {
            panic!(
                "Subtracting {} seconds would result into an out of range datetime",
                seconds
            )
        });

        Self {
            days,
            nanoseconds,
            offset: self.offset,
        }
    }

    fn sub_millis(&self, millis: u32) -> Self {
        let total_nanos = self.days as i128 * NANOS_PER_DAY as i128
            + sub_millis(self.nanoseconds as i64, millis) as i128;

        let (days, nanoseconds) = nanos_to_days_nanos(total_nanos).unwrap_or_else(|_| {
            panic!(
                "Subtracting {} milliseconds would result into an out of range datetime",
                millis
            )
        });

        Self {
            days,
            nanoseconds,
            offset: self.offset,
        }
    }

    fn sub_micros(&self, micros: u32) -> Self {
        let total_nanos = self.days as i128 * NANOS_PER_DAY as i128
            + sub_micros(self.nanoseconds as i64, micros) as i128;

        let (days, nanoseconds) = nanos_to_days_nanos(total_nanos).unwrap_or_else(|_| {
            panic!(
                "Subtracting {} microseconds would result into an out of range datetime",
                micros
            )
        });

        Self {
            days,
            nanoseconds,
            offset: self.offset,
        }
    }

    fn sub_nanos(&self, nanos: u32) -> Self {
        let total_nanos =
            self.days as i128 * NANOS_PER_DAY as i128 + self.nanoseconds as i128 - nanos as i128;

        let (days, nanoseconds) = nanos_to_days_nanos(total_nanos).unwrap_or_else(|_| {
            panic!(
                "Subtracting {} nanoseconds would result into an out of range datetime",
                nanos
            )
        });

        Self {
            days,
            nanoseconds,
            offset: self.offset,
        }
    }

    fn clear_until_hour(&self) -> Self {
        let offset_seconds = self.offset.resolve();

        let (days, _) = add_offset_to_dn(self.days, self.nanoseconds, offset_seconds);
        let (days, nanos) = remove_offset_from_dn(days, 0, offset_seconds);
        Self {
            days,
            nanoseconds: nanos,
            offset: self.offset,
        }
    }

    fn clear_until_minute(&self) -> Self {
        let offset_seconds = self.offset.resolve();

        let (days, nanoseconds) = add_offset_to_dn(self.days, self.nanoseconds, offset_seconds);
        let (days, nanoseconds) =
            remove_offset_from_dn(days, clear_nanos_until_minute(nanoseconds), offset_seconds);
        Self {
            days,
            nanoseconds,
            offset: self.offset,
        }
    }

    fn clear_until_second(&self) -> Self {
        let offset_seconds = self.offset.resolve();

        let (days, nanoseconds) = add_offset_to_dn(self.days, self.nanoseconds, offset_seconds);
        let (days, nanoseconds) =
            remove_offset_from_dn(days, clear_nanos_until_second(nanoseconds), offset_seconds);
        Self {
            days,
            nanoseconds,
            offset: self.offset,
        }
    }

    fn clear_until_milli(&self) -> Self {
        let offset_seconds = self.offset.resolve();

        let (days, nanoseconds) = add_offset_to_dn(self.days, self.nanoseconds, offset_seconds);
        let (days, nanoseconds) =
            remove_offset_from_dn(days, clear_nanos_until_milli(nanoseconds), offset_seconds);
        Self {
            days,
            nanoseconds,
            offset: self.offset,
        }
    }

    fn clear_until_micro(&self) -> Self {
        let offset_seconds = self.offset.resolve();

        let (days, nanoseconds) = add_offset_to_dn(self.days, self.nanoseconds, offset_seconds);
        let (days, nanoseconds) =
            remove_offset_from_dn(days, clear_nanos_until_micro(nanoseconds), offset_seconds);
        Self {
            days,
            nanoseconds,
            offset: self.offset,
        }
    }

    fn clear_until_nano(&self) -> Self {
        let offset_seconds = self.offset.resolve();

        let (days, nanoseconds) = add_offset_to_dn(self.days, self.nanoseconds, offset_seconds);
        let (days, nanoseconds) =
            remove_offset_from_dn(days, clear_nanos_until_nanos(nanoseconds), offset_seconds);
        Self {
            days,
            nanoseconds,
            offset: self.offset,
        }
    }

    type SubDayReturn = i64;

    fn hours_since(&self, compare: &Self) -> Self::SubDayReturn {
        let self_total_hours = days_nanos_to_hours(self.days, self.nanoseconds);
        let self_subhour_nanos = nanos_to_subhour_nanos(self.nanoseconds);

        let compare_total_hours = days_nanos_to_hours(compare.days, compare.nanoseconds);
        let compare_subhour_nanos = nanos_to_subhour_nanos(compare.nanoseconds);

        since_i64(
            self_total_hours,
            self_subhour_nanos,
            compare_total_hours,
            compare_subhour_nanos,
        )
    }

    fn minutes_since(&self, compare: &Self) -> Self::SubDayReturn {
        let self_total_minutes = days_nanos_to_minutes(self.days, self.nanoseconds);
        let self_subminute_nanos = nanos_to_subminute_nanos(self.nanoseconds);

        let compare_total_minutes = days_nanos_to_minutes(compare.days, compare.nanoseconds);
        let compare_subminute_nanos = nanos_to_subminute_nanos(compare.nanoseconds);

        since_i64(
            self_total_minutes,
            self_subminute_nanos,
            compare_total_minutes,
            compare_subminute_nanos,
        )
    }

    fn seconds_since(&self, compare: &Self) -> Self::SubDayReturn {
        let self_total_seconds = days_nanos_to_seconds(self.days, self.nanoseconds);
        let self_subsecond_nanos = nanos_to_subsecond_nanos(self.nanoseconds);

        let compare_total_seconds = days_nanos_to_seconds(compare.days, compare.nanoseconds);
        let compare_subsecond_nanos = nanos_to_subsecond_nanos(compare.nanoseconds);

        since_i64(
            self_total_seconds,
            self_subsecond_nanos,
            compare_total_seconds,
            compare_subsecond_nanos,
        )
    }

    type SubSecReturn = i128;

    fn millis_since(&self, compare: &Self) -> Self::SubSecReturn {
        let self_total_millis = days_nanos_to_millis(self.days, self.nanoseconds);
        let self_submilli_nanos = nanos_to_submilli_nanos(self.nanoseconds);

        let compare_total_millis = days_nanos_to_millis(compare.days, compare.nanoseconds);
        let compare_submilli_nanos = nanos_to_submilli_nanos(compare.nanoseconds);

        since_i128(
            self_total_millis,
            self_submilli_nanos,
            compare_total_millis,
            compare_submilli_nanos,
        )
    }

    fn micros_since(&self, compare: &Self) -> Self::SubSecReturn {
        let self_total_micros = days_nanos_to_micros(self.days, self.nanoseconds);
        let self_submicro_nanos = nanos_to_submicro_nanos(self.nanoseconds);

        let compare_total_micros = days_nanos_to_micros(compare.days, compare.nanoseconds);
        let compare_submicro_nanos = nanos_to_submicro_nanos(compare.nanoseconds);

        since_i128(
            self_total_micros,
            self_submicro_nanos,
            compare_total_micros,
            compare_submicro_nanos,
        )
    }

    fn nanos_since(&self, compare: &Self) -> Self::SubSecReturn {
        let self_total_nanos = days_nanos_to_nanos(self.days, self.nanoseconds);

        let compare_total_nanos = days_nanos_to_nanos(compare.days, compare.nanoseconds);

        self_total_nanos - compare_total_nanos
    }
}

// ########################################
//
//  OffsetUtility trait implementation
//
// ########################################

impl OffsetUtilities for DateTime {
    fn set_offset(&self, offset: Offset) -> Self {
        let offset_seconds = offset.resolve();

        let offset_days = (self.as_seconds() + offset_seconds as i64) / SECS_PER_DAY_U64 as i64;
        let offset_nanos = (self.nanoseconds / NANOS_PER_SEC) as i64 + offset_seconds as i64;
        if offset_days < i32::MIN as i64
            || offset_days > i32::MAX as i64
            || (offset_days == i32::MIN as i64 && offset_nanos.is_negative())
        {
            panic!("Offset would result in an out of range date");
        }

        Self {
            days: self.days,
            nanoseconds: self.nanoseconds,
            offset,
        }
    }

    fn as_offset(&self, offset: Offset) -> Self {
        let new_nanos = self.as_nanos() - offset.resolve() as i128 * NANOS_PER_SEC as i128;
        Self::from_nanos(new_nanos).unwrap().set_offset(offset)
    }

    fn get_offset(&self) -> Offset {
        self.offset
    }
}

// ########################################
//
//  Private helper functions
//
// ########################################

impl DateTime {
    /// Creates a new [`DateTime`] instance from seconds.
    pub(crate) fn from_seconds(seconds: i64) -> Result<Self, AstrolabeError> {
        let (days, nanoseconds) = secs_to_days_nanos(seconds)?;

        Ok(Self {
            days,
            nanoseconds,
            offset: Offset::default(),
        })
    }

    /// Returns the number of seconds since January 1, 0001 00:00:00 UTC. (Negative if date is before)
    pub(crate) fn as_seconds(&self) -> i64 {
        days_nanos_to_secs(self.days, self.nanoseconds)
    }

    /// Creates a new [`DateTime`] instance from nanoseconds.
    pub(crate) fn from_nanos(nanos: i128) -> Result<Self, AstrolabeError> {
        let (days, nanoseconds) = nanos_to_days_nanos(nanos)?;

        Ok(Self {
            days,
            nanoseconds,
            offset: Offset::default(),
        })
    }

    /// Returns the number of nanoseconds since January 1, 0001 00:00:00 UTC. (Negative if date is before)
    pub(crate) fn as_nanos(&self) -> i128 {
        days_nanos_to_nanos(self.days, self.nanoseconds)
    }
}

// ########################################
//
//  Standard trait implementations
//
// ########################################

impl From<&DateTime> for DateTime {
    fn from(date_time: &DateTime) -> Self {
        Self {
            days: date_time.days,
            nanoseconds: date_time.nanoseconds,
            offset: date_time.offset,
        }
    }
}

impl From<Date> for DateTime {
    fn from(value: Date) -> Self {
        Self {
            days: value.days,
            nanoseconds: 0,
            offset: Offset::default(),
        }
    }
}
impl From<&Date> for DateTime {
    fn from(value: &Date) -> Self {
        Self {
            days: value.days,
            nanoseconds: 0,
            offset: Offset::default(),
        }
    }
}

impl From<Time> for DateTime {
    fn from(value: Time) -> Self {
        Self {
            days: 0,
            nanoseconds: value.as_nanos(),
            offset: value.get_offset(),
        }
    }
}
impl From<&Time> for DateTime {
    fn from(time: &Time) -> Self {
        Self {
            days: 0,
            nanoseconds: time.as_nanos(),
            offset: time.get_offset(),
        }
    }
}

impl Display for DateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.format("yyyy/MM/dd HH:mm:ss"))
    }
}

impl PartialEq for DateTime {
    fn eq(&self, rhs: &Self) -> bool {
        self.as_nanos() == rhs.as_nanos()
    }
}
impl PartialOrd for DateTime {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DateTime {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.as_nanos().cmp(&other.as_nanos())
    }
}

impl FromStr for DateTime {
    type Err = AstrolabeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse_rfc3339(s)
    }
}

impl Add<Time> for DateTime {
    type Output = Self;

    fn add(self, rhs: Time) -> Self::Output {
        let nanos = self.as_nanos() + rhs.as_nanos() as i128;
        Self {
            days: (nanos / NANOS_PER_DAY as i128) as i32,
            nanoseconds: (nanos % NANOS_PER_DAY as i128) as u64,
            offset: self.offset,
        }
    }
}
impl AddAssign<Time> for DateTime {
    fn add_assign(&mut self, rhs: Time) {
        *self = *self + rhs;
    }
}

impl Sub<Time> for DateTime {
    type Output = Self;

    fn sub(self, rhs: Time) -> Self::Output {
        let nanos = self.as_nanos() - rhs.as_nanos() as i128;
        Self {
            days: (nanos / NANOS_PER_DAY as i128) as i32,
            nanoseconds: (nanos % NANOS_PER_DAY as i128) as u64,
            offset: self.offset,
        }
    }
}
impl SubAssign<Time> for DateTime {
    fn sub_assign(&mut self, rhs: Time) {
        *self = *self - rhs;
    }
}

impl Add<Duration> for DateTime {
    type Output = Self;

    fn add(self, rhs: Duration) -> Self::Output {
        let nanos = self.as_nanos() + rhs.as_nanos() as i128;
        Self {
            days: (nanos / NANOS_PER_DAY as i128) as i32,
            nanoseconds: (nanos % NANOS_PER_DAY as i128) as u64,
            offset: self.offset,
        }
    }
}
impl AddAssign<Duration> for DateTime {
    fn add_assign(&mut self, rhs: Duration) {
        *self = *self + rhs;
    }
}

impl Sub<Duration> for DateTime {
    type Output = Self;

    fn sub(self, rhs: Duration) -> Self::Output {
        let nanos = self.as_nanos() - rhs.as_nanos() as i128;
        Self {
            days: (nanos / NANOS_PER_DAY as i128) as i32,
            nanoseconds: (nanos % NANOS_PER_DAY as i128) as u64,
            offset: self.offset,
        }
    }
}
impl SubAssign<Duration> for DateTime {
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
    use crate::DateTime;
    use crate::Precision;
    use serde::de;
    use serde::ser;
    use std::fmt;

    /// Serialize a [`DateTime`] instance as an RFC 3339 string.
    impl ser::Serialize for DateTime {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ser::Serializer,
        {
            serializer.serialize_str(&self.format_rfc3339(Precision::Seconds))
        }
    }

    struct DateTimeVisitor;

    impl<'de> de::Visitor<'de> for DateTimeVisitor {
        type Value = DateTime;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("an RFC 3339 formatted date string")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            value.parse().map_err(E::custom)
        }
    }

    /// Deserialize an RFC 3339 string into a [`DateTime`] instance.
    impl<'de> de::Deserialize<'de> for DateTime {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            deserializer.deserialize_str(DateTimeVisitor)
        }
    }
}
