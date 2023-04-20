use crate::{
    errors::{invalid_format::create_invalid_format, AstrolabeError},
    util::convert::days_to_wday,
    DateTime, DateTimeUnit,
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

#[derive(Debug, Clone)]
pub struct CronSchedule {
    minutes: HashSet<u8>,
    hours: HashSet<u8>,
    days_of_month: HashSet<u8>,
    months: HashSet<u8>,
    days_of_week: HashSet<u8>,
    last_schedule: Option<DateTime>,
}

impl CronSchedule {
    pub fn parse(string: &str) -> Result<Self, AstrolabeError> {
        let fields: Vec<&str> = string.split_whitespace().collect();

        if fields.len() != 5 {
            return Err(create_invalid_format(
                "Invalid number of cron fields, has to consists of 5 fields".to_string(),
            ));
        }

        let minutes = parse_cron_part(fields[0], 0, 59, &CronPartType::Numeric).map_err(|err| {
            create_invalid_format(format!("Failed parsing minute field: {}", err))
        })?;

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

        Ok(CronSchedule {
            minutes,
            hours,
            days_of_month,
            months,
            days_of_week,
            last_schedule: None,
        })
    }
}

impl Iterator for CronSchedule {
    type Item = DateTime;

    fn next(&mut self) -> Option<Self::Item> {
        // Using unwrap because it's safe to assume that the provided value is valid
        let now = DateTime::now().set(0, DateTimeUnit::Sec).unwrap();
        let last = match self.last_schedule {
            Some(last) if last >= now => last,
            _ => now,
        };

        let mut next = last.apply(1, DateTimeUnit::Min).ok()?;

        let dom_restricted = self.days_of_month.len() != 31;
        let dow_restricted = self.days_of_week.len() != 7;

        loop {
            if next.get(DateTimeUnit::Year) - last.get(DateTimeUnit::Year) > 4 {
                return None;
            }

            if !self.months.contains(&(next.get(DateTimeUnit::Month) as u8)) {
                next = next.apply(1, DateTimeUnit::Month).ok()?;
                continue;
            }

            let day_of_month = next.get(DateTimeUnit::Day) as u8;
            let day_of_week = days_to_wday(next.as_days(), false) as u8;
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
                next = next.apply(1, DateTimeUnit::Day).ok()?;
                continue;
            }

            if !self.hours.contains(&(next.get(DateTimeUnit::Hour) as u8)) {
                next = next.apply(1, DateTimeUnit::Hour).ok()?;
                continue;
            }

            if !self.minutes.contains(&(next.get(DateTimeUnit::Min) as u8)) {
                next = next.apply(1, DateTimeUnit::Min).ok()?;
                continue;
            }

            break;
        }

        self.last_schedule = Some(next);
        Some(next)
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
            let step = step
                .parse()
                .map_err(|_| format!("Can't parse step value to u8: {step}"))?;
            values.extend((min..=max).step_by(step));
        } else if part.contains('-') {
            let mut range_parts = part.split('-');
            let start = range_parts
                .next()
                .ok_or("Can't find start number of range".to_string())?;
            let start = parse_value(start, cron_type)?;
            let end = range_parts
                .next()
                .ok_or("Can't find end number of range".to_string())?;
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
