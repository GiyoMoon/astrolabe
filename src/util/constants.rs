pub(crate) const NANOS_PER_SEC: u64 = 1_000_000_000;
pub(crate) const NANOS_PER_MINUTE: u64 = SECS_PER_MINUTE_U64 * NANOS_PER_SEC;
pub(crate) const NANOS_PER_HOUR: u64 = SECS_PER_HOUR_U64 * NANOS_PER_SEC;
pub(crate) const NANOS_PER_DAY: u64 = SECS_PER_DAY_U64 * NANOS_PER_SEC;

pub(crate) const SECS_PER_MINUTE: u32 = 60;
pub(crate) const SECS_PER_MINUTE_U64: u64 = 60;

pub(crate) const SECS_PER_HOUR: u32 = 60 * SECS_PER_MINUTE;
pub(crate) const SECS_PER_HOUR_U64: u64 = 60 * SECS_PER_MINUTE_U64;

pub(crate) const SECS_PER_DAY: u32 = 24 * SECS_PER_HOUR;
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

pub(crate) const BUG_MSG: &str = "This shouldn't happen. Please report this bug on GitHub (https://github.com/GiyoMoon/astrolabe/issues). Thanks!";
