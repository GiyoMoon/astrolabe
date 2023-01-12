use crate::{
    errors::{
        invalid_format::create_invalid_format, out_of_range::create_simple_oor, AstrolabeError,
    },
    shared::{DAYS_TO_1970, DAYS_TO_1970_I64, SECS_PER_DAY_U64},
    util::{
        convert::{date_to_days, days_to_date, year_doy_to_days},
        format::format_date_part,
        manipulation::{apply_date_unit, set_date_unit},
        parse::{parse_date_part, parse_format_string, ParseUnit, ParsedDate},
    },
};
use std::{
    fmt::Display,
    str::FromStr,
    time::{SystemTime, UNIX_EPOCH},
};

/// Date units for functions like [`Date::get`] or [`Date::apply`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DateUnit {
    #[allow(missing_docs)]
    Year,
    /// **Note**: When used in the [`Date::apply`] function, this unit adds or removes calendar months, not 30 days.
    ///
    /// ```rust
    /// # use astrolabe::{Date, DateUnit};
    /// let date = Date::from_ymd(1970, 1, 31).unwrap();
    /// assert_eq!("1970-02-28", date.apply(1, DateUnit::Month).unwrap().format("yyyy-MM-dd"));
    /// assert_eq!("1970-03-31", date.apply(2, DateUnit::Month).unwrap().format("yyyy-MM-dd"));
    /// assert_eq!("1970-04-30", date.apply(3, DateUnit::Month).unwrap().format("yyyy-MM-dd"));
    /// ```
    Month,
    #[allow(missing_docs)]
    Day,
}

/// Date in the proleptic Gregorian calendar.
///
/// Range: `30. June -5879611`..=`12. July 5879611`. Please note that year 0 does not exist. After year -1 follows year 1.
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Date {
    days: i32,
}

impl Date {
    /// Creates a new [`Date`] instance with [`SystemTime::now()`].
    ///
    /// ```rust
    /// # use astrolabe::{Date, DateUnit};
    /// let date = Date::now();
    /// assert!(2021 < date.get(DateUnit::Year));
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

