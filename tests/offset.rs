#[cfg(test)]
mod tests {
    use astrolabe::{DateTime, Offset, Unit};

    #[test]
    fn set_offset_time() {
        // Check error handling
        let date_time = DateTime::from_ymdhms(1970, 1, 1, 0, 0, 0).unwrap();
        assert!(date_time.set_offset_time(0, 0, 1, Offset::West).is_err());
        assert!(date_time.set_offset_time(0, 1, 0, Offset::West).is_err());
        assert!(date_time.set_offset_time(1, 0, 0, Offset::West).is_err());
        assert!(date_time.set_offset_time(24, 0, 0, Offset::East).is_err());
        assert!(date_time.set_offset_time(0, 60, 0, Offset::East).is_err());
        assert!(date_time.set_offset_time(0, 0, 60, Offset::East).is_err());
        let modified = date_time.set_offset_time(1, 1, 1, Offset::East).unwrap();
        assert_eq!(0, modified.timestamp());
        assert_eq!(3661, modified.get_offset());
        let modified = date_time
            .add(1, Unit::Day)
            .set_offset_time(1, 1, 1, Offset::West)
            .unwrap();
        assert_eq!(86400, modified.timestamp());
        assert_eq!(-3661, modified.get_offset());
    }

    #[test]
    fn set_offset() {
        // Check error handling
        let date_time = DateTime::from_ymdhms(1970, 1, 1, 0, 0, 0).unwrap();
        assert!(date_time.set_offset(-1).is_err());
        let modified = date_time.set_offset(3661).unwrap();
        assert_eq!(0, modified.timestamp());
        assert_eq!(3661, modified.get_offset());
        let modified = date_time.add(1, Unit::Day).set_offset(-3661).unwrap();
        assert_eq!(86400, modified.timestamp());
        assert_eq!(-3661, modified.get_offset());
    }

    #[test]
    fn as_offset_time() {
        // Check error handling
        let date_time = DateTime::from_ymdhms(1970, 1, 1, 0, 0, 0).unwrap();
        assert!(date_time.as_offset_time(0, 0, 1, Offset::East).is_err());
        assert!(date_time.as_offset_time(0, 1, 0, Offset::East).is_err());
        assert!(date_time.as_offset_time(1, 0, 0, Offset::East).is_err());
        assert!(date_time.as_offset_time(24, 0, 0, Offset::West).is_err());
        assert!(date_time.as_offset_time(0, 60, 0, Offset::West).is_err());
        assert!(date_time.as_offset_time(0, 0, 60, Offset::West).is_err());
        let modified = date_time.as_offset_time(1, 1, 1, Offset::West).unwrap();
        assert_eq!(3661, modified.timestamp());
        assert_eq!(-3661, modified.get_offset());
        let modified = date_time
            .add(1, Unit::Day)
            .as_offset_time(1, 1, 1, Offset::East)
            .unwrap();
        assert_eq!(82739, modified.timestamp());
        assert_eq!(3661, modified.get_offset());
    }

    #[test]
    fn as_offset() {
        // Check error handling
        let date_time = DateTime::from_ymdhms(1970, 1, 1, 0, 0, 0).unwrap();
        assert!(date_time.as_offset(1).is_err());
        let modified = date_time.as_offset(-3661).unwrap();
        assert_eq!(3661, modified.timestamp());
        assert_eq!(-3661, modified.get_offset());
        let modified = date_time.add(1, Unit::Day).as_offset(3661).unwrap();
        assert_eq!(82739, modified.timestamp());
        assert_eq!(3661, modified.get_offset());
    }

