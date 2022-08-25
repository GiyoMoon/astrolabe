#[cfg(test)]
mod datetime_tests {
    use astrolabe::{DateTime, DateTimeUnit, Precision};

    #[test]
    fn derive() {
        // Default
        let date_time = DateTime::default();
        // Debug
        println!("{:?}", date_time);
        // Display
        assert_eq!("0001/01/01 00:00:00", format!("{}", date_time));
        // From<&DateTime>
        let _ = DateTime::from(&date_time);
        // Clone
        let clone = date_time.clone();
        // PartialEq
        assert!(date_time == clone);

        let clone = date_time.apply(1, DateTimeUnit::Nanos).unwrap();
        // PartialEq
        assert!(date_time != clone);

        let unit = DateTimeUnit::Day;
        // Debug
        println!("{:?}", unit);
        // Clone
        let clone = unit.clone();
        // PartialEq
        assert!(unit == clone);
    }

    #[test]
    fn now() {
        assert!(2021 < DateTime::now().get(DateTimeUnit::Year));
    }

    #[test]
    fn days() {
        assert_eq!(0, DateTime::from_days(0).as_days());
        assert_eq!(i32::MAX, DateTime::from_days(i32::MAX).as_days());
        assert_eq!(i32::MIN, DateTime::from_days(i32::MIN).as_days());
    }

    #[test]
    fn seconds() {
        assert_eq!(0, DateTime::from_seconds(0).unwrap().as_seconds());
        assert_eq!(-1, DateTime::from_seconds(-1).unwrap().as_seconds());
        assert_eq!(
            185_542_587_187_199,
            DateTime::from_seconds(185_542_587_187_199)
                .unwrap()
                .as_seconds()
        );
        assert_eq!(
            "5879611/07/12 23:59:59",
            DateTime::from_seconds(185_542_587_187_199)
                .unwrap()
                .format("yyyy/MM/dd HH:mm:ss")
        );
        assert!(DateTime::from_seconds(185_542_587_187_200).is_err());
        assert_eq!(
            -185_542_587_187_200,
            DateTime::from_seconds(-185_542_587_187_200)
                .unwrap()
                .as_seconds()
        );
        assert_eq!(
            "-5879611/06/23 00:00:00",
            DateTime::from_seconds(-185_542_587_187_200)
                .unwrap()
                .format("yyyy/MM/dd HH:mm:ss")
        );
        assert!(DateTime::from_seconds(-185_542_587_187_201).is_err());
    }

    #[test]
    fn nanoseconds() {
        assert_eq!(0, DateTime::from_nanoseconds(0).unwrap().as_nanoseconds());
        assert_eq!(
            "0002/01/01 12:34:56",
            DateTime::from_nanoseconds(31_581_296_000_000_000)
                .unwrap()
                .format("yyyy/MM/dd HH:mm:ss")
        );
        //41'104
        assert_eq!(
            "-0001/01/01 00:00:00",
            DateTime::from_nanoseconds(-31_622_400_000_000_000)
                .unwrap()
                .format("yyyy/MM/dd HH:mm:ss")
        );
        assert_eq!(
            185_542_587_187_199_999_999_999,
            DateTime::from_nanoseconds(185_542_587_187_199_999_999_999)
                .unwrap()
                .as_nanoseconds()
        );
        assert_eq!(
            "5879611/07/12 23:59:59",
            DateTime::from_nanoseconds(185_542_587_187_199_999_999_999)
                .unwrap()
                .format("yyyy/MM/dd HH:mm:ss")
        );
        assert!(DateTime::from_nanoseconds(185_542_587_187_200_000_000_000).is_err());
        assert_eq!(
            -185_542_587_187_200_000_000_000,
            DateTime::from_nanoseconds(-185_542_587_187_200_000_000_000)
                .unwrap()
                .as_nanoseconds()
        );
        assert_eq!(
            "-5879611/06/23 00:00:00",
            DateTime::from_nanoseconds(-185_542_587_187_200_000_000_000)
                .unwrap()
                .format("yyyy/MM/dd HH:mm:ss")
        );
        assert!(DateTime::from_nanoseconds(-185_542_587_273_600_000_000_000).is_err());
    }

