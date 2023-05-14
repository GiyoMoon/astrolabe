#[cfg(test)]
mod time_tests {
    use astrolabe::{Time, TimeUnit, TimeUtilities};

    #[test]
    fn derive() {
        // Default
        let time = Time::default();
        // Debug
        println!("{:?}", time);
        // Display
        assert_eq!("00:00:00", format!("{}", time));
        // From<&DateTime>
        let _ = Time::from(&time);
        // Clone
        #[allow(clippy::clone_on_copy)]
        let clone = time.clone();
        // PartialEq
        assert!(time == clone);
        // PartialOrd
        assert_eq!(std::cmp::Ordering::Equal, time.cmp(&clone));

        let clone = time.apply(1, TimeUnit::Nanos).unwrap();
        // PartialEq
        assert!(time != clone);

        // Ord
        assert!(time < clone);
        // PartialOrd
        assert_eq!(std::cmp::Ordering::Less, time.cmp(&clone));

        let clone2 = clone.apply(-1, TimeUnit::Nanos).unwrap();
        // Ord
        assert!(clone > clone2);
        // PartialOrd
        assert_eq!(std::cmp::Ordering::Greater, clone.cmp(&clone2));

        // Check that offset doesn't affect Eq and Ord
        let clone = time.set_offset(1).unwrap();
        // PartialEq
        assert!(time == clone);
        // PartialOrd
        assert_eq!(std::cmp::Ordering::Equal, time.cmp(&clone));

        let unit = TimeUnit::Sec;
        // Debug
        println!("{:?}", unit);
        // Clone
        #[allow(clippy::redundant_clone)]
        let clone = unit.clone();
        // PartialEq
        assert!(unit == clone);

        assert!("12:32:01".parse::<Time>().is_ok());
        assert!("blabla".parse::<Time>().is_err());
    }

    #[test]
    fn now() {
        let _ = Time::now();
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

    fn from_hms_ok(expected: u32, hour: u32, minute: u32, second: u32) {
        assert_eq!(
            expected,
            Time::from_hms(hour, minute, second).unwrap().as_seconds()
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
        assert_eq!(0, Time::from_nanoseconds(0).unwrap().as_nanoseconds());
        assert_eq!(
            86_399_999_999_999,
            Time::from_nanoseconds(86_399_999_999_999)
                .unwrap()
                .as_nanoseconds()
        );
        assert!(Time::from_nanoseconds(86_400_000_000_000).is_err())
    }

    #[test]
    fn between() {
        let time1 = Time::from_seconds(0).unwrap();
        let time2 = Time::from_seconds(123).unwrap();
        assert_eq!(123_000_000_000, time1.nanos_since(&time2).unsigned_abs());
        assert_eq!(123_000_000_000, time2.nanos_since(&time1).unsigned_abs());
    }

    #[test]
    fn get() {
        let time = Time::from_nanoseconds(10_921_123_456_789).unwrap();

        assert_eq!(3, time.hour());
        assert_eq!(2, time.minute());
        assert_eq!(1, time.second());
        assert_eq!(123, time.milli());
        assert_eq!(123_456, time.micro());
        assert_eq!(123_456_789, time.nano());
    }

    #[test]
    fn apply() {
        let mut time = Time::default().apply(1, TimeUnit::Hour).unwrap();

        time = time.apply(1, TimeUnit::Hour).unwrap();
        time = time.apply(2, TimeUnit::Min).unwrap();
        time = time.apply(3, TimeUnit::Sec).unwrap();
        time = time.apply(4, TimeUnit::Centis).unwrap();
        time = time.apply(5, TimeUnit::Millis).unwrap();
        time = time.apply(6, TimeUnit::Micros).unwrap();
        time = time.apply(7, TimeUnit::Nanos).unwrap();

        assert_eq!(2, time.hour());
        assert_eq!(2, time.minute());
        assert_eq!(3, time.second());
        assert_eq!(45, time.milli());
        assert_eq!(45_006, time.micro());
        assert_eq!(45_006_007, time.nano());

        time = time.apply(-2, TimeUnit::Hour).unwrap();
        time = time.apply(-2, TimeUnit::Min).unwrap();
        time = time.apply(-3, TimeUnit::Sec).unwrap();
        time = time.apply(-4, TimeUnit::Centis).unwrap();
        time = time.apply(-5, TimeUnit::Millis).unwrap();
        time = time.apply(-6, TimeUnit::Micros).unwrap();
        time = time.apply(-7, TimeUnit::Nanos).unwrap();

        assert_eq!(0, time.as_nanoseconds());

        assert!(time.apply(-1, TimeUnit::Nanos).is_err());
        assert!(time.apply(24, TimeUnit::Hour).is_err());
    }

    #[test]
    fn set() {
        let mut time = Time::default()
            .apply(34_661_123_456_789, TimeUnit::Nanos)
            .unwrap();

        time = time.set_minute(1).unwrap();
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
    }

    #[test]
    fn implementations() {
        let default = Time::default();
        assert_eq!(0, default.as_nanoseconds());
        let time = Time::from_nanoseconds(12345).unwrap();
        let time_copy = Time::from(&time);
        assert_eq!(12345, time.as_nanoseconds());
        assert_eq!(12345, time_copy.as_nanoseconds());
    }
}