    #[test]
    fn offset_get() {
        let date_time = DateTime::from_ymdhms(1970, 1, 2, 0, 0, 0)
            .unwrap()
            .set_offset(-3661)
            .unwrap();
        assert_eq!(22, date_time.get(Unit::Hour));
        assert_eq!(1, date_time.get(Unit::Day));
        let date_time = date_time.set(5, Unit::Hour).unwrap();
        assert_eq!(5, date_time.get(Unit::Hour));
        assert_eq!(1, date_time.get(Unit::Day));
        let date_time = date_time.set(23, Unit::Hour).unwrap();
        assert_eq!(23, date_time.get(Unit::Hour));
        assert_eq!(1, date_time.get(Unit::Day));
        let date_time = date_time.set(0, Unit::Hour).unwrap();
        assert_eq!(0, date_time.get(Unit::Hour));
        assert_eq!(1, date_time.get(Unit::Day));
        let date_time = date_time.set(5, Unit::Min).unwrap();
        assert_eq!(5, date_time.get(Unit::Min));
        assert_eq!(1, date_time.get(Unit::Day));
        let date_time = date_time.set(23, Unit::Min).unwrap();
        assert_eq!(23, date_time.get(Unit::Min));
        assert_eq!(1, date_time.get(Unit::Day));
        let date_time = date_time.set(0, Unit::Min).unwrap();
        assert_eq!(0, date_time.get(Unit::Min));
        assert_eq!(1, date_time.get(Unit::Day));
        let date_time = date_time.set(5, Unit::Sec).unwrap();
        assert_eq!(5, date_time.get(Unit::Sec));
        assert_eq!(1, date_time.get(Unit::Day));
        let date_time = date_time.set(23, Unit::Sec).unwrap();
        assert_eq!(23, date_time.get(Unit::Sec));
        assert_eq!(1, date_time.get(Unit::Day));
        let date_time = date_time.set(0, Unit::Sec).unwrap();
        assert_eq!(0, date_time.get(Unit::Sec));
        assert_eq!(1, date_time.get(Unit::Day));

        let date_time = DateTime::from_ymdhms(1970, 1, 2, 0, 0, 0)
            .unwrap()
            .set_offset(3600)
            .unwrap();
        assert_eq!(1, date_time.get(Unit::Hour));
        assert_eq!(2, date_time.get(Unit::Day));
        let date_time = date_time.set(5, Unit::Hour).unwrap();
        assert_eq!(5, date_time.get(Unit::Hour));
        assert_eq!(2, date_time.get(Unit::Day));
        let date_time = date_time.set(23, Unit::Hour).unwrap();
        assert_eq!(23, date_time.get(Unit::Hour));
        assert_eq!(2, date_time.get(Unit::Day));
        let date_time = date_time.set(0, Unit::Hour).unwrap();
        assert_eq!(0, date_time.get(Unit::Hour));
        assert_eq!(2, date_time.get(Unit::Day));
        let date_time = date_time.set(5, Unit::Min).unwrap();
        assert_eq!(5, date_time.get(Unit::Min));
        assert_eq!(2, date_time.get(Unit::Day));
        let date_time = date_time.set(23, Unit::Min).unwrap();
        assert_eq!(23, date_time.get(Unit::Min));
        assert_eq!(2, date_time.get(Unit::Day));
        let date_time = date_time.set(0, Unit::Min).unwrap();
        assert_eq!(0, date_time.get(Unit::Min));
        assert_eq!(2, date_time.get(Unit::Day));
        let date_time = date_time.set(5, Unit::Sec).unwrap();
        assert_eq!(5, date_time.get(Unit::Sec));
        assert_eq!(2, date_time.get(Unit::Day));
        let date_time = date_time.set(23, Unit::Sec).unwrap();
        assert_eq!(23, date_time.get(Unit::Sec));
        assert_eq!(2, date_time.get(Unit::Day));
        let date_time = date_time.set(0, Unit::Sec).unwrap();
        assert_eq!(0, date_time.get(Unit::Sec));
        assert_eq!(2, date_time.get(Unit::Day));

        let date_time = DateTime::from_ymdhms(1970, 1, 2, 0, 0, 0)
            .unwrap()
            .set_offset(82800)
            .unwrap();
        assert_eq!(23, date_time.get(Unit::Hour));
        assert_eq!(2, date_time.get(Unit::Day));
        let date_time = date_time.set(5, Unit::Hour).unwrap();
        assert_eq!(5, date_time.get(Unit::Hour));
        assert_eq!(2, date_time.get(Unit::Day));
        let date_time = date_time.set(23, Unit::Hour).unwrap();
        assert_eq!(23, date_time.get(Unit::Hour));
        assert_eq!(2, date_time.get(Unit::Day));
        let date_time = date_time.set(0, Unit::Hour).unwrap();
        assert_eq!(0, date_time.get(Unit::Hour));
        assert_eq!(2, date_time.get(Unit::Day));
        let date_time = date_time.set(5, Unit::Min).unwrap();
        assert_eq!(5, date_time.get(Unit::Min));
        assert_eq!(2, date_time.get(Unit::Day));
        let date_time = date_time.set(23, Unit::Min).unwrap();
        assert_eq!(23, date_time.get(Unit::Min));
        assert_eq!(2, date_time.get(Unit::Day));
        let date_time = date_time.set(0, Unit::Min).unwrap();
        assert_eq!(0, date_time.get(Unit::Min));
        assert_eq!(2, date_time.get(Unit::Day));
        let date_time = date_time.set(5, Unit::Sec).unwrap();
        assert_eq!(5, date_time.get(Unit::Sec));
        assert_eq!(2, date_time.get(Unit::Day));
        let date_time = date_time.set(23, Unit::Sec).unwrap();
        assert_eq!(23, date_time.get(Unit::Sec));
        assert_eq!(2, date_time.get(Unit::Day));
        let date_time = date_time.set(0, Unit::Sec).unwrap();
        assert_eq!(0, date_time.get(Unit::Sec));
        assert_eq!(2, date_time.get(Unit::Day));

        let date_time = DateTime::from_ymdhms(1970, 1, 2, 0, 0, 0)
            .unwrap()
            .set_offset(-82800)
            .unwrap();
        assert_eq!(1, date_time.get(Unit::Hour));
        assert_eq!(1, date_time.get(Unit::Day));
        let date_time = date_time.set(5, Unit::Hour).unwrap();
        assert_eq!(5, date_time.get(Unit::Hour));
        assert_eq!(1, date_time.get(Unit::Day));
        let date_time = date_time.set(23, Unit::Hour).unwrap();
        assert_eq!(23, date_time.get(Unit::Hour));
        assert_eq!(1, date_time.get(Unit::Day));
        let date_time = date_time.set(0, Unit::Hour).unwrap();
        assert_eq!(0, date_time.get(Unit::Hour));
        assert_eq!(1, date_time.get(Unit::Day));
        let date_time = date_time.set(5, Unit::Min).unwrap();
        assert_eq!(5, date_time.get(Unit::Min));
        assert_eq!(1, date_time.get(Unit::Day));
        let date_time = date_time.set(23, Unit::Min).unwrap();
        assert_eq!(23, date_time.get(Unit::Min));
        assert_eq!(1, date_time.get(Unit::Day));
        let date_time = date_time.set(0, Unit::Min).unwrap();
        assert_eq!(0, date_time.get(Unit::Min));
        assert_eq!(1, date_time.get(Unit::Day));
        let date_time = date_time.set(5, Unit::Sec).unwrap();
        assert_eq!(5, date_time.get(Unit::Sec));
        assert_eq!(1, date_time.get(Unit::Day));
        let date_time = date_time.set(23, Unit::Sec).unwrap();
        assert_eq!(23, date_time.get(Unit::Sec));
        assert_eq!(1, date_time.get(Unit::Day));
        let date_time = date_time.set(0, Unit::Sec).unwrap();
        assert_eq!(0, date_time.get(Unit::Sec));
        assert_eq!(1, date_time.get(Unit::Day));

        let date_time = DateTime::from_ymdhms(1970, 1, 2, 0, 0, 0)
            .unwrap()
            .set_offset(-43200)
            .unwrap();
        assert_eq!(12, date_time.get(Unit::Hour));
        assert_eq!(1, date_time.get(Unit::Day));
        let date_time = date_time.set(5, Unit::Hour).unwrap();
        assert_eq!(5, date_time.get(Unit::Hour));
        assert_eq!(1, date_time.get(Unit::Day));
        let date_time = date_time.set(23, Unit::Hour).unwrap();
        assert_eq!(23, date_time.get(Unit::Hour));
        assert_eq!(1, date_time.get(Unit::Day));
        let date_time = date_time.set(0, Unit::Hour).unwrap();
        assert_eq!(0, date_time.get(Unit::Hour));
        assert_eq!(1, date_time.get(Unit::Day));
        let date_time = date_time.set(5, Unit::Min).unwrap();
        assert_eq!(5, date_time.get(Unit::Min));
        assert_eq!(1, date_time.get(Unit::Day));
        let date_time = date_time.set(23, Unit::Min).unwrap();
        assert_eq!(23, date_time.get(Unit::Min));
        assert_eq!(1, date_time.get(Unit::Day));
        let date_time = date_time.set(0, Unit::Min).unwrap();
        assert_eq!(0, date_time.get(Unit::Min));
        assert_eq!(1, date_time.get(Unit::Day));
        let date_time = date_time.set(5, Unit::Sec).unwrap();
        assert_eq!(5, date_time.get(Unit::Sec));
        assert_eq!(1, date_time.get(Unit::Day));
        let date_time = date_time.set(23, Unit::Sec).unwrap();
        assert_eq!(23, date_time.get(Unit::Sec));
        assert_eq!(1, date_time.get(Unit::Day));
        let date_time = date_time.set(0, Unit::Sec).unwrap();
        assert_eq!(0, date_time.get(Unit::Sec));
        assert_eq!(1, date_time.get(Unit::Day));
    }
}
