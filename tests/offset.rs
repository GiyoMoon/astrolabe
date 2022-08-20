#[cfg(test)]
mod offset_tests {
    use astrolabe::{DateTime, DateTimeUnit, Offset, Time, TimeUnit};

    #[test]
    fn set_offset_time() {
        let time = Time::from_hms(0, 0, 0).unwrap();
        assert_eq!(0, time.get_offset());
        let time = time.set_offset_time(1, 0, 0, Offset::East).unwrap();
        assert_eq!(3600, time.get_offset());
        let time = time.set_offset_time(1, 0, 0, Offset::West).unwrap();
        assert_eq!(-3600, time.get_offset());
        let time = time.set_offset_time(23, 59, 59, Offset::East).unwrap();
        assert_eq!(86399, time.get_offset());
        let time = time.set_offset_time(23, 59, 59, Offset::West).unwrap();
        assert_eq!(-86399, time.get_offset());

        assert!(time.set_offset_time(24, 0, 0, Offset::East).is_err());
        assert!(time.set_offset_time(24, 0, 0, Offset::West).is_err());

        let date_time = DateTime::from_hms(0, 0, 0).unwrap();
        assert_eq!(0, date_time.get_offset());
        let date_time = date_time.set_offset_time(1, 0, 0, Offset::East).unwrap();
        assert_eq!(3600, date_time.get_offset());
        let date_time = date_time.set_offset_time(1, 0, 0, Offset::West).unwrap();
        assert_eq!(-3600, date_time.get_offset());
        let date_time = date_time.set_offset_time(23, 59, 59, Offset::East).unwrap();
        assert_eq!(86399, date_time.get_offset());
        let date_time = date_time.set_offset_time(23, 59, 59, Offset::West).unwrap();
        assert_eq!(-86399, date_time.get_offset());

        assert!(date_time.set_offset_time(24, 0, 0, Offset::East).is_err());
        assert!(date_time.set_offset_time(24, 0, 0, Offset::West).is_err());

        let date_time = DateTime::from_ymdhms(5_879_611, 7, 12, 23, 0, 0).unwrap();
        assert!(date_time.set_offset_time(0, 59, 59, Offset::East).is_ok());
        assert!(date_time.set_offset_time(1, 0, 0, Offset::East).is_err());
        let date_time = DateTime::from_ymdhms(-5_879_611, 6, 23, 1, 0, 0).unwrap();
        assert!(date_time.set_offset_time(1, 0, 0, Offset::West).is_ok());
        assert!(date_time.set_offset_time(1, 0, 1, Offset::West).is_err());
    }

    #[test]
    fn set_offset() {
        let time = Time::from_hms(0, 0, 0).unwrap();
        assert_eq!(0, time.get_offset());
        let time = time.set_offset(3600).unwrap();
        assert_eq!(3600, time.get_offset());
        let time = time.set_offset(-3600).unwrap();
        assert_eq!(-3600, time.get_offset());
        let time = time.set_offset(86399).unwrap();
        assert_eq!(86399, time.get_offset());
        let time = time.set_offset(-86399).unwrap();
        assert_eq!(-86399, time.get_offset());

        assert!(time.set_offset(86400).is_err());
        assert!(time.set_offset(-86400).is_err());

        let date_time = DateTime::from_hms(0, 0, 0).unwrap();
        assert_eq!(0, date_time.get_offset());
        let date_time = date_time.set_offset(3600).unwrap();
        assert_eq!(3600, date_time.get_offset());
        let date_time = date_time.set_offset(-3600).unwrap();
        assert_eq!(-3600, date_time.get_offset());
        let date_time = date_time.set_offset(86399).unwrap();
        assert_eq!(86399, date_time.get_offset());
        let date_time = date_time.set_offset(-86399).unwrap();
        assert_eq!(-86399, date_time.get_offset());

        assert!(date_time.set_offset(86400).is_err());
        assert!(date_time.set_offset(-86400).is_err());

        let date_time = DateTime::from_ymdhms(5_879_611, 7, 12, 23, 0, 0).unwrap();
        assert!(date_time.set_offset(3599).is_ok());
        assert!(date_time.set_offset(3600).is_err());
        let date_time = DateTime::from_ymdhms(-5_879_611, 6, 23, 1, 0, 0).unwrap();
        assert!(date_time.set_offset(-3600).is_ok());
        assert!(date_time.set_offset(-3601).is_err());
    }

