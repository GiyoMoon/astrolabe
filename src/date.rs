use crate::{
    shared::{DAYS_TO_1970, DAYS_TO_1970_I64, SECS_PER_DAY_U64},
    util::{
        convert::{date_to_days, days_to_d_units},
        format::{format_date_part, parse_format_string},
        manipulation::{apply_date_unit, set_date_unit},
    },
    AstrolabeError,
};
use std::time::{SystemTime, UNIX_EPOCH};

/// Date units for functions like [`Date::get`] or [`Date::apply`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DateUnit {
    #[allow(missing_docs)]
    Year,
    /// **Note**: When used in the [`Date::apply`] function, this unit adds or removes calendar months, not 30 days.
    ///
    /// # Example
    /// ```rust
    /// use astrolabe::{Date, DateUnit};
    ///
    /// let date = Date::from_ymd(1970, 1, 31).unwrap();
    /// assert_eq!("1970-02-28", date.apply(1, DateUnit::Month).unwrap().format("yyyy-MM-dd").unwrap());
    /// assert_eq!("1970-03-31", date.apply(2, DateUnit::Month).unwrap().format("yyyy-MM-dd").unwrap());
    /// assert_eq!("1970-04-30", date.apply(3, DateUnit::Month).unwrap().format("yyyy-MM-dd").unwrap());
    /// ```
    Month,
    #[allow(missing_docs)]
    Day,
}

/// Date in the proleptic Gregorian calendar.
///
/// Ranges from `30. June -5879611` to `12. July 5879611`
#[derive(Debug)]
pub struct Date(i32);

impl Date {
    /// Creates a new [`Date`] instance with [`SystemTime::now()`].
    ///
    /// # Example
    /// ```rust
    /// use astrolabe::{Date, DateUnit};
    ///
    /// let date = Date::now();
    /// assert!(2021 < date.get(DateUnit::Year));
    /// ```
    pub fn now() -> Self {
        let days = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            / SECS_PER_DAY_U64
            + DAYS_TO_1970;
        Date(days as i32)
    }

    /// Creates a new [`Date`] instance from days.
    ///
    /// # Example
    /// ```rust
    /// use astrolabe::Date;
    ///
    /// let date = Date::from_days(738276);
    /// assert_eq!(1_651_449_600, date.timestamp());
    /// ```
    pub fn from_days(days: i32) -> Self {
        Date(days)
    }

    /// Creates a new [`Date`] instance from year, month and day (day of month).
    ///
    /// Returns an [`OutOfRange`](AstrolabeError::OutOfRange) error if the provided date is invalid.
    ///
    /// # Example
    /// ```rust
    /// use astrolabe::Date;
    ///
    /// let date = Date::from_ymd(2022, 05, 02).unwrap();
    /// assert_eq!(1_651_449_600, date.timestamp());
    /// ```
    pub fn from_ymd(year: i32, month: u32, day: u32) -> Result<Self, AstrolabeError> {
        let days = date_to_days(year, month, day)?;

        Ok(Date(days))
    }

    /// Creates a new [`Date`] instance from a unix timestamp (non-leap seconds since January 1, 1970 00:00:00 UTC).
    ///
    /// Returns an [`OutOfRange`](AstrolabeError::OutOfRange) error if the provided timestamp would result in an out of range date.
    ///
    /// # Example
    /// ```rust
    /// use astrolabe::Date;
    ///
    /// let date = Date::from_timestamp(0).unwrap();
    /// assert_eq!(0, date.timestamp());
    /// ```
    pub fn from_timestamp(timestamp: i64) -> Result<Self, AstrolabeError> {
        let days = (timestamp / SECS_PER_DAY_U64 as i64 + DAYS_TO_1970_I64)
            .try_into()
            .map_err(|_| AstrolabeError::OutOfRange)?;
        Ok(Date(days))
    }

    /// Returns the number of days since January 1, 0001 (Negative if date is before)
    ///
    /// # Example
    /// ```rust
    /// use astrolabe::Date;
    ///
    /// let date = Date::from_ymd(1, 1, 1).unwrap();
    /// assert_eq!(0, date.as_days());
    /// ```
    pub fn as_days(&self) -> i32 {
        self.0
    }

    /// Returns the number of non-leap seconds since January 1, 1970 00:00:00 UTC. (Negative if date is before)
    ///
    /// # Example
    /// ```rust
    /// use astrolabe::Date;
    ///
    /// let date = Date::from_ymd(2000, 1, 1).unwrap();
    /// assert_eq!(946_684_800, date.timestamp());
    /// ```
    pub fn timestamp(&self) -> i64 {
        (self.0 as i64 - DAYS_TO_1970_I64) * SECS_PER_DAY_U64 as i64
    }

