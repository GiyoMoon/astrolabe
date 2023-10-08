use super::errors::TimeZoneError;

/// Helper to read data from a byte slice
pub(super) struct Cursor<'a> {
    /// Slice representing the remaining data to be read
    remaining: &'a [u8],
}

impl<'a> Cursor<'a> {
    pub(super) fn new(bytes: &'a [u8]) -> Self {
        Self { remaining: bytes }
    }

    pub(super) fn read_exact(&mut self, len: usize) -> Result<&'a [u8], TimeZoneError> {
        if self.remaining.len() < len {
            return Err(TimeZoneError::Cursor("End of byte slice reached"));
        }
        let (data, remaining) = self.remaining.split_at(len);
        self.remaining = remaining;
        Ok(data)
    }

    pub(super) fn read_until(&mut self, char: char) -> &'a [u8] {
        let mut index = 0;
        for byte in self.remaining.iter() {
            if *byte == char as u8 {
                break;
            }
            index += 1;
        }
        let (data, remaining) = self.remaining.split_at(index);
        self.remaining = remaining;
        data
    }

    pub(super) fn read_while(&mut self, until: impl Fn(&u8) -> bool) -> &'a [u8] {
        let mut index = 0;
        for byte in self.remaining.iter() {
            if !until(byte) {
                break;
            }
            index += 1;
        }
        let (data, remaining) = self.remaining.split_at(index);
        self.remaining = remaining;
        data
    }

    pub(super) fn read_tag(&mut self, bytes: &[u8]) -> Result<&'a [u8], TimeZoneError> {
        if self.remaining.len() < bytes.len() {
            return Err(TimeZoneError::Cursor("End of byte slice reached"));
        }
        let (data, remaining) = self.remaining.split_at(bytes.len());
        if data != bytes {
            return Err(TimeZoneError::Cursor("Unexpected bytes read"));
        }
        self.remaining = remaining;
        Ok(data)
    }

    pub(super) fn remaining(&self) -> &'a [u8] {
        self.remaining
    }

    pub(super) fn empty(&self) -> bool {
        self.remaining.is_empty()
    }

    pub(super) fn get_next(&self) -> Result<u8, TimeZoneError> {
        if self.remaining.is_empty() {
            return Err(TimeZoneError::Cursor("End of byte slice reached"));
        }
        Ok(self.remaining[0])
    }
}
