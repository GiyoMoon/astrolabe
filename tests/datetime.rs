#[cfg(test)]
mod datetime_tests {
    use astrolabe::{Date, DateTime, DateUtilities, Precision, Time, TimeUtilities};

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
        #[allow(clippy::clone_on_copy)]
        let clone = date_time.clone();
        // PartialEq
        assert!(date_time == clone);
        // PartialOrd
        assert_eq!(std::cmp::Ordering::Equal, date_time.cmp(&clone));

        let clone = date_time.add_nanos(1).unwrap();
        // PartialEq
        assert!(date_time != clone);

        // Ord
        assert!(date_time < clone);
        // PartialOrd
        assert_eq!(std::cmp::Ordering::Less, date_time.cmp(&clone));

        let clone = date_time.sub_nanos(1).unwrap();
        // Ord
        assert!(date_time > clone);
        // PartialOrd
        assert_eq!(std::cmp::Ordering::Greater, date_time.cmp(&clone));

        // Check that offset doesn't affect Eq and Ord
        let clone = date_time.set_offset(1).unwrap();
        // PartialEq
        assert!(date_time == clone);
        // PartialOrd
        assert_eq!(std::cmp::Ordering::Equal, date_time.cmp(&clone));

        assert!("2022-05-02T12:32:01Z".parse::<DateTime>().is_ok());
        assert!("blabla".parse::<DateTime>().is_err());
    }

    #[test]
    fn now() {
        assert!(2021 < DateTime::now().year());
    }

    #[test]
    fn from_ymd() {
        // check allowed limits
        from_ymd_ok(1, 1, 1);
        from_ymd_ok(1, 12, 1);
        from_ymd_ok(1, 1, 31);
        from_ymd_ok(1, 2, 28);
        from_ymd_ok(4, 2, 29);
        from_ymd_ok(1, 4, 30);
        from_ymd_ok(5_879_611, 7, 12);
        from_ymd_ok(-5_879_611, 6, 23);

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

    fn from_ymd_ok(year: i32, month: u32, day: u32) {
        assert_eq!(
            (year, month, day),
            DateTime::from_ymd(year, month, day).unwrap().as_ymd()
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
            Time::from(DateTime::from_hms(hour, minute, second).unwrap()).as_nanoseconds()
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
        assert_eq!(100000000, date_time.nano());

        let date_time = DateTime::parse_rfc3339("2022-05-02T15:30:20.123456789Z").unwrap();
        assert_eq!(
            "2022-05-02T15:30:20Z",
            date_time.format_rfc3339(Precision::Seconds)
        );
        assert_eq!(0, date_time.get_offset());
        assert_eq!(1651505420, date_time.timestamp());
        assert_eq!(123456789, date_time.nano());

        let date_time = DateTime::parse_rfc3339("2022-05-02T15:30:20.123456789+12:34").unwrap();
        assert_eq!(
            "2022-05-02T15:30:20+12:34",
            date_time.format_rfc3339(Precision::Seconds)
        );
        assert_eq!(45240, date_time.get_offset());
        assert_eq!(1651460180, date_time.timestamp());
        assert_eq!(123456789, date_time.nano());

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
            Time::from(DateTime::default().set_time(Time::default())).as_nanoseconds()
        );
        assert_eq!(
            86_399_999_999_999,
            Time::from(
                DateTime::default().set_time(Time::from_nanoseconds(86_399_999_999_999).unwrap())
            )
            .as_nanoseconds()
        );
    }

    #[test]
    fn date_and_time() {
        let date_time = DateTime::from_ymdhms(2022, 5, 2, 12, 32, 1).unwrap();
        let date = Date::from(date_time);
        let time = Time::from(date_time);
        assert_eq!("2022/05/02", date.format("yyyy/MM/dd"));
        assert_eq!("12:32:01", time.format("HH:mm:ss"));
    }

    #[test]
    fn get() {
        let date_time = DateTime::from_ymd(2000, 5, 10).unwrap();
        assert_eq!(2000, date_time.year());
        assert_eq!(5, date_time.month());
        assert_eq!(10, date_time.day());

        let date_time = DateTime::from(Time::from_nanoseconds(10_921_123_456_789).unwrap());

        assert_eq!(3, date_time.hour());
        assert_eq!(2, date_time.minute());
        assert_eq!(1, date_time.second());
        assert_eq!(123, date_time.milli());
        assert_eq!(123_456, date_time.micro());
        assert_eq!(123_456_789, date_time.nano())
    }

    #[test]
    fn apply() {
        let date_time = DateTime::from_ymd(1970, 1, 1).unwrap();

        let modified = date_time.add_days(123).unwrap();
        assert_eq!(10627200, modified.timestamp());
        let modified = date_time.add_months(11).unwrap();
        assert_eq!("1970-12-01", modified.format("yyyy-MM-dd"));
        let modified = date_time.add_months(12).unwrap();
        assert_eq!("1971-01-01", modified.format("yyyy-MM-dd"));
        let modified = date_time.add_months(14).unwrap();
        assert_eq!("1971-03-01", modified.format("yyyy-MM-dd"));

        let date_time2 = DateTime::from_ymd(2020, 2, 29).unwrap();
        let modified = date_time2.add_years(4).unwrap();
        assert_eq!("2024-02-29", modified.format("yyyy-MM-dd"));
        let modified = date_time2.add_years(1).unwrap();
        assert_eq!("2021-02-28", modified.format("yyyy-MM-dd"));

        // Leap year cases
        let modified = date_time.add_days(30).unwrap();
        assert_eq!("1970-01-31", modified.format("yyyy-MM-dd"));
        let modified = modified.add_months(1).unwrap();
        assert_eq!("1970-02-28", modified.format("yyyy-MM-dd"));
        let modified = modified.add_years(2).unwrap();
        assert_eq!("1972-02-28", modified.format("yyyy-MM-dd"));
        let modified = date_time.add_years(2).unwrap().add_days(30).unwrap();
        assert_eq!("1972-01-31", modified.format("yyyy-MM-dd"));
        let modified = modified.add_months(1).unwrap();
        assert_eq!("1972-02-29", modified.format("yyyy-MM-dd"));

        let date_time = DateTime::from_ymd(1971, 1, 1).unwrap();
        let modified = date_time.sub_months(1).unwrap();
        assert_eq!("1970-12-01", modified.format("yyyy-MM-dd"));

        let date_time = DateTime::from_ymd(1972, 3, 31).unwrap();
        let modified = date_time.sub_months(1).unwrap();
        assert_eq!("1972-02-29", modified.format("yyyy-MM-dd"));
        let modified = modified.sub_months(1).unwrap();
        assert_eq!("1972-01-29", modified.format("yyyy-MM-dd"));

        let mut date_time = DateTime::default().add_hours(1).unwrap();

        date_time = date_time.add_hours(1).unwrap();
        date_time = date_time.add_minutes(2).unwrap();
        date_time = date_time.add_seconds(3).unwrap();
        date_time = date_time.add_millis(5).unwrap();
        date_time = date_time.add_micros(6).unwrap();
        date_time = date_time.add_nanos(7).unwrap();

        assert_eq!(2, date_time.hour());
        assert_eq!(2, date_time.minute());
        assert_eq!(3, date_time.second());
        assert_eq!(5, date_time.milli());
        assert_eq!(5_006, date_time.micro());
        assert_eq!(5_006_007, date_time.nano());

        date_time = date_time.sub_hours(2).unwrap();
        date_time = date_time.sub_minutes(2).unwrap();
        date_time = date_time.sub_seconds(3).unwrap();
        date_time = date_time.sub_millis(5).unwrap();
        date_time = date_time.sub_micros(6).unwrap();
        date_time = date_time.sub_nanos(7).unwrap();

        assert_eq!(0, date_time.as_nanoseconds());

        let date_time = DateTime::from_ymd(1, 1, 1).unwrap();
        assert!(date_time.add_years(i32::MAX as u32 + 1).is_err());
        assert!(date_time.add_years(5_879_611).is_err());

        let date_time = DateTime::from_ymd(1, 1, 2).unwrap();
        assert!(date_time.add_days(i32::MAX as u32).is_err());

        let date_time = DateTime::from_ymdhms(5_879_611, 7, 12, 23, 59, 59).unwrap();
        assert!(date_time.add_months(1).is_err());
        assert!(date_time.add_seconds(1).is_err());
    }

    #[test]
    fn set() {
        let date_time = DateTime::from_ymd(2000, 5, 10).unwrap();
        let modified = date_time.set_year(2022).unwrap();
        assert_eq!(2022, modified.year());
        let modified = date_time.set_month(1).unwrap();
        assert_eq!(2000, modified.year());
        assert_eq!(1, modified.month());
        let modified = date_time.set_day(13).unwrap();
        assert_eq!(2000, modified.year());
        assert_eq!(5, modified.month());
        assert_eq!(13, modified.day());

        assert!(date_time.set_year(5_879_612).is_err());
        assert!(date_time.set_month(13).is_err());
        assert!(date_time.set_month(2).unwrap().set_day(31).is_err());
        assert!(date_time.set_day(32).is_err());
        assert!(date_time.set_year(0).is_err());

        let mut date_time = DateTime::default().add_nanos(34_661_123_456_789).unwrap();

        date_time = date_time.set_hour(1).unwrap();
        date_time = date_time.set_minute(2).unwrap();
        date_time = date_time.set_second(3).unwrap();

        assert_eq!(1, date_time.hour());
        assert_eq!(2, date_time.minute());
        assert_eq!(3, date_time.second());

        date_time = date_time.set_milli(5).unwrap();
        assert_eq!(5, date_time.milli());
        date_time = date_time.set_micro(6).unwrap();
        assert_eq!(6, date_time.micro());
        date_time = date_time.set_nano(7).unwrap();
        assert_eq!(7, date_time.nano());

        assert!(date_time.set_month(0).is_err());
        assert!(date_time.set_day(0).is_err());

        assert!(date_time.set_hour(24).is_err());
        assert!(date_time.set_minute(60).is_err());
        assert!(date_time.set_second(60).is_err());
        assert!(date_time.set_milli(1_000).is_err());
        assert!(date_time.set_micro(1_000_000).is_err());
        assert!(date_time.set_nano(1_000_000_000).is_err());
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
        let date_time = DateTime::from(Time::from_nanoseconds(12345).unwrap());
        let date_time_copy = DateTime::from(&date_time);
        assert_eq!(12345, date_time.as_nanoseconds());
        assert_eq!(12345, date_time_copy.as_nanoseconds());
    }

    #[test]
    fn special_cases() {
        assert_eq!(2400, DateTime::from_ymd(2400, 2, 29).unwrap().year());
        assert_eq!(-4, DateTime::from_ymd(-4, 1, 1).unwrap().year());
        assert_eq!((4, 4, 1), DateTime::from_ymd(4, 4, 1).unwrap().as_ymd());
        assert_eq!((4, 6, 1), DateTime::from_ymd(4, 6, 1).unwrap().as_ymd());
        assert_eq!((4, 7, 1), DateTime::from_ymd(4, 7, 1).unwrap().as_ymd());
        assert_eq!((4, 8, 1), DateTime::from_ymd(4, 8, 1).unwrap().as_ymd());
        assert_eq!((4, 9, 1), DateTime::from_ymd(4, 9, 1).unwrap().as_ymd());
        assert_eq!((4, 10, 1), DateTime::from_ymd(4, 10, 1).unwrap().as_ymd());
        assert_eq!((4, 11, 1), DateTime::from_ymd(4, 11, 1).unwrap().as_ymd());
        assert_eq!((1, 8, 1), DateTime::from_ymd(1, 8, 1).unwrap().as_ymd());
        assert_eq!((1, 11, 1), DateTime::from_ymd(1, 11, 1).unwrap().as_ymd());
    }
}