    /// Returns the number of days between two [`Date`] instances.
    ///
    /// # Example
    /// ```rust
    /// use astrolabe::Date;
    ///
    /// let date = Date::from_ymd(1970, 1, 1).unwrap();
    /// let date_2 = Date::from_ymd(1970, 2, 1).unwrap();
    /// assert_eq!(31, date.between(&date_2));
    /// assert_eq!(31, date_2.between(&date));
    /// ```
    pub fn between(&self, compare: &Date) -> u32 {
        (self.0 - compare.0).unsigned_abs()
    }

    /// Get a specific [`DateUnit`].
    ///
    /// # Example
    /// ```rust
    /// use astrolabe::{Date, DateUnit};
    ///
    /// let date = Date::from_ymd(2022, 5, 2).unwrap();
    /// assert_eq!(2022, date.get(DateUnit::Year));
    /// assert_eq!(5, date.get(DateUnit::Month));
    /// assert_eq!(2, date.get(DateUnit::Day));
    /// ```
    pub fn get(&self, unit: DateUnit) -> i32 {
        match unit {
            DateUnit::Year => days_to_d_units(self.0).0,
            DateUnit::Month => days_to_d_units(self.0).1 as i32,
            DateUnit::Day => days_to_d_units(self.0).2 as i32,
        }
    }

    /// Creates a new [`Date`] instance with a specified amount of time applied (added or subtracted).
    ///
    /// **Note**: When using [`DateUnit::Month`], it adds calendar months and not 30 days. See it's [documentation](DateUnit::Month) for examples.
    ///
    /// Returns an [`OutOfRange`](AstrolabeError::OutOfRange) error if the provided value would result in an out of range date.
    ///
    /// # Example
    /// ```rust
    /// use astrolabe::{Date, DateUnit};
    ///
    /// let date = Date::from_ymd(1970, 1, 1).unwrap();
    /// let applied = date.apply(1, DateUnit::Day).unwrap();
    /// assert_eq!("1970-01-01", date.format("yyyy-MM-dd").unwrap());
    /// assert_eq!("1970-01-02", applied.format("yyyy-MM-dd").unwrap());
    /// let applied_2 = applied.apply(-1, DateUnit::Day).unwrap();
    /// assert_eq!("1970-01-01", applied_2.format("yyyy-MM-dd").unwrap());
    /// ```
    pub fn apply(&self, amount: i32, unit: DateUnit) -> Result<Date, AstrolabeError> {
        Ok(Date::from_days(apply_date_unit(
            self.0,
            amount as i64,
            unit,
        )?))
    }

    /// Creates a new [`Date`] instance with a specific [`DateUnit`] set to the provided value.
    ///
    /// Returns an [`OutOfRange`](AstrolabeError::OutOfRange) error if the provided value is invalid or out of range.
    ///
    /// # Example
    /// ```rust
    /// use astrolabe::{Date, DateUnit};
    ///
    /// let date = Date::from_ymd(2022, 5, 2).unwrap();
    /// assert_eq!(2000, date.set(2000, DateUnit::Year).unwrap().get(DateUnit::Year));
    /// assert_eq!(10, date.set(10, DateUnit::Day).unwrap().get(DateUnit::Day));
    /// ```
    pub fn set(&self, value: i32, unit: DateUnit) -> Result<Date, AstrolabeError> {
        Ok(Date(set_date_unit(self.0, value, unit)?))
    }

    /// Formatting with format strings based on [Unicode Date Field Symbols](https://www.unicode.org/reports/tr35/tr35-dates.html#Date_Field_Symbol_Table).
    ///
    /// Returns an [`InvalidFormat`](AstrolabeError::InvalidFormat`) error if the provided format string can't be parsed.
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
    /// use astrolabe::Date;
    ///
    /// let date = Date::from_ymd(2022, 5, 2).unwrap();
    /// assert_eq!("2022/05/02", date.format("yyyy/MM/dd").unwrap());
    /// // Escape characters
    /// assert_eq!("2022/MM/dd", date.format("yyyy/'MM/dd'").unwrap());
    /// assert_eq!("2022/'05/02'", date.format("yyyy/''MM/dd''").unwrap());
    /// ```
    ///
    pub fn format(&self, format: &str) -> Result<String, AstrolabeError> {
        let parts = parse_format_string(format)?;
        let days = self.as_days();
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

                Ok(format_date_part(part, days)?.chars().collect::<Vec<char>>())
            })
            .flat_map(|result| match result {
                Ok(vec) => vec.into_iter().map(Ok).collect(),
                Err(er) => vec![Err(er)],
            })
            .collect::<Result<String, AstrolabeError>>()
    }
}

impl From<&Date> for Date {
    fn from(date: &Date) -> Self {
        Date::from_days(date.as_days())
    }
}

impl Default for Date {
    fn default() -> Self {
        Self::from_days(0)
    }
}
