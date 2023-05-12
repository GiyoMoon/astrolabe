use super::time::convert::{days_nanos_to_nanos, nanos_to_days_nanos};
use crate::shared::{NANOS_PER_DAY, NANOS_PER_SEC};

/// Adds a given offset to nanoseconds
pub(crate) fn add_offset_to_nanos(nanoseconds: u64, offset: i32) -> u64 {
    (((nanoseconds as i64 + offset as i64 * NANOS_PER_SEC as i64) + NANOS_PER_DAY as i64)
        % NANOS_PER_DAY as i64)
        .unsigned_abs()
}

/// Removes a given offset from nanoseconds
pub(crate) fn remove_offset_from_nanos(nanoseconds: u64, offset: i32) -> u64 {
    (((nanoseconds as i64 - offset as i64 * NANOS_PER_SEC as i64) + NANOS_PER_DAY as i64)
        % NANOS_PER_DAY as i64)
        .unsigned_abs()
}

/// Adds a given offset to days and nanoseconds
pub(crate) fn add_offset_to_dn(days: i32, nanoseconds: u64, offset: i32) -> (i32, u64) {
    let mut nanos = days_nanos_to_nanos(days, nanoseconds);
    nanos += offset as i128 * NANOS_PER_SEC as i128;
    let (days, nanoseconds) = nanos_to_days_nanos(nanos).unwrap();
    (days, nanoseconds)
}

/// Removes a given offset from days and nanoseconds
pub(crate) fn remove_offset_from_dn(days: i32, nanoseconds: u64, offset: i32) -> (i32, u64) {
    let mut nanos = days_nanos_to_nanos(days, nanoseconds);
    nanos -= offset as i128 * NANOS_PER_SEC as i128;
    nanos_to_days_nanos(nanos).unwrap()
}