    #[test]
    fn from_ymd() {
        // check allowed limits
        from_ymd_ok(0, 1, 1, 1);
        from_ymd_ok(334, 1, 12, 1);
        from_ymd_ok(30, 1, 1, 31);
        from_ymd_ok(58, 1, 2, 28);
        from_ymd_ok(1154, 4, 2, 29);
        from_ymd_ok(119, 1, 4, 30);
        from_ymd_ok(i32::MAX, 5_879_611, 7, 12);
        from_ymd_ok(i32::MIN, -5_879_611, 6, 23);

        // check invalid limits
        from_ymd_err(1, 0, 1);
        from_ymd_err(1, 13, 1);
        from_ymd_err(1, 1, 0);
        from_ymd_err(1, 1, 32);
        from_ymd_err(1, 2, 29);
        from_ymd_err(4, 2, 30);
        from_ymd_err(1, 4, 31);
        from_ymd_err(5_879_611, 7, 13);
        from_ymd_err(5_879_612, 1, 1);
        from_ymd_err(5_879_611, 8, 1);
        from_ymd_err(-5_879_611, 6, 22);
        from_ymd_err(-5_879_612, 1, 1);
        from_ymd_err(-5_879_611, 5, 1);
    }

    fn from_ymd_ok(expected: i32, year: i32, month: u32, day: u32) {
        assert_eq!(
            expected,
            DateTime::from_ymd(year, month, day).unwrap().as_days()
        );
    }

    fn from_ymd_err(year: i32, month: u32, day: u32) {
        assert!(DateTime::from_ymd(year, month, day).is_err());
    }

    #[test]
    fn from_hms() {
        // check allowed limits
        from_hms_ok(0, 0, 0, 0);
        from_hms_ok(82800, 23, 0, 0);
        from_hms_ok(3540, 0, 59, 0);
        from_hms_ok(59, 0, 0, 59);
        from_hms_ok(86399, 23, 59, 59);

        // check invalid limits
        from_hms_err(24, 0, 0);
        from_hms_err(0, 60, 0);
        from_hms_err(0, 0, 60);
        from_hms_err(24, 60, 60);
    }

    fn from_hms_ok(expected: u64, hour: u32, minute: u32, second: u32) {
        assert_eq!(
            expected,
            DateTime::from_hms(hour, minute, second)
                .unwrap()
                .time()
                .as_nanoseconds()
                / 1_000_000_000
        );
    }

    fn from_hms_err(hour: u32, minute: u32, second: u32) {
        assert!(DateTime::from_hms(hour, minute, second).is_err());
    }

    #[test]
    fn from_ymdhms() {
        const SECS_PER_DAY: i64 = 86400;
        // check allowed limits
        from_ymdhms_ok(0, 1, 1, 1, 0, 0, 0);
        from_ymdhms_ok(334 * SECS_PER_DAY, 1, 12, 1, 0, 0, 0);
        from_ymdhms_ok(30 * SECS_PER_DAY, 1, 1, 31, 0, 0, 0);
        from_ymdhms_ok(58 * SECS_PER_DAY, 1, 2, 28, 0, 0, 0);
        from_ymdhms_ok(1154 * SECS_PER_DAY, 4, 2, 29, 0, 0, 0);
        from_ymdhms_ok(119 * SECS_PER_DAY, 1, 4, 30, 0, 0, 0);
        from_ymdhms_ok(i32::MAX as i64 * SECS_PER_DAY, 5_879_611, 7, 12, 0, 0, 0);
        from_ymdhms_ok(i32::MIN as i64 * SECS_PER_DAY, -5_879_611, 6, 23, 0, 0, 0);

        // check invalid limits
        from_ymdhms_err(1, 0, 1, 0, 0, 0);
        from_ymdhms_err(1, 13, 1, 0, 0, 0);
        from_ymdhms_err(1, 1, 0, 0, 0, 0);
        from_ymdhms_err(1, 1, 32, 0, 0, 0);
        from_ymdhms_err(1, 2, 29, 0, 0, 0);
        from_ymdhms_err(4, 2, 30, 0, 0, 0);
        from_ymdhms_err(1, 4, 31, 0, 0, 0);
        from_ymdhms_err(5_879_611, 7, 13, 0, 0, 0);
        from_ymdhms_err(5_879_612, 1, 1, 0, 0, 0);
        from_ymdhms_err(5_879_611, 8, 1, 0, 0, 0);
        from_ymdhms_err(-5_879_611, 6, 22, 0, 0, 0);
        from_ymdhms_err(-5_879_612, 1, 1, 0, 0, 0);
        from_ymdhms_err(-5_879_611, 5, 1, 0, 0, 0);

        // check allowed limits
        from_ymdhms_ok(0, 1, 1, 1, 0, 0, 0);
        from_ymdhms_ok(82800, 1, 1, 1, 23, 0, 0);
        from_ymdhms_ok(3540, 1, 1, 1, 0, 59, 0);
        from_ymdhms_ok(59, 1, 1, 1, 0, 0, 59);
        from_ymdhms_ok(86399, 1, 1, 1, 23, 59, 59);

        // check invalid limits
        from_ymdhms_err(1, 1, 1, 24, 0, 0);
        from_ymdhms_err(1, 1, 1, 0, 60, 0);
        from_ymdhms_err(1, 1, 1, 0, 0, 60);
        from_ymdhms_err(1, 1, 1, 24, 60, 60);
    }

