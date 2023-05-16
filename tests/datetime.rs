#[cfg(test)]
mod datetime_tests {
    use std::time::Duration;

    use astrolabe::{
        Date, DateTime, DateUtilities, OffsetUtilities, Precision, Time, TimeUtilities,
    };

    #[test]
    fn debug() {
        let date_time = DateTime::default();
        assert_eq!(
            "DateTime { days: 0, nanoseconds: 0, offset: 0 }",
            format!("{:?}", date_time)
        );
    }

    #[test]
    fn default() {
        let date_time = DateTime::default();
        assert_eq!(1, date_time.year());
        assert_eq!(1, date_time.month());
        assert_eq!(1, date_time.day());
    }

    #[test]
    fn copy() {
        let date_time = DateTime::default();
        let date_2 = date_time;
        assert_eq!(date_time, date_2);
    }

    #[test]
    fn clone() {
        let date_time = DateTime::default();
        #[allow(clippy::clone_on_copy)]
        let date_time_2 = date_time.clone();
        assert_eq!(date_time, date_time_2);
    }

    #[test]
    fn eq() {
        let date_time = DateTime::default();
        let date_time_2 = date_time;
        assert!(date_time == date_time_2);
    }

    #[test]
    fn ord() {
        let date_time = DateTime::default();
        let date_time_2 = date_time.add_days(1).unwrap();
        assert!(date_time < date_time_2);
        assert_eq!(std::cmp::Ordering::Less, date_time.cmp(&date_time_2));
    }

    #[test]
    fn now() {
        assert!(2021 < DateTime::now().year());
    }

