use crate::{
    errors::{out_of_range::create_simple_oor, AstrolabeError},
    util::{
        constants::{DAYS_TO_1970, DAYS_TO_1970_I64, SECS_PER_DAY_U64},
        date::{
            convert::{
                date_to_days, days_to_date, days_to_doy, days_to_wday, year_doy_to_days,
                years_between,
            },
            manipulate::{
                add_days, add_months, add_years, set_day, set_day_of_year, set_month, set_year,
                sub_days, sub_months, sub_years,
            },
        },
        format::format_date_part,
        parse::{parse_date_part, parse_format_string, ParseUnit, ParsedDate},
    },
    DateTime, DateUtilities,
};
use std::{
    fmt::Display,
    ops::{Add, AddAssign, Sub, SubAssign},
    str::FromStr,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

/// Date in the proleptic Gregorian calendar.
///
/// See the [`DateUtilities`](#impl-DateUtilities-for-Date) implementation for get, set and manipulation methods.
///
/// Range: `30. June -5879611`..=`12. July 5879611`. Please note that year 0 does not exist. After year -1 follows year 1.
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Date {
    pub(crate) days: i32,
}

impl Date {
    /// Creates a new [`Date`] instance with [`SystemTime::now()`].
    ///
    /// ```rust
    /// # use astrolabe::{Date, DateUtilities};
    /// let date = Date::now();
    /// assert!(2021 < date.year());
    /// ```
    pub fn now() -> Self {
        let days = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs()
            / SECS_PER_DAY_U64
            + DAYS_TO_1970;
        Self { days: days as i32 }
    }

    /// Creates a new [`Date`] instance from year, month and day (day of month).
    ///
    /// Returns an [`OutOfRange`](AstrolabeError::OutOfRange) error if the provided values are invalid.
    ///
    /// ```rust
    /// # use astrolabe::Date;
    /// let date = Date::from_ymd(2022, 05, 02).unwrap();
    /// assert_eq!("2022/05/02", date.format("yyyy/MM/dd"));
    /// ```
    pub fn from_ymd(year: i32, month: u32, day: u32) -> Result<Self, AstrolabeError> {
        let days = date_to_days(year, month, day)?;

        Ok(Self { days })
    }

    /// Returns the date.
    ///
    /// ```rust
    /// # use astrolabe::Date;
    /// let date = Date::from_ymd(2022, 05, 02).unwrap();
    /// let (year, month, day) = date.as_ymd();
    /// assert_eq!(2022, year);
    /// assert_eq!(5, month);
    /// assert_eq!(2, day);
    /// ```
    pub fn as_ymd(&self) -> (i32, u32, u32) {
        days_to_date(self.days)
    }

    /// Parses a string with a given format and creates a new [`Date`] instance from it. See [`Date::format`] for a list of available symbols.
    ///
    /// Returns an [`InvalidFormat`](AstrolabeError::InvalidFormat) error if the given string could not be parsed with the given format.
    ///
    /// ```rust
    /// # use astrolabe::Date;
    /// let date = Date::parse("2022-05-02", "yyyy-MM-dd").unwrap();
    /// assert_eq!("2022/05/02", date.format("yyyy/MM/dd"));
    /// ```
    pub fn parse(string: &str, format: &str) -> Result<Self, AstrolabeError> {
        let parts = parse_format_string(format);

        let mut date = ParsedDate::default();
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

            let parsed_part = parse_date_part(&part, &mut string)?;
            if let Some(parsed_part) = parsed_part {
                match parsed_part.unit {
                    ParseUnit::Year => date.year = Some(parsed_part.value as i32),
                    ParseUnit::Month => date.month = Some(parsed_part.value as u32),
                    ParseUnit::DayOfMonth => date.day_of_month = Some(parsed_part.value as u32),
                    // Can't be any other variant than `ParseUnit::DayOfYear`
                    _ => date.day_of_year = Some(parsed_part.value as u32),
                };
            };
        }

        // Use day of year if present, otherwise use month + day of month
        Ok(if date.day_of_year.is_some() {
            let days = year_doy_to_days(date.year.unwrap_or(1), date.day_of_year.unwrap())?;
            Self { days }
        } else {
            Self::from_ymd(
                date.year.unwrap_or(1),
                date.month.unwrap_or(1),
                date.day_of_month.unwrap_or(1),
            )?
        })
    }

    /// Formatting with format strings based on [Unicode Date Field Symbols](https://www.unicode.org/reports/tr35/tr35-dates.html#Date_Field_Symbol_Table).
    ///
    /// Please note that not all symbols are implemented. If you need something that is not implemented, please open an issue on [GitHub](https://github.com/GiyoMoon/astrolabe/issues) describing your need.
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
    /// | week       | w        | 8, 27                         | Week of year                             |
    /// |            | ww       | 08, 27                        | *                                        |
    /// | days       | d        | 1                             | Day of month                             |
    /// |            | dd       | 01                            | *                                        |
    /// |            | D        | 1, 24, 135                     | Day of year, *                           |
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
    ///
    /// `*` = Default
    ///
    /// If the sequence is longer than listed in the table, the output will be the same as the default pattern for this unit (marked with `*`).
    ///
    /// Surround any character with apostrophes (`'`) to escape them.
    /// If you want escape `'`, write `''`.
    ///
    /// ```rust
    /// # use astrolabe::Date;
    /// let date = Date::from_ymd(2022, 5, 2).unwrap();
    /// assert_eq!("2022/05/02", date.format("yyyy/MM/dd"));
    /// // Escape characters
    /// assert_eq!("2022/MM/dd", date.format("yyyy/'MM/dd'"));
    /// assert_eq!("2022/'05/02'", date.format("yyyy/''MM/dd''"));
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

                format_date_part(part, self.days)
                    .chars()
                    .collect::<Vec<char>>()
            })
            .collect::<String>()
    }

    /// Returns the duration between the provided date.
    pub fn duration_between(&self, compare: &Self) -> Duration {
        Duration::from_secs(self.days_since(compare).unsigned_abs() * SECS_PER_DAY_U64)
    }
}

