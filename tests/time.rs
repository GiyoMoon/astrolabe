#[cfg(test)]
mod time_tests {
    use std::time::Duration;

    use astrolabe::{DateTime, Time, TimeUtilities};

    #[test]
    fn debug() {
        let time = Time::default();
        assert_eq!("Time { nanoseconds: 0, offset: 0 }", format!("{:?}", time));
    }

    #[test]
    fn default() {
        let time = Time::default();
        assert_eq!(0, time.hour());
        assert_eq!(0, time.minute());
        assert_eq!(0, time.second());
        assert_eq!(0, time.milli());
        assert_eq!(0, time.micro());
        assert_eq!(0, time.nano());
    }

    #[test]
    fn copy() {
        let time = Time::default();
        let date_2 = time;
        assert_eq!(time, date_2);
    }

    #[test]
    fn clone() {
        let time = Time::default();
        #[allow(clippy::clone_on_copy)]
        let time_2 = time.clone();
        assert_eq!(time, time_2);
    }

    #[test]
    fn eq() {
        let time = Time::default();
        let time_2 = time;
        assert!(time == time_2);
    }

    #[test]
    fn ord() {
        let time = Time::default();
        let time_2 = time.add_hours(1).unwrap();
        assert!(time < time_2);
        assert_eq!(std::cmp::Ordering::Less, time.cmp(&time_2));
    }

    #[test]
    fn now() {
        let _ = Time::now();
    }

    #[test]
    fn from_hms() {
        // check allowed limits
        from_hms_ok(0, 0, 0);
        from_hms_ok(23, 0, 0);
        from_hms_ok(0, 59, 0);
        from_hms_ok(0, 0, 59);
        from_hms_ok(23, 59, 59);

        // check invalid limits
        from_hms_err(24, 0, 0);
        from_hms_err(0, 60, 0);
        from_hms_err(0, 0, 60);
        from_hms_err(24, 60, 60);
    }

    fn from_hms_ok(hour: u32, minute: u32, second: u32) {
        assert_eq!(
            (hour, minute, second),
            Time::from_hms(hour, minute, second).unwrap().as_hms()
        );
    }

    fn from_hms_err(hour: u32, minute: u32, second: u32) {
        assert!(Time::from_hms(hour, minute, second).is_err());
    }

    #[test]
    fn seconds() {
        assert_eq!(0, Time::from_seconds(0).unwrap().as_seconds());
        assert_eq!(86399, Time::from_seconds(86399).unwrap().as_seconds());
        assert!(Time::from_seconds(86400).is_err())
    }

    #[test]
    fn nanoseconds() {
        assert_eq!(0, Time::from_nanos(0).unwrap().as_nanos());
        assert_eq!(
            86_399_999_999_999,
            Time::from_nanos(86_399_999_999_999).unwrap().as_nanos()
        );
        assert!(Time::from_nanos(86_400_000_000_000).is_err())
    }

    #[test]
    fn duration_between() {
        duration_between_ok(Duration::from_secs(0), Time::default(), Time::default());
        duration_between_ok(
            Duration::from_secs(0),
            Time::from_hms(12, 32, 1).unwrap(),
            Time::from_hms(12, 32, 1).unwrap(),
        );
        duration_between_ok(
            Duration::from_secs(1),
            Time::from_hms(12, 32, 1).unwrap(),
            Time::from_hms(12, 32, 2).unwrap(),
        );
        duration_between_ok(
            Duration::from_secs(58),
            Time::from_hms(12, 32, 1).unwrap(),
            Time::from_hms(12, 32, 59).unwrap(),
        );
        duration_between_ok(
            Duration::from_secs(1),
            Time::from_hms(12, 32, 3).unwrap(),
            Time::from_hms(12, 32, 2).unwrap(),
        );
        duration_between_ok(
            Duration::from_secs(86_399),
            Time::from_hms(0, 0, 0).unwrap(),
            Time::from_hms(23, 59, 59).unwrap(),
        );
    }

    fn duration_between_ok(expected: Duration, start: Time, end: Time) {
        assert_eq!(expected, start.duration_between(&end));
    }

