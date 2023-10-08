use crate::{errors::AstrolabeError, offset::Offset};

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

/// Defines functions to get and manipulate date units.
///
/// Used by [`DateTime`](crate::DateTime) and [`Date`](crate::Date).
pub trait DateUtilities: Sized {
    /// Returns the year.
    fn year(&self) -> i32;
    /// Returns the month of the year (`1-12`).
    fn month(&self) -> u32;
    /// Returns the day of the month (`1-31`).
    fn day(&self) -> u32;
    /// Returns the day of the year (`1-365` or `1-366`).
    fn day_of_year(&self) -> u32;
    /// Returns the day of the week (`0-6`, `0` is Sunday).
    fn weekday(&self) -> u8;

    /// Creates a date from a unix timestamp (non-leap seconds since January 1, 1970 00:00:00 UTC).
    ///
    /// Panics if the provided timestamp would result in an out of range date.
    fn from_timestamp(timestamp: i64) -> Self;
    /// Returns the number of non-leap seconds since January 1, 1970 00:00:00 UTC. (Negative if date is before)
    fn timestamp(&self) -> i64;

    /// Sets the year to the provided value. Has to be in range `-5879611..=5879611`.
    fn set_year(&self, year: i32) -> Result<Self, AstrolabeError>;
    /// Sets the month of the year to the provided value. Has to be in range `1..=12`.
    ///
    /// Returns an [`OutOfRange`](AstrolabeError::OutOfRange) error if the provided value is out of range.
    fn set_month(&self, month: u32) -> Result<Self, AstrolabeError>;
    /// Sets the day of the month to the provided value. Has to be in range `1..=31` and cannot be greater than the number of days in the current month.
    ///
    /// Returns an [`OutOfRange`](AstrolabeError::OutOfRange) error if the provided value is out of range.
    fn set_day(&self, day: u32) -> Result<Self, AstrolabeError>;
    /// Sets the day of the year to the provided value. Has to be in range `1..=365` or `1..=366` in case of a leap year.
    ///
    /// Returns an [`OutOfRange`](AstrolabeError::OutOfRange) error if the provided value is out of range.
    fn set_day_of_year(&self, day_of_year: u32) -> Result<Self, AstrolabeError>;

    /// Adds the provided years to the current date.
    ///
    /// Panics if the provided value would result in an out of range date.
    fn add_years(&self, years: u32) -> Self;
    /// Adds the provided months to the current date.
    ///
    /// Panics if the provided value would result in an out of range date.
    fn add_months(&self, months: u32) -> Self;
    /// Adds the provided days to the current date.
    ///
    /// Panics if the provided value would result in an out of range date.
    fn add_days(&self, days: u32) -> Self;

    /// Subtracts the provided years from the current date.
    ///
    /// Panics if the provided value would result in an out of range date.
    fn sub_years(&self, years: u32) -> Self;
    /// Subtracts the provided months from the current date.
    ///
    /// Panics if the provided value would result in an out of range date.
    fn sub_months(&self, months: u32) -> Self;
    /// Subtracts the provided days from the current date.
    ///
    /// Panics if the provided value would result in an out of range date.
    fn sub_days(&self, days: u32) -> Self;

    /// Clears date/time units until the year (inclusive).
    fn clear_until_year(&self) -> Self;
    /// Clears date/time units until the month (inclusive).
    fn clear_until_month(&self) -> Self;
    /// Clears date/time units until the day (inclusive).
    fn clear_until_day(&self) -> Self;

    /// Returns full years since the provided date.
    fn years_since(&self, compare: &Self) -> i32;
    /// Returns full months since the provided date.
    fn months_since(&self, compare: &Self) -> i32;
    /// Returns full days since the provided date.
    fn days_since(&self, compare: &Self) -> i64;
}

/// Defines functions to get and manipulate time units.
///
/// Used by [`DateTime`](crate::DateTime) and [`Time`](crate::Time).
pub trait TimeUtilities: Sized {
    /// Returns the hour (`0-23`).
    fn hour(&self) -> u32;
    /// Returns the minute of the hour (`0-59`).
    fn minute(&self) -> u32;
    /// Returns the second of the minute (`0-59`).
    fn second(&self) -> u32;
    /// Returns the millisecond of the second (`0-999`).
    fn milli(&self) -> u32;
    /// Returns the microsecond of the second (`0-999_999`).
    fn micro(&self) -> u32;
    /// Returns the nanosecond of the second (`0-999_999_999`).
    fn nano(&self) -> u32;