    /// Creates a new [`Date`] instance from a unix timestamp (non-leap seconds since January 1, 1970 00:00:00 UTC).
    ///
    /// Returns an [`OutOfRange`](AstrolabeError::OutOfRange) error if the provided timestamp would result in an out of range date.
    ///
    /// ```rust
    /// # use astrolabe::Date;
    /// let date = Date::from_timestamp(0).unwrap();
    /// assert_eq!("1970/01/01", date.format("yyyy/MM/dd"));
    /// ```
    pub fn from_timestamp(timestamp: i64) -> Result<Self, AstrolabeError> {
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

    /// Creates a new [`Date`] instance from days since January 1, 0001.
    ///
    /// ```rust
    /// # use astrolabe::Date;
    /// let date = Date::from_days(738276);
    /// assert_eq!("2022/05/02", date.format("yyyy/MM/dd"));
    /// ```
    pub fn from_days(days: i32) -> Self {
        Self { days }
    }

    /// Returns the number of days since January 1, 0001. (Negative if date is before)
    ///
    /// ```rust
    /// # use astrolabe::Date;
    /// let date = Date::from_ymd(1, 1, 1).unwrap();
    /// assert_eq!(0, date.as_days());
    /// ```
    pub fn as_days(&self) -> i32 {
        self.days
    }

    /// Returns the number of non-leap seconds since January 1, 1970 00:00:00 UTC. (Negative if date is before)
    ///
    /// ```rust
    /// # use astrolabe::Date;
    /// let date = Date::from_ymd(2000, 1, 1).unwrap();
    /// assert_eq!(946_684_800, date.timestamp());
    /// ```
    pub fn timestamp(&self) -> i64 {
        (self.days as i64 - DAYS_TO_1970_I64) * SECS_PER_DAY_U64 as i64
    }

    /// Returns the number of days between two [`Date`] instances.
    ///
    /// ```rust
    /// # use astrolabe::Date;
    /// let date = Date::from_ymd(1970, 1, 1).unwrap();
    /// let date_2 = Date::from_ymd(1970, 2, 1).unwrap();
    /// assert_eq!(31, date.between(&date_2));
    /// assert_eq!(31, date_2.between(&date));
    /// ```
    pub fn between(&self, compare: &Self) -> u32 {
        (self.days - compare.days).unsigned_abs()
    }

    /// Get a specific [`DateUnit`].
    ///
    /// ```rust
    /// # use astrolabe::{Date, DateUnit};
    /// let date = Date::from_ymd(2022, 5, 2).unwrap();
    /// assert_eq!(2022, date.get(DateUnit::Year));
    /// assert_eq!(5, date.get(DateUnit::Month));
    /// assert_eq!(2, date.get(DateUnit::Day));
    /// ```
    pub fn get(&self, unit: DateUnit) -> i32 {
        match unit {
            DateUnit::Year => days_to_date(self.days).0,
            DateUnit::Month => days_to_date(self.days).1 as i32,
            DateUnit::Day => days_to_date(self.days).2 as i32,
        }
    }

    /// Creates a new [`Date`] instance with a specific [`DateUnit`] set to the provided value.
    ///
    /// Returns an [`OutOfRange`](AstrolabeError::OutOfRange) error if the provided value is invalid or out of range.
    ///
    /// ```rust
    /// # use astrolabe::{Date, DateUnit};
    /// let mut date = Date::from_ymd(2022, 5, 2).unwrap();
    /// date = date.set(2000, DateUnit::Year).unwrap();
    /// date = date.set(10, DateUnit::Day).unwrap();
    /// assert_eq!("2000/05/10", date.format("yyyy/MM/dd"));
    /// ```
    pub fn set(&self, value: i32, unit: DateUnit) -> Result<Self, AstrolabeError> {
        Ok(Self {
            days: set_date_unit(self.days, value, unit)?,
        })
    }

    /// Creates a new [`Date`] instance with a specified amount of time applied (added or subtracted).
    ///
    /// **Note**: When using [`DateUnit::Month`], it adds calendar months and not 30 days. See it's [documentation](DateUnit::Month) for examples.
    ///
    /// Returns an [`OutOfRange`](AstrolabeError::OutOfRange) error if the provided value would result in an out of range date.
    ///
    /// ```rust
    /// # use astrolabe::{Date, DateUnit};
    /// let date = Date::from_ymd(1970, 1, 1).unwrap();
    ///
    /// let applied = date.apply(1, DateUnit::Day).unwrap();
    /// assert_eq!("1970/01/01", date.format("yyyy/MM/dd"));
    /// assert_eq!("1970/01/02", applied.format("yyyy/MM/dd"));
    ///
    /// let applied_2 = applied.apply(-1, DateUnit::Day).unwrap();
    /// assert_eq!("1970/01/01", applied_2.format("yyyy/MM/dd"));
    /// ```
    pub fn apply(&self, amount: i32, unit: DateUnit) -> Result<Self, AstrolabeError> {
        Ok(Self::from_days(apply_date_unit(
            self.days,
            amount as i64,
            unit,
        )?))
    }

    /// Parses a custom string with a given format and creates a new [`Date`] instance from it. See [`Date::format`] for a list of available symbols.
    ///
    /// **Note**: To successfully parse a string, you need to either provide `year`, `month` and `day of month` or `year` and `day of year`.
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

        Ok(
            if date.year.is_some() && date.month.is_some() && date.day_of_month.is_some() {
                Date::from_ymd(
                    date.year.unwrap(),
                    date.month.unwrap(),
                    date.day_of_month.unwrap(),
                )?
            } else if date.year.is_some() && date.day_of_year.is_some() {
                let days = year_doy_to_days(date.year.unwrap(), date.day_of_year.unwrap())?;
                Date::from_days(days)
            } else {
                return Err(create_invalid_format("Not enough data to create a Date instance from this string. Please include year and either month and day of month or day of year".to_string()));
            },
        )
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
}

impl From<&Date> for Date {
    fn from(date: &Date) -> Self {
        Self { days: date.days }
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
        Date::parse(s, "yyyy-MM-dd")
    }
}

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