    #[test]
    fn get() {
        let time = Time::from_hms(12, 32, 1)
            .unwrap()
            .set_nano(123456789)
            .unwrap();

        assert_eq!(12, time.hour());
        assert_eq!(32, time.minute());
        assert_eq!(1, time.second());
        assert_eq!(123, time.milli());
        assert_eq!(123_456, time.micro());
        assert_eq!(123_456_789, time.nano());

        let time = Time::from_hms(0, 0, 0).unwrap().set_nano(0).unwrap();

        assert_eq!(0, time.hour());
        assert_eq!(0, time.minute());
        assert_eq!(0, time.second());
        assert_eq!(0, time.milli());
        assert_eq!(0, time.micro());
        assert_eq!(0, time.nano());
    }

    #[test]
    fn add_sub() {
        let mut time = Time::default().add_hours(1).unwrap();

        time = time.add_hours(1).unwrap();
        time = time.add_minutes(2).unwrap();
        time = time.add_seconds(3).unwrap();
        time = time.add_millis(5).unwrap();
        time = time.add_micros(6).unwrap();
        time = time.add_nanos(7).unwrap();

        assert_eq!(2, time.hour());
        assert_eq!(2, time.minute());
        assert_eq!(3, time.second());
        assert_eq!(5, time.milli());
        assert_eq!(5_006, time.micro());
        assert_eq!(5_006_007, time.nano());

        time = time.sub_hours(2).unwrap();
        time = time.sub_minutes(2).unwrap();
        time = time.sub_seconds(3).unwrap();
        time = time.sub_millis(5).unwrap();
        time = time.sub_micros(6).unwrap();
        time = time.sub_nanos(7).unwrap();

        assert_eq!(0, time.as_nanos());

        assert!(time.sub_nanos(1).is_err());
        assert!(time.add_hours(24).is_err());

        let time = Time::from_hms(23, 59, 59)
            .unwrap()
            .set_nano(999_999_999)
            .unwrap();
        assert!(time.add_hours(1).is_err());
        assert!(time.add_minutes(1).is_err());
        assert!(time.add_seconds(1).is_err());
        assert!(time.add_millis(1).is_err());
        assert!(time.add_micros(1).is_err());
        assert!(time.add_nanos(1).is_err());

        let time = Time::default();
        assert!(time.sub_hours(1).is_err());
        assert!(time.sub_minutes(1).is_err());
        assert!(time.sub_seconds(1).is_err());
        assert!(time.sub_millis(1).is_err());
        assert!(time.sub_micros(1).is_err());
        assert!(time.sub_nanos(1).is_err());
    }

    #[test]
    fn set() {
        let mut time = Time::default().add_nanos(u32::MAX).unwrap();

        time = time.set_hour(1).unwrap();
        time = time.set_minute(2).unwrap();
        time = time.set_second(3).unwrap();

        assert_eq!(1, time.hour());
        assert_eq!(2, time.minute());
        assert_eq!(3, time.second());

        time = time.set_milli(5).unwrap();
        assert_eq!(5, time.milli());
        time = time.set_micro(6).unwrap();
        assert_eq!(6, time.micro());
        time = time.set_nano(7).unwrap();
        assert_eq!(7, time.nano());

        assert!(time.set_hour(24).is_err());
        assert!(time.set_minute(60).is_err());
        assert!(time.set_second(60).is_err());
        assert!(time.set_milli(1_000).is_err());
        assert!(time.set_micro(1_000_000).is_err());
        assert!(time.set_nano(1_000_000_000).is_err());
    }

