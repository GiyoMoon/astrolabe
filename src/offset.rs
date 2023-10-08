use std::fs;

use crate::{
    errors::{out_of_range::create_simple_oor, AstrolabeError},
    local::timezone::TimeZone,
    util::{
        constants::{SECS_PER_DAY, SECS_PER_HOUR, SECS_PER_MINUTE},
        time::convert::time_to_day_seconds,
    },
    DateTime, DateUtilities,
};

/// Represents an offset from UTC
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Offset {
    /// Fixed offset in seconds
    Fixed(i32),
    /// Local system timezone. Only works on UNIX systems. On other systems, this is equivalent to `Fixed(0)`.
    Local,
}

impl Offset {
    /// Creates a fixed offset from seconds
    ///
    /// Returns an [`OutOfRange`](AstrolabeError::OutOfRange) error if the provided seconds are not the range `-86_399..=86_399` (`UTC-23:59:59-UTC+23:59:59`).
    pub fn from_seconds(seconds: i32) -> Result<Self, AstrolabeError> {
        if seconds <= -(SECS_PER_DAY as i32) || seconds >= SECS_PER_DAY as i32 {
            return Err(create_simple_oor(
                "seconds",
                -(SECS_PER_DAY as i128) + 1,
                SECS_PER_DAY as i128 - 1,
                seconds as i128,
            ));
        }

        Ok(Self::Fixed(seconds))
    }

    /// Creates a fixed offset from hours, minutes and seconds.
    ///
    /// Returns an [`OutOfRange`](AstrolabeError::OutOfRange) error if the provided offset is not between `UTC-23:59:59` and `UTC+23:59:59`.
    pub fn from_hms(hour: i32, minute: u32, second: u32) -> Result<Self, AstrolabeError> {
        let mut seconds = time_to_day_seconds(hour.unsigned_abs(), minute, second)? as i32;
        seconds = if hour.is_negative() {
            -seconds
        } else {
            seconds
        };

        Ok(Self::Fixed(seconds))
    }

    /// Resolves the offset to seconds from UTC
    pub fn resolve(self) -> i32 {
        match self {
            Self::Fixed(offset) => offset,
            Self::Local => {
                #[cfg(not(unix))]
                return 0;
                #[cfg(unix)]
                return {
                    let result = fs::read("/etc/localtime");
                    match result {
                        Ok(bytes) => {
                            TimeZone::from_tzif(&bytes)
                                .unwrap()
                                .to_local_time_type(DateTime::now().timestamp())
                                .utoff
                        }
                        Err(_) => 0,
                    }
                };
            }
        }
    }

    /// Returns the offset as hours, minutes and seconds.
    pub fn resolve_hms(self) -> (i32, u32, u32) {
        let offset_seconds = self.resolve();

        let hour = offset_seconds / SECS_PER_HOUR as i32;
        let minute = offset_seconds % SECS_PER_HOUR as i32 / SECS_PER_MINUTE as i32;
        let second = offset_seconds % SECS_PER_MINUTE as i32;

        (hour, minute.unsigned_abs(), second.unsigned_abs())
    }
}

impl Default for Offset {
    fn default() -> Self {
        Self::Fixed(0)
    }
}
