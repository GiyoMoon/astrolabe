#[cfg(test)]
mod date_tests {
    use astrolabe::{Date, DateUnit, DateUtilities};

    #[test]
    fn derive() {
        // Default
        let date = Date::default();
        // Debug
        println!("{:?}", date);
        // Display
        assert_eq!("0001/01/01", format!("{}", date));
        // From<&Date>
        let _ = Date::from(&date);
        // Clone
        #[allow(clippy::clone_on_copy)]
        let clone = date.clone();
        // PartialEq
        assert!(date == clone);

        let clone = date.apply(1, DateUnit::Day).unwrap();
        // PartialEq
        assert!(date != clone);

        // Ord
        assert!(date < clone);

        // PartialOrd
        assert_eq!(std::cmp::Ordering::Less, date.cmp(&clone));

        let unit = DateUnit::Day;
        // Debug
        println!("{:?}", unit);
        // Clone
        #[allow(clippy::redundant_clone)]
        let clone = unit.clone();
        // PartialEq
        assert!(unit == clone);

        assert!("2022-05-02".parse::<Date>().is_ok());
        assert!("blabla".parse::<Date>().is_err());
    }

    #[test]
    fn now() {
        assert!(2021 < Date::now().get(DateUnit::Year));
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
            Date::from_ymd(year, month, day).unwrap().as_ymd()
        );
    }

    fn from_ymd_err(year: i32, month: u32, day: u32) {
        assert!(Date::from_ymd(year, month, day).is_err());
    }

    #[test]
    fn timestamp() {
        assert_eq!(0, Date::from_timestamp(0).unwrap().timestamp());
        assert_eq!(
            "1970/01/01",
            Date::from_timestamp(0).unwrap().format("yyyy/MM/dd")
        );
        assert_eq!(
            "1969/12/31",
            Date::from_timestamp(-1).unwrap().format("yyyy/MM/dd")
        );
        assert_eq!(
            185_480_451_504_000,
            Date::from_timestamp(185_480_451_590_399)
                .unwrap()
                .timestamp()
        );
        assert_eq!(
            "5879611/07/12",
            Date::from_timestamp(185_480_451_590_399)
                .unwrap()
                .format("yyyy/MM/dd")
        );

        assert_eq!(
            -185_604_722_784_000,
            Date::from_timestamp(-185_604_722_784_000)
                .unwrap()
                .timestamp()
        );
        assert_eq!(
            "-5879611/06/23",
            Date::from_timestamp(-185_604_722_784_000)
                .unwrap()
                .format("yyyy/MM/dd")
        );

        assert!(Date::from_timestamp(185_480_451_590_400).is_err());
        assert!(Date::from_timestamp(-185_604_722_784_001).is_err());
    }

    #[test]
    fn get() {
        let date = Date::from_ymd(2000, 5, 10).unwrap();
        assert_eq!(2000, date.get(DateUnit::Year));
        assert_eq!(5, date.get(DateUnit::Month));
        assert_eq!(10, date.get(DateUnit::Day));
    }

    #[test]
    fn apply() {
        let date = Date::from_ymd(1970, 1, 1).unwrap();

        let modified = date.apply(123, DateUnit::Day).unwrap();
        assert_eq!(10627200, modified.timestamp());
        let modified = date.apply(11, DateUnit::Month).unwrap();
        assert_eq!("1970-12-01", modified.format("yyyy-MM-dd"));
        let modified = date.apply(12, DateUnit::Month).unwrap();
        assert_eq!("1971-01-01", modified.format("yyyy-MM-dd"));
        let modified = date.apply(14, DateUnit::Month).unwrap();
        assert_eq!("1971-03-01", modified.format("yyyy-MM-dd"));

        // Leap year cases
        let modified = date.apply(30, DateUnit::Day).unwrap();
        assert_eq!("1970-01-31", modified.format("yyyy-MM-dd"));
        let modified = modified.apply(1, DateUnit::Month).unwrap();
        assert_eq!("1970-02-28", modified.format("yyyy-MM-dd"));
        let modified = modified.apply(2, DateUnit::Year).unwrap();
        assert_eq!("1972-02-28", modified.format("yyyy-MM-dd"));
        let modified = date
            .apply(2, DateUnit::Year)
            .unwrap()
            .apply(30, DateUnit::Day)
            .unwrap();
        assert_eq!("1972-01-31", modified.format("yyyy-MM-dd"));
        let modified = modified.apply(1, DateUnit::Month).unwrap();
        assert_eq!("1972-02-29", modified.format("yyyy-MM-dd"));

        let date = Date::from_ymd(1971, 1, 1).unwrap();
        let modified = date.apply(-1, DateUnit::Month).unwrap();
        assert_eq!("1970-12-01", modified.format("yyyy-MM-dd"));

        let date = Date::from_ymd(1972, 3, 31).unwrap();
        let modified = date.apply(-1, DateUnit::Month).unwrap();
        assert_eq!("1972-02-29", modified.format("yyyy-MM-dd"));
        let modified = modified.apply(-1, DateUnit::Month).unwrap();
        assert_eq!("1972-01-29", modified.format("yyyy-MM-dd"));

        let date = Date::from_ymd(5_879_611, 7, 12).unwrap();
        assert!(date.apply(1, DateUnit::Day).is_err());
    }

    #[test]
    fn set() {
        let date = Date::from_ymd(2000, 5, 10).unwrap();
        let modified = date.set(2022, DateUnit::Year).unwrap();
        assert_eq!(2022, modified.get(DateUnit::Year));
        let modified = date.set(1, DateUnit::Month).unwrap();
        assert_eq!(2000, modified.get(DateUnit::Year));
        assert_eq!(1, modified.get(DateUnit::Month));
        let modified = date.set(13, DateUnit::Day).unwrap();
        assert_eq!(2000, modified.get(DateUnit::Year));
        assert_eq!(5, modified.get(DateUnit::Month));
        assert_eq!(13, modified.get(DateUnit::Day));

        assert!(date.set(5_879_612, DateUnit::Year).is_err());
        assert!(date.set(13, DateUnit::Month).is_err());
        assert!(date
            .set(2, DateUnit::Month)
            .unwrap()
            .set(31, DateUnit::Day)
            .is_err());
        assert!(date.set(32, DateUnit::Day).is_err());
        assert!(date.set(0, DateUnit::Year).is_err());
    }

    #[test]
    fn negative_years() {
        let date = Date::from_ymd(1, 1, 1).unwrap();
        assert_eq!("0001-01-01", date.format("yyyy-MM-dd"));
        assert_eq!("01-01-01", date.format("yy-MM-dd"));
        assert_eq!(1, date.get(DateUnit::Year));
        let date = Date::from_ymd(-1, 1, 1).unwrap();
        assert_eq!("-0001-01-01", date.format("yyyy-MM-dd"));
        assert_eq!("-01-01-01", date.format("yy-MM-dd"));
        assert_eq!(-1, date.get(DateUnit::Year));
        let date = Date::from_ymd(-2, 12, 31).unwrap();
        assert_eq!("-0002-12-31", date.format("yyyy-MM-dd"));
        assert_eq!("-02-12-31", date.format("yy-MM-dd"));
        assert_eq!(-2, date.get(DateUnit::Year));
    }
}