    fn from_ymdhms_ok(
        expected: i64,
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        minute: u32,
        second: u32,
    ) {
        assert_eq!(
            expected,
            DateTime::from_ymdhms(year, month, day, hour, minute, second)
                .unwrap()
                .as_seconds()
        );
    }

    fn from_ymdhms_err(year: i32, month: u32, day: u32, hour: u32, minute: u32, second: u32) {
        assert!(DateTime::from_ymdhms(year, month, day, hour, minute, second).is_err());
    }

    #[test]
    fn timestamp() {
        assert_eq!(0, DateTime::from_timestamp(0).unwrap().timestamp());
        assert_eq!(
            185_480_451_590_399,
            DateTime::from_timestamp(185_480_451_590_399)
                .unwrap()
                .timestamp()
        );
        assert_eq!(
            "5879611/07/12 23:59:59",
            DateTime::from_timestamp(185_480_451_590_399)
                .unwrap()
                .format("yyyy/MM/dd HH:mm:ss")
        );

        assert_eq!(
            -185_604_722_784_000,
            DateTime::from_timestamp(-185_604_722_784_000)
                .unwrap()
                .timestamp()
        );
        assert_eq!(
            "-5879611/06/23 00:00:00",
            DateTime::from_timestamp(-185_604_722_784_000)
                .unwrap()
                .format("yyyy/MM/dd HH:mm:ss")
        );

        assert!(DateTime::from_timestamp(185_480_451_590_400).is_err());
        assert!(DateTime::from_timestamp(-185_604_722_784_001).is_err());
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
        assert_eq!(100000000, date_time.get(DateTimeUnit::Nanos));

        let date_time = DateTime::parse_rfc3339("2022-05-02T15:30:20.123456789Z").unwrap();
        assert_eq!(
            "2022-05-02T15:30:20Z",
            date_time.format_rfc3339(Precision::Seconds)
        );
        assert_eq!(0, date_time.get_offset());
        assert_eq!(1651505420, date_time.timestamp());
        assert_eq!(123456789, date_time.get(DateTimeUnit::Nanos));

        let date_time = DateTime::parse_rfc3339("2022-05-02T15:30:20.123456789+12:34").unwrap();
        assert_eq!(
            "2022-05-02T15:30:20+12:34",
            date_time.format_rfc3339(Precision::Seconds)
        );
        assert_eq!(45240, date_time.get_offset());
        assert_eq!(1651460180, date_time.timestamp());
        assert_eq!(123456789, date_time.get(DateTimeUnit::Nanos));

        assert!(DateTime::parse_rfc3339("2022-05-02T15:30:20").is_err());
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

        assert!(DateTime::parse_rfc3339("test-05-02T15:30:20Z").is_err());
        assert!(DateTime::parse_rfc3339("2022-ee-02T15:30:20Z").is_err());
        assert!(DateTime::parse_rfc3339("2022-05-eeT15:30:20Z").is_err());
        assert!(DateTime::parse_rfc3339("2022-05-02Tee:30:20Z").is_err());
        assert!(DateTime::parse_rfc3339("2022-05-02T15:ee:20Z").is_err());
        assert!(DateTime::parse_rfc3339("2022-05-02T15:30:eeZ").is_err());
        assert!(DateTime::parse_rfc3339("2022-05-02T15:30:20.02").is_err());
        assert!(DateTime::parse_rfc3339("2022-05-02T15:30:20.02+1").is_err());

        assert!(DateTime::parse_rfc3339("2022-05-02T15:30:20.02+ee:00").is_err());
        assert!(DateTime::parse_rfc3339("2022-05-02T15:30:20.02+01:ee").is_err());
        assert!(DateTime::parse_rfc3339("2022-05-02T15:30:20.02+24:00").is_err());
        assert!(DateTime::parse_rfc3339("2022-05-02T15:30:20.02+01:60").is_err());
    }