    #[test]
    fn clear() {
        let time = Time::from_hms(12, 32, 1)
            .unwrap()
            .set_nano(123456789)
            .unwrap();
        let modified = time.clear_until_hour();
        assert_eq!(0, modified.hour());
        assert_eq!(0, modified.minute());
        assert_eq!(0, modified.second());
        assert_eq!(0, modified.milli());
        assert_eq!(0, modified.micro());
        assert_eq!(0, modified.nano());
        let modified = time.clear_until_minute();
        assert_eq!(12, modified.hour());
        assert_eq!(0, modified.minute());
        assert_eq!(0, modified.second());
        assert_eq!(0, modified.milli());
        assert_eq!(0, modified.micro());
        assert_eq!(0, modified.nano());
        let modified = time.clear_until_second();
        assert_eq!(12, modified.hour());
        assert_eq!(32, modified.minute());
        assert_eq!(0, modified.second());
        assert_eq!(0, modified.milli());
        assert_eq!(0, modified.micro());
        assert_eq!(0, modified.nano());
        let modified = time.clear_until_milli();
        assert_eq!(12, modified.hour());
        assert_eq!(32, modified.minute());
        assert_eq!(1, modified.second());
        assert_eq!(0, modified.milli());
        assert_eq!(0, modified.micro());
        assert_eq!(0, modified.nano());
        let modified = time.clear_until_micro();
        assert_eq!(12, modified.hour());
        assert_eq!(32, modified.minute());
        assert_eq!(1, modified.second());
        assert_eq!(123, modified.milli());
        assert_eq!(123000, modified.micro());
        assert_eq!(123000000, modified.nano());
        let modified = time.clear_until_nano();
        assert_eq!(12, modified.hour());
        assert_eq!(32, modified.minute());
        assert_eq!(1, modified.second());
        assert_eq!(123, modified.milli());
        assert_eq!(123456, modified.micro());
        assert_eq!(123456000, modified.nano());
    }

    #[test]
    fn since() {
        let time = Time::default();
        assert_eq!(0, time.hours_since(&time));
        assert_eq!(0, time.minutes_since(&time));
        assert_eq!(0, time.seconds_since(&time));
        assert_eq!(0, time.millis_since(&time));
        assert_eq!(0, time.micros_since(&time));
        assert_eq!(0, time.nanos_since(&time));
        let time2 = Time::from_hms(0, 59, 59).unwrap().set_nano(0).unwrap();
        assert_eq!(0, time2.hours_since(&time));
        assert_eq!(59, time2.minutes_since(&time));
        assert_eq!(3_599, time2.seconds_since(&time));
        assert_eq!(3_599_000, time2.millis_since(&time));
        assert_eq!(3_599_000_000, time2.micros_since(&time));
        assert_eq!(3_599_000_000_000, time2.nanos_since(&time));
        assert_eq!(0, time.hours_since(&time2));
        assert_eq!(-59, time.minutes_since(&time2));
        assert_eq!(-3_599, time.seconds_since(&time2));
        assert_eq!(-3_599_000, time.millis_since(&time2));
        assert_eq!(-3_599_000_000, time.micros_since(&time2));
        assert_eq!(-3_599_000_000_000, time.nanos_since(&time2));
        let time2 = Time::from_hms(0, 0, 0).unwrap().set_nano(1).unwrap();
        assert_eq!(0, time2.hours_since(&time));
        assert_eq!(0, time2.minutes_since(&time));
        assert_eq!(0, time2.seconds_since(&time));
        assert_eq!(0, time2.millis_since(&time));
        assert_eq!(0, time2.micros_since(&time));
        assert_eq!(1, time2.nanos_since(&time));
        assert_eq!(0, time.hours_since(&time2));
        assert_eq!(0, time.minutes_since(&time2));
        assert_eq!(0, time.seconds_since(&time2));
        assert_eq!(0, time.millis_since(&time2));
        assert_eq!(0, time.micros_since(&time2));
        assert_eq!(-1, time.nanos_since(&time2));
        let time2 = Time::from_hms(23, 59, 59)
            .unwrap()
            .set_nano(999_999_999)
            .unwrap();
        assert_eq!(23, time2.hours_since(&time));
        assert_eq!(1_439, time2.minutes_since(&time));
        assert_eq!(86_399, time2.seconds_since(&time));
        assert_eq!(86_399_999, time2.millis_since(&time));
        assert_eq!(86_399_999_999, time2.micros_since(&time));
        assert_eq!(86_399_999_999_999, time2.nanos_since(&time));
        assert_eq!(-23, time.hours_since(&time2));
        assert_eq!(-1_439, time.minutes_since(&time2));
        assert_eq!(-86_399, time.seconds_since(&time2));
        assert_eq!(-86_399_999, time.millis_since(&time2));
        assert_eq!(-86_399_999_999, time.micros_since(&time2));
        assert_eq!(-86_399_999_999_999, time.nanos_since(&time2));

        let time = Time::from_hms(0, 0, 0)
            .unwrap()
            .set_nano(999_999_999)
            .unwrap();
        let time2 = Time::from_hms(1, 0, 0).unwrap().set_nano(0).unwrap();
        assert_eq!(0, time2.hours_since(&time));
        assert_eq!(59, time2.minutes_since(&time));
        assert_eq!(3_599, time2.seconds_since(&time));
        assert_eq!(3_599_000, time2.millis_since(&time));
        assert_eq!(3_599_000_000, time2.micros_since(&time));
        assert_eq!(3_599_000_000_001, time2.nanos_since(&time));
        assert_eq!(0, time.hours_since(&time2));
        assert_eq!(-59, time.minutes_since(&time2));
        assert_eq!(-3_599, time.seconds_since(&time2));
        assert_eq!(-3_599_000, time.millis_since(&time2));
        assert_eq!(-3_599_000_000, time.micros_since(&time2));
        assert_eq!(-3_599_000_000_001, time.nanos_since(&time2));
    }

