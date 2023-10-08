#[cfg(test)]
mod offset_tests {
    use astrolabe::{DateTime, DateUtilities, Offset, OffsetUtilities, Time, TimeUtilities};

    #[test]
    fn offset() {
        assert!(Offset::from_hms(0, 0, 0).is_ok());
        assert!(Offset::from_hms(0, 0, 59).is_ok());
        assert!(Offset::from_hms(0, 59, 59).is_ok());
        assert!(Offset::from_hms(23, 59, 59).is_ok());
        assert!(Offset::from_hms(24, 59, 59).is_err());
        assert!(Offset::from_hms(23, 60, 59).is_err());
        assert!(Offset::from_hms(23, 59, 60).is_err());

        assert!(Offset::from_hms(-23, 0, 0).is_ok());
        assert!(Offset::from_hms(-23, 59, 59).is_ok());
        assert!(Offset::from_hms(-24, 59, 59).is_err());
        assert!(Offset::from_hms(-23, 60, 59).is_err());
        assert!(Offset::from_hms(-23, 59, 60).is_err());

        assert!(Offset::from_seconds(0).is_ok());
        assert!(Offset::from_seconds(-86_399).is_ok());
        assert!(Offset::from_seconds(86_399).is_ok());
        assert!(Offset::from_seconds(-86_400).is_err());
        assert!(Offset::from_seconds(86_400).is_err());
    }

    #[test]
    fn set_offset_hms() {
        let time = Time::from_hms(0, 0, 0).unwrap();
        assert_eq!(0, time.get_offset().resolve());
        assert_eq!((0, 0, 0), time.get_offset().resolve_hms());
        let time = time.set_offset(Offset::from_hms(1, 0, 0).unwrap());
        assert_eq!(3600, time.get_offset().resolve());
        assert_eq!((1, 0, 0), time.get_offset().resolve_hms());
        let time = time.set_offset(Offset::from_hms(-1, 0, 0).unwrap());
        assert_eq!(-3600, time.get_offset().resolve());
        assert_eq!((-1, 0, 0), time.get_offset().resolve_hms());
        let time = time.set_offset(Offset::from_hms(23, 59, 59).unwrap());
        assert_eq!(86399, time.get_offset().resolve());
        assert_eq!((23, 59, 59), time.get_offset().resolve_hms());
        let time = time.set_offset(Offset::from_hms(-23, 59, 59).unwrap());
        assert_eq!(-86399, time.get_offset().resolve());
        assert_eq!((-23, 59, 59), time.get_offset().resolve_hms());

        let date_time = DateTime::from_hms(0, 0, 0).unwrap();
        assert_eq!(0, date_time.get_offset().resolve());
        assert_eq!((0, 0, 0), date_time.get_offset().resolve_hms());
        let date_time = date_time.set_offset(Offset::from_hms(1, 0, 0).unwrap());
        assert_eq!(3600, date_time.get_offset().resolve());
        assert_eq!((1, 0, 0), date_time.get_offset().resolve_hms());
        let date_time = date_time.set_offset(Offset::from_hms(-1, 0, 0).unwrap());
        assert_eq!(-3600, date_time.get_offset().resolve());
        assert_eq!((-1, 0, 0), date_time.get_offset().resolve_hms());
        let date_time = date_time.set_offset(Offset::from_hms(23, 59, 59).unwrap());
        assert_eq!(86399, date_time.get_offset().resolve());
        assert_eq!((23, 59, 59), date_time.get_offset().resolve_hms());
        let date_time = date_time.set_offset(Offset::from_hms(-23, 59, 59).unwrap());
        assert_eq!(-86399, date_time.get_offset().resolve());
        assert_eq!((-23, 59, 59), date_time.get_offset().resolve_hms());
    }

