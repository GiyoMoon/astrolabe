use std::{
    ops::{Add, Sub},
    time::{Duration, SystemTime},
};

use super::{
    convert::{month_to_ymdays, ts_to_d_units},
    leap::is_leap_year,
};
use crate::{DateTime, Unit};

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
            // Use unwrap here because month and day will always be valid
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
                    // Use unwrap here because month will always be between 1 and 12
                    let (_, mdays) = month_to_ymdays(target_year, target_month).unwrap();
                    if day > mdays {
                        mdays
                    } else {
                        day
                    }
                }
            };
            // Use unwrap here because month and day will always be valid
            DateTime::from_ymd(target_year, target_month, target_day).unwrap()
        }
        Unit::Day => {
            let dur = Duration::new(amount * 60 * 60 * 24, 0);
            apply_duration(old, dur, atype)
        }
        Unit::Hour => {
            let dur = Duration::new(amount * 60 * 60, 0);
            apply_duration(old, dur, atype)
        }
        Unit::Min => {
            let dur = Duration::new(amount * 60, 0);
            apply_duration(old, dur, atype)
        }
        Unit::Sec => {
            let dur = Duration::new(amount, 0);
            apply_duration(old, dur, atype)
        }
        Unit::Milli => {
            let dur = Duration::new(0, (amount * 1000000) as u32);
            apply_duration(old, dur, atype)
        }
        Unit::Micro => {
            let dur = Duration::new(0, (amount * 1000) as u32);
            apply_duration(old, dur, atype)
        }
        Unit::Nano => {
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
