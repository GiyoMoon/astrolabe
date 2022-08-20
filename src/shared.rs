/// Error parsing [`DateTime`](crate::DateTime)/[`Date`](crate::Date)/[`Time`](crate::Time) struct.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AstrolabeError {
    /// Numeric component is out of range.
    OutOfRange,
    /// Input string could not be parsed.
    InvalidFormat,
}

/// Used to define if an offset is `UTC+` or `UTC-` (eastern or western hemisphere).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Offset {
    /// Offset in the eastern hemisphere (`UTC±00:00 - UTC+23:59:59`).
    East,
    /// Offset in the western hemisphere (`UTC±00:00 - UTC-23:59:59`).
    West,
}

/// Used for specifing the precision for RFC3339 timestamps.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Precision {
    /// Only seconds -> `2022-05-02T15:30:20Z`
    Seconds = 0,
    /// 2 decimal places -> `2022-05-02T15:30:20.00Z`
    Centis = 2,
    /// 3 decimal places -> `2022-05-02T15:30:20.000Z`
    Millis = 3,
    /// 6 decimal places -> `2022-05-02T15:30:20.000000Z`
    Micros = 6,
    /// 9 decimal places -> `2022-05-02T15:30:20.000000000Z`
    Nanos = 9,
}

pub(crate) const NANOS_PER_SEC: u64 = 1_000_000_000;
pub(crate) const NANOS_PER_DAY: u64 = SECS_PER_DAY_U64 * NANOS_PER_SEC;

pub(crate) const SECS_PER_MINUTE: u32 = 60;
pub(crate) const SECS_PER_HOUR: u32 = 60 * SECS_PER_MINUTE;
pub(crate) const SECS_PER_DAY: u32 = 24 * SECS_PER_HOUR;
pub(crate) const SECS_PER_MINUTE_U64: u64 = 60;
pub(crate) const SECS_PER_HOUR_U64: u64 = 60 * SECS_PER_MINUTE_U64;
pub(crate) const SECS_PER_DAY_U64: u64 = 24 * SECS_PER_HOUR_U64;

pub(crate) const DAYS_TO_1970: u64 = 719_162;
pub(crate) const DAYS_TO_1970_I64: i64 = 719_162;

pub(crate) const MAX_DATE: (i32, u32, u32) = (5_879_611, 7, 12);
pub(crate) const MIN_DATE: (i32, u32, u32) = (-5_879_611, 6, 23);