    #[test]
    fn set_offset() {
        let time = Time::from_hms(0, 0, 0).unwrap();
        assert_eq!(0, time.get_offset().resolve());
        let time = time.set_offset(Offset::from_seconds(3600).unwrap());
        assert_eq!(3600, time.get_offset().resolve());
        let time = time.set_offset(Offset::from_seconds(-3600).unwrap());
        assert_eq!(-3600, time.get_offset().resolve());
        let time = time.set_offset(Offset::from_seconds(86399).unwrap());
        assert_eq!(86399, time.get_offset().resolve());
        let time = time.set_offset(Offset::from_seconds(-86399).unwrap());
        assert_eq!(-86399, time.get_offset().resolve());

        assert!(Offset::from_seconds(86400).is_err());
        assert!(Offset::from_seconds(-86400).is_err());

        let date_time = DateTime::from_hms(0, 0, 0).unwrap();
        assert_eq!(0, date_time.get_offset().resolve());
        let date_time = date_time.set_offset(Offset::from_seconds(3600).unwrap());
        assert_eq!(3600, date_time.get_offset().resolve());
        let date_time = date_time.set_offset(Offset::from_seconds(-3600).unwrap());
        assert_eq!(-3600, date_time.get_offset().resolve());
        let date_time = date_time.set_offset(Offset::from_seconds(86399).unwrap());
        assert_eq!(86399, date_time.get_offset().resolve());
        let date_time = date_time.set_offset(Offset::from_seconds(-86399).unwrap());
        assert_eq!(-86399, date_time.get_offset().resolve());

        assert!(Offset::from_seconds(86400).is_err());
        assert!(Offset::from_seconds(-86400).is_err());
    }

    #[test]
    fn as_offset_hms() {
        assert!(Offset::from_hms(24, 0, 0).is_err());
        assert!(Offset::from_hms(-24, 0, 0).is_err());

        let time = Time::from_hms(0, 0, 0).unwrap();
        assert_eq!(0, time.get_offset().resolve());
        let time = time.as_offset(Offset::from_hms(1, 0, 0).unwrap());
        assert_eq!(3600, time.get_offset().resolve());
        let time = time.as_offset(Offset::from_hms(-1, 0, 0).unwrap());
        assert_eq!(-3600, time.get_offset().resolve());
        let time = time.as_offset(Offset::from_hms(23, 59, 59).unwrap());
        assert_eq!(86399, time.get_offset().resolve());
        let time = time.as_offset(Offset::from_hms(-23, 59, 59).unwrap());
        assert_eq!(-86399, time.get_offset().resolve());

        let date_time = DateTime::from_hms(0, 0, 0).unwrap();
        assert_eq!(0, date_time.get_offset().resolve());
        let date_time = date_time.as_offset(Offset::from_hms(1, 0, 0).unwrap());
        assert_eq!(3600, date_time.get_offset().resolve());
        let date_time = date_time.as_offset(Offset::from_hms(-1, 0, 0).unwrap());
        assert_eq!(-3600, date_time.get_offset().resolve());
        let date_time = date_time.as_offset(Offset::from_hms(23, 59, 59).unwrap());
        assert_eq!(86399, date_time.get_offset().resolve());
        let date_time = date_time.as_offset(Offset::from_hms(-23, 59, 59).unwrap());
        assert_eq!(-86399, date_time.get_offset().resolve());
    }

    #[test]
    fn as_offset() {
        let time = Time::from_hms(0, 0, 0).unwrap();
        assert_eq!(0, time.get_offset().resolve());
        let time = time.as_offset(Offset::from_seconds(3600).unwrap());
        assert_eq!(3600, time.get_offset().resolve());
        let time = time.as_offset(Offset::from_seconds(-3600).unwrap());
        assert_eq!(-3600, time.get_offset().resolve());
        let time = time.as_offset(Offset::from_seconds(86399).unwrap());
        assert_eq!(86399, time.get_offset().resolve());
        let time = time.as_offset(Offset::from_seconds(-86399).unwrap());
        assert_eq!(-86399, time.get_offset().resolve());

        assert!(Offset::from_seconds(86400).is_err());
        assert!(Offset::from_seconds(-86400).is_err());

        let date_time = DateTime::from_hms(0, 0, 0).unwrap();
        assert_eq!(0, date_time.get_offset().resolve());
        let date_time = date_time.as_offset(Offset::from_seconds(3600).unwrap());
        assert_eq!(3600, date_time.get_offset().resolve());
        let date_time = date_time.as_offset(Offset::from_seconds(-3600).unwrap());
        assert_eq!(-3600, date_time.get_offset().resolve());
        let date_time = date_time.as_offset(Offset::from_seconds(86399).unwrap());
        assert_eq!(86399, date_time.get_offset().resolve());
        let date_time = date_time.as_offset(Offset::from_seconds(-86399).unwrap());
        assert_eq!(-86399, date_time.get_offset().resolve());

        assert!(Offset::from_seconds(86400).is_err());
        assert!(Offset::from_seconds(-86400).is_err());
    }

