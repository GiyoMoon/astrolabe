/// Error parsing or formatting [`Date`](crate::Date) struct.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AstrolabeError {
    /// Failed parsing the provided format string.
    InvalidFormat,
    /// Numeric component is out of range.
    OutOfRange,
}

pub(crate) const SECS_PER_MINUTE: u32 = 60;
pub(crate) const SECS_PER_HOUR: u32 = 60 * SECS_PER_MINUTE;
pub(crate) const SECS_PER_DAY: u32 = 24 * SECS_PER_HOUR;

pub(crate) const DAYS_TO_1970: u64 = 719_162;

pub(crate) const MAX_DATE: (i32, u32, u32) = (5_879_611, 7, 12);
pub(crate) const MIN_DATE: (i32, u32, u32) = (-5_879_610, 6, 23);
