use super::{
    cursor::Cursor,
    errors::TimeZoneError,
    header::{Header, Version},
};

/// TZif data block
pub(super) struct DataBlock<'a> {
    /// Size in bytes of each transition time.
    /// 4 bytes for version 1, 8 bytes for version 2 and 3 TZif files
    pub(super) time_size: usize,
    /// Transition times at which the rules for computing local time change
    pub(super) transition_times: &'a [u8],
    /// Index into the local time types array
    pub(super) transition_types: &'a [u8],
    /// Local time types specifying UTC offsets and DST
    pub(super) local_time_types: &'a [u8],
    _time_zone_designations: &'a [u8],
    _leap_seconds: &'a [u8],
    _standard_wall: &'a [u8],
    _ut_local: &'a [u8],
}

impl<'a> DataBlock<'a> {
    /// Parse a TZif data block
    pub(super) fn parse(
        cursor: &mut Cursor<'a>,
        header: &Header,
        version: Version,
    ) -> Result<Self, TimeZoneError> {
        let time_size = match version {
            Version::V1 => 4,
            Version::V2 | Version::V3 => 8,
        };

        Ok(Self {
            time_size,
            transition_times: cursor.read_exact(header.transition_count * time_size)?,
            transition_types: cursor.read_exact(header.transition_count)?,
            local_time_types: cursor.read_exact(header.type_count * 6)?,
            _time_zone_designations: cursor.read_exact(header.char_count)?,
            _leap_seconds: cursor.read_exact(header.leap_count * (time_size + 4))?,
            _standard_wall: cursor.read_exact(header.isstd_count)?,
            _ut_local: cursor.read_exact(header.isut_count)?,
        })
    }
}