    #[test]
    fn get() {
        let time = Time::from_hms(0, 0, 0).unwrap();
        assert_eq!(0, time.hour());
        assert_eq!(0, time.minute());
        assert_eq!(0, time.second());
        let time = time.set_offset(Offset::from_seconds(3661).unwrap());
        assert_eq!(1, time.hour());
        assert_eq!(1, time.minute());
        assert_eq!(1, time.second());
        let time = time.set_offset(Offset::from_seconds(-3661).unwrap());
        assert_eq!(22, time.hour());
        assert_eq!(58, time.minute());
        assert_eq!(59, time.second());

        let time = Time::from_hms(0, 0, 0).unwrap();
        assert_eq!(0, time.hour());
        assert_eq!(0, time.minute());
        assert_eq!(0, time.second());
        let time = time.set_offset(Offset::from_seconds(3661).unwrap());
        assert_eq!(1, time.hour());
        assert_eq!(1, time.minute());
        assert_eq!(1, time.second());
        let time = time.set_offset(Offset::from_seconds(-3661).unwrap());
        assert_eq!(22, time.hour());
        assert_eq!(58, time.minute());
        assert_eq!(59, time.second());

        let date_time = DateTime::now().set_offset(Offset::Local);
        assert_eq!(Offset::Local, date_time.get_offset());
        date_time.get_offset().resolve();
    }

    #[test]
    fn apply() {
        let time = Time::from_hms(0, 0, 0)
            .unwrap()
            .set_offset(Offset::from_seconds(3661).unwrap());
        assert_eq!(3661, time.add_hours(1).get_offset().resolve());
        let time = DateTime::from_hms(0, 0, 0)
            .unwrap()
            .set_offset(Offset::from_seconds(3661).unwrap());
        assert_eq!(3661, time.add_years(1).get_offset().resolve());
        assert_eq!(3661, time.add_hours(1).get_offset().resolve());
    }

