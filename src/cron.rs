use crate::{
    errors::{invalid_format::create_invalid_format, AstrolabeError},
    DateTime, DateUtilities, TimeUtilities,
};
use std::{collections::HashSet, str::FromStr};

pub(crate) enum Month {
    Jan,
    Feb,
    Mar,
    Apr,
    May,
    Jun,
    Jul,
    Aug,
    Sep,
    Oct,
    Nov,
    Dec,
}

impl FromStr for Month {
    type Err = AstrolabeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_lowercase().as_str() {
            "jan" => Self::Jan,
            "feb" => Self::Feb,
            "mar" => Self::Mar,
            "apr" => Self::Apr,
            "may" => Self::May,
            "jun" => Self::Jun,
            "jul" => Self::Jul,
            "aug" => Self::Aug,
            "sep" => Self::Sep,
            "oct" => Self::Oct,
            "nov" => Self::Nov,
            "dec" => Self::Dec,
            _ => {
                return Err(create_invalid_format(format!(
                    "Input is not a valid month: {}",
                    s
                )))
            }
        })
    }
}

pub(crate) enum DayOfWeek {
    Sun,
    Mon,
    Tue,
    Wed,
    Thu,
    Fri,
    Sat,
}

impl FromStr for DayOfWeek {
    type Err = AstrolabeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_lowercase().as_str() {
            "sun" => Self::Sun,
            "mon" => Self::Mon,
            "tue" => Self::Tue,
            "wed" => Self::Wed,
            "thu" => Self::Thu,
            "fri" => Self::Fri,
            "sat" => Self::Sat,
            _ => {
                return Err(create_invalid_format(format!(
                    "Input is not a valid day of week: {}",
                    s
                )))
            }
        })
    }
}

#[derive(PartialEq, Eq)]
enum CronPartType {
    Numeric,
    Month,
    DayOfWeek,
}

/// A cron expression parser. Implements [`std::Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html) to generate corresponding [`DateTime`] structs.
#[derive(Debug, Clone)]
pub struct CronSchedule {
    minutes: HashSet<u8>,
    hours: HashSet<u8>,
    days_of_month: HashSet<u8>,
    months: HashSet<u8>,
    days_of_week: HashSet<u8>,
    last_schedule: Option<DateTime>,
    #[cfg(test)]
    now: Option<DateTime>,
}

impl CronSchedule {
    /// Creates a new [`CronSchedule`] based on the provided cron expression.
    /// Aims to be compatible with [crontab](https://man7.org/linux/man-pages/man5/crontab.5.html) from Linux.
    ///
    /// Returns an [`InvalidFormat`](AstrolabeError::InvalidFormat) error if the given expression could not be parsed.
    ///
    /// Format: `"minute hour day-of-month month day-of-week"`
    ///
    /// | Field        | Allowed values               |
    /// | ------------ | ---------------------------- |
    /// | minute       | 0-59                         |
    /// | hour         | 0-23                         |
    /// | day of month | 1-31                         |
    /// | month        | 1-12, Jan-Dec                |
    /// | day of week  | 0-7 (0/7 is Sunday), Sun-Sat |
    ///
    /// - Use `*` (asterisk) to indicate that all values of the field are valid.
    /// - Every field also allows `,` (comma) and `-` (hyphen) to specify multiple values and ranges. You can also combine them, for example `1,3-5,10-15`.
    /// - Step values are also supported, for example `*/5` in the minute field means every 5 minutes.
    ///
    /// ```rust
    /// # use astrolabe::CronSchedule;
    /// // Every 5 minutes
    /// let schedule = CronSchedule::parse("*/5 * * * *").unwrap();
    /// for date in schedule.take(3) {
    ///    println!("{}", date);
    /// }
    /// // Prints for example:
    /// // 2022-05-02 16:15:00
    /// // 2022-05-02 16:20:00
    /// // 2022-05-02 16:25:00
    ///
    /// // Every weekday at 10:00
    /// let schedule = CronSchedule::parse("0 10 * * Mon-Fri").unwrap();
    /// for date in schedule.take(3) {
    ///    println!("{}", date.format("yyyy-MM-dd HH:mm:ss eeee"));
    /// }
    /// // Prints for example:
    /// // 2022-05-03 10:00:00 Tuesday
    /// // 2022-05-04 10:00:00 Wednesday
    /// // 2022-05-05 10:00:00 Thursday
    /// ```
    #[cfg(not(test))]
    pub fn parse(expression: &str) -> Result<Self, AstrolabeError> {
        let fields = parse_expression(expression)?;

        Ok(CronSchedule {
            minutes: fields.0,
            hours: fields.1,
            days_of_month: fields.2,
            months: fields.3,
            days_of_week: fields.4,
            last_schedule: None,
        })
    }