    /// Sets the hour to the provided value. Has to be in range `0..=23`.
    ///
    /// Returns an [`OutOfRange`](AstrolabeError::OutOfRange) error if the provided value is out of range.
    fn set_hour(&self, hour: u32) -> Result<Self, AstrolabeError>;
    /// Sets the minute to the provided value. Has to be in range `0..=59`.
    ///
    /// Returns an [`OutOfRange`](AstrolabeError::OutOfRange) error if the provided value is out of range.
    fn set_minute(&self, minute: u32) -> Result<Self, AstrolabeError>;
    /// Sets the second to the provided value. Has to be in range `0..=59`.
    ///
    /// Returns an [`OutOfRange`](AstrolabeError::OutOfRange) error if the provided value is out of range.
    fn set_second(&self, second: u32) -> Result<Self, AstrolabeError>;
    /// Sets the millisecond to the provided value. Has to be in range `0..=100`.
    ///
    /// Returns an [`OutOfRange`](AstrolabeError::OutOfRange) error if the provided value is out of range.
    fn set_milli(&self, milli: u32) -> Result<Self, AstrolabeError>;
    /// Sets the microsecond to the provided value. Has to be in range `0..=100_000`.
    ///
    /// Returns an [`OutOfRange`](AstrolabeError::OutOfRange) error if the provided value is out of range.
    fn set_micro(&self, micro: u32) -> Result<Self, AstrolabeError>;
    /// Sets the nanosecond to the provided value. Has to be in range `0..=100_000_000`.
    ///
    /// Returns an [`OutOfRange`](AstrolabeError::OutOfRange) error if the provided value is out of range.
    fn set_nano(&self, nano: u32) -> Result<Self, AstrolabeError>;

    /// Adds the provided hours.
    fn add_hours(&self, hours: u32) -> Self;
    /// Adds the provided minutes.
    fn add_minutes(&self, minutes: u32) -> Self;
    /// Adds the provided seconds.
    fn add_seconds(&self, seconds: u32) -> Self;
    /// Adds the provided milliseconds.
    fn add_millis(&self, millis: u32) -> Self;
    /// Adds the provided microseconds.
    fn add_micros(&self, micros: u32) -> Self;
    /// Adds the provided nanoseconds.
    fn add_nanos(&self, nanos: u32) -> Self;

    /// Subtracts the provided hours.
    fn sub_hours(&self, hours: u32) -> Self;
    /// Subtracts the provided minutes.
    fn sub_minutes(&self, minutes: u32) -> Self;
    /// Subtracts the provided seconds.
    fn sub_seconds(&self, seconds: u32) -> Self;
    /// Subtracts the provided milliseconds.
    fn sub_millis(&self, millis: u32) -> Self;
    /// Subtracts the provided microseconds.
    fn sub_micros(&self, micros: u32) -> Self;
    /// Subtracts the provided nanoseconds.
    fn sub_nanos(&self, nanos: u32) -> Self;

    /// Clears date/time units until the hour (inclusive).
    fn clear_until_hour(&self) -> Self;
    /// Clears date/time units until the minute (inclusive).
    fn clear_until_minute(&self) -> Self;
    /// Clears date/time units until the second (inclusive).
    fn clear_until_second(&self) -> Self;
    /// Clears date/time units until the millisecond (inclusive).
    fn clear_until_milli(&self) -> Self;
    /// Clears date/time units until the microsecond (inclusive).
    fn clear_until_micro(&self) -> Self;
    /// Clears date/time units until the nanosecond (inclusive).
    fn clear_until_nano(&self) -> Self;

    /// Return type for the `hour_`, `minutes_` and `seconds_since` functions. Is `i32` for [`Time`](crate::Time) and `i64` for [`DateTime`](crate::DateTime).
    type SubDayReturn;
    /// Returns full hours since the provided time.
    fn hours_since(&self, compare: &Self) -> Self::SubDayReturn;
    /// Returns full minutes since the provided time.
    fn minutes_since(&self, compare: &Self) -> Self::SubDayReturn;
    /// Returns full seconds since the provided time.
    fn seconds_since(&self, compare: &Self) -> Self::SubDayReturn;

    /// Return type for the `millis_`, `micros_` and `manos_since` functions. Is `i64` for [`Time`](crate::Time) and `i128` for [`DateTime`](crate::DateTime).
    type SubSecReturn;
    /// Returns full milliseconds since the provided time.
    fn millis_since(&self, compare: &Self) -> Self::SubSecReturn;
    /// Returns full microseconds since the provided time.
    fn micros_since(&self, compare: &Self) -> Self::SubSecReturn;
    /// Returns full nanoseconds since the provided time.
    fn nanos_since(&self, compare: &Self) -> Self::SubSecReturn;
}

/// Defines functions to get and manipulate the offset.
///
/// Offset can range anywhere from `UTC-23:59:59` to `UTC+23:59:59`.
/// The offset affects all `format`, `get` and `set` functions.
/// Used by [`DateTime`](crate::DateTime) and [`Time`](crate::Time).
pub trait OffsetUtilities: Sized {
    /// Sets the offset
    ///
    /// Examples:
    /// - `UTC+1` is `set_offset(Offset::Fixed(3600))`
    /// - `UTC-1` is `set_offset(Offset::Fixed(-3600))`
    /// - To set the offset to the local system timezone, use `set_offset(Offset::Local)`
    fn set_offset(&self, offset: Offset) -> Self;
    /// Sets the offset, assuming the current instance has the provided offset applied. The new instance will have the specified offset and the datetime itself will be converted to `UTC`.
    ///
    /// Examples:
    /// - `UTC+1` is `as_offset(Offset::Fixed(3600))`
    /// - `UTC-1` is `as_offset(Offset::Fixed(-3600))`.
    /// - To set the offset to the local system timezone, use `as_offset(Offset::Local)`
    fn as_offset(&self, offset: Offset) -> Self;
    /// Returns the offset
    fn get_offset(&self) -> Offset;
}
