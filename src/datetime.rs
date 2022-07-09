#[cfg(feature = "format")]
use crate::format::format_part;
use crate::format::{date_from_timestamp, time_from_timestamp, zero_padded};
#[cfg(feature = "format")]
use fancy_regex::Regex;
#[cfg(feature = "format")]
use once_cell::sync::Lazy;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[cfg(feature = "format")]
static FORMAT_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"([^'])\1*|''|'(''|[^'])+('|$)").unwrap());

/// Error parsing or formatting [`DateTime`] struct
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// Failed parsing format string
    InvalidFormat,
    /// Numeric component is out of range
    OutOfRange,
}

/// Used for specifing the precision for RFC3339 timestamps
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Precision {
    /// Only seconds -> `2022-05-02T15:30:20Z`
    Seconds,
    /// 2 decimal places -> `2022-05-02T15:30:20.00Z`
    Centi,
    /// 3 decimal places -> `2022-05-02T15:30:20.000Z`
    Millis,
    /// 6 decimal places -> `2022-05-02T15:30:20.000000Z`
    Micros,
    /// 9 decimal places -> `2022-05-02T15:30:20.000000000Z`
    Nanos,
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
    /// # Example
    ///
    /// ```rust
    /// use astrolabe::DateTime;
    ///
    /// let date_time = DateTime::from_ymd(1970, 1, 1).unwrap();
    /// assert_eq!(0, date_time.timestamp());
    /// ```
    pub fn from_ymd(year: u64, month: u64, day: u64) -> Result<Self, Error> {
        if year < 1970 {
            return Err(Error::OutOfRange);
        }

        let days = days_from_ymd(year, month, day)?;

        Ok(DateTime(UNIX_EPOCH + Duration::new(days * 86400, 0)))
    }

    /// Creates a new [`DateTime`] struct from year, month, day (day of month), hours, minutes and seconds
    ///
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
    ) -> Result<Self, Error> {
        if year < 1970 || hour > 23 || min > 59 || sec > 59 {
            return Err(Error::OutOfRange);
        }

        let days = days_from_ymd(year, month, day)?;
        let day_seconds = hour * 3600 + min * 60 + sec;

        Ok(DateTime(
            UNIX_EPOCH + Duration::new(days * 86400 + day_seconds, 0),
        ))
    }

    /// Returns the number of non-leap seconds since January 1, 1970 0:00:00 UTC
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
        SystemTime::from(self)
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
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
        let duration = self
            .0
            .duration_since(UNIX_EPOCH)
            .expect("All times should be after epoch");
        let (year, month, day) = date_from_timestamp(duration.as_secs());
        let (hour, min, sec) = time_from_timestamp(duration.as_secs());
        let nanos = duration.subsec_nanos() as i64;

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
                Precision::Centi => format!(".{}", &zero_padded(nanos, 2)[..2]),
                Precision::Millis => format!(".{}", &zero_padded(nanos, 3)[..3]),
                Precision::Micros => format!(".{}", &zero_padded(nanos, 6)[..6]),
                Precision::Nanos => format!(".{}", &zero_padded(nanos, 9)[..9]),
            }
        )
    }

    /// Formatting with specific format strings based on [Unicode Date Field Symbols](https://www.unicode.org/reports/tr35/tr35-dates.html#Date_Field_Symbol_Table)
    ///
    /// # Available Symbols:
    ///
    /// | Field Type | Pattern | Examples                      | Hint                                 |
    /// | ---------- | ------- | ----------------------------- | ------------------------------------ |
    /// | year       | y       | 2, 20, 201, 2017, 20173       |                                      |
    /// |            | yy      | 02, 20, 01, 17, 73            |                                      |
    /// |            | yyy     | 002, 020, 201, 2017, 20173    |                                      |
    /// |            | yyyy    | 0002, 0020, 0201, 2017, 20173 |                                      |
    /// |            | yyyyy+  | ...                           | Unlimited length, padded with zeros. |
    /// | month      | M       | 9, 12                         |                                      |
    /// |            | MM      | 09, 12                        |                                      |
    /// |            | MMM     | Sep                           |                                      |
    /// |            | MMMM    | September                     | *                                    |
    /// |            | MMMMM   | S                             |                                      |
    /// | days       | d       | 1                             | Day of month                         |
    /// |            | dd      | 01                            | *                                    |
    /// | hour       | h       | 1, 12                         | [1-12]                               |
    /// |            | hh      | 01, 12                        | *                                    |
    /// |            | H       | 0, 23                         | [0-23]                               |
    /// |            | HH      | 00, 23                        | *                                    |
    /// |            | K       | 0, 11                         | [0-11]                               |
    /// |            | KK      | 00, 11                        | *                                    |
    /// |            | h       | 1, 24                         | [1-24]                               |
    /// |            | hh      | 01, 24                        | *                                    |
    /// | minute     | m       | 0, 59                         |                                      |
    /// |            | mm      | 00, 59                        | *                                    |
    /// | second     | s       | 0, 59                         |                                      |
    /// |            | ss      | 00, 59                        | *                                    |
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
    #[cfg(feature = "format")]
    #[cfg_attr(docsrs, doc(cfg(feature = "format")))]
    pub fn format(&self, format: &str) -> Result<String, Error> {
        FORMAT_REGEX
            .captures_iter(format)
            .into_iter()
            .map(|capture| -> Result<Vec<char>, Error> {
                let part = capture
                    .map_err(|_| Error::InvalidFormat)?
                    .get(0)
                    .ok_or(Error::InvalidFormat)?
                    .as_str();

                // Escaped apostrophes
                if part == "''" {
                    return Ok("'".chars().collect::<Vec<char>>());
                }

                // Escape parts starting with apostrophe
                if part.starts_with('\'') {
                    let part = part.replace("''", "'");
                    if part.ends_with('\'') {
                        return Ok(part[1..part.len() - 1].chars().collect::<Vec<char>>());
                    } else {
                        return Ok(part[1..part.len()].chars().collect::<Vec<char>>());
                    }
                }

                Ok(format_part(
                    part,
                    self.0
                        .duration_since(UNIX_EPOCH)
                        .expect("All times should be after epoch"),
                )?
                .chars()
                .collect::<Vec<char>>())
            })
            .flat_map(|result| match result {
                Ok(vec) => vec.into_iter().map(Ok).collect(),
                Err(er) => vec![Err(er)],
            })
            .collect::<Result<String, Error>>()
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

fn days_from_ymd(year: u64, month: u64, day: u64) -> Result<u64, Error> {
    let leap_years = leap_years_before(year);
    let is_leap = is_leap_year(year);
    let (mut ydays, mdays) = match month {
        1 => (0, 31),
        2 if is_leap => (31, 29),
        2 => (31, 28),
        3 => (59, 31),
        4 => (90, 30),
        5 => (120, 31),
        6 => (151, 30),
        7 => (181, 31),
        8 => (212, 31),
        9 => (243, 30),
        10 => (273, 31),
        11 => (304, 30),
        12 => (334, 31),
        _ => return Err(Error::OutOfRange),
    };

    if day > mdays || day == 0 {
        return Err(Error::OutOfRange);
    }
    ydays += day - 1;

    if is_leap && month > 2 {
        ydays += 1;
    }
    Ok((year - 1970) * 365 + leap_years + ydays)
}

fn leap_years_before(mut year: u64) -> u64 {
    year -= 1;
    (year - 1968) / 4 - (year - 1900) / 100 + (year - 1600) / 400
}

fn is_leap_year(year: u64) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}
