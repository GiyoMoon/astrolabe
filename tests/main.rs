mod format;

#[cfg(test)]
mod tests {
    use astrolabe::{Date, DateUnit};

    #[test]
    fn from_ymd() {
        // check allowed limits
        from_ymd_ok(0, 1, 1, 1);
        from_ymd_ok(334, 1, 12, 1);
        from_ymd_ok(30, 1, 1, 31);
        from_ymd_ok(58, 1, 2, 28);
        from_ymd_ok(1153, 4, 2, 28);
        from_ymd_ok(119, 1, 4, 30);
        from_ymd_ok(2_147_483_647, 5_879_611, 7, 12);
        from_ymd_ok(-2_147_483_648, -5_879_610, 6, 23);

        // check invalid limits
        from_ymd_err(1, 0, 1);
        from_ymd_err(1, 13, 1);
        from_ymd_err(1, 1, 0);
        from_ymd_err(1, 1, 32);
        from_ymd_err(1, 2, 29);
        from_ymd_err(1, 4, 31);
        from_ymd_err(5879611, 7, 13);
        from_ymd_err(5879612, 1, 1);
        from_ymd_err(5879611, 8, 1);
        from_ymd_err(-5879610, 6, 22);
        from_ymd_err(-5879611, 1, 1);
        from_ymd_err(-5879610, 5, 1);
    }

    fn from_ymd_ok(expected: i32, year: i32, month: u32, day: u32) {
        assert_eq!(expected, Date::from_ymd(year, month, day).unwrap().days());
    }

    fn from_ymd_err(year: i32, month: u32, day: u32) {
        assert!(Date::from_ymd(year, month, day).is_err());
    }

    #[test]
    fn manipulation() {
        let date_time = Date::from_ymd(1970, 1, 1).unwrap();

        let modified = date_time.apply(123, DateUnit::Day).unwrap();
        assert_eq!(10627200, modified.timestamp());
        let modified = date_time.apply(11, DateUnit::Month).unwrap();
        assert_eq!("1970-12-01", modified.format("yyyy-MM-dd").unwrap());
        let modified = date_time.apply(12, DateUnit::Month).unwrap();
        assert_eq!("1971-01-01", modified.format("yyyy-MM-dd").unwrap());
        let modified = date_time.apply(14, DateUnit::Month).unwrap();
        assert_eq!("1971-03-01", modified.format("yyyy-MM-dd").unwrap());

        // Leap year cases
        let modified = date_time.apply(30, DateUnit::Day).unwrap();
        assert_eq!("1970-01-31", modified.format("yyyy-MM-dd").unwrap());
        let modified = modified.apply(1, DateUnit::Month).unwrap();
        assert_eq!("1970-02-28", modified.format("yyyy-MM-dd").unwrap());
        let modified = modified.apply(2, DateUnit::Year).unwrap();
        assert_eq!("1972-02-28", modified.format("yyyy-MM-dd").unwrap());
        let modified = date_time
            .apply(2, DateUnit::Year)
            .unwrap()
            .apply(30, DateUnit::Day)
            .unwrap();
        assert_eq!("1972-01-31", modified.format("yyyy-MM-dd").unwrap());
        let modified = modified.apply(1, DateUnit::Month).unwrap();
        assert_eq!("1972-02-29", modified.format("yyyy-MM-dd").unwrap());

        let date_time = Date::from_ymd(1971, 1, 1).unwrap();
        let modified = date_time.apply(-1, DateUnit::Month).unwrap();
        assert_eq!("1970-12-01", modified.format("yyyy-MM-dd").unwrap());

        let date_time = Date::from_ymd(1972, 3, 31).unwrap();
        let modified = date_time.apply(-1, DateUnit::Month).unwrap();
        assert_eq!("1972-02-29", modified.format("yyyy-MM-dd").unwrap());
        let modified = modified.apply(-1, DateUnit::Month).unwrap();
        assert_eq!("1972-01-29", modified.format("yyyy-MM-dd").unwrap());
    }

    #[test]
    fn get() {
        let date_time = Date::from_ymd(2000, 5, 10).unwrap();
        assert_eq!(2000, date_time.get(DateUnit::Year));
        assert_eq!(5, date_time.get(DateUnit::Month));
        assert_eq!(10, date_time.get(DateUnit::Day));
    }

    #[test]
    fn set() {
        let date_time = Date::from_ymd(2000, 5, 10).unwrap();
        let modified = date_time.set(2022, DateUnit::Year).unwrap();
        assert_eq!(2022, modified.get(DateUnit::Year));
        let modified = date_time.set(1, DateUnit::Month).unwrap();
        assert_eq!(2000, modified.get(DateUnit::Year));
        assert_eq!(1, modified.get(DateUnit::Month));
        let modified = date_time.set(13, DateUnit::Day).unwrap();
        assert_eq!(2000, modified.get(DateUnit::Year));
        assert_eq!(5, modified.get(DateUnit::Month));
        assert_eq!(13, modified.get(DateUnit::Day));

        assert!(date_time.set(5_879_612, DateUnit::Year).is_err());
        assert!(date_time.set(13, DateUnit::Month).is_err());
        assert!(date_time
            .set(2, DateUnit::Month)
            .unwrap()
            .set(31, DateUnit::Day)
            .is_err());
        assert!(date_time.set(32, DateUnit::Day).is_err());
    }
}