    /// Mock function of [`CronSchedule::parse`] for testing.
    /// Allows to set a custom [`DateTime`] as the current time.
    #[cfg(test)]
    pub fn parse(expression: &str, now: Option<DateTime>) -> Result<Self, AstrolabeError> {
        let fields = parse_expression(expression)?;

        Ok(CronSchedule {
            minutes: fields.0,
            hours: fields.1,
            days_of_month: fields.2,
            months: fields.3,
            days_of_week: fields.4,
            last_schedule: None,
            now,
        })
    }
}

type CronParts = (
    HashSet<u8>,
    HashSet<u8>,
    HashSet<u8>,
    HashSet<u8>,
    HashSet<u8>,
);

fn parse_expression(expression: &str) -> Result<CronParts, AstrolabeError> {
    let fields: Vec<&str> = expression.split_whitespace().collect();

    if fields.len() != 5 {
        return Err(create_invalid_format(
            "Invalid number of cron fields, has to consists of 5 fields".to_string(),
        ));
    }

    let minutes = parse_cron_part(fields[0], 0, 59, &CronPartType::Numeric)
        .map_err(|err| create_invalid_format(format!("Failed parsing minute field: {}", err)))?;

    let hours = parse_cron_part(fields[1], 0, 23, &CronPartType::Numeric)
        .map_err(|err| create_invalid_format(format!("Failed parsing hour field: {}", err)))?;

    let days_of_month =
        parse_cron_part(fields[2], 1, 31, &CronPartType::Numeric).map_err(|err| {
            create_invalid_format(format!("Failed parsing day of month field: {}", err))
        })?;

    let months = parse_cron_part(fields[3], 1, 12, &CronPartType::Month)
        .map_err(|err| create_invalid_format(format!("Failed parsing month field: {}", err)))?;

    let days_of_week =
        parse_cron_part(fields[4], 0, 6, &CronPartType::DayOfWeek).map_err(|err| {
            create_invalid_format(format!("Failed parsing day of week field: {}", err))
        })?;

    Ok((minutes, hours, days_of_month, months, days_of_week))
}

impl Iterator for CronSchedule {
    type Item = DateTime;

    fn next(&mut self) -> Option<Self::Item> {
        #[cfg(not(test))]
        let now = DateTime::now().clear_until_second();
        #[cfg(test)]
        let now = self.now.unwrap_or(DateTime::now()).clear_until_second();

        let last = match self.last_schedule {
            Some(last) if last >= now => last,
            _ => now,
        };

        let mut next = last.add_minutes(1).ok()?;

        let dom_restricted = self.days_of_month.len() != 31;
        let dow_restricted = self.days_of_week.len() != 7;

        loop {
            if !self.months.contains(&(next.month() as u8)) {
                next = next.add_months(1).ok()?.clear_until_day();
                continue;
            }

            let day_of_month = next.day() as u8;
            let day_of_week = next.weekday();

            // If both are restricted, the datetime will be valid if either field
            // matches the current time.
            // If only one is restricted, the datetime will be valid if the
            // restricted field matches the current time.
            if (dom_restricted
                && dow_restricted
                && !self.days_of_month.contains(&day_of_month)
                && !self.days_of_week.contains(&day_of_week))
                || (dom_restricted
                    && !dow_restricted
                    && !self.days_of_month.contains(&day_of_month))
                || (dow_restricted && !dom_restricted && !self.days_of_week.contains(&day_of_week))
            {
                next = next.add_days(1).ok()?.clear_until_hour();
                continue;
            }

            if !self.hours.contains(&(next.hour() as u8)) {
                next = next.add_hours(1).ok()?.clear_until_minute();
                continue;
            }

            if !self.minutes.contains(&(next.minute() as u8)) {
                next = next.add_minutes(1).ok()?.clear_until_second();
                continue;
            }

            break;
        }

        self.last_schedule = Some(next);
        Some(next)
    }
}

