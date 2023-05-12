use super::convert::{nanos_to_time, nanos_to_unit};
use crate::{
    errors::{out_of_range::create_conditional_oor, AstrolabeError},
    shared::{
        NANOS_PER_SEC, SECS_PER_HOUR, SECS_PER_HOUR_U64, SECS_PER_MINUTE, SECS_PER_MINUTE_U64,
    },
    TimeUnit,
};

/// Applies (add/subtract) a specified [`TimeUnit`] to nanoseconds
pub(crate) fn apply_time_unit(old_nanos: i128, amount: i64, unit: TimeUnit) -> i128 {
    match unit {
        TimeUnit::Hour => {
            let amount_nanos = amount as i128 * SECS_PER_HOUR_U64 as i128 * NANOS_PER_SEC as i128;

            old_nanos + amount_nanos
        }
        TimeUnit::Min => {
            let amount_nanos = amount as i128 * SECS_PER_MINUTE_U64 as i128 * NANOS_PER_SEC as i128;

            old_nanos + amount_nanos
        }
        TimeUnit::Sec => {
            let amount_nanos = amount as i128 * NANOS_PER_SEC as i128;

            old_nanos + amount_nanos
        }
        TimeUnit::Centis => {
            let amount_nanos = amount as i128 * 10_000_000;

            old_nanos + amount_nanos
        }
        TimeUnit::Millis => {
            let amount_nanos = amount as i128 * 1_000_000;

            old_nanos + amount_nanos
        }
        TimeUnit::Micros => {
            let amount_nanos = amount as i128 * 1_000;

            old_nanos + amount_nanos
        }
        TimeUnit::Nanos => {
            let amount_nanos = amount as i128;

            old_nanos + amount_nanos
        }
    }
}

/// Sets a specific [`TimeUnit`] to the provided value
pub(crate) fn set_time_unit(
    old_nanos: u64,
    value: u32,
    unit: TimeUnit,
) -> Result<u64, AstrolabeError> {
    Ok(match unit {
        TimeUnit::Hour => {
            if value > 23 {
                return Err(create_conditional_oor(
                    "value",
                    0,
                    23,
                    value as i128,
                    "because unit is \"Hour\"".to_string(),
                ));
            }
            let (_, min, sec) = nanos_to_time(old_nanos);

            (value * SECS_PER_HOUR + min * SECS_PER_MINUTE + sec) as u64 * NANOS_PER_SEC
                + nanos_to_unit(old_nanos, TimeUnit::Nanos)
        }
        TimeUnit::Min => {
            if value > 59 {
                return Err(create_conditional_oor(
                    "value",
                    0,
                    59,
                    value as i128,
                    "because unit is \"Min\"".to_string(),
                ));
            }
            let (hour, _, sec) = nanos_to_time(old_nanos);

            (hour * SECS_PER_HOUR + value * SECS_PER_MINUTE + sec) as u64 * NANOS_PER_SEC
                + nanos_to_unit(old_nanos, TimeUnit::Nanos)
        }
        TimeUnit::Sec => {
            if value > 59 {
                return Err(create_conditional_oor(
                    "value",
                    0,
                    59,
                    value as i128,
                    "because unit is \"Sec\"".to_string(),
                ));
            }
            let (hour, min, _) = nanos_to_time(old_nanos);

            (hour * SECS_PER_HOUR + min * SECS_PER_MINUTE + value) as u64 * NANOS_PER_SEC
                + nanos_to_unit(old_nanos, TimeUnit::Nanos)
        }
        TimeUnit::Centis => {
            if value > 99 {
                return Err(create_conditional_oor(
                    "value",
                    0,
                    99,
                    value as i128,
                    "because unit is \"Centis\"".to_string(),
                ));
            }

            old_nanos / NANOS_PER_SEC * NANOS_PER_SEC
                + value as u64 * 10_000_000
                + old_nanos % 10_000_000
        }
        TimeUnit::Millis => {
            if value > 999 {
                return Err(create_conditional_oor(
                    "value",
                    0,
                    999,
                    value as i128,
                    "because unit is \"Millis\"".to_string(),
                ));
            }

            old_nanos / NANOS_PER_SEC * NANOS_PER_SEC
                + value as u64 * 1_000_000
                + old_nanos % 1_000_000
        }
        TimeUnit::Micros => {
            if value > 999_999 {
                return Err(create_conditional_oor(
                    "value",
                    0,
                    999_999,
                    value as i128,
                    "because unit is \"Micros\"".to_string(),
                ));
            }

            old_nanos / NANOS_PER_SEC * NANOS_PER_SEC + value as u64 * 1_000 + old_nanos % 1_000
        }
        TimeUnit::Nanos => {
            if value > 999_999_999 {
                return Err(create_conditional_oor(
                    "value",
                    0,
                    999_999_999,
                    value as i128,
                    "because unit is \"Nanos\"".to_string(),
                ));
            }

            old_nanos / NANOS_PER_SEC * NANOS_PER_SEC + value as u64
        }
    })
}