    #[test]
    fn as_offset_time() {
        let time = Time::from_hms(0, 0, 0).unwrap();
        assert_eq!(0, time.get_offset());
        let time = time.as_offset_time(1, 0, 0, Offset::East).unwrap();
        assert_eq!(3600, time.get_offset());
        let time = time.as_offset_time(1, 0, 0, Offset::West).unwrap();
        assert_eq!(-3600, time.get_offset());
        let time = time.as_offset_time(23, 59, 59, Offset::East).unwrap();
        assert_eq!(86399, time.get_offset());
        let time = time.as_offset_time(23, 59, 59, Offset::West).unwrap();
        assert_eq!(-86399, time.get_offset());

        assert!(time.as_offset_time(24, 0, 0, Offset::East).is_err());
        assert!(time.as_offset_time(24, 0, 0, Offset::West).is_err());

        let date_time = DateTime::from_hms(0, 0, 0).unwrap();
        assert_eq!(0, date_time.get_offset());
        let date_time = date_time.as_offset_time(1, 0, 0, Offset::East).unwrap();
        assert_eq!(3600, date_time.get_offset());
        let date_time = date_time.as_offset_time(1, 0, 0, Offset::West).unwrap();
        assert_eq!(-3600, date_time.get_offset());
        let date_time = date_time.as_offset_time(23, 59, 59, Offset::East).unwrap();
        assert_eq!(86399, date_time.get_offset());
        let date_time = date_time.as_offset_time(23, 59, 59, Offset::West).unwrap();
        assert_eq!(-86399, date_time.get_offset());

        assert!(date_time.as_offset_time(24, 0, 0, Offset::East).is_err());
        assert!(date_time.as_offset_time(24, 0, 0, Offset::West).is_err());

        let date_time = DateTime::from_ymdhms(5_879_611, 7, 12, 23, 0, 0).unwrap();
        assert!(date_time.as_offset_time(0, 59, 59, Offset::West).is_ok());
        assert!(date_time.as_offset_time(1, 0, 0, Offset::West).is_err());
        let date_time = DateTime::from_ymdhms(-5_879_611, 6, 23, 1, 0, 0).unwrap();
        assert!(date_time.as_offset_time(1, 0, 0, Offset::East).is_ok());
        assert!(date_time.as_offset_time(1, 0, 1, Offset::East).is_err());
    }

    #[test]
    fn as_offset() {
        let time = Time::from_hms(0, 0, 0).unwrap();
        assert_eq!(0, time.get_offset());
        let time = time.as_offset(3600).unwrap();
        assert_eq!(3600, time.get_offset());
        let time = time.as_offset(-3600).unwrap();
        assert_eq!(-3600, time.get_offset());
        let time = time.as_offset(86399).unwrap();
        assert_eq!(86399, time.get_offset());
        let time = time.as_offset(-86399).unwrap();
        assert_eq!(-86399, time.get_offset());

        assert!(time.as_offset(86400).is_err());
        assert!(time.as_offset(-86400).is_err());

        let date_time = DateTime::from_hms(0, 0, 0).unwrap();
        assert_eq!(0, date_time.get_offset());
        let date_time = date_time.as_offset(3600).unwrap();
        assert_eq!(3600, date_time.get_offset());
        let date_time = date_time.as_offset(-3600).unwrap();
        assert_eq!(-3600, date_time.get_offset());
        let date_time = date_time.as_offset(86399).unwrap();
        assert_eq!(86399, date_time.get_offset());
        let date_time = date_time.as_offset(-86399).unwrap();
        assert_eq!(-86399, date_time.get_offset());

        assert!(date_time.as_offset(86400).is_err());
        assert!(date_time.as_offset(-86400).is_err());
    }