impl FromStr for CronSchedule {
    type Err = AstrolabeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        #[cfg(not(test))]
        return Self::parse(s);
        #[cfg(test)]
        return Self::parse(s, None);
    }
}

fn parse_cron_part(
    field: &str,
    min: u8,
    max: u8,
    cron_type: &CronPartType,
) -> Result<HashSet<u8>, String> {
    if cron_type == &CronPartType::Numeric && !is_numeric_part(field) {
        return Err(format!(
            "Invalid character in numeric cron field: {}",
            field.chars().find(|c| !is_numeric_char(c)).unwrap_or(' ')
        ));
    }

    let mut values = HashSet::new();

    for part in field.split(',') {
        if part == "*" {
            values.extend(min..=max);
        } else if let Some(step) = part.strip_prefix("*/") {
            let step: u8 = step
                .parse()
                .map_err(|_| format!("Can't parse step value to u8: {step}"))?;
            if step == 0 {
                return Err("Step value can't be 0".to_string());
            }
            values.extend((min..=max).step_by(step as usize));
        } else if part.contains('-') {
            let mut range_parts = part.split('-');
            let start = range_parts.next().unwrap_or_default();
            if start.is_empty() {
                return Err("Can't find start number of range".to_string());
            }
            let start = parse_value(start, cron_type)?;
            let end = range_parts.next().unwrap_or_default();
            if end.is_empty() {
                return Err("Can't find end number of range".to_string());
            }
            let end = parse_value(end, cron_type)?;

            if start > end {
                return Err(
                  format!("The start number of a range must be greater than or equal than the end value: {}>={}", start, end)
              );
            }

            if start < min || end > max {
                return Err(format!(
                    "Only numbers between {} and {} are allowed",
                    min, max
                ));
            }

            values.extend(start..=end);
        } else {
            let value = parse_value(part, cron_type)?;

            if value < min || value > max {
                return Err(format!(
                    "Only numbers between {} and {} are allowed",
                    min, max
                ));
            }

            values.insert(value);
        }
    }

    Ok(values)
}

fn parse_value(value: &str, cron_type: &CronPartType) -> Result<u8, String> {
    Ok(match cron_type {
        CronPartType::Month if !is_numeric_part(value) => {
            let value = value.to_lowercase();
            Month::from_str(&value)? as u8 + 1
        }
        CronPartType::DayOfWeek if !is_numeric_part(value) => {
            let value = value.to_lowercase();
            DayOfWeek::from_str(&value)? as u8
        }
        CronPartType::DayOfWeek if value == "7" => 0,
        _ => value
            .parse::<u8>()
            .map_err(|_| format!("Can't parse value to u8: {value}"))?,
    })
}

fn is_numeric_part(part: &str) -> bool {
    part.chars().all(|c| is_numeric_char(&c))
}

fn is_numeric_char(char: &char) -> bool {
    char.is_ascii_digit() || char == &'*' || char == &',' || char == &'-' || char == &'/'
}

#[cfg(test)]
mod cron_tests {
    use crate::{CronSchedule, DateTime};