    #[test]
    fn time() {
        assert_eq!(
            0,
            DateTime::default()
                .set_time(0)
                .unwrap()
                .time()
                .as_nanoseconds()
        );
        assert_eq!(
            86_399_999_999_999,
            DateTime::default()
                .set_time(86_399_999_999_999)
                .unwrap()
                .time()
                .as_nanoseconds()
        );
        assert!(DateTime::default().set_time(86_400_000_000_000).is_err());
    }

    #[test]
    fn date_and_time() {
        let date_time = DateTime::from_ymdhms(2022, 05, 02, 12, 32, 01).unwrap();
        let date = date_time.date();
        let time = date_time.time();
        assert_eq!("2022/05/02", date.format("yyyy/MM/dd"));
        assert_eq!("12:32:01", time.format("HH:mm:ss"));
    }

    #[test]
    fn between() {
        let date_time1 = DateTime::from_days(123);
        let date_time2 = DateTime::from_days(200);
        assert_eq!(77 * 86400, date_time1.between(&date_time2));
        assert_eq!(77 * 86400, date_time2.between(&date_time1));
    }

    #[test]
    fn get() {
        let date_time = DateTime::from_ymd(2000, 5, 10).unwrap();
        assert_eq!(2000, date_time.get(DateTimeUnit::Year));
        assert_eq!(5, date_time.get(DateTimeUnit::Month));
        assert_eq!(10, date_time.get(DateTimeUnit::Day));

        let date_time = DateTime::from_nanoseconds(10_921_123_456_789).unwrap();

        assert_eq!(3, date_time.get(DateTimeUnit::Hour));
        assert_eq!(2, date_time.get(DateTimeUnit::Min));
        assert_eq!(1, date_time.get(DateTimeUnit::Sec));
        assert_eq!(12, date_time.get(DateTimeUnit::Centis));
        assert_eq!(123, date_time.get(DateTimeUnit::Millis));
        assert_eq!(123_456, date_time.get(DateTimeUnit::Micros));
        assert_eq!(123_456_789, date_time.get(DateTimeUnit::Nanos))
    }