    #[test]
    fn set_get() {
        let date_time = DateTime::from_ymdhms(1970, 1, 2, 0, 0, 0)
            .unwrap()
            .set_offset(Offset::from_seconds(-3661).unwrap());
        assert_eq!(22, date_time.hour());
        assert_eq!(1, date_time.day());
        let date_time = date_time.set_hour(5).unwrap();
        assert_eq!(5, date_time.hour());
        assert_eq!(1, date_time.day());
        let date_time = date_time.set_hour(23).unwrap();
        assert_eq!(23, date_time.hour());
        assert_eq!(1, date_time.day());
        let date_time = date_time.set_hour(0).unwrap();
        assert_eq!(0, date_time.hour());
        assert_eq!(1, date_time.day());
        let date_time = date_time.set_minute(5).unwrap();
        assert_eq!(5, date_time.minute());
        assert_eq!(1, date_time.day());
        let date_time = date_time.set_minute(23).unwrap();
        assert_eq!(23, date_time.minute());
        assert_eq!(1, date_time.day());
        let date_time = date_time.set_minute(0).unwrap();
        assert_eq!(0, date_time.minute());
        assert_eq!(1, date_time.day());
        let date_time = date_time.set_second(5).unwrap();
        assert_eq!(5, date_time.second());
        assert_eq!(1, date_time.day());
        let date_time = date_time.set_second(23).unwrap();
        assert_eq!(23, date_time.second());
        assert_eq!(1, date_time.day());
        let date_time = date_time.set_second(0).unwrap();
        assert_eq!(0, date_time.second());
        assert_eq!(1, date_time.day());

        let date_time = DateTime::from_ymdhms(1970, 1, 2, 0, 0, 0)
            .unwrap()
            .set_offset(Offset::from_seconds(3600).unwrap());
        assert_eq!(1, date_time.hour());
        assert_eq!(2, date_time.day());
        let date_time = date_time.set_hour(5).unwrap();
        assert_eq!(5, date_time.hour());
        assert_eq!(2, date_time.day());
        let date_time = date_time.set_hour(23).unwrap();
        assert_eq!(23, date_time.hour());
        assert_eq!(2, date_time.day());
        let date_time = date_time.set_hour(0).unwrap();
        assert_eq!(0, date_time.hour());
        assert_eq!(2, date_time.day());
        let date_time = date_time.set_minute(5).unwrap();
        assert_eq!(5, date_time.minute());
        assert_eq!(2, date_time.day());
        let date_time = date_time.set_minute(23).unwrap();
        assert_eq!(23, date_time.minute());
        assert_eq!(2, date_time.day());
        let date_time = date_time.set_minute(0).unwrap();
        assert_eq!(0, date_time.minute());
        assert_eq!(2, date_time.day());
        let date_time = date_time.set_second(5).unwrap();
        assert_eq!(5, date_time.second());
        assert_eq!(2, date_time.day());
        let date_time = date_time.set_second(23).unwrap();
        assert_eq!(23, date_time.second());
        assert_eq!(2, date_time.day());
        let date_time = date_time.set_second(0).unwrap();
        assert_eq!(0, date_time.second());
        assert_eq!(2, date_time.day());

        let date_time = DateTime::from_ymdhms(1970, 1, 2, 0, 0, 0)
            .unwrap()
            .set_offset(Offset::from_seconds(82800).unwrap());
        assert_eq!(23, date_time.hour());
        assert_eq!(2, date_time.day());
        let date_time = date_time.set_hour(5).unwrap();
        assert_eq!(5, date_time.hour());
        assert_eq!(2, date_time.day());
        let date_time = date_time.set_hour(23).unwrap();
        assert_eq!(23, date_time.hour());
        assert_eq!(2, date_time.day());
        let date_time = date_time.set_hour(0).unwrap();
        assert_eq!(0, date_time.hour());
        assert_eq!(2, date_time.day());
        let date_time = date_time.set_minute(5).unwrap();
        assert_eq!(5, date_time.minute());
        assert_eq!(2, date_time.day());
        let date_time = date_time.set_minute(23).unwrap();
        assert_eq!(23, date_time.minute());
        assert_eq!(2, date_time.day());
        let date_time = date_time.set_minute(0).unwrap();
        assert_eq!(0, date_time.minute());
        assert_eq!(2, date_time.day());
        let date_time = date_time.set_second(5).unwrap();
        assert_eq!(5, date_time.second());
        assert_eq!(2, date_time.day());
        let date_time = date_time.set_second(23).unwrap();
        assert_eq!(23, date_time.second());
        assert_eq!(2, date_time.day());
        let date_time = date_time.set_second(0).unwrap();
        assert_eq!(0, date_time.second());
        assert_eq!(2, date_time.day());

        let date_time = DateTime::from_ymdhms(1970, 1, 2, 0, 0, 0)
            .unwrap()
            .set_offset(Offset::from_seconds(-82800).unwrap());
        assert_eq!(1, date_time.hour());
        assert_eq!(1, date_time.day());
        let date_time = date_time.set_hour(5).unwrap();
        assert_eq!(5, date_time.hour());
        assert_eq!(1, date_time.day());
        let date_time = date_time.set_hour(23).unwrap();
        assert_eq!(23, date_time.hour());
        assert_eq!(1, date_time.day());
        let date_time = date_time.set_hour(0).unwrap();
        assert_eq!(0, date_time.hour());
        assert_eq!(1, date_time.day());
        let date_time = date_time.set_minute(5).unwrap();
        assert_eq!(5, date_time.minute());
        assert_eq!(1, date_time.day());
        let date_time = date_time.set_minute(23).unwrap();
        assert_eq!(23, date_time.minute());
        assert_eq!(1, date_time.day());
        let date_time = date_time.set_minute(0).unwrap();
        assert_eq!(0, date_time.minute());
        assert_eq!(1, date_time.day());
        let date_time = date_time.set_second(5).unwrap();
        assert_eq!(5, date_time.second());
        assert_eq!(1, date_time.day());
        let date_time = date_time.set_second(23).unwrap();
        assert_eq!(23, date_time.second());
        assert_eq!(1, date_time.day());
        let date_time = date_time.set_second(0).unwrap();
        assert_eq!(0, date_time.second());
        assert_eq!(1, date_time.day());

        let date_time = DateTime::from_ymdhms(1970, 1, 2, 0, 0, 0)
            .unwrap()
            .set_offset(Offset::from_seconds(-43200).unwrap());
        assert_eq!(12, date_time.hour());
        assert_eq!(1, date_time.day());
        let date_time = date_time.set_hour(5).unwrap();
        assert_eq!(5, date_time.hour());
        assert_eq!(1, date_time.day());
        let date_time = date_time.set_hour(23).unwrap();
        assert_eq!(23, date_time.hour());
        assert_eq!(1, date_time.day());
        let date_time = date_time.set_hour(0).unwrap();
        assert_eq!(0, date_time.hour());
        assert_eq!(1, date_time.day());
        let date_time = date_time.set_minute(5).unwrap();
        assert_eq!(5, date_time.minute());
        assert_eq!(1, date_time.day());
        let date_time = date_time.set_minute(23).unwrap();
        assert_eq!(23, date_time.minute());
        assert_eq!(1, date_time.day());
        let date_time = date_time.set_minute(0).unwrap();
        assert_eq!(0, date_time.minute());
        assert_eq!(1, date_time.day());
        let date_time = date_time.set_second(5).unwrap();
        assert_eq!(5, date_time.second());
        assert_eq!(1, date_time.day());
        let date_time = date_time.set_second(23).unwrap();
        assert_eq!(23, date_time.second());
        assert_eq!(1, date_time.day());
        let date_time = date_time.set_second(0).unwrap();
        assert_eq!(0, date_time.second());
        assert_eq!(1, date_time.day());
    }

