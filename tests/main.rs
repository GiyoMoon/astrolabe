mod format;

#[cfg(test)]
mod tests {
    use std::time::{Duration, SystemTime, UNIX_EPOCH};

    use astrolabe::{DateTime, Precision, Unit};

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
    fn duration() {
        let date_time = DateTime::from_ymd(1970, 1, 1).unwrap();
        assert_eq!(0, date_time.timestamp());
        assert_eq!(0, date_time.duration().as_secs());
        let date_time2 = DateTime::from_ymd(2000, 1, 1).unwrap();
        assert_eq!(946684800, date_time2.timestamp());
        assert_eq!(946684800, date_time2.duration().as_secs());

        assert_eq!(946684800, date_time.between(&date_time2).as_secs());
        assert_eq!(946684800, date_time2.between(&date_time).as_secs());
        let date_time3 = DateTime::from_ymd(2022, 5, 2).unwrap();
        assert_eq!(704764800, date_time2.between(&date_time3).as_secs());
        assert_eq!(704764800, date_time3.between(&date_time2).as_secs());
    }

    #[test]
    fn manipulation() {
        let date_time = DateTime::from_ymd(1970, 1, 1).unwrap();
        let modified = date_time.add_dur(Duration::new(86400, 0));
        assert_eq!(0, date_time.timestamp());
        assert_eq!(86400, modified.timestamp());
        let modified2 = modified.remove_dur(Duration::new(86400, 0));
        assert_eq!(0, date_time.timestamp());
        assert_eq!(86400, modified.timestamp());
        assert_eq!(0, modified2.timestamp());

        let modified = date_time.add(123, Unit::Nano);
        assert_eq!(123, modified.duration().as_nanos());
        let modified = date_time.add(123, Unit::Micro);
        assert_eq!(123000, modified.duration().as_nanos());
        let modified = date_time.add(123, Unit::Milli);
        assert_eq!(123000000, modified.duration().as_nanos());
        let modified = date_time.add(123, Unit::Sec);
        assert_eq!(123, modified.duration().as_secs());
        let modified = date_time.add(123, Unit::Min);
        assert_eq!(7380, modified.duration().as_secs());
        let modified = date_time.add(123, Unit::Hour);
        assert_eq!(442800, modified.duration().as_secs());
        let modified = date_time.add(123, Unit::Day);
        assert_eq!(10627200, modified.duration().as_secs());
        let modified = date_time.add(11, Unit::Month);
        assert_eq!("1970-12-01", modified.format("yyyy-MM-dd").unwrap());
        let modified = date_time.add(12, Unit::Month);
        assert_eq!("1971-01-01", modified.format("yyyy-MM-dd").unwrap());
        let modified = date_time.add(14, Unit::Month);
        assert_eq!("1971-03-01", modified.format("yyyy-MM-dd").unwrap());

        // Leap year cases
        let modified = date_time.add(30, Unit::Day);
        assert_eq!("1970-01-31", modified.format("yyyy-MM-dd").unwrap());
        let modified = modified.add(1, Unit::Month);
        assert_eq!("1970-02-28", modified.format("yyyy-MM-dd").unwrap());
        let modified = modified.add(2, Unit::Year);
        assert_eq!("1972-02-28", modified.format("yyyy-MM-dd").unwrap());
        let modified = date_time.add(2, Unit::Year).add(30, Unit::Day);
        assert_eq!("1972-01-31", modified.format("yyyy-MM-dd").unwrap());
        let modified = modified.add(1, Unit::Month);
        assert_eq!("1972-02-29", modified.format("yyyy-MM-dd").unwrap());

        let date_time = DateTime::from_ymd(1971, 1, 1).unwrap();
        let modified = date_time.sub(1, Unit::Month);
        assert_eq!("1970-12-01", modified.format("yyyy-MM-dd").unwrap());

        let date_time = DateTime::from_ymd(1972, 3, 31).unwrap();
        let modified = date_time.sub(1, Unit::Month);
        assert_eq!("1972-02-29", modified.format("yyyy-MM-dd").unwrap());
        let modified = modified.sub(1, Unit::Month);
        assert_eq!("1972-01-29", modified.format("yyyy-MM-dd").unwrap());
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