    #[test]
    fn apply() {
        let date_time = DateTime::from_ymd(1970, 1, 1).unwrap();

        let modified = date_time.apply(123, DateTimeUnit::Day).unwrap();
        assert_eq!(10627200, modified.timestamp());
        let modified = date_time.apply(11, DateTimeUnit::Month).unwrap();
        assert_eq!("1970-12-01", modified.format("yyyy-MM-dd"));
        let modified = date_time.apply(12, DateTimeUnit::Month).unwrap();
        assert_eq!("1971-01-01", modified.format("yyyy-MM-dd"));
        let modified = date_time.apply(14, DateTimeUnit::Month).unwrap();
        assert_eq!("1971-03-01", modified.format("yyyy-MM-dd"));

        let date_time2 = DateTime::from_ymd(2020, 2, 29).unwrap();
        let modified = date_time2.apply(4, DateTimeUnit::Year).unwrap();
        assert_eq!("2024-02-29", modified.format("yyyy-MM-dd"));
        let modified = date_time2.apply(1, DateTimeUnit::Year).unwrap();
        assert_eq!("2021-02-28", modified.format("yyyy-MM-dd"));

        // Leap year cases
        let modified = date_time.apply(30, DateTimeUnit::Day).unwrap();
        assert_eq!("1970-01-31", modified.format("yyyy-MM-dd"));
        let modified = modified.apply(1, DateTimeUnit::Month).unwrap();
        assert_eq!("1970-02-28", modified.format("yyyy-MM-dd"));
        let modified = modified.apply(2, DateTimeUnit::Year).unwrap();
        assert_eq!("1972-02-28", modified.format("yyyy-MM-dd"));
        let modified = date_time
            .apply(2, DateTimeUnit::Year)
            .unwrap()
            .apply(30, DateTimeUnit::Day)
            .unwrap();
        assert_eq!("1972-01-31", modified.format("yyyy-MM-dd"));
        let modified = modified.apply(1, DateTimeUnit::Month).unwrap();
        assert_eq!("1972-02-29", modified.format("yyyy-MM-dd"));

        let date_time = DateTime::from_ymd(1971, 1, 1).unwrap();
        let modified = date_time.apply(-1, DateTimeUnit::Month).unwrap();
        assert_eq!("1970-12-01", modified.format("yyyy-MM-dd"));

        let date_time = DateTime::from_ymd(1972, 3, 31).unwrap();
        let modified = date_time.apply(-1, DateTimeUnit::Month).unwrap();
        assert_eq!("1972-02-29", modified.format("yyyy-MM-dd"));
        let modified = modified.apply(-1, DateTimeUnit::Month).unwrap();
        assert_eq!("1972-01-29", modified.format("yyyy-MM-dd"));

        let mut date_time = DateTime::default().apply(1, DateTimeUnit::Hour).unwrap();

        date_time = date_time.apply(1, DateTimeUnit::Hour).unwrap();
        date_time = date_time.apply(2, DateTimeUnit::Min).unwrap();
        date_time = date_time.apply(3, DateTimeUnit::Sec).unwrap();
        date_time = date_time.apply(4, DateTimeUnit::Centis).unwrap();
        date_time = date_time.apply(5, DateTimeUnit::Millis).unwrap();
        date_time = date_time.apply(6, DateTimeUnit::Micros).unwrap();
        date_time = date_time.apply(7, DateTimeUnit::Nanos).unwrap();

        assert_eq!(2, date_time.get(DateTimeUnit::Hour));
        assert_eq!(2, date_time.get(DateTimeUnit::Min));
        assert_eq!(3, date_time.get(DateTimeUnit::Sec));
        assert_eq!(4, date_time.get(DateTimeUnit::Centis));
        assert_eq!(45, date_time.get(DateTimeUnit::Millis));
        assert_eq!(45_006, date_time.get(DateTimeUnit::Micros));
        assert_eq!(45_006_007, date_time.get(DateTimeUnit::Nanos));

        date_time = date_time.apply(-2, DateTimeUnit::Hour).unwrap();
        date_time = date_time.apply(-2, DateTimeUnit::Min).unwrap();
        date_time = date_time.apply(-3, DateTimeUnit::Sec).unwrap();
        date_time = date_time.apply(-4, DateTimeUnit::Centis).unwrap();
        date_time = date_time.apply(-5, DateTimeUnit::Millis).unwrap();
        date_time = date_time.apply(-6, DateTimeUnit::Micros).unwrap();
        date_time = date_time.apply(-7, DateTimeUnit::Nanos).unwrap();

        assert_eq!(0, date_time.as_nanoseconds());

        let date_time = DateTime::from_ymd(1, 1, 1).unwrap();
        assert!(date_time
            .apply(i32::MAX as i64 + 1, DateTimeUnit::Year)
            .is_err());
        assert!(date_time.apply(5_879_611, DateTimeUnit::Year).is_err());

        let date_time = DateTime::from_ymd(1, 1, 2).unwrap();
        assert!(date_time.apply(i32::MAX as i64, DateTimeUnit::Day).is_err());

        let date_time = DateTime::from_ymdhms(5_879_611, 7, 12, 23, 59, 59).unwrap();
        assert!(date_time.apply(1, DateTimeUnit::Month).is_err());
        assert!(date_time.apply(1, DateTimeUnit::Sec).is_err());
    }