    #[test]
    fn iterator() {
        let now = DateTime::from_ymdhms(2022, 1, 1, 0, 0, 0).unwrap();

        let expected = vec!["2022/01/01 00:01:00", "2022/01/01 00:02:00"];
        cron_next("* * * * *", expected, now);

        // Steps
        let expected = vec!["2022/01/01 00:05:00", "2022/01/01 00:10:00"];
        cron_next("*/5 * * * *", expected, now);
        let expected = vec![
            "2022/01/01 05:00:00",
            "2022/01/01 10:00:00",
            "2022/01/01 15:00:00",
            "2022/01/01 20:00:00",
        ];
        cron_next("0 */5 * * *", expected, now);

        // Multiple values
        let expected = vec![
            "2022/01/01 00:04:00",
            "2022/01/01 00:05:00",
            "2022/01/01 00:08:00",
            "2022/01/01 01:04:00",
        ];
        cron_next("4,5,8 * * * *", expected, now);

        // Range
        let expected = vec![
            "2022/01/01 00:09:00",
            "2022/01/01 00:10:00",
            "2022/01/01 00:11:00",
            "2022/01/01 00:12:00",
            "2022/01/01 01:09:00",
        ];
        cron_next("9-12 * * * *", expected, now);

        // Months
        let expected = vec![
            "2022/02/01 00:00:00",
            "2022/03/01 00:00:00",
            "2022/04/01 00:00:00",
            "2022/05/01 00:00:00",
            "2022/06/01 00:00:00",
            "2022/07/01 00:00:00",
            "2022/08/01 00:00:00",
            "2022/09/01 00:00:00",
            "2022/10/01 00:00:00",
            "2022/11/01 00:00:00",
            "2022/12/01 00:00:00",
            "2023/01/01 00:00:00",
        ];
        cron_next(
            "0 0 1 jan,feb,mar,apr,may,jun,jul,aug,sep,oct,nov,dec *",
            expected,
            now,
        );
        let expected = vec![
            "2022/02/01 00:00:00",
            "2022/07/01 00:00:00",
            "2022/12/01 00:00:00",
            "2023/01/01 00:00:00",
        ];
        cron_next("0 0 1 jan,feb,jul,dec *", expected, now);
        assert!(CronSchedule::parse("* * * bla *", Some(now)).is_err());

        // Day of week
        let expected = vec![
            "2022/01/02 00:00:00",
            "2022/01/03 00:00:00",
            "2022/01/04 00:00:00",
            "2022/01/05 00:00:00",
            "2022/01/06 00:00:00",
            "2022/01/07 00:00:00",
            "2022/01/08 00:00:00",
            "2022/01/09 00:00:00",
        ];
        cron_next("0 0 * * sun,mon,tue,wed,thu,fri,sat", expected, now);
        let expected = vec![
            "2022/01/02 00:00:00",
            "2022/01/04 00:00:00",
            "2022/01/08 00:00:00",
            "2022/01/09 00:00:00",
        ];
        cron_next("0 0 * * sun,tue,sat", expected, now);
        assert!(CronSchedule::parse("* * * * bla", Some(now)).is_err());

        // Day of month and day of week combinationes
        let expected = vec![
            "2022/01/03 00:00:00",
            "2022/01/10 00:00:00",
            "2022/01/17 00:00:00",
            "2022/01/20 00:00:00",
        ];
        cron_next("0 0 20 * mon", expected, now);

        // Test if iterator returns none at overflow
        let now = DateTime::from_ymdhms(5_879_611, 7, 12, 23, 59, 0).unwrap();
        cron_next_none("* * * * *", now);
        let now = DateTime::from_ymdhms(5_879_611, 7, 12, 23, 58, 0).unwrap();
        cron_next_none("* * * 8 *", now);
        cron_next_none("* * 13 * *", now);
        cron_next_none("* 0 * * *", now);
        cron_next_none("0 * * * *", now);
    }

    fn cron_next(cron: &str, expected: Vec<&str>, now: DateTime) {
        let mut schedule = CronSchedule::parse(cron, Some(now)).unwrap();

        for expected in expected {
            let next = schedule.next().unwrap();
            assert_eq!(expected, next.format("yyyy/MM/dd HH:mm:ss"));
        }
    }

    fn cron_next_none(cron: &str, now: DateTime) {
        assert!(CronSchedule::parse(cron, Some(now))
            .unwrap()
            .next()
            .is_none());
    }
}
