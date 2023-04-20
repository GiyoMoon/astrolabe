/// Used to define if an offset is `UTC+` or `UTC-` (eastern or western hemisphere).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Offset {
    /// Offset in the eastern hemisphere (`UTC±00:00 - UTC+23:59:59`).
    East,
    /// Offset in the western hemisphere (`UTC±00:00 - UTC-23:59:59`).
    West,
}

/// Used for specifing the precision for RFC 3339 timestamps.
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

/// year, month, day of month, day of year
pub(crate) const MAX_DATE: (i32, u32, u32, u32) = (5_879_611, 7, 12, 193);
/// year, month, day of month, day of year
pub(crate) const MIN_DATE: (i32, u32, u32, u32) = (-5_879_611, 6, 23, 174);

pub(crate) const MONTH_ABBREVIATED: [&str; 12] = [
    "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];

pub(crate) const MONTH_WIDE: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];
pub(crate) const MONTH_NARROW: [&str; 12] =
    ["J", "F", "M", "A", "M", "J", "J", "A", "S", "O", "N", "D"];

pub(crate) const WDAY_ABBREVIATED: [&str; 7] = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];

pub(crate) const WDAY_WIDE: [&str; 7] = [
    "Sunday",
    "Monday",
    "Tuesday",
    "Wednesday",
    "Thursday",
    "Friday",
    "Saturday",
];
pub(crate) const WDAY_NARROW: [&str; 7] = ["S", "M", "T", "W", "T", "F", "S"];
pub(crate) const WDAY_SHORT: [&str; 7] = ["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"];

#[cfg(feature = "cron")]
pub(crate) const MONTH_ABBREVIATED_LOWER: [&str; 12] = [
    "jan", "feb", "mar", "apr", "may", "jun", "jul", "aug", "sep", "oct", "nov", "dec",
];
#[cfg(feature = "cron")]
pub(crate) const WDAY_ABBREVIATED_LOWER: [&str; 7] =
    ["sun", "mon", "tue", "wed", "thu", "fri", "sat"];