    #[test]
    fn from() {
        let time = Time::from_hms(12, 32, 1)
            .unwrap()
            .set_nano(123_456_789)
            .unwrap();
        assert_eq!(
            "12:32:01.123456789",
            Time::from(&time).format("HH:mm:ss.nnnnn")
        );
        let date_time = DateTime::from_ymdhms(2022, 5, 10, 12, 32, 1)
            .unwrap()
            .set_nano(123456789)
            .unwrap();
        assert_eq!(
            "12:32:01.123456789",
            Time::from(date_time).format("HH:mm:ss.nnnnn")
        );
        assert_eq!(
            "12:32:01.123456789",
            Time::from(&date_time).format("HH:mm:ss.nnnnn")
        );
    }

    #[test]
    fn display() {
        let time = Time::from_hms(12, 31, 1).unwrap();
        assert_eq!("12:31:01", format!("{}", time));
    }

    #[test]
    fn time_add() {
        let mut time = Time::from_hms(12, 32, 1).unwrap();
        let modified = time + Time::from_seconds(60 * 3 + 3).unwrap();
        assert_eq!("12:35:04.000000000", modified.format("HH:mm:ss.nnnnn"));
        time += Time::from_seconds(60 * 3 + 3).unwrap();
        assert_eq!("12:35:04.000000000", time.format("HH:mm:ss.nnnnn"));
    }

    #[test]
    fn time_sub() {
        let mut time = Time::from_hms(12, 32, 1).unwrap();
        let modified = time - Time::from_seconds(60 * 3 + 3).unwrap();
        assert_eq!("12:28:58.000000000", modified.format("HH:mm:ss.nnnnn"));
        time -= Time::from_seconds(60 * 3 + 3).unwrap();
        assert_eq!("12:28:58.000000000", time.format("HH:mm:ss.nnnnn"));
    }

    #[test]
    fn std_add() {
        let mut time = Time::from_hms(12, 32, 1).unwrap();
        let modified = time + Duration::from_secs(60 * 3 + 3);
        assert_eq!("12:35:04.000000000", modified.format("HH:mm:ss.nnnnn"));
        time += Duration::from_secs(60 * 3 + 3);
        assert_eq!("12:35:04.000000000", time.format("HH:mm:ss.nnnnn"));
    }

    #[test]
    fn std_sub() {
        let mut time = Time::from_hms(12, 32, 1).unwrap();
        let modified = time - Duration::from_secs(60 * 3 + 3);
        assert_eq!("12:28:58.000000000", modified.format("HH:mm:ss.nnnnn"));
        time -= Duration::from_secs(60 * 3 + 3);
        assert_eq!("12:28:58.000000000", time.format("HH:mm:ss.nnnnn"));
    }
}