// ########################################
//
//  DateUtility trait implementation
//
// ########################################

impl DateUtilities for Date {
    fn year(&self) -> i32 {
        days_to_date(self.days).0
    }

    fn month(&self) -> u32 {
        days_to_date(self.days).1
    }

    fn day(&self) -> u32 {
        days_to_date(self.days).2
    }

    fn day_of_year(&self) -> u32 {
        days_to_doy(self.days)
    }

    fn weekday(&self) -> u8 {
        days_to_wday(self.days, false) as u8
    }

    fn from_timestamp(timestamp: i64) -> Result<Self, AstrolabeError> {
        let days = (timestamp / SECS_PER_DAY_U64 as i64 + DAYS_TO_1970_I64
            - i64::from(
                timestamp.is_negative() && timestamp.unsigned_abs() % SECS_PER_DAY_U64 != 0,
            ))
        .try_into()
        .map_err(|_| {
            create_simple_oor(
                "timestamp",
                (i32::MIN as i128 - DAYS_TO_1970_I64 as i128) * SECS_PER_DAY_U64 as i128,
                (i32::MAX as i128 - DAYS_TO_1970_I64 as i128) * SECS_PER_DAY_U64 as i128
                    + SECS_PER_DAY_U64 as i128
                    - 1,
                timestamp as i128,
            )
        })?;

        Ok(Self { days })
    }

    fn timestamp(&self) -> i64 {
        (self.days as i64 - DAYS_TO_1970_I64) * SECS_PER_DAY_U64 as i64
    }

    fn set_year(&self, year: i32) -> Result<Self, AstrolabeError> {
        let new_days = set_year(self.days, year)?;
        Ok(Self { days: new_days })
    }

    fn set_month(&self, month: u32) -> Result<Self, AstrolabeError> {
        let new_days = set_month(self.days, month)?;
        Ok(Self { days: new_days })
    }

    fn set_day(&self, day: u32) -> Result<Self, AstrolabeError> {
        let new_days = set_day(self.days, day)?;
        Ok(Self { days: new_days })
    }

    fn set_day_of_year(&self, day_of_year: u32) -> Result<Self, AstrolabeError> {
        let new_days = set_day_of_year(self.days, day_of_year)?;
        Ok(Self { days: new_days })
    }

    fn add_years(&self, years: u32) -> Result<Self, AstrolabeError> {
        let new_days = add_years(self.days, years)?;
        Ok(Self { days: new_days })
    }

    fn add_months(&self, months: u32) -> Result<Self, AstrolabeError> {
        let new_days = add_months(self.days, months)?;
        Ok(Self { days: new_days })
    }