    #[test]
    fn get() {
        let time = Time::from_hms(0, 0, 0).unwrap();
        assert_eq!(0, time.get(TimeUnit::Hour));
        assert_eq!(0, time.get(TimeUnit::Min));
        assert_eq!(0, time.get(TimeUnit::Sec));
        let time = time.set_offset(3661).unwrap();
        assert_eq!(1, time.get(TimeUnit::Hour));
        assert_eq!(1, time.get(TimeUnit::Min));
        assert_eq!(1, time.get(TimeUnit::Sec));
        let time = time.set_offset(-3661).unwrap();
        assert_eq!(22, time.get(TimeUnit::Hour));
        assert_eq!(58, time.get(TimeUnit::Min));
        assert_eq!(59, time.get(TimeUnit::Sec));

        let time = Time::from_hms(0, 0, 0).unwrap();
        assert_eq!(0, time.get(TimeUnit::Hour));
        assert_eq!(0, time.get(TimeUnit::Min));
        assert_eq!(0, time.get(TimeUnit::Sec));
        let time = time.set_offset(3661).unwrap();
        assert_eq!(1, time.get(TimeUnit::Hour));
        assert_eq!(1, time.get(TimeUnit::Min));
        assert_eq!(1, time.get(TimeUnit::Sec));
        let time = time.set_offset(-3661).unwrap();
        assert_eq!(22, time.get(TimeUnit::Hour));
        assert_eq!(58, time.get(TimeUnit::Min));
        assert_eq!(59, time.get(TimeUnit::Sec));

        let date_time = DateTime::from_ymdhms(5_879_611, 7, 12, 23, 0, 0).unwrap();
        assert!(date_time.as_offset(-3599).is_ok());
        assert!(date_time.as_offset(-3600).is_err());
        let date_time = DateTime::from_ymdhms(-5_879_611, 6, 23, 1, 0, 0).unwrap();
        assert!(date_time.as_offset(3600).is_ok());
        assert!(date_time.as_offset(3601).is_err());
    }

    #[test]
    fn apply() {
        let time = Time::from_hms(0, 0, 0).unwrap().set_offset(3661).unwrap();
        assert_eq!(3661, time.apply(1, TimeUnit::Hour).unwrap().get_offset());
        let time = DateTime::from_hms(0, 0, 0)
            .unwrap()
            .set_offset(3661)
            .unwrap();
        assert_eq!(
            3661,
            time.apply(1, DateTimeUnit::Year).unwrap().get_offset()
        );
        assert_eq!(
            3661,
            time.apply(1, DateTimeUnit::Hour).unwrap().get_offset()
        );
    }