    #[test]
    fn from_ymd() {
        from_ymd_ok(-1, 12, 31);
        from_ymd_ok(-451, 2, 12);
        from_ymd_ok(1998, 5, 4);

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
            Time::from(DateTime::from_hms(hour, minute, second).unwrap()).as_nanos()
                / 1_000_000_000
        );
    }

    fn from_hms_err(hour: u32, minute: u32, second: u32) {
        assert!(DateTime::from_hms(hour, minute, second).is_err());
    }

    #[test]
    fn from_ymdhms() {
        // check allowed limits
        from_ymdhms_ok(1, 1, 1, 0, 0, 0);
        from_ymdhms_ok(1, 12, 1, 0, 0, 0);
        from_ymdhms_ok(1, 1, 31, 0, 0, 0);
        from_ymdhms_ok(1, 2, 28, 0, 0, 0);
        from_ymdhms_ok(4, 2, 29, 0, 0, 0);
        from_ymdhms_ok(1, 4, 30, 0, 0, 0);
        from_ymdhms_ok(5_879_611, 7, 12, 0, 0, 0);
        from_ymdhms_ok(-5_879_611, 6, 23, 0, 0, 0);
        from_ymdhms_ok(5_879_611, 7, 12, 23, 59, 59);

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
        from_ymdhms_ok(1, 1, 1, 0, 0, 0);
        from_ymdhms_ok(1, 1, 1, 23, 0, 0);
        from_ymdhms_ok(1, 1, 1, 0, 59, 0);
        from_ymdhms_ok(1, 1, 1, 0, 0, 59);
        from_ymdhms_ok(1, 1, 1, 23, 59, 59);

        // check invalid limits
        from_ymdhms_err(1, 1, 1, 24, 0, 0);
        from_ymdhms_err(1, 1, 1, 0, 60, 0);
        from_ymdhms_err(1, 1, 1, 0, 0, 60);
        from_ymdhms_err(1, 1, 1, 24, 60, 60);
    }

    fn from_ymdhms_ok(year: i32, month: u32, day: u32, hour: u32, minute: u32, second: u32) {
        assert_eq!(
            (year, month, day, hour, minute, second),
            DateTime::from_ymdhms(year, month, day, hour, minute, second)
                .unwrap()
                .as_ymdhms()
        );
    }

    fn from_ymdhms_err(year: i32, month: u32, day: u32, hour: u32, minute: u32, second: u32) {
        assert!(DateTime::from_ymdhms(year, month, day, hour, minute, second).is_err());
    }

    #[test]
    fn duration_between() {
        duration_between_ok(
            Duration::from_secs(0),
            DateTime::from_ymd(1, 1, 1).unwrap(),
            DateTime::from_ymd(1, 1, 1).unwrap(),
        );
        duration_between_ok(
            Duration::from_secs(0),
            DateTime::from_ymd(1970, 1, 1).unwrap(),
            DateTime::from_ymd(1970, 1, 1).unwrap(),
        );
        duration_between_ok(
            Duration::from_secs(0),
            DateTime::from_ymd(2022, 5, 2).unwrap(),
            DateTime::from_ymd(2022, 5, 2).unwrap(),
        );
        duration_between_ok(
            Duration::from_secs(24 * 60 * 60),
            DateTime::from_ymd(2022, 5, 2).unwrap(),
            DateTime::from_ymd(2022, 5, 3).unwrap(),
        );
        duration_between_ok(
            Duration::from_secs(24 * 60 * 60 * 30),
            DateTime::from_ymd(2022, 5, 1).unwrap(),
            DateTime::from_ymd(2022, 5, 31).unwrap(),
        );
        duration_between_ok(
            Duration::from_secs(24 * 60 * 60),
            DateTime::from_ymd(2022, 5, 3).unwrap(),
            DateTime::from_ymd(2022, 5, 2).unwrap(),
        );
        duration_between_ok(
            Duration::from_secs(24 * 60 * 60),
            DateTime::from_ymd(-1, 12, 31).unwrap(),
            DateTime::from_ymd(1, 1, 1).unwrap(),
        );
        duration_between_ok(
            Duration::from_secs(371085174288000),
            DateTime::from_ymd(-5_879_611, 6, 23).unwrap(),
            DateTime::from_ymd(5_879_611, 7, 12).unwrap(),
        );
    }

    fn duration_between_ok(expected: Duration, start: DateTime, end: DateTime) {
        assert_eq!(expected, start.duration_between(&end));
    }

    #[test]
    fn get() {
        let date_time = DateTime::from_ymdhms(2022, 5, 2, 12, 32, 1)
            .unwrap()
            .set_nano(123456789)
            .unwrap();
        assert_eq!(2022, date_time.year());
        assert_eq!(5, date_time.month());
        assert_eq!(2, date_time.day());
        assert_eq!(122, date_time.day_of_year());
        assert_eq!(1, date_time.weekday());
        assert_eq!(12, date_time.hour());
        assert_eq!(32, date_time.minute());
        assert_eq!(1, date_time.second());
        assert_eq!(123, date_time.milli());
        assert_eq!(123456, date_time.micro());
        assert_eq!(123456789, date_time.nano());
        let date_time = DateTime::from_ymdhms(1, 1, 1, 0, 0, 0).unwrap();
        assert_eq!(1, date_time.year());
        assert_eq!(1, date_time.month());
        assert_eq!(1, date_time.day());
        assert_eq!(1, date_time.day_of_year());
        assert_eq!(1, date_time.weekday());
        assert_eq!(0, date_time.hour());
        assert_eq!(0, date_time.minute());
        assert_eq!(0, date_time.second());
        assert_eq!(0, date_time.milli());
        assert_eq!(0, date_time.micro());
        assert_eq!(0, date_time.nano());
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
        assert_eq!(
            "-5879611/06/23 00:00:01",
            DateTime::from_timestamp(-185_604_722_783_999)
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
    fn time() {
        assert_eq!(
            0,
            Time::from(DateTime::default().set_time(Time::default())).as_nanos()
        );
        assert_eq!(
            86_399_999_999_999,
            Time::from(DateTime::default().set_time(Time::from_nanos(86_399_999_999_999).unwrap()))
                .as_nanos()
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
    fn add_sub_date() {
        let date_time = DateTime::from_ymd(1970, 1, 1).unwrap();

        let modified = date_time.add_days(123).unwrap();
        assert_eq!(10627200, modified.timestamp());
        let modified = date_time.add_months(11).unwrap();
        assert_eq!("1970-12-01", modified.format("yyyy-MM-dd"));
        let modified = date_time.add_months(12).unwrap();
        assert_eq!("1971-01-01", modified.format("yyyy-MM-dd"));
        let modified = date_time.add_months(14).unwrap();
        assert_eq!("1971-03-01", modified.format("yyyy-MM-dd"));

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

        let date_time = DateTime::from_ymd(5_879_611, 7, 12).unwrap();
        assert!(date_time.add_days(1).is_err());
        let date_time = DateTime::from_ymd(5_879_611, 6, 13).unwrap();
        assert!(date_time.add_months(1).is_err());
        let date_time = DateTime::from_ymd(5_879_610, 7, 13).unwrap();
        assert!(date_time.add_years(1).is_err());

        let date_time = DateTime::from_ymd(-5_879_611, 6, 23).unwrap();
        assert!(date_time.sub_days(1).is_err());
        let date_time = DateTime::from_ymd(-5_879_611, 7, 22).unwrap();
        assert!(date_time.sub_months(1).is_err());
        let date_time = DateTime::from_ymd(-5_879_610, 6, 22).unwrap();
        assert!(date_time.sub_years(1).is_err());

        let date_time = DateTime::from_ymd(1, 1, 1).unwrap();
        assert_eq!(
            "-0001-12-31",
            date_time.sub_days(1).unwrap().format("yyyy-MM-dd")
        );

        assert_eq!(
            "-0001-12-01",
            date_time.sub_months(1).unwrap().format("yyyy-MM-dd")
        );

        assert_eq!(
            "-0001-01-01",
            date_time.sub_years(1).unwrap().format("yyyy-MM-dd")
        );

        let date_time = DateTime::from_ymd(-1, 12, 31).unwrap();
        assert_eq!(
            "-0001-12-30",
            date_time.sub_days(1).unwrap().format("yyyy-MM-dd")
        );

        assert_eq!(
            "-0001-11-30",
            date_time.sub_months(1).unwrap().format("yyyy-MM-dd")
        );

        assert_eq!(
            "-0002-12-31",
            date_time.sub_years(1).unwrap().format("yyyy-MM-dd")
        );

        let date_time = DateTime::from_ymd(1, 1, 1).unwrap();
        assert_eq!(
            "0001-01-02",
            date_time.add_days(1).unwrap().format("yyyy-MM-dd")
        );

        assert_eq!(
            "0001-02-01",
            date_time.add_months(1).unwrap().format("yyyy-MM-dd")
        );

        assert_eq!(
            "0002-01-01",
            date_time.add_years(1).unwrap().format("yyyy-MM-dd")
        );

        let date_time = DateTime::from_ymd(-1, 12, 31).unwrap();
        assert_eq!(
            "0001-01-01",
            date_time.add_days(1).unwrap().format("yyyy-MM-dd")
        );

        assert_eq!(
            "0001-01-31",
            date_time.add_months(1).unwrap().format("yyyy-MM-dd")
        );

        assert_eq!(
            "0001-12-31",
            date_time.add_years(1).unwrap().format("yyyy-MM-dd")
        );

        let date_time = DateTime::from_ymd(2020, 2, 29).unwrap();
        assert_eq!(
            "2021-02-28",
            date_time.add_years(1).unwrap().format("yyyy-MM-dd")
        );
        assert_eq!(
            "2019-02-28",
            date_time.sub_years(1).unwrap().format("yyyy-MM-dd")
        );

        let date_time = DateTime::from_ymd(1, 1, 1).unwrap();
        assert_eq!(
            "0001-02-01",
            date_time.add_months(1).unwrap().format("yyyy-MM-dd")
        );
    }

    #[test]
    fn add_sub_time() {
        let date_time = DateTime::from_ymd(5_879_611, 7, 12).unwrap();
        assert!(date_time.add_days(1).is_err());
        let date_time = DateTime::from_ymd(5_879_611, 6, 13).unwrap();
        assert!(date_time.add_months(1).is_err());
        let date_time = DateTime::from_ymd(5_879_610, 7, 13).unwrap();
        assert!(date_time.add_years(1).is_err());

        let date_time = DateTime::from_ymd(-5_879_611, 6, 23).unwrap();
        assert!(date_time.sub_days(1).is_err());
        let date_time = DateTime::from_ymd(-5_879_611, 7, 22).unwrap();
        assert!(date_time.sub_months(1).is_err());
        let date_time = DateTime::from_ymd(-5_879_610, 6, 22).unwrap();
        assert!(date_time.sub_years(1).is_err());

        let date_time = DateTime::from_ymdhms(5_879_611, 7, 12, 23, 0, 0).unwrap();
        assert!(date_time.add_hours(1).is_err());
        let date_time = DateTime::from_ymdhms(5_879_611, 7, 12, 23, 59, 0).unwrap();
        assert!(date_time.add_minutes(1).is_err());
        let date_time = DateTime::from_ymdhms(5_879_611, 7, 12, 23, 59, 59).unwrap();
        assert!(date_time.add_seconds(1).is_err());
        let date_time = DateTime::from_ymdhms(5_879_611, 7, 12, 23, 59, 59)
            .unwrap()
            .set_nano(999_000_000)
            .unwrap();
        assert!(date_time.add_millis(1).is_err());
        let date_time = DateTime::from_ymdhms(5_879_611, 7, 12, 23, 59, 59)
            .unwrap()
            .set_nano(999_999_000)
            .unwrap();

        assert!(date_time.add_micros(1).is_err());
        let date_time = DateTime::from_ymdhms(5_879_611, 7, 12, 23, 59, 59)
            .unwrap()
            .set_nano(999_999_999)
            .unwrap();
        assert!(date_time.add_nanos(1).is_err());

        let date_time = DateTime::from_ymdhms(-5_879_611, 6, 23, 0, 59, 59).unwrap();
        assert!(date_time.sub_hours(1).is_err());
        let date_time = DateTime::from_ymdhms(-5_879_611, 6, 23, 0, 0, 59).unwrap();
        assert!(date_time.sub_minutes(1).is_err());
        let date_time = DateTime::from_ymdhms(-5_879_611, 6, 23, 0, 0, 0).unwrap();
        assert!(date_time.sub_seconds(1).is_err());
        let date_time = DateTime::from_ymdhms(-5_879_611, 6, 23, 0, 0, 0)
            .unwrap()
            .set_nano(999_999)
            .unwrap();
        assert!(date_time.sub_millis(1).is_err());
        let date_time = DateTime::from_ymdhms(-5_879_611, 6, 23, 0, 0, 0)
            .unwrap()
            .set_nano(999)
            .unwrap();

        assert!(date_time.sub_micros(1).is_err());
        let date_time = DateTime::from_ymdhms(-5_879_611, 6, 23, 0, 0, 0)
            .unwrap()
            .set_nano(0)
            .unwrap();
        assert!(date_time.sub_nanos(1).is_err());

        let date_time = DateTime::from_ymd(1, 1, 1).unwrap();
        assert_eq!(
            "-0001-12-31T23:59:59.999999999Z",
            date_time
                .sub_nanos(1)
                .unwrap()
                .format_rfc3339(Precision::Nanos)
        );
        assert_eq!(
            "-0001-12-31T23:59:59.999999000Z",
            date_time
                .sub_micros(1)
                .unwrap()
                .format_rfc3339(Precision::Nanos)
        );
        assert_eq!(
            "-0001-12-31T23:59:59.999000000Z",
            date_time
                .sub_millis(1)
                .unwrap()
                .format_rfc3339(Precision::Nanos)
        );
        assert_eq!(
            "-0001-12-31T23:59:59.000000000Z",
            date_time
                .sub_seconds(1)
                .unwrap()
                .format_rfc3339(Precision::Nanos)
        );
        assert_eq!(
            "-0001-12-31T23:59:00.000000000Z",
            date_time
                .sub_minutes(1)
                .unwrap()
                .format_rfc3339(Precision::Nanos)
        );
        assert_eq!(
            "-0001-12-31T23:00:00.000000000Z",
            date_time
                .sub_hours(1)
                .unwrap()
                .format_rfc3339(Precision::Nanos)
        );
        assert_eq!(
            "-0001-12-31T00:00:00.000000000Z",
            date_time
                .sub_days(1)
                .unwrap()
                .format_rfc3339(Precision::Nanos)
        );
        assert_eq!(
            "-0001-12-01T00:00:00.000000000Z",
            date_time
                .sub_months(1)
                .unwrap()
                .format_rfc3339(Precision::Nanos)
        );
        assert_eq!(
            "-0001-01-01T00:00:00.000000000Z",
            date_time
                .sub_years(1)
                .unwrap()
                .format_rfc3339(Precision::Nanos)
        );

        let date_time = DateTime::from_ymdhms(-1, 12, 31, 23, 59, 59)
            .unwrap()
            .set_nano(999_999_999)
            .unwrap();
        assert_eq!(
            "0001-01-01T00:00:00.000000000Z",
            date_time
                .add_nanos(1)
                .unwrap()
                .format_rfc3339(Precision::Nanos)
        );
        assert_eq!(
            "0001-01-01T00:00:00.000000999Z",
            date_time
                .add_micros(1)
                .unwrap()
                .format_rfc3339(Precision::Nanos)
        );
        assert_eq!(
            "0001-01-01T00:00:00.000999999Z",
            date_time
                .add_millis(1)
                .unwrap()
                .format_rfc3339(Precision::Nanos)
        );
        assert_eq!(
            "0001-01-01T00:00:00.999999999Z",
            date_time
                .add_seconds(1)
                .unwrap()
                .format_rfc3339(Precision::Nanos)
        );
        assert_eq!(
            "0001-01-01T00:00:59.999999999Z",
            date_time
                .add_minutes(1)
                .unwrap()
                .format_rfc3339(Precision::Nanos)
        );
        assert_eq!(
            "0001-01-01T00:59:59.999999999Z",
            date_time
                .add_hours(1)
                .unwrap()
                .format_rfc3339(Precision::Nanos)
        );
        assert_eq!(
            "0001-01-01T23:59:59.999999999Z",
            date_time
                .add_days(1)
                .unwrap()
                .format_rfc3339(Precision::Nanos)
        );
        assert_eq!(
            "0001-01-31T23:59:59.999999999Z",
            date_time
                .add_months(1)
                .unwrap()
                .format_rfc3339(Precision::Nanos)
        );
        assert_eq!(
            "0001-12-31T23:59:59.999999999Z",
            date_time
                .add_years(1)
                .unwrap()
                .format_rfc3339(Precision::Nanos)
        );
    }

    #[test]
    fn set() {
        let date_time = DateTime::from_ymdhms(2000, 5, 10, 12, 32, 1)
            .unwrap()
            .set_nano(999999999)
            .unwrap();
        let modified = date_time.set_year(2022).unwrap();
        assert_eq!(2022, modified.year());
        let modified = date_time.set_month(1).unwrap();
        assert_eq!(2000, modified.year());
        assert_eq!(1, modified.month());
        let modified = date_time.set_day(13).unwrap();
        assert_eq!(2000, modified.year());
        assert_eq!(5, modified.month());
        assert_eq!(13, modified.day());
        let modified = date_time.set_hour(23).unwrap();
        assert_eq!(2000, modified.year());
        assert_eq!(5, modified.month());
        assert_eq!(10, modified.day());
        assert_eq!(23, modified.hour());
        let modified = date_time.set_minute(59).unwrap();
        assert_eq!(2000, modified.year());
        assert_eq!(5, modified.month());
        assert_eq!(10, modified.day());
        assert_eq!(12, modified.hour());
        assert_eq!(59, modified.minute());
        let modified = date_time.set_second(59).unwrap();
        assert_eq!(2000, modified.year());
        assert_eq!(5, modified.month());
        assert_eq!(10, modified.day());
        assert_eq!(12, modified.hour());
        assert_eq!(32, modified.minute());
        assert_eq!(59, modified.second());
        let modified = date_time.set_milli(111).unwrap();
        assert_eq!(2000, modified.year());
        assert_eq!(5, modified.month());
        assert_eq!(10, modified.day());
        assert_eq!(12, modified.hour());
        assert_eq!(32, modified.minute());
        assert_eq!(1, modified.second());
        assert_eq!(111, modified.milli());
        assert_eq!(111999, modified.micro());
        assert_eq!(111999999, modified.nano());
        let modified = date_time.set_micro(222).unwrap();
        assert_eq!(2000, modified.year());
        assert_eq!(5, modified.month());
        assert_eq!(10, modified.day());
        assert_eq!(12, modified.hour());
        assert_eq!(32, modified.minute());
        assert_eq!(1, modified.second());
        assert_eq!(0, modified.milli());
        assert_eq!(222, modified.micro());
        assert_eq!(222999, modified.nano());
        let modified = date_time.set_nano(333).unwrap();
        assert_eq!(2000, modified.year());
        assert_eq!(5, modified.month());
        assert_eq!(10, modified.day());
        assert_eq!(12, modified.hour());
        assert_eq!(32, modified.minute());
        assert_eq!(1, modified.second());
        assert_eq!(0, modified.milli());
        assert_eq!(0, modified.micro());
        assert_eq!(333, modified.nano());

        assert!(date_time.set_year(5_879_612).is_err());
        assert!(date_time.set_month(13).is_err());
        assert!(date_time.set_month(2).unwrap().set_day(31).is_err());
        assert!(date_time.set_day(32).is_err());
        assert!(date_time.set_year(0).is_err());
        assert!(date_time.set_hour(24).is_err());
        assert!(date_time.set_minute(60).is_err());
        assert!(date_time.set_second(60).is_err());
        assert!(date_time.set_milli(1_000).is_err());
        assert!(date_time.set_micro(1_000_000).is_err());
        assert!(date_time.set_nano(1_000_000_000).is_err());

        let date_time = DateTime::from_ymd(1, 1, 1).unwrap();
        assert_eq!(
            "0001-01-02",
            date_time.set_day(2).unwrap().format("yyyy-MM-dd")
        );
        assert_eq!(
            "0001-02-01",
            date_time.set_month(2).unwrap().format("yyyy-MM-dd")
        );
        assert_eq!(
            "0002-01-01",
            date_time.set_year(2).unwrap().format("yyyy-MM-dd")
        );
        assert_eq!(
            "-0001-01-01",
            date_time.set_year(-1).unwrap().format("yyyy-MM-dd")
        );
        assert_eq!(
            "0001-01-02",
            date_time.set_day_of_year(2).unwrap().format("yyyy-MM-dd")
        );
        assert_eq!(
            "0001-12-31",
            date_time.set_day_of_year(365).unwrap().format("yyyy-MM-dd")
        );
        assert!(date_time.set_day_of_year(366).is_err());

        let date_time = DateTime::from_ymd(-1, 1, 1).unwrap();
        assert_eq!(
            "-0001-01-02",
            date_time.set_day(2).unwrap().format("yyyy-MM-dd")
        );
        assert_eq!(
            "-0001-02-01",
            date_time.set_month(2).unwrap().format("yyyy-MM-dd")
        );
        assert_eq!(
            "-0002-01-01",
            date_time.set_year(-2).unwrap().format("yyyy-MM-dd")
        );
        assert_eq!(
            "0001-01-01",
            date_time.set_year(1).unwrap().format("yyyy-MM-dd")
        );
        assert_eq!(
            "-0001-01-02",
            date_time.set_day_of_year(2).unwrap().format("yyyy-MM-dd")
        );
        assert_eq!(
            "-0001-12-31",
            date_time.set_day_of_year(366).unwrap().format("yyyy-MM-dd")
        );
        assert!(date_time.set_day_of_year(367).is_err());
    }

    #[test]
    fn clear() {
        let date_time = DateTime::from_ymdhms(2022, 5, 10, 12, 32, 1)
            .unwrap()
            .set_nano(123456789)
            .unwrap();
        let modified = date_time.clear_until_year();
        assert_eq!(1, modified.year());
        assert_eq!(1, modified.month());
        assert_eq!(1, modified.day());
        assert_eq!(0, modified.hour());
        assert_eq!(0, modified.minute());
        assert_eq!(0, modified.second());
        assert_eq!(0, modified.milli());
        assert_eq!(0, modified.micro());
        assert_eq!(0, modified.nano());
        let modified = date_time.clear_until_month();
        assert_eq!(2022, modified.year());
        assert_eq!(1, modified.month());
        assert_eq!(1, modified.day());
        assert_eq!(0, modified.hour());
        assert_eq!(0, modified.minute());
        assert_eq!(0, modified.second());
        assert_eq!(0, modified.milli());
        assert_eq!(0, modified.micro());
        assert_eq!(0, modified.nano());
        let modified = date_time.clear_until_day();
        assert_eq!(2022, modified.year());
        assert_eq!(5, modified.month());
        assert_eq!(1, modified.day());
        assert_eq!(0, modified.hour());
        assert_eq!(0, modified.minute());
        assert_eq!(0, modified.second());
        assert_eq!(0, modified.milli());
        assert_eq!(0, modified.micro());
        assert_eq!(0, modified.nano());
        let modified = date_time.clear_until_hour();
        assert_eq!(2022, modified.year());
        assert_eq!(5, modified.month());
        assert_eq!(10, modified.day());
        assert_eq!(0, modified.hour());
        assert_eq!(0, modified.minute());
        assert_eq!(0, modified.second());
        assert_eq!(0, modified.milli());
        assert_eq!(0, modified.micro());
        assert_eq!(0, modified.nano());
        let modified = date_time.clear_until_minute();
        assert_eq!(2022, modified.year());
        assert_eq!(5, modified.month());
        assert_eq!(10, modified.day());
        assert_eq!(12, modified.hour());
        assert_eq!(0, modified.minute());
        assert_eq!(0, modified.second());
        assert_eq!(0, modified.milli());
        assert_eq!(0, modified.micro());
        assert_eq!(0, modified.nano());
        let modified = date_time.clear_until_second();
        assert_eq!(2022, modified.year());
        assert_eq!(5, modified.month());
        assert_eq!(10, modified.day());
        assert_eq!(12, modified.hour());
        assert_eq!(32, modified.minute());
        assert_eq!(0, modified.second());
        assert_eq!(0, modified.milli());
        assert_eq!(0, modified.micro());
        assert_eq!(0, modified.nano());
        let modified = date_time.clear_until_milli();
        assert_eq!(2022, modified.year());
        assert_eq!(5, modified.month());
        assert_eq!(10, modified.day());
        assert_eq!(12, modified.hour());
        assert_eq!(32, modified.minute());
        assert_eq!(1, modified.second());
        assert_eq!(0, modified.milli());
        assert_eq!(0, modified.micro());
        assert_eq!(0, modified.nano());
        let modified = date_time.clear_until_micro();
        assert_eq!(2022, modified.year());
        assert_eq!(5, modified.month());
        assert_eq!(10, modified.day());
        assert_eq!(12, modified.hour());
        assert_eq!(32, modified.minute());
        assert_eq!(1, modified.second());
        assert_eq!(123, modified.milli());
        assert_eq!(123000, modified.micro());
        assert_eq!(123000000, modified.nano());
        let modified = date_time.clear_until_nano();
        assert_eq!(2022, modified.year());
        assert_eq!(5, modified.month());
        assert_eq!(10, modified.day());
        assert_eq!(12, modified.hour());
        assert_eq!(32, modified.minute());
        assert_eq!(1, modified.second());
        assert_eq!(123, modified.milli());
        assert_eq!(123456, modified.micro());
        assert_eq!(123456000, modified.nano());

        let date_time = DateTime::from_ymd(-2022, 5, 10).unwrap();
        let modified = date_time.clear_until_year();
        assert_eq!(1, modified.year());
        assert_eq!(1, modified.month());
        assert_eq!(1, modified.day());
        let modified = date_time.clear_until_month();
        assert_eq!(-2022, modified.year());
        assert_eq!(1, modified.month());
        assert_eq!(1, modified.day());
        let modified = date_time.clear_until_day();
        assert_eq!(-2022, modified.year());
        assert_eq!(5, modified.month());
        assert_eq!(1, modified.day());
    }

    #[test]
    fn since() {
        // tests the years_since, months_since, days_since methods
        let date_time = DateTime::from_ymd(2022, 5, 10).unwrap();
        assert_eq!(0, date_time.years_since(&date_time));
        // assert_eq!(0, date_time.months_since(&date_time));
        assert_eq!(0, date_time.days_since(&date_time));
        assert_eq!(0, date_time.hours_since(&date_time));
        assert_eq!(0, date_time.minutes_since(&date_time));
        assert_eq!(0, date_time.seconds_since(&date_time));
        assert_eq!(0, date_time.millis_since(&date_time));
        assert_eq!(0, date_time.micros_since(&date_time));
        assert_eq!(0, date_time.nanos_since(&date_time));
        let date_time2 = DateTime::from_ymd(2023, 5, 10).unwrap();
        assert_eq!(1, date_time2.years_since(&date_time));
        // assert_eq!(12, date_time2.months_since(&date_time));
        assert_eq!(365, date_time2.days_since(&date_time));
        assert_eq!(8760, date_time2.hours_since(&date_time));
        assert_eq!(525_600, date_time2.minutes_since(&date_time));
        assert_eq!(31_536_000, date_time2.seconds_since(&date_time));
        assert_eq!(31_536_000_000, date_time2.millis_since(&date_time));
        assert_eq!(31_536_000_000_000, date_time2.micros_since(&date_time));
        assert_eq!(31_536_000_000_000_000, date_time2.nanos_since(&date_time));
        assert_eq!(-1, date_time.years_since(&date_time2));
        // assert_eq!(-12, date_time.months_since(&date_time2));
        assert_eq!(-365, date_time.days_since(&date_time2));
        assert_eq!(-8760, date_time.hours_since(&date_time2));
        assert_eq!(-525_600, date_time.minutes_since(&date_time2));
        assert_eq!(-31_536_000, date_time.seconds_since(&date_time2));
        assert_eq!(-31_536_000_000, date_time.millis_since(&date_time2));
        assert_eq!(-31_536_000_000_000, date_time.micros_since(&date_time2));
        assert_eq!(-31_536_000_000_000_000, date_time.nanos_since(&date_time2));
        let date_time2 = DateTime::from_ymd(2022, 6, 10).unwrap();
        assert_eq!(0, date_time2.years_since(&date_time));
        // assert_eq!(1, date_time2.months_since(&date_time));
        assert_eq!(31, date_time2.days_since(&date_time));
        assert_eq!(744, date_time2.hours_since(&date_time));
        assert_eq!(44_640, date_time2.minutes_since(&date_time));
        assert_eq!(2_678_400, date_time2.seconds_since(&date_time));
        assert_eq!(2_678_400_000, date_time2.millis_since(&date_time));
        assert_eq!(2_678_400_000_000, date_time2.micros_since(&date_time));
        assert_eq!(2_678_400_000_000_000, date_time2.nanos_since(&date_time));
        assert_eq!(0, date_time.years_since(&date_time2));
        // assert_eq!(-1, date_time.months_since(&date_time2));
        assert_eq!(-31, date_time.days_since(&date_time2));
        assert_eq!(-744, date_time.hours_since(&date_time2));
        assert_eq!(-44_640, date_time.minutes_since(&date_time2));
        assert_eq!(-2_678_400, date_time.seconds_since(&date_time2));
        assert_eq!(-2_678_400_000, date_time.millis_since(&date_time2));
        assert_eq!(-2_678_400_000_000, date_time.micros_since(&date_time2));
        assert_eq!(-2_678_400_000_000_000, date_time.nanos_since(&date_time2));
        let date_time2 = DateTime::from_ymd(2022, 5, 11).unwrap();
        assert_eq!(0, date_time2.years_since(&date_time));
        // assert_eq!(0, date_time2.months_since(&date_time));
        assert_eq!(1, date_time2.days_since(&date_time));
        assert_eq!(0, date_time.years_since(&date_time2));
        // assert_eq!(0, date_time.months_since(&date_time2));
        assert_eq!(-1, date_time.days_since(&date_time2));
        let date_time2 = DateTime::from_ymd(2023, 5, 9).unwrap();
        assert_eq!(0, date_time2.years_since(&date_time));
        // assert_eq!(11, date_time2.months_since(&date_time));
        assert_eq!(364, date_time2.days_since(&date_time));
        assert_eq!(0, date_time.years_since(&date_time2));
        // assert_eq!(-11, date_time.months_since(&date_time2));
        assert_eq!(-364, date_time.days_since(&date_time2));
        let date_time = DateTime::from_ymd(2022, 12, 1).unwrap();
        let date_time2 = DateTime::from_ymd(2023, 1, 1).unwrap();
        assert_eq!(0, date_time2.years_since(&date_time));
        // assert_eq!(1, date_time2.months_since(&date_time));
        assert_eq!(31, date_time2.days_since(&date_time));
        assert_eq!(0, date_time.years_since(&date_time2));
        // assert_eq!(-1, date_time.months_since(&date_time2));
        assert_eq!(-31, date_time.days_since(&date_time2));
        let date_time = DateTime::from_ymd(2022, 12, 31).unwrap();
        let date_time2 = DateTime::from_ymd(2023, 1, 1).unwrap();
        assert_eq!(0, date_time2.years_since(&date_time));
        // assert_eq!(0, date_time2.months_since(&date_time));
        assert_eq!(1, date_time2.days_since(&date_time));
        assert_eq!(0, date_time.years_since(&date_time2));
        // assert_eq!(0, date_time.months_since(&date_time2));
        assert_eq!(-1, date_time.days_since(&date_time2));
        let date_time = DateTime::from_ymd(-1, 12, 31).unwrap();
        let date_time2 = DateTime::from_ymd(1, 1, 1).unwrap();
        assert_eq!(0, date_time2.years_since(&date_time));
        // assert_eq!(0, date_time2.months_since(&date_time));
        assert_eq!(1, date_time2.days_since(&date_time));
        assert_eq!(24, date_time2.hours_since(&date_time));
        assert_eq!(1_440, date_time2.minutes_since(&date_time));
        assert_eq!(86_400, date_time2.seconds_since(&date_time));
        assert_eq!(86_400_000, date_time2.millis_since(&date_time));
        assert_eq!(86_400_000_000, date_time2.micros_since(&date_time));
        assert_eq!(86_400_000_000_000, date_time2.nanos_since(&date_time));
        assert_eq!(0, date_time.years_since(&date_time2));
        // assert_eq!(0, date_time.months_since(&date_time2));
        assert_eq!(-1, date_time.days_since(&date_time2));
        assert_eq!(-24, date_time.hours_since(&date_time2));
        assert_eq!(-1_440, date_time.minutes_since(&date_time2));
        assert_eq!(-86_400, date_time.seconds_since(&date_time2));
        assert_eq!(-86_400_000, date_time.millis_since(&date_time2));
        assert_eq!(-86_400_000_000, date_time.micros_since(&date_time2));
        assert_eq!(-86_400_000_000_000, date_time.nanos_since(&date_time2));
        let date_time = DateTime::from_ymdhms(-1, 12, 31, 1, 0, 0).unwrap();
        let date_time2 = DateTime::from_ymdhms(1, 1, 1, 0, 0, 0).unwrap();
        assert_eq!(0, date_time2.years_since(&date_time));
        // assert_eq!(0, date_time2.months_since(&date_time));
        assert_eq!(0, date_time2.days_since(&date_time));
        assert_eq!(23, date_time2.hours_since(&date_time));
        assert_eq!(1_380, date_time2.minutes_since(&date_time));
        assert_eq!(82_800, date_time2.seconds_since(&date_time));
        assert_eq!(82_800_000, date_time2.millis_since(&date_time));
        assert_eq!(82_800_000_000, date_time2.micros_since(&date_time));
        assert_eq!(82_800_000_000_000, date_time2.nanos_since(&date_time));
        assert_eq!(0, date_time.years_since(&date_time2));
        // assert_eq!(0, date_time.months_since(&date_time2));
        assert_eq!(0, date_time.days_since(&date_time2));
        assert_eq!(-23, date_time.hours_since(&date_time2));
        assert_eq!(-1_380, date_time.minutes_since(&date_time2));
        assert_eq!(-82_800, date_time.seconds_since(&date_time2));
        assert_eq!(-82_800_000, date_time.millis_since(&date_time2));
        assert_eq!(-82_800_000_000, date_time.micros_since(&date_time2));
        assert_eq!(-82_800_000_000_000, date_time.nanos_since(&date_time2));
        let date_time = DateTime::from_ymdhms(-1, 12, 31, 23, 59, 59)
            .unwrap()
            .set_nano(999_999_999)
            .unwrap();
        let date_time2 = DateTime::from_ymdhms(1, 1, 1, 0, 0, 0)
            .unwrap()
            .set_nano(000_000_000)
            .unwrap();
        assert_eq!(0, date_time2.years_since(&date_time));
        // assert_eq!(0, date_time2.months_since(&date_time));
        assert_eq!(0, date_time2.days_since(&date_time));
        assert_eq!(0, date_time2.hours_since(&date_time));
        assert_eq!(0, date_time2.minutes_since(&date_time));
        assert_eq!(0, date_time2.seconds_since(&date_time));
        assert_eq!(0, date_time2.millis_since(&date_time));
        assert_eq!(0, date_time2.micros_since(&date_time));
        assert_eq!(1, date_time2.nanos_since(&date_time));
        assert_eq!(0, date_time.years_since(&date_time2));
        // assert_eq!(0, date_time.months_since(&date_time2));
        assert_eq!(0, date_time.days_since(&date_time2));
        assert_eq!(0, date_time.hours_since(&date_time2));
        assert_eq!(0, date_time.minutes_since(&date_time2));
        assert_eq!(0, date_time.seconds_since(&date_time2));
        assert_eq!(0, date_time.millis_since(&date_time2));
        assert_eq!(0, date_time.micros_since(&date_time2));
        assert_eq!(-1, date_time.nanos_since(&date_time2));
    }

    #[test]
    fn from() {
        let date_time = DateTime::from_ymdhms(2022, 5, 10, 12, 32, 1)
            .unwrap()
            .set_nano(123_456_789)
            .unwrap();
        assert_eq!(
            "2022-05-10T12:32:01.123456789Z",
            DateTime::from(&date_time).format_rfc3339(Precision::Nanos)
        );
        let date = Date::from_ymd(2022, 5, 10).unwrap();
        assert_eq!(
            "2022-05-10T00:00:00.000000000Z",
            DateTime::from(date).format_rfc3339(Precision::Nanos)
        );
        assert_eq!(
            "2022-05-10T00:00:00.000000000Z",
            DateTime::from(&date).format_rfc3339(Precision::Nanos)
        );
        let time = Time::from_hms(12, 32, 1)
            .unwrap()
            .set_nano(123456789)
            .unwrap();
        assert_eq!(
            "0001-01-01T12:32:01.123456789Z",
            DateTime::from(time).format_rfc3339(Precision::Nanos)
        );
        assert_eq!(
            "0001-01-01T12:32:01.123456789Z",
            DateTime::from(&time).format_rfc3339(Precision::Nanos)
        );
    }

    #[test]
    fn display() {
        let date_time = DateTime::from_ymdhms(2022, 5, 10, 12, 31, 1).unwrap();
        assert_eq!("2022/05/10 12:31:01", format!("{}", date_time));
    }

    #[test]
    fn time_add() {
        let mut date_time = DateTime::from_ymdhms(2022, 5, 10, 12, 32, 1).unwrap();
        let modified = date_time + Time::from_seconds(60 * 3 + 3).unwrap();
        assert_eq!(
            "2022-05-10T12:35:04Z",
            modified.format_rfc3339(Precision::Seconds)
        );
        date_time += Time::from_seconds(60 * 3 + 3).unwrap();
        assert_eq!(
            "2022-05-10T12:35:04Z",
            date_time.format_rfc3339(Precision::Seconds)
        );
    }

    #[test]
    fn time_sub() {
        let mut date_time = DateTime::from_ymdhms(2022, 5, 10, 12, 32, 1).unwrap();
        let modified = date_time - Time::from_seconds(60 * 3 + 3).unwrap();
        assert_eq!(
            "2022-05-10T12:28:58Z",
            modified.format_rfc3339(Precision::Seconds)
        );
        date_time -= Time::from_seconds(60 * 3 + 3).unwrap();
        assert_eq!(
            "2022-05-10T12:28:58Z",
            date_time.format_rfc3339(Precision::Seconds)
        );
    }

    #[test]
    fn std_add() {
        let mut date_time = DateTime::from_ymd(2022, 5, 10).unwrap();
        let modified = date_time + Duration::from_secs(24 * 60 * 60);
        assert_eq!("2022-05-11", modified.format("yyyy-MM-dd"));
        date_time += Duration::from_secs(24 * 60 * 60);
        assert_eq!("2022-05-11", date_time.format("yyyy-MM-dd"));
    }

    #[test]
    fn std_sub() {
        let mut date_time = DateTime::from_ymd(2022, 5, 10).unwrap();
        let modified = date_time - Duration::from_secs(24 * 60 * 60);
        assert_eq!("2022-05-09", modified.format("yyyy-MM-dd"));
        date_time -= Duration::from_secs(24 * 60 * 60);
        assert_eq!("2022-05-09", date_time.format("yyyy-MM-dd"));
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
