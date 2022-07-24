mod format;
mod offset;

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
        let modified2 = modified.sub_dur(Duration::new(86400, 0));
        assert_eq!(0, date_time.timestamp());
        assert_eq!(86400, modified.timestamp());
        assert_eq!(0, modified2.timestamp());

        let modified = date_time.add(123, Unit::Nanos);
        assert_eq!(123, modified.duration().as_nanos());
        let modified = date_time.add(123, Unit::Micros);
        assert_eq!(123000, modified.duration().as_nanos());
        let modified = date_time.add(123, Unit::Millis);
        assert_eq!(123000000, modified.duration().as_nanos());
        let modified = date_time.add(12, Unit::Centis);
        assert_eq!(120000000, modified.duration().as_nanos());
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
    fn get() {
        let date_time = DateTime::from_ymdhms(2000, 5, 10, 15, 34, 12)
            .unwrap()
            .add_dur(Duration::new(0, 123456789));
        assert_eq!(2000, date_time.get(Unit::Year));
        assert_eq!(5, date_time.get(Unit::Month));
        assert_eq!(10, date_time.get(Unit::Day));
        assert_eq!(15, date_time.get(Unit::Hour));
        assert_eq!(34, date_time.get(Unit::Min));
        assert_eq!(12, date_time.get(Unit::Sec));
        assert_eq!(12, date_time.get(Unit::Centis));
        assert_eq!(123, date_time.get(Unit::Millis));
        assert_eq!(123456, date_time.get(Unit::Micros));
        assert_eq!(123456789, date_time.get(Unit::Nanos));
    }

    #[test]
    fn set() {
        let date_time = DateTime::from_ymdhms(2000, 5, 10, 15, 34, 12)
            .unwrap()
            .add_dur(Duration::new(0, 123456789));
        let modified = date_time.set(2022, Unit::Year).unwrap();
        assert_eq!(2022, modified.get(Unit::Year));
        let modified = date_time.set(1, Unit::Month).unwrap();
        assert_eq!(2000, modified.get(Unit::Year));
        assert_eq!(1, modified.get(Unit::Month));
        let modified = date_time.set(13, Unit::Day).unwrap();
        assert_eq!(2000, modified.get(Unit::Year));
        assert_eq!(5, modified.get(Unit::Month));
        assert_eq!(13, modified.get(Unit::Day));
        let modified = date_time.set(3, Unit::Hour).unwrap();
        assert_eq!(2000, modified.get(Unit::Year));
        assert_eq!(5, modified.get(Unit::Month));
        assert_eq!(10, modified.get(Unit::Day));
        assert_eq!(3, modified.get(Unit::Hour));
        let modified = date_time.set(15, Unit::Min).unwrap();
        assert_eq!(2000, modified.get(Unit::Year));
        assert_eq!(5, modified.get(Unit::Month));
        assert_eq!(10, modified.get(Unit::Day));
        assert_eq!(15, modified.get(Unit::Hour));
        assert_eq!(15, modified.get(Unit::Min));
        let modified = date_time.set(10, Unit::Sec).unwrap();
        assert_eq!(2000, modified.get(Unit::Year));
        assert_eq!(5, modified.get(Unit::Month));
        assert_eq!(10, modified.get(Unit::Day));
        assert_eq!(15, modified.get(Unit::Hour));
        assert_eq!(34, modified.get(Unit::Min));
        assert_eq!(10, modified.get(Unit::Sec));
        let modified = date_time.set(99, Unit::Centis).unwrap();
        assert_eq!(2000, modified.get(Unit::Year));
        assert_eq!(5, modified.get(Unit::Month));
        assert_eq!(10, modified.get(Unit::Day));
        assert_eq!(15, modified.get(Unit::Hour));
        assert_eq!(34, modified.get(Unit::Min));
        assert_eq!(12, modified.get(Unit::Sec));
        assert_eq!(99, modified.get(Unit::Centis));
        assert_eq!(993456789, modified.get(Unit::Nanos));
        let modified = date_time.set(999, Unit::Millis).unwrap();
        assert_eq!(2000, modified.get(Unit::Year));
        assert_eq!(5, modified.get(Unit::Month));
        assert_eq!(10, modified.get(Unit::Day));
        assert_eq!(15, modified.get(Unit::Hour));
        assert_eq!(34, modified.get(Unit::Min));
        assert_eq!(12, modified.get(Unit::Sec));
        assert_eq!(999, modified.get(Unit::Millis));
        assert_eq!(999456789, modified.get(Unit::Nanos));
        let modified = date_time.set(999999, Unit::Micros).unwrap();
        assert_eq!(2000, modified.get(Unit::Year));
        assert_eq!(5, modified.get(Unit::Month));
        assert_eq!(10, modified.get(Unit::Day));
        assert_eq!(15, modified.get(Unit::Hour));
        assert_eq!(34, modified.get(Unit::Min));
        assert_eq!(12, modified.get(Unit::Sec));
        assert_eq!(999999, modified.get(Unit::Micros));
        assert_eq!(999999789, modified.get(Unit::Nanos));
        let modified = date_time.set(999999999, Unit::Nanos).unwrap();
        assert_eq!(2000, modified.get(Unit::Year));
        assert_eq!(5, modified.get(Unit::Month));
        assert_eq!(10, modified.get(Unit::Day));
        assert_eq!(15, modified.get(Unit::Hour));
        assert_eq!(34, modified.get(Unit::Min));
        assert_eq!(12, modified.get(Unit::Sec));
        assert_eq!(999999999, modified.get(Unit::Nanos));

        assert!(date_time.set(1969, Unit::Year).is_err());
        assert!(date_time.set(13, Unit::Month).is_err());
        assert!(date_time
            .set(2, Unit::Month)
            .unwrap()
            .set(31, Unit::Day)
            .is_err());
        assert!(date_time.set(32, Unit::Day).is_err());
        assert!(date_time.set(24, Unit::Hour).is_err());
        assert!(date_time.set(60, Unit::Min).is_err());
        assert!(date_time.set(60, Unit::Sec).is_err());
        assert!(date_time.set(100, Unit::Centis).is_err());
        assert!(date_time.set(1000, Unit::Millis).is_err());
        assert!(date_time.set(1000000, Unit::Micros).is_err());
        assert!(date_time.set(1000000000, Unit::Nanos).is_err());
    }

    #[test]
    fn parse_rfc3339() {
        let date_time = DateTime::parse_rfc3339("2022-05-02T15:30:20Z").unwrap();
        assert_eq!(
            "2022-05-02T15:30:20Z",
            date_time.format_rfc3339(Precision::Seconds)
        );
        assert_eq!(0, date_time.get_offset());
        assert_eq!(1651505420, date_time.timestamp());

        let date_time = DateTime::parse_rfc3339("2022-05-02T15:30:20+12:34").unwrap();
        assert_eq!(
            "2022-05-02T15:30:20+12:34",
            date_time.format_rfc3339(Precision::Seconds)
        );
        assert_eq!(45240, date_time.get_offset());
        assert_eq!(1651460180, date_time.timestamp());

        let date_time = DateTime::parse_rfc3339("2022-05-02T15:30:20-12:34").unwrap();
        assert_eq!(
            "2022-05-02T15:30:20-12:34",
            date_time.format_rfc3339(Precision::Seconds)
        );
        assert_eq!(-45240, date_time.get_offset());
        assert_eq!(1651550660, date_time.timestamp());

        let date_time = DateTime::parse_rfc3339("2022-05-02T15:30:20.1Z").unwrap();
        assert_eq!(
            "2022-05-02T15:30:20Z",
            date_time.format_rfc3339(Precision::Seconds)
        );
        assert_eq!(0, date_time.get_offset());
        assert_eq!(1651505420, date_time.timestamp());
        assert_eq!(100000000, date_time.duration().subsec_nanos());

        let date_time = DateTime::parse_rfc3339("2022-05-02T15:30:20.123456789Z").unwrap();
        assert_eq!(
            "2022-05-02T15:30:20Z",
            date_time.format_rfc3339(Precision::Seconds)
        );
        assert_eq!(0, date_time.get_offset());
        assert_eq!(1651505420, date_time.timestamp());
        assert_eq!(123456789, date_time.duration().subsec_nanos());

        let date_time = DateTime::parse_rfc3339("2022-05-02T15:30:20.123456789+12:34").unwrap();
        assert_eq!(
            "2022-05-02T15:30:20+12:34",
            date_time.format_rfc3339(Precision::Seconds)
        );
        assert_eq!(45240, date_time.get_offset());
        assert_eq!(1651460180, date_time.timestamp());
        assert_eq!(123456789, date_time.duration().subsec_nanos());

        assert!(DateTime::parse_rfc3339("2022-05-02T15:30:20").is_err());
        assert!(DateTime::parse_rfc3339("1969-05-02T15:30:20Z").is_err());
        assert!(DateTime::parse_rfc3339("2022-13-02T15:30:20Z").is_err());
        assert!(DateTime::parse_rfc3339("2022-05-32T15:30:20Z").is_err());
        assert!(DateTime::parse_rfc3339("2022-05-02T24:30:20Z").is_err());
        assert!(DateTime::parse_rfc3339("2022-05-02T15:60:20Z").is_err());
        assert!(DateTime::parse_rfc3339("2022-05-02T15:30:60Z").is_err());
        assert!(DateTime::parse_rfc3339("2022-05-02T15:30:20+").is_err());
        assert!(DateTime::parse_rfc3339("2022-05-02T15:30:20+1").is_err());
        assert!(DateTime::parse_rfc3339("2022-05-02T15:30:20+10").is_err());
        assert!(DateTime::parse_rfc3339("2022-05-02T15:30:20+10:").is_err());
        assert!(DateTime::parse_rfc3339("2022-05-02T15:30:20").is_err());
        assert!(DateTime::parse_rfc3339("2022-05-02T15:30:20+10:0").is_err());
        assert!(DateTime::parse_rfc3339("2022-05-02T15:30:20.Z").is_err());
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

        let with_offset = DateTime::from_ymdhms(1970, 1, 2, 0, 0, 0)
            .unwrap()
            .set_offset(3660)
            .unwrap();
        assert_eq!(
            "1970-01-02T01:01:00+01:01",
            with_offset.format_rfc3339(Precision::Seconds)
        );
        let with_offset = with_offset.set_offset(-3660).unwrap();
        assert_eq!(
            "1970-01-01T22:59:00-01:01",
            with_offset.format_rfc3339(Precision::Seconds)
        );
    }
}