    #[test]
    fn format() {
        let time = Time::from_hms(1, 1, 1)
            .unwrap()
            .set_offset(Offset::from_seconds(3661).unwrap());
        assert_eq!("02:02:02", time.format("HH:mm:ss"));

        let time = Time::from_hms(0, 0, 0)
            .unwrap()
            .set_offset(Offset::from_seconds(-3661).unwrap());
        assert_eq!("22:58:59", time.format("HH:mm:ss"));

        let date_time = DateTime::from_hms(1, 1, 1)
            .unwrap()
            .set_offset(Offset::from_seconds(3661).unwrap());
        assert_eq!("02:02:02", date_time.format("HH:mm:ss"));

        let date_time = DateTime::from_hms(0, 0, 0)
            .unwrap()
            .set_offset(Offset::from_seconds(-3661).unwrap());
        assert_eq!("22:58:59", date_time.format("HH:mm:ss"));
    }

    #[test]
    fn local() {
        let offset = Offset::Local;
        offset.resolve();
    }

    #[test]
    fn clone() {
        let offset = Offset::Local;
        #[allow(clippy::clone_on_copy)]
        let _ = offset.clone();
    }

    #[test]
    #[should_panic]
    fn overflow() {
        let date_time = DateTime::from_ymdhms(5_879_611, 7, 12, 23, 59, 59).unwrap();
        date_time.set_offset(Offset::Fixed(1));
    }

    #[test]
    #[should_panic]
    fn underflow() {
        let date_time = DateTime::from_ymdhms(-5_879_611, 6, 23, 0, 0, 0).unwrap();
        date_time.set_offset(Offset::Fixed(-1));
    }
}