    #[test]
    fn set_get() {
        let date_time = DateTime::from_ymdhms(1970, 1, 2, 0, 0, 0)
            .unwrap()
            .set_offset(-3661)
            .unwrap();
        assert_eq!(22, date_time.get(DateTimeUnit::Hour));
        assert_eq!(1, date_time.get(DateTimeUnit::Day));
        let date_time = date_time.set(5, DateTimeUnit::Hour).unwrap();
        assert_eq!(5, date_time.get(DateTimeUnit::Hour));
        assert_eq!(1, date_time.get(DateTimeUnit::Day));
        let date_time = date_time.set(23, DateTimeUnit::Hour).unwrap();
        assert_eq!(23, date_time.get(DateTimeUnit::Hour));
        assert_eq!(1, date_time.get(DateTimeUnit::Day));
        let date_time = date_time.set(0, DateTimeUnit::Hour).unwrap();
        assert_eq!(0, date_time.get(DateTimeUnit::Hour));
        assert_eq!(1, date_time.get(DateTimeUnit::Day));
        let date_time = date_time.set(5, DateTimeUnit::Min).unwrap();
        assert_eq!(5, date_time.get(DateTimeUnit::Min));
        assert_eq!(1, date_time.get(DateTimeUnit::Day));
        let date_time = date_time.set(23, DateTimeUnit::Min).unwrap();
        assert_eq!(23, date_time.get(DateTimeUnit::Min));
        assert_eq!(1, date_time.get(DateTimeUnit::Day));
        let date_time = date_time.set(0, DateTimeUnit::Min).unwrap();
        assert_eq!(0, date_time.get(DateTimeUnit::Min));
        assert_eq!(1, date_time.get(DateTimeUnit::Day));
        let date_time = date_time.set(5, DateTimeUnit::Sec).unwrap();
        assert_eq!(5, date_time.get(DateTimeUnit::Sec));
        assert_eq!(1, date_time.get(DateTimeUnit::Day));
        let date_time = date_time.set(23, DateTimeUnit::Sec).unwrap();
        assert_eq!(23, date_time.get(DateTimeUnit::Sec));
        assert_eq!(1, date_time.get(DateTimeUnit::Day));
        let date_time = date_time.set(0, DateTimeUnit::Sec).unwrap();
        assert_eq!(0, date_time.get(DateTimeUnit::Sec));
        assert_eq!(1, date_time.get(DateTimeUnit::Day));

        let date_time = DateTime::from_ymdhms(1970, 1, 2, 0, 0, 0)
            .unwrap()
            .set_offset(3600)
            .unwrap();
        assert_eq!(1, date_time.get(DateTimeUnit::Hour));
        assert_eq!(2, date_time.get(DateTimeUnit::Day));
        let date_time = date_time.set(5, DateTimeUnit::Hour).unwrap();
        assert_eq!(5, date_time.get(DateTimeUnit::Hour));
        assert_eq!(2, date_time.get(DateTimeUnit::Day));
        let date_time = date_time.set(23, DateTimeUnit::Hour).unwrap();
        assert_eq!(23, date_time.get(DateTimeUnit::Hour));
        assert_eq!(2, date_time.get(DateTimeUnit::Day));
        let date_time = date_time.set(0, DateTimeUnit::Hour).unwrap();
        assert_eq!(0, date_time.get(DateTimeUnit::Hour));
        assert_eq!(2, date_time.get(DateTimeUnit::Day));
        let date_time = date_time.set(5, DateTimeUnit::Min).unwrap();
        assert_eq!(5, date_time.get(DateTimeUnit::Min));
        assert_eq!(2, date_time.get(DateTimeUnit::Day));
        let date_time = date_time.set(23, DateTimeUnit::Min).unwrap();
        assert_eq!(23, date_time.get(DateTimeUnit::Min));
        assert_eq!(2, date_time.get(DateTimeUnit::Day));
        let date_time = date_time.set(0, DateTimeUnit::Min).unwrap();
        assert_eq!(0, date_time.get(DateTimeUnit::Min));
        assert_eq!(2, date_time.get(DateTimeUnit::Day));
        let date_time = date_time.set(5, DateTimeUnit::Sec).unwrap();
        assert_eq!(5, date_time.get(DateTimeUnit::Sec));
        assert_eq!(2, date_time.get(DateTimeUnit::Day));
        let date_time = date_time.set(23, DateTimeUnit::Sec).unwrap();
        assert_eq!(23, date_time.get(DateTimeUnit::Sec));
        assert_eq!(2, date_time.get(DateTimeUnit::Day));
        let date_time = date_time.set(0, DateTimeUnit::Sec).unwrap();
        assert_eq!(0, date_time.get(DateTimeUnit::Sec));
        assert_eq!(2, date_time.get(DateTimeUnit::Day));

        let date_time = DateTime::from_ymdhms(1970, 1, 2, 0, 0, 0)
            .unwrap()
            .set_offset(82800)
            .unwrap();
        assert_eq!(23, date_time.get(DateTimeUnit::Hour));
        assert_eq!(2, date_time.get(DateTimeUnit::Day));
        let date_time = date_time.set(5, DateTimeUnit::Hour).unwrap();
        assert_eq!(5, date_time.get(DateTimeUnit::Hour));
        assert_eq!(2, date_time.get(DateTimeUnit::Day));
        let date_time = date_time.set(23, DateTimeUnit::Hour).unwrap();
        assert_eq!(23, date_time.get(DateTimeUnit::Hour));
        assert_eq!(2, date_time.get(DateTimeUnit::Day));
        let date_time = date_time.set(0, DateTimeUnit::Hour).unwrap();
        assert_eq!(0, date_time.get(DateTimeUnit::Hour));
        assert_eq!(2, date_time.get(DateTimeUnit::Day));
        let date_time = date_time.set(5, DateTimeUnit::Min).unwrap();
        assert_eq!(5, date_time.get(DateTimeUnit::Min));
        assert_eq!(2, date_time.get(DateTimeUnit::Day));
        let date_time = date_time.set(23, DateTimeUnit::Min).unwrap();
        assert_eq!(23, date_time.get(DateTimeUnit::Min));
        assert_eq!(2, date_time.get(DateTimeUnit::Day));
        let date_time = date_time.set(0, DateTimeUnit::Min).unwrap();
        assert_eq!(0, date_time.get(DateTimeUnit::Min));
        assert_eq!(2, date_time.get(DateTimeUnit::Day));
        let date_time = date_time.set(5, DateTimeUnit::Sec).unwrap();
        assert_eq!(5, date_time.get(DateTimeUnit::Sec));
        assert_eq!(2, date_time.get(DateTimeUnit::Day));
        let date_time = date_time.set(23, DateTimeUnit::Sec).unwrap();
        assert_eq!(23, date_time.get(DateTimeUnit::Sec));
        assert_eq!(2, date_time.get(DateTimeUnit::Day));
        let date_time = date_time.set(0, DateTimeUnit::Sec).unwrap();
        assert_eq!(0, date_time.get(DateTimeUnit::Sec));
        assert_eq!(2, date_time.get(DateTimeUnit::Day));

        let date_time = DateTime::from_ymdhms(1970, 1, 2, 0, 0, 0)
            .unwrap()
            .set_offset(-82800)
            .unwrap();
        assert_eq!(1, date_time.get(DateTimeUnit::Hour));
        assert_eq!(1, date_time.get(DateTimeUnit::Day));
        let date_time = date_time.set(5, DateTimeUnit::Hour).unwrap();
        assert_eq!(5, date_time.get(DateTimeUnit::Hour));
        assert_eq!(1, date_time.get(DateTimeUnit::Day));
        let date_time = date_time.set(23, DateTimeUnit::Hour).unwrap();
        assert_eq!(23, date_time.get(DateTimeUnit::Hour));
        assert_eq!(1, date_time.get(DateTimeUnit::Day));
        let date_time = date_time.set(0, DateTimeUnit::Hour).unwrap();
        assert_eq!(0, date_time.get(DateTimeUnit::Hour));
        assert_eq!(1, date_time.get(DateTimeUnit::Day));
        let date_time = date_time.set(5, DateTimeUnit::Min).unwrap();
        assert_eq!(5, date_time.get(DateTimeUnit::Min));
        assert_eq!(1, date_time.get(DateTimeUnit::Day));
        let date_time = date_time.set(23, DateTimeUnit::Min).unwrap();
        assert_eq!(23, date_time.get(DateTimeUnit::Min));
        assert_eq!(1, date_time.get(DateTimeUnit::Day));
        let date_time = date_time.set(0, DateTimeUnit::Min).unwrap();
        assert_eq!(0, date_time.get(DateTimeUnit::Min));
        assert_eq!(1, date_time.get(DateTimeUnit::Day));
        let date_time = date_time.set(5, DateTimeUnit::Sec).unwrap();
        assert_eq!(5, date_time.get(DateTimeUnit::Sec));
        assert_eq!(1, date_time.get(DateTimeUnit::Day));
        let date_time = date_time.set(23, DateTimeUnit::Sec).unwrap();
        assert_eq!(23, date_time.get(DateTimeUnit::Sec));
        assert_eq!(1, date_time.get(DateTimeUnit::Day));
        let date_time = date_time.set(0, DateTimeUnit::Sec).unwrap();
        assert_eq!(0, date_time.get(DateTimeUnit::Sec));
        assert_eq!(1, date_time.get(DateTimeUnit::Day));

        let date_time = DateTime::from_ymdhms(1970, 1, 2, 0, 0, 0)
            .unwrap()
            .set_offset(-43200)
            .unwrap();
        assert_eq!(12, date_time.get(DateTimeUnit::Hour));
        assert_eq!(1, date_time.get(DateTimeUnit::Day));
        let date_time = date_time.set(5, DateTimeUnit::Hour).unwrap();
        assert_eq!(5, date_time.get(DateTimeUnit::Hour));
        assert_eq!(1, date_time.get(DateTimeUnit::Day));
        let date_time = date_time.set(23, DateTimeUnit::Hour).unwrap();
        assert_eq!(23, date_time.get(DateTimeUnit::Hour));
        assert_eq!(1, date_time.get(DateTimeUnit::Day));
        let date_time = date_time.set(0, DateTimeUnit::Hour).unwrap();
        assert_eq!(0, date_time.get(DateTimeUnit::Hour));
        assert_eq!(1, date_time.get(DateTimeUnit::Day));
        let date_time = date_time.set(5, DateTimeUnit::Min).unwrap();
        assert_eq!(5, date_time.get(DateTimeUnit::Min));
        assert_eq!(1, date_time.get(DateTimeUnit::Day));
        let date_time = date_time.set(23, DateTimeUnit::Min).unwrap();
        assert_eq!(23, date_time.get(DateTimeUnit::Min));
        assert_eq!(1, date_time.get(DateTimeUnit::Day));
        let date_time = date_time.set(0, DateTimeUnit::Min).unwrap();
        assert_eq!(0, date_time.get(DateTimeUnit::Min));
        assert_eq!(1, date_time.get(DateTimeUnit::Day));
        let date_time = date_time.set(5, DateTimeUnit::Sec).unwrap();
        assert_eq!(5, date_time.get(DateTimeUnit::Sec));
        assert_eq!(1, date_time.get(DateTimeUnit::Day));
        let date_time = date_time.set(23, DateTimeUnit::Sec).unwrap();
        assert_eq!(23, date_time.get(DateTimeUnit::Sec));
        assert_eq!(1, date_time.get(DateTimeUnit::Day));
        let date_time = date_time.set(0, DateTimeUnit::Sec).unwrap();
        assert_eq!(0, date_time.get(DateTimeUnit::Sec));
        assert_eq!(1, date_time.get(DateTimeUnit::Day));
    }

    #[test]
    fn format() {
        let time = Time::from_hms(1, 1, 1).unwrap().set_offset(3661).unwrap();
        assert_eq!("02:02:02", time.format("HH:mm:ss"));

        let time = Time::from_hms(0, 0, 0).unwrap().set_offset(-3661).unwrap();
        assert_eq!("22:58:59", time.format("HH:mm:ss"));

        let date_time = DateTime::from_hms(1, 1, 1)
            .unwrap()
            .set_offset(3661)
            .unwrap();
        assert_eq!("02:02:02", date_time.format("HH:mm:ss"));

        let date_time = DateTime::from_hms(0, 0, 0)
            .unwrap()
            .set_offset(-3661)
            .unwrap();
        assert_eq!("22:58:59", date_time.format("HH:mm:ss"));
    }
}
