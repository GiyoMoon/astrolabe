#[cfg(test)]
mod date_tests {
    use astrolabe::{Date, DateTime, DateUtilities};
    use std::time::Duration;

    #[test]
    fn debug() {
        let date = Date::default();
        assert_eq!("Date { days: 0 }", format!("{:?}", date));
    }

    #[test]
    fn default() {
        let date = Date::default();
        assert_eq!(1, date.year());
        assert_eq!(1, date.month());
        assert_eq!(1, date.day());
    }

    #[test]
    fn copy() {
        let date = Date::default();
        let date_2 = date;
        assert_eq!(date, date_2);
    }

    #[test]
    fn clone() {
        let date = Date::default();
        #[allow(clippy::clone_on_copy)]
        let date_2 = date.clone();
        assert_eq!(date, date_2);
    }

    #[test]
    fn eq() {
        let date = Date::default();
        let date_2 = date;
        assert!(date == date_2);
    }

    #[test]
    fn ord() {
        let date = Date::default();
        let date_2 = date.add_days(1).unwrap();
        assert!(date < date_2);
        assert_eq!(std::cmp::Ordering::Less, date.cmp(&date_2));
    }

    #[test]
    fn now() {
        assert!(2021 < Date::now().year());
    }

    #[test]
    fn ymd() {
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
            Date::from_ymd(year, month, day).unwrap().as_ymd()
        );
    }

    fn from_ymd_err(year: i32, month: u32, day: u32) {
        assert!(Date::from_ymd(year, month, day).is_err());
    }

    #[test]
    fn duration_between() {
        duration_between_ok(
            Duration::from_secs(0),
            Date::from_ymd(1, 1, 1).unwrap(),
            Date::from_ymd(1, 1, 1).unwrap(),
        );
        duration_between_ok(
            Duration::from_secs(0),
            Date::from_ymd(1970, 1, 1).unwrap(),
            Date::from_ymd(1970, 1, 1).unwrap(),
        );
        duration_between_ok(
            Duration::from_secs(0),
            Date::from_ymd(2022, 5, 2).unwrap(),
            Date::from_ymd(2022, 5, 2).unwrap(),
        );
        duration_between_ok(
            Duration::from_secs(24 * 60 * 60),
            Date::from_ymd(2022, 5, 2).unwrap(),
            Date::from_ymd(2022, 5, 3).unwrap(),
        );
        duration_between_ok(
            Duration::from_secs(24 * 60 * 60 * 30),
            Date::from_ymd(2022, 5, 1).unwrap(),
            Date::from_ymd(2022, 5, 31).unwrap(),
        );
        duration_between_ok(
            Duration::from_secs(24 * 60 * 60),
            Date::from_ymd(2022, 5, 3).unwrap(),
            Date::from_ymd(2022, 5, 2).unwrap(),
        );
        duration_between_ok(
            Duration::from_secs(24 * 60 * 60),
            Date::from_ymd(-1, 12, 31).unwrap(),
            Date::from_ymd(1, 1, 1).unwrap(),
        );
        duration_between_ok(
            Duration::from_secs(371085174288000),
            Date::from_ymd(-5_879_611, 6, 23).unwrap(),
            Date::from_ymd(5_879_611, 7, 12).unwrap(),
        );
    }

    fn duration_between_ok(expected: Duration, start: Date, end: Date) {
        assert_eq!(expected, start.duration_between(&end));
    }

    #[test]
    fn get() {
        let date = Date::from_ymd(2022, 5, 2).unwrap();
        assert_eq!(2022, date.year());
        assert_eq!(5, date.month());
        assert_eq!(2, date.day());
        assert_eq!(122, date.day_of_year());
        assert_eq!(1, date.weekday());
        let date = Date::from_ymd(1, 1, 1).unwrap();
        assert_eq!(1, date.year());
        assert_eq!(1, date.month());
        assert_eq!(1, date.day());
        assert_eq!(1, date.day_of_year());
        assert_eq!(1, date.weekday());
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
    fn add_sub() {
        let date = Date::from_ymd(1970, 1, 1).unwrap();

        let modified = date.add_days(123).unwrap();
        assert_eq!(10627200, modified.timestamp());
        let modified = date.add_months(11).unwrap();
        assert_eq!("1970-12-01", modified.format("yyyy-MM-dd"));
        let modified = date.add_months(12).unwrap();
        assert_eq!("1971-01-01", modified.format("yyyy-MM-dd"));
        let modified = date.add_months(14).unwrap();
        assert_eq!("1971-03-01", modified.format("yyyy-MM-dd"));

        // Leap year cases
        let modified = date.add_days(30).unwrap();
        assert_eq!("1970-01-31", modified.format("yyyy-MM-dd"));
        let modified = modified.add_months(1).unwrap();
        assert_eq!("1970-02-28", modified.format("yyyy-MM-dd"));
        let modified = modified.add_years(2).unwrap();
        assert_eq!("1972-02-28", modified.format("yyyy-MM-dd"));
        let modified = date.add_years(2).unwrap().add_days(30).unwrap();
        assert_eq!("1972-01-31", modified.format("yyyy-MM-dd"));
        let modified = modified.add_months(1).unwrap();
        assert_eq!("1972-02-29", modified.format("yyyy-MM-dd"));

        let date = Date::from_ymd(1971, 1, 1).unwrap();
        let modified = date.sub_months(1).unwrap();
        assert_eq!("1970-12-01", modified.format("yyyy-MM-dd"));

        let date = Date::from_ymd(1972, 3, 31).unwrap();
        let modified = date.sub_months(1).unwrap();
        assert_eq!("1972-02-29", modified.format("yyyy-MM-dd"));
        let modified = modified.sub_months(1).unwrap();
        assert_eq!("1972-01-29", modified.format("yyyy-MM-dd"));

        let date = Date::from_ymd(5_879_611, 7, 12).unwrap();
        assert!(date.add_days(1).is_err());
        let date = Date::from_ymd(5_879_611, 6, 13).unwrap();
        assert!(date.add_months(1).is_err());
        let date = Date::from_ymd(5_879_610, 7, 13).unwrap();
        assert!(date.add_years(1).is_err());

        let date = Date::from_ymd(-5_879_611, 6, 23).unwrap();
        assert!(date.sub_days(1).is_err());
        let date = Date::from_ymd(-5_879_611, 7, 22).unwrap();
        assert!(date.sub_months(1).is_err());
        let date = Date::from_ymd(-5_879_610, 6, 22).unwrap();
        assert!(date.sub_years(1).is_err());

        let date = Date::from_ymd(1, 1, 1).unwrap();
        assert_eq!(
            "-0001-12-31",
            date.sub_days(1).unwrap().format("yyyy-MM-dd")
        );

        assert_eq!(
            "-0001-12-01",
            date.sub_months(1).unwrap().format("yyyy-MM-dd")
        );

        assert_eq!(
            "-0001-01-01",
            date.sub_years(1).unwrap().format("yyyy-MM-dd")
        );

        let date = Date::from_ymd(-1, 12, 31).unwrap();
        assert_eq!(
            "-0001-12-30",
            date.sub_days(1).unwrap().format("yyyy-MM-dd")
        );

        assert_eq!(
            "-0001-11-30",
            date.sub_months(1).unwrap().format("yyyy-MM-dd")
        );

        assert_eq!(
            "-0002-12-31",
            date.sub_years(1).unwrap().format("yyyy-MM-dd")
        );

        let date = Date::from_ymd(1, 1, 1).unwrap();
        assert_eq!("0001-01-02", date.add_days(1).unwrap().format("yyyy-MM-dd"));

        assert_eq!(
            "0001-02-01",
            date.add_months(1).unwrap().format("yyyy-MM-dd")
        );

        assert_eq!(
            "0002-01-01",
            date.add_years(1).unwrap().format("yyyy-MM-dd")
        );

        let date = Date::from_ymd(-1, 12, 31).unwrap();
        assert_eq!("0001-01-01", date.add_days(1).unwrap().format("yyyy-MM-dd"));

        assert_eq!(
            "0001-01-31",
            date.add_months(1).unwrap().format("yyyy-MM-dd")
        );

        assert_eq!(
            "0001-12-31",
            date.add_years(1).unwrap().format("yyyy-MM-dd")
        );

        let date = Date::from_ymd(2020, 2, 29).unwrap();
        assert_eq!(
            "2021-02-28",
            date.add_years(1).unwrap().format("yyyy-MM-dd")
        );
        assert_eq!(
            "2019-02-28",
            date.sub_years(1).unwrap().format("yyyy-MM-dd")
        );

        let date = Date::from_ymd(1, 1, 1).unwrap();
        assert_eq!(
            "0001-02-01",
            date.add_months(1).unwrap().format("yyyy-MM-dd")
        );
    }

    #[test]
    fn set() {
        let date = Date::from_ymd(2000, 5, 10).unwrap();
        let modified = date.set_year(2022).unwrap();
        assert_eq!(2022, modified.year());
        let modified = date.set_month(1).unwrap();
        assert_eq!(2000, modified.year());
        assert_eq!(1, modified.month());
        let modified = date.set_day(13).unwrap();
        assert_eq!(2000, modified.year());
        assert_eq!(5, modified.month());
        assert_eq!(13, modified.day());

        assert!(date.set_year(5_879_612).is_err());
        assert!(date.set_month(13).is_err());
        assert!(date.set_month(2).unwrap().set_day(31).is_err());
        assert!(date.set_day(32).is_err());
        assert!(date.set_year(0).is_err());

        let date = Date::from_ymd(1, 1, 1).unwrap();
        assert_eq!("0001-01-02", date.set_day(2).unwrap().format("yyyy-MM-dd"));
        assert_eq!(
            "0001-02-01",
            date.set_month(2).unwrap().format("yyyy-MM-dd")
        );
        assert_eq!("0002-01-01", date.set_year(2).unwrap().format("yyyy-MM-dd"));
        assert_eq!(
            "-0001-01-01",
            date.set_year(-1).unwrap().format("yyyy-MM-dd")
        );
        assert_eq!(
            "0001-01-02",
            date.set_day_of_year(2).unwrap().format("yyyy-MM-dd")
        );
        assert_eq!(
            "0001-12-31",
            date.set_day_of_year(365).unwrap().format("yyyy-MM-dd")
        );
        assert!(date.set_day_of_year(366).is_err());

        let date = Date::from_ymd(-1, 1, 1).unwrap();
        assert_eq!("-0001-01-02", date.set_day(2).unwrap().format("yyyy-MM-dd"));
        assert_eq!(
            "-0001-02-01",
            date.set_month(2).unwrap().format("yyyy-MM-dd")
        );
        assert_eq!(
            "-0002-01-01",
            date.set_year(-2).unwrap().format("yyyy-MM-dd")
        );
        assert_eq!("0001-01-01", date.set_year(1).unwrap().format("yyyy-MM-dd"));
        assert_eq!(
            "-0001-01-02",
            date.set_day_of_year(2).unwrap().format("yyyy-MM-dd")
        );
        assert_eq!(
            "-0001-12-31",
            date.set_day_of_year(366).unwrap().format("yyyy-MM-dd")
        );
        assert!(date.set_day_of_year(367).is_err());
    }

    #[test]
    fn clear() {
        let date = Date::from_ymd(2022, 5, 10).unwrap();
        let modified = date.clear_until_year();
        assert_eq!(1, modified.year());
        assert_eq!(1, modified.month());
        assert_eq!(1, modified.day());
        let modified = date.clear_until_month();
        assert_eq!(2022, modified.year());
        assert_eq!(1, modified.month());
        assert_eq!(1, modified.day());
        let modified = date.clear_until_day();
        assert_eq!(2022, modified.year());
        assert_eq!(5, modified.month());
        assert_eq!(1, modified.day());

        let date = Date::from_ymd(-2022, 5, 10).unwrap();
        let modified = date.clear_until_year();
        assert_eq!(1, modified.year());
        assert_eq!(1, modified.month());
        assert_eq!(1, modified.day());
        let modified = date.clear_until_month();
        assert_eq!(-2022, modified.year());
        assert_eq!(1, modified.month());
        assert_eq!(1, modified.day());
        let modified = date.clear_until_day();
        assert_eq!(-2022, modified.year());
        assert_eq!(5, modified.month());
        assert_eq!(1, modified.day());
    }

    #[test]
    fn since() {
        // tests the years_since, months_since, days_since methods
        let date = Date::from_ymd(2022, 5, 10).unwrap();
        assert_eq!(0, date.years_since(&date));
        // assert_eq!(0, date.months_since(&date));
        assert_eq!(0, date.days_since(&date));
        assert_eq!(0, date.years_since(&date));
        // assert_eq!(0, date.months_since(&date));
        assert_eq!(0, date.days_since(&date));
        let date2 = Date::from_ymd(2023, 5, 10).unwrap();
        assert_eq!(1, date2.years_since(&date));
        // assert_eq!(12, date2.months_since(&date));
        assert_eq!(365, date2.days_since(&date));
        assert_eq!(-1, date.years_since(&date2));
        // assert_eq!(-12, date.months_since(&date2));
        assert_eq!(-365, date.days_since(&date2));
        let date2 = Date::from_ymd(2022, 6, 10).unwrap();
        assert_eq!(0, date2.years_since(&date));
        // assert_eq!(1, date2.months_since(&date));
        assert_eq!(31, date2.days_since(&date));
        assert_eq!(0, date.years_since(&date2));
        // assert_eq!(-1, date.months_since(&date2));
        assert_eq!(-31, date.days_since(&date2));
        let date2 = Date::from_ymd(2022, 5, 11).unwrap();
        assert_eq!(0, date2.years_since(&date));
        // assert_eq!(0, date2.months_since(&date));
        assert_eq!(1, date2.days_since(&date));
        assert_eq!(0, date.years_since(&date2));
        // assert_eq!(0, date.months_since(&date2));
        assert_eq!(-1, date.days_since(&date2));
        let date2 = Date::from_ymd(2023, 5, 9).unwrap();
        assert_eq!(0, date2.years_since(&date));
        // assert_eq!(11, date2.months_since(&date));
        assert_eq!(364, date2.days_since(&date));
        assert_eq!(0, date.years_since(&date2));
        // assert_eq!(-11, date.months_since(&date2));
        assert_eq!(-364, date.days_since(&date2));
        let date = Date::from_ymd(-1, 12, 31).unwrap();
        let date2 = Date::from_ymd(1, 1, 1).unwrap();
        assert_eq!(0, date2.years_since(&date));
        // assert_eq!(0, date2.months_since(&date));
        assert_eq!(1, date2.days_since(&date));
        assert_eq!(0, date.years_since(&date2));
        // assert_eq!(0, date.months_since(&date2));
        assert_eq!(-1, date.days_since(&date2));
    }

    #[test]
    fn negative_years() {
        let date = Date::from_ymd(1, 1, 1).unwrap();
        assert_eq!("0001-01-01", date.format("yyyy-MM-dd"));
        assert_eq!("01-01-01", date.format("yy-MM-dd"));
        assert_eq!(1, date.year());
        let date = Date::from_ymd(-1, 1, 1).unwrap();
        assert_eq!("-0001-01-01", date.format("yyyy-MM-dd"));
        assert_eq!("-01-01-01", date.format("yy-MM-dd"));
        assert_eq!(-1, date.year());
        let date = Date::from_ymd(-1, 12, 31).unwrap();
        assert_eq!("-0001-12-31", date.format("yyyy-MM-dd"));
        assert_eq!("-01-12-31", date.format("yy-MM-dd"));
        assert_eq!(-1, date.year());
        let date = Date::from_ymd(-1, 2, 29).unwrap();
        assert_eq!("-0001-02-29", date.format("yyyy-MM-dd"));
        assert_eq!("-01-02-29", date.format("yy-MM-dd"));
        assert_eq!(-1, date.year());
        let date = Date::from_ymd(-2, 12, 31).unwrap();
        assert_eq!("-0002-12-31", date.format("yyyy-MM-dd"));
        assert_eq!("-02-12-31", date.format("yy-MM-dd"));
        assert_eq!(-2, date.year());
    }

    #[test]
    fn from() {
        let date = Date::from_ymd(2022, 5, 10).unwrap();
        assert_eq!("2022-05-10", Date::from(&date).format("yyyy-MM-dd"));
        let date_time = DateTime::from_ymd(2022, 5, 10).unwrap();
        assert_eq!("2022-05-10", Date::from(date_time).format("yyyy-MM-dd"));
        assert_eq!("2022-05-10", Date::from(&date_time).format("yyyy-MM-dd"));
    }

    #[test]
    fn std_add() {
        let mut date = Date::from_ymd(2022, 5, 10).unwrap();
        let modified = date + Duration::from_secs(24 * 60 * 60);
        assert_eq!("2022-05-11", modified.format("yyyy-MM-dd"));
        date += Duration::from_secs(24 * 60 * 60);
        assert_eq!("2022-05-11", date.format("yyyy-MM-dd"));
    }

    #[test]
    fn std_sub() {
        let mut date = Date::from_ymd(2022, 5, 10).unwrap();
        let modified = date - Duration::from_secs(24 * 60 * 60);
        assert_eq!("2022-05-09", modified.format("yyyy-MM-dd"));
        date -= Duration::from_secs(24 * 60 * 60);
        assert_eq!("2022-05-09", date.format("yyyy-MM-dd"));
    }

    #[test]
    fn display() {
        let date = Date::from_ymd(2022, 5, 10).unwrap();
        assert_eq!("2022/05/10", format!("{}", date));
    }
}
