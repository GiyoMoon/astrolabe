use crate::util::constants::BUG_MSG;

use super::{cursor::Cursor, errors::TimeZoneError};

/// TZif version
#[derive(Clone, Copy, PartialEq)]
pub(super) enum Version {
    /// Version 1
    V1,
    /// Version 2
    V2,
    /// Version 3
    V3,
}

/// TZif header
pub(super) struct Header {
    /// TZif version
    pub(super) ver: Version,
    /// Number of UT/local indicators
    pub(super) isut_count: usize,
    /// Number of standard/wall indicators
    pub(super) isstd_count: usize,
    /// Number of leap-second records
    pub(super) leap_count: usize,
    /// Number of transition times
    pub(super) transition_count: usize,
    /// Number of local time type records
    pub(super) type_count: usize,
    /// Number of time zone designations bytes
    pub(super) char_count: usize,
}

impl Header {
    /// Parses the TZif header
    pub(super) fn parse(cursor: &mut Cursor) -> Result<Self, TimeZoneError> {
        let magic = cursor.read_exact(4)?;
        if magic != *b"TZif" {
            return Err(TimeZoneError::InvalidTzFile("TZif magic not found"));
        }

        let ver = match cursor.read_exact(1)? {
            [0x00] => Version::V1,
            [0x32] => Version::V2,
            [0x33] => Version::V3,
            _ => {
                return Err(TimeZoneError::UnsupportedTzFile(
                    "TZif version not supported. Only version 1, 2 and 3 are supported",
                ))
            }
        };

        cursor.read_exact(15)?;

        let isut_count = u32::from_be_bytes(cursor.read_exact(4)?.try_into().expect(BUG_MSG));
        let isstd_count = u32::from_be_bytes(cursor.read_exact(4)?.try_into().expect(BUG_MSG));
        let leap_count = u32::from_be_bytes(cursor.read_exact(4)?.try_into().expect(BUG_MSG));
        let transition_count = u32::from_be_bytes(cursor.read_exact(4)?.try_into().expect(BUG_MSG));
        let type_count = u32::from_be_bytes(cursor.read_exact(4)?.try_into().expect(BUG_MSG));
        let char_count = u32::from_be_bytes(cursor.read_exact(4)?.try_into().expect(BUG_MSG));

        Ok(Self {
            ver,
            isut_count: isut_count as usize,
            isstd_count: isstd_count as usize,
            leap_count: leap_count as usize,
            transition_count: transition_count as usize,
            type_count: type_count as usize,
            char_count: char_count as usize,
        })
    }
}
