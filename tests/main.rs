mod format;

#[cfg(test)]
mod tests {
    use std::time::{SystemTime, UNIX_EPOCH};

    use astrolabe::{DateTime, Precision};

    #[test]
    fn from_ymd() {
        // check allowed limits
        from_ymd_ok(0, 1970, 1, 1);
        from_ymd_ok(28857600, 1970, 12, 1);
        from_ymd_ok(31449600, 1970, 12, 31);
        from_ymd_ok(946684800, 2000, 1, 1);
        from_ymd_ok(5011200, 1970, 2, 28);
        from_ymd_ok(68169600, 1972, 2, 29);

        // check invalid limits
        from_ymd_err(1969, 1, 1);
        from_ymd_err(1970, 0, 1);
        from_ymd_err(1970, 1, 0);
        from_ymd_err(1970, 1, 32);
        from_ymd_err(1970, 2, 29);
        from_ymd_err(1972, 2, 30);
    }

    fn from_ymd_ok(expected: u64, year: u64, month: u64, day: u64) {
        assert_eq!(
            expected,
            SystemTime::from(DateTime::from_ymd(year, month, day).unwrap())
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
        );
    }

    fn from_ymd_err(year: u64, month: u64, day: u64) {
        assert!(DateTime::from_ymd(year, month, day).is_err());
    }

    #[test]
    fn from_ymdhms() {
        // check allowed limits
        from_ymdhms_ok(0, 1970, 1, 1, 0, 0, 0);
        from_ymdhms_ok(28857600, 1970, 12, 1, 0, 0, 0);
        from_ymdhms_ok(31449600, 1970, 12, 31, 0, 0, 0);
        from_ymdhms_ok(946684800, 2000, 1, 1, 0, 0, 0);
        from_ymdhms_ok(5011200, 1970, 2, 28, 0, 0, 0);
        from_ymdhms_ok(68169600, 1972, 2, 29, 0, 0, 0);

        from_ymdhms_ok(82800, 1970, 1, 1, 23, 0, 0);
        from_ymdhms_ok(3540, 1970, 1, 1, 0, 59, 0);
        from_ymdhms_ok(59, 1970, 1, 1, 0, 0, 59);
        from_ymdhms_ok(86399, 1970, 1, 1, 23, 59, 59);

        // check invalid limits
        from_ymdhms_err(1969, 1, 1, 0, 0, 0);
        from_ymdhms_err(1970, 0, 1, 0, 0, 0);
        from_ymdhms_err(1970, 1, 0, 0, 0, 0);
        from_ymdhms_err(1970, 1, 32, 0, 0, 0);
        from_ymdhms_err(1970, 2, 29, 0, 0, 0);
        from_ymdhms_err(1972, 2, 30, 0, 0, 0);

        from_ymdhms_err(1970, 1, 1, 24, 0, 0);
        from_ymdhms_err(1970, 1, 1, 0, 60, 0);
        from_ymdhms_err(1970, 1, 1, 0, 0, 60);
    }

    fn from_ymdhms_ok(
        expected: u64,
        year: u64,
        month: u64,
        day: u64,
        hour: u64,
        minute: u64,
        second: u64,
    ) {
        assert_eq!(
            expected,
            SystemTime::from(
                DateTime::from_ymdhms(year, month, day, hour, minute, second).unwrap()
            )
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
        );
    }

    fn from_ymdhms_err(year: u64, month: u64, day: u64, hour: u64, minute: u64, second: u64) {
        assert!(DateTime::from_ymdhms(year, month, day, hour, minute, second).is_err());
    }

    #[test]
    fn format_rfc3339() {
        let date_time = DateTime::from_ymdhms(1970, 1, 1, 0, 0, 0).unwrap();
        assert_eq!(
            "1970-01-01T00:00:00Z",
            date_time.format_rfc3339(Precision::Seconds)
        );
        assert_eq!(
            "1970-01-01T00:00:00.000Z",
            date_time.format_rfc3339(Precision::Millis)
        );
        assert_eq!(
            "1970-01-01T00:00:00.000000Z",
            date_time.format_rfc3339(Precision::Micros)
        );
        assert_eq!(
            "1970-01-01T00:00:00.000000000Z",
            date_time.format_rfc3339(Precision::Nanos)
        );
        let date_time = DateTime::from_ymdhms(2000, 12, 31, 23, 59, 59).unwrap();
        assert_eq!(
            "2000-12-31T23:59:59Z",
            date_time.format_rfc3339(Precision::Seconds)
        );
        assert_eq!(
            "2000-12-31T23:59:59.000Z",
            date_time.format_rfc3339(Precision::Millis)
        );
        assert_eq!(
            "2000-12-31T23:59:59.000000Z",
            date_time.format_rfc3339(Precision::Micros)
        );
        assert_eq!(
            "2000-12-31T23:59:59.000000000Z",
            date_time.format_rfc3339(Precision::Nanos)
        );
    }
}