    #[test]
    fn set() {
        let date_time = DateTime::from_ymd(2000, 5, 10).unwrap();
        let modified = date_time.set(2022, DateTimeUnit::Year).unwrap();
        assert_eq!(2022, modified.get(DateTimeUnit::Year));
        let modified = date_time.set(1, DateTimeUnit::Month).unwrap();
        assert_eq!(2000, modified.get(DateTimeUnit::Year));
        assert_eq!(1, modified.get(DateTimeUnit::Month));
        let modified = date_time.set(13, DateTimeUnit::Day).unwrap();
        assert_eq!(2000, modified.get(DateTimeUnit::Year));
        assert_eq!(5, modified.get(DateTimeUnit::Month));
        assert_eq!(13, modified.get(DateTimeUnit::Day));

        assert!(date_time.set(5_879_612, DateTimeUnit::Year).is_err());
        assert!(date_time.set(13, DateTimeUnit::Month).is_err());
        assert!(date_time
            .set(2, DateTimeUnit::Month)
            .unwrap()
            .set(31, DateTimeUnit::Day)
            .is_err());
        assert!(date_time.set(32, DateTimeUnit::Day).is_err());
        assert!(date_time.set(0, DateTimeUnit::Year).is_err());

        let mut date_time = DateTime::default()
            .apply(34_661_123_456_789, DateTimeUnit::Nanos)
            .unwrap();

        date_time = date_time.set(1, DateTimeUnit::Hour).unwrap();
        date_time = date_time.set(2, DateTimeUnit::Min).unwrap();
        date_time = date_time.set(3, DateTimeUnit::Sec).unwrap();
        date_time = date_time.set(4, DateTimeUnit::Centis).unwrap();

        assert_eq!(1, date_time.get(DateTimeUnit::Hour));
        assert_eq!(2, date_time.get(DateTimeUnit::Min));
        assert_eq!(3, date_time.get(DateTimeUnit::Sec));
        assert_eq!(4, date_time.get(DateTimeUnit::Centis));

        date_time = date_time.set(5, DateTimeUnit::Millis).unwrap();
        assert_eq!(5, date_time.get(DateTimeUnit::Millis));
        date_time = date_time.set(6, DateTimeUnit::Micros).unwrap();
        assert_eq!(6, date_time.get(DateTimeUnit::Micros));
        date_time = date_time.set(7, DateTimeUnit::Nanos).unwrap();
        assert_eq!(7, date_time.get(DateTimeUnit::Nanos));

        assert!(date_time.set(0, DateTimeUnit::Month).is_err());
        assert!(date_time.set(-1, DateTimeUnit::Month).is_err());
        assert!(date_time.set(0, DateTimeUnit::Day).is_err());
        assert!(date_time.set(-1, DateTimeUnit::Day).is_err());
        assert!(date_time.set(-1, DateTimeUnit::Hour).is_err());
        assert!(date_time.set(-1, DateTimeUnit::Min).is_err());
        assert!(date_time.set(-1, DateTimeUnit::Sec).is_err());
        assert!(date_time.set(-1, DateTimeUnit::Centis).is_err());
        assert!(date_time.set(-1, DateTimeUnit::Millis).is_err());
        assert!(date_time.set(-1, DateTimeUnit::Micros).is_err());
        assert!(date_time.set(-1, DateTimeUnit::Nanos).is_err());

        assert!(date_time.set(24, DateTimeUnit::Hour).is_err());
        assert!(date_time.set(60, DateTimeUnit::Min).is_err());
        assert!(date_time.set(60, DateTimeUnit::Sec).is_err());
        assert!(date_time.set(100, DateTimeUnit::Centis).is_err());
        assert!(date_time.set(1_000, DateTimeUnit::Millis).is_err());
        assert!(date_time.set(1_000_000, DateTimeUnit::Micros).is_err());
        assert!(date_time.set(1_000_000_000, DateTimeUnit::Nanos).is_err());
    }

    #[test]
    fn format_rfc3339() {
        let date_time = DateTime::from_ymdhms(1970, 1, 1, 0, 0, 0).unwrap();
        assert_eq!(
            "1970-01-01T00:00:00Z",
            date_time.format_rfc3339(Precision::Seconds)
        );
        assert_eq!(
            "1970-01-01T00:00:00.00Z",
            date_time.format_rfc3339(Precision::Centis)
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
            "2000-12-31T23:59:59.00Z",
            date_time.format_rfc3339(Precision::Centis)
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

    #[test]
    fn implementations() {
        let default = DateTime::default();
        assert_eq!(0, default.as_nanoseconds());
        let date_time = DateTime::from_nanoseconds(12345).unwrap();
        let date_time_copy = DateTime::from(&date_time);
        assert_eq!(12345, date_time.as_nanoseconds());
        assert_eq!(12345, date_time_copy.as_nanoseconds());
    }

    #[test]
    fn special_cases() {
        assert_eq!(2400, DateTime::from_days(876_275).get(DateTimeUnit::Year));
        assert_eq!(
            -4,
            DateTime::from_ymd(-4, 1, 1)
                .unwrap()
                .get(DateTimeUnit::Year)
        );
        assert_eq!(1186, DateTime::from_ymd(4, 4, 1).unwrap().as_days());
        assert_eq!(1247, DateTime::from_ymd(4, 6, 1).unwrap().as_days());
        assert_eq!(1277, DateTime::from_ymd(4, 7, 1).unwrap().as_days());
        assert_eq!(1308, DateTime::from_ymd(4, 8, 1).unwrap().as_days());
        assert_eq!(1339, DateTime::from_ymd(4, 9, 1).unwrap().as_days());
        assert_eq!(1369, DateTime::from_ymd(4, 10, 1).unwrap().as_days());
        assert_eq!(1400, DateTime::from_ymd(4, 11, 1).unwrap().as_days());
        assert_eq!(212, DateTime::from_ymd(1, 8, 1).unwrap().as_days());
        assert_eq!(304, DateTime::from_ymd(1, 11, 1).unwrap().as_days());
    }
}