    fn add_days(&self, days: u32) -> Result<Self, AstrolabeError> {
        let new_days = add_days(self.days, days)?;
        Ok(Self { days: new_days })
    }

    fn sub_years(&self, years: u32) -> Result<Self, AstrolabeError> {
        let new_days = sub_years(self.days, years)?;
        Ok(Self { days: new_days })
    }

    fn sub_months(&self, months: u32) -> Result<Self, AstrolabeError> {
        let new_days = sub_months(self.days, months)?;
        Ok(Self { days: new_days })
    }

    fn sub_days(&self, days: u32) -> Result<Self, AstrolabeError> {
        let new_days = sub_days(self.days, days)?;
        Ok(Self { days: new_days })
    }

    fn clear_until_year(&self) -> Self {
        Self { days: 0 }
    }

    fn clear_until_month(&self) -> Self {
        let year = days_to_date(self.days).0;
        let new_days = date_to_days(year, 1, 1).unwrap();
        Self { days: new_days }
    }

    fn clear_until_day(&self) -> Self {
        let (year, month, _) = days_to_date(self.days);
        let new_days = date_to_days(year, month, 1).unwrap();
        Self { days: new_days }
    }

    fn years_since(&self, compare: &Self) -> i32 {
        let self_year = days_to_date(self.days).0;
        let self_doy = days_to_doy(self.days);

        let other_year = days_to_date(compare.days).0;
        let other_doy = days_to_doy(compare.days);

        years_between(self_year, self_doy, 0, other_year, other_doy, 0)
    }

    fn months_since(&self, _compare: &Self) -> i32 {
        todo!()
    }

    fn days_since(&self, compare: &Self) -> i64 {
        self.days as i64 - compare.days as i64
    }
}

// ########################################
//
//  Standard trait implementations
//
// ########################################

impl From<&Date> for Date {
    fn from(date: &Date) -> Self {
        Self { days: date.days }
    }
}

impl From<DateTime> for Date {
    fn from(value: DateTime) -> Self {
        Self { days: value.days }
    }
}
impl From<&DateTime> for Date {
    fn from(value: &DateTime) -> Self {
        Self { days: value.days }
    }
}

impl Add<Duration> for Date {
    type Output = Self;

    /// Performs the `+` operation.
    ///
    /// Only adds full days (`86 400` seconds) to [`Date`]. Any additional duration will be ignored.
    fn add(self, rhs: Duration) -> Self::Output {
        let days = self.days + (rhs.as_secs() / SECS_PER_DAY_U64) as i32;
        Self { days }
    }
}
impl AddAssign<Duration> for Date {
    /// Performs the `+=` operation.
    ///
    /// Only adds full days (`86 400` seconds) to [`Date`]. Any additional duration will be ignored.
    fn add_assign(&mut self, rhs: Duration) {
        *self = *self + rhs;
    }
}

impl Sub<Duration> for Date {
    type Output = Self;

    /// Performs the `-` operation.
    ///
    /// Only removes full days (`86 400` seconds) to [`Date`]. Any additional duration will be ignored.
    fn sub(self, rhs: Duration) -> Self::Output {
        let days = self.days - (rhs.as_secs() / SECS_PER_DAY_U64) as i32;
        Self { days }
    }
}
impl SubAssign<Duration> for Date {
    /// Performs the `-=` operation.
    ///
    /// Only removes full days (`86 400` seconds) to [`Date`]. Any additional duration will be ignored.
    fn sub_assign(&mut self, rhs: Duration) {
        *self = *self - rhs;
    }
}

impl Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.format("yyyy/MM/dd"))
    }
}

impl FromStr for Date {
    type Err = AstrolabeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s, "yyyy-MM-dd")
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
    use crate::Date;
    use serde::de;
    use serde::ser;
    use std::fmt;

    /// Serialize a [`Date`] instance as `yyyy-MM-dd`.
    impl ser::Serialize for Date {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ser::Serializer,
        {
            serializer.serialize_str(&self.format("yyyy-MM-dd"))
        }
    }

    struct DateVisitor;

    impl<'de> de::Visitor<'de> for DateVisitor {
        type Value = Date;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a formatted date string in the format `yyyy-MM-dd`")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            value.parse().map_err(E::custom)
        }
    }

    /// Deserialize a `yyyy-MM-dd` formatted string into a [`Date`] instance.
    impl<'de> de::Deserialize<'de> for Date {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            deserializer.deserialize_str(DateVisitor)
        }
    }
}
