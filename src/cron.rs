use crate::{
    errors::{invalid_format::create_invalid_format, AstrolabeError},
    shared::{MONTH_ABBREVIATED_LOWER, WDAY_ABBREVIATED_LOWER},
    util::convert::days_to_wday,
    DateTime, DateTimeUnit,
};
use std::collections::HashSet;

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
        let components: Vec<&str> = string.split_whitespace().collect();

        if components.len() != 5 {
            return Err(create_invalid_format(
                "Invalid count of components for cron string, has to consists of 5 parts."
                    .to_string(),
            ));
        }

        let minutes = parse_cron_part(components[0], 0, 59, &CronPartType::Numeric).expect("TODO");

        let hours = parse_cron_part(components[1], 0, 23, &CronPartType::Numeric).expect("TODO");

        let days_of_month =
            parse_cron_part(components[2], 1, 31, &CronPartType::Numeric).expect("TODO");

        let months = parse_cron_part(components[3], 1, 12, &CronPartType::Month).expect("TODO");

        let days_of_week =
            parse_cron_part(components[4], 0, 6, &CronPartType::DayOfWeek).expect("TODO");

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

        let mut next = last
            .apply(1, DateTimeUnit::Min)
            .expect("TODO Date out of range");

        let dom_restricted = self.days_of_month.len() != 31;
        let dow_restricted = self.days_of_week.len() != 7;

        loop {
            if next.get(DateTimeUnit::Year) - last.get(DateTimeUnit::Year) > 4 {
                return None;
            }

            if !self.months.contains(&(next.get(DateTimeUnit::Month) as u8)) {
                next = next
                    .apply(1, DateTimeUnit::Month)
                    .expect("TODO Date out of range");
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
                next = next
                    .apply(1, DateTimeUnit::Day)
                    .expect("TODO Date out of range");
                continue;
            }

            if !self.hours.contains(&(next.get(DateTimeUnit::Hour) as u8)) {
                next = next
                    .apply(1, DateTimeUnit::Hour)
                    .expect("TODO Date out of range");
                continue;
            }

            if !self.minutes.contains(&(next.get(DateTimeUnit::Min) as u8)) {
                next = next
                    .apply(1, DateTimeUnit::Min)
                    .expect("TODO Date out of range");
                continue;
            }

            break;
        }

        self.last_schedule = Some(next);
        Some(next)
    }
}

fn parse_cron_part(
    component: &str,
    min: u8,
    max: u8,
    cron_type: &CronPartType,
) -> Result<HashSet<u8>, String> {
    if cron_type == &CronPartType::Numeric && !is_numeric_part(component) {
        return Err(format!(
            "Invalid character in numeric cron component: {}",
            component
                .chars()
                .find(|c| !is_numeric_char(c))
                .unwrap_or(' ')
        ));
    }

    let mut values = HashSet::new();

    for part in component.split(',') {
        if part == "*" {
            values.extend(min..=max);
        } else if let Some(step) = part.strip_prefix("*/") {
            values.extend((min..=max).step_by(step.parse().expect("TODO")));
        } else if part.contains('-') {
            let mut range_parts = part.split('-');
            let start = parse_value(range_parts.next().expect("TODO"), cron_type)?;
            let end = parse_value(range_parts.next().expect("TODO"), cron_type)?;

            if start > end {
                return Err(
                    "The start value of a range must be greater than or equal than the end value."
                        .to_string(),
                );
            }

            if start < min || end > max {
                return Err(format!(
                    "Component only allows numeric values between {} and {}.",
                    min, max
                ));
            }

            values.extend(start..=end);
        } else {
            let value = parse_value(part, cron_type)?;

            if value < min || value > max {
                return Err(format!(
                    "Component only allows numeric values between {} and {}.",
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
            MONTH_ABBREVIATED_LOWER
                .iter()
                .position(|&month| month == value)
                .expect("TODO") as u8
                + 1
        }
        CronPartType::DayOfWeek if !is_numeric_part(value) => {
            let value = value.to_lowercase();
            WDAY_ABBREVIATED_LOWER
                .iter()
                .position(|&day_of_week| day_of_week == value)
                .expect("TODO") as u8
        }
        CronPartType::DayOfWeek if value == "7" => 0,
        _ => value.parse::<u8>().expect("TODO"),
    })
}

fn is_numeric_part(part: &str) -> bool {
    part.chars().all(|c| is_numeric_char(&c))
}

fn is_numeric_char(char: &char) -> bool {
    char.is_ascii_digit() || char == &'*' || char == &',' || char == &'-' || char == &'/'
}
