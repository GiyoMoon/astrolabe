use super::{
    convert::{month_to_ymdays, ts_to_d_units, SECS_PER_DAY, SECS_PER_HOUR, SECS_PER_MINUTE},
    leap::is_leap_year,
};
use crate::{DateTime, Unit};
use std::{
    ops::{Add, Sub},
    time::{Duration, SystemTime},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ApplyType {
    Add,
    Sub,
}

/// Applies (add/remove)  a specified [`Unit`] to [`DateTime`]
pub(crate) fn apply_unit(old: &DateTime, amount: u64, unit: Unit, atype: ApplyType) -> DateTime {
    match unit {
        Unit::Year => {
            let (year, month, mut day) = ts_to_d_units(old.timestamp());
            if is_leap_year(year) && month == 2 && day == 29 {
                day = 28;
            }
            let target_year = match atype {
                ApplyType::Add => year + amount,
                ApplyType::Sub => year - amount,
            };
            // Using unwrap because it's safe to assume that month and day is valid
            DateTime::from_ymd(target_year, month, day).unwrap()
        }
        Unit::Month => {
            let (year, month, day) = ts_to_d_units(old.timestamp());
            let target_year = match atype {
                ApplyType::Add => (year * 12 + month + amount - 1) / 12,
                ApplyType::Sub => (year * 12 + month - amount - 1) / 12,
            };
            let target_month = match atype {
                ApplyType::Add => {
                    if (month + amount) % 12 == 0 {
                        12
                    } else {
                        (month + amount) % 12
                    }
                }
                ApplyType::Sub => {
                    if (month - amount) % 12 == 0 {
                        12
                    } else {
                        (month - amount) % 12
                    }
                }
            };
            let target_day = match day {
                day if day < 29 => day,
                _ => {
                    // Using unwrap because it's safe to assume that month is valid
                    let (_, mdays) = month_to_ymdays(target_year, target_month).unwrap();
                    if day > mdays {
                        mdays
                    } else {
                        day
                    }
                }
            };
            // Using unwrap because it's safe to assume that month and day is valid
            DateTime::from_ymd(target_year, target_month, target_day).unwrap()
        }
        Unit::Day => {
            let dur = Duration::new(amount * SECS_PER_DAY, 0);
            apply_duration(old, dur, atype)
        }
        Unit::Hour => {
            let dur = Duration::new(amount * SECS_PER_HOUR, 0);
            apply_duration(old, dur, atype)
        }
        Unit::Min => {
            let dur = Duration::new(amount * SECS_PER_MINUTE, 0);
            apply_duration(old, dur, atype)
        }
        Unit::Sec => {
            let dur = Duration::new(amount, 0);
            apply_duration(old, dur, atype)
        }
        Unit::Centis => {
            let dur = Duration::new(0, amount as u32 * 10000000);
            apply_duration(old, dur, atype)
        }
        Unit::Millis => {
            let dur = Duration::new(0, amount as u32 * 1000000);
            apply_duration(old, dur, atype)
        }
        Unit::Micros => {
            let dur = Duration::new(0, amount as u32 * 1000);
            apply_duration(old, dur, atype)
        }
        Unit::Nanos => {
            let dur = Duration::new(0, amount as u32);
            apply_duration(old, dur, atype)
        }
    }
}

/// Applies (add/remove) a specified [`Duration`] to [`DateTime`]
pub(crate) fn apply_duration(old: &DateTime, duration: Duration, atype: ApplyType) -> DateTime {
    match atype {
        ApplyType::Add => DateTime::from(SystemTime::from(old).add(duration)),
        ApplyType::Sub => DateTime::from(SystemTime::from(old).sub(duration)),
    }
}
