#[cfg(test)]
mod tests {
    use astrolabe::Date;

    #[test]
    fn format_era() {
        let date_time = Date::from_ymd(1970, 1, 1).unwrap();
        assert_eq!("AD", date_time.format("G").unwrap());
        assert_eq!("AD", date_time.format("GG").unwrap());
        assert_eq!("AD", date_time.format("GGG").unwrap());
        assert_eq!("Anno Domini", date_time.format("GGGG").unwrap());
        assert_eq!("A", date_time.format("GGGGG").unwrap());
        assert_eq!("Anno Domini", date_time.format("GGGGGG").unwrap());
        assert_eq!("Anno Domini", date_time.format("GGGGGGG").unwrap());
    }

    #[test]
    fn format_year() {
        let date_time = Date::from_ymd(1970, 1, 1).unwrap();
        assert_eq!("1970", date_time.format("y").unwrap());
        assert_eq!("70", date_time.format("yy").unwrap());
        assert_eq!("1970", date_time.format("yyy").unwrap());
        assert_eq!("1970", date_time.format("yyyy").unwrap());
        assert_eq!("01970", date_time.format("yyyyy").unwrap());

        let date_time = Date::from_ymd(2000, 12, 31).unwrap();
        assert_eq!("2000", date_time.format("y").unwrap());
        assert_eq!("00", date_time.format("yy").unwrap());
        assert_eq!("2000", date_time.format("yyy").unwrap());
        assert_eq!("2000", date_time.format("yyyy").unwrap());
        assert_eq!("02000", date_time.format("yyyyy").unwrap());

        let date_time = Date::from_ymd(2345, 1, 1).unwrap();
        assert_eq!("2345", date_time.format("y").unwrap());
        assert_eq!("45", date_time.format("yy").unwrap());
        assert_eq!("2345", date_time.format("yyy").unwrap());
        assert_eq!("2345", date_time.format("yyyy").unwrap());
        assert_eq!("02345", date_time.format("yyyyy").unwrap());
    }

    #[test]
    fn format_quarter() {
        let date_time = Date::from_ymd(1970, 1, 1).unwrap();
        assert_eq!("1", date_time.format("q").unwrap());
        assert_eq!("01", date_time.format("qq").unwrap());
        assert_eq!("Q1", date_time.format("qqq").unwrap());
        assert_eq!("1st quarter", date_time.format("qqqq").unwrap());
        assert_eq!("1", date_time.format("qqqqq").unwrap());
        assert_eq!("1", date_time.format("qqqqqq").unwrap());
        let date_time = Date::from_ymd(1970, 3, 31).unwrap();
        assert_eq!("1st quarter", date_time.format("qqqq").unwrap());
        let date_time = Date::from_ymd(1970, 4, 1).unwrap();
        assert_eq!("2nd quarter", date_time.format("qqqq").unwrap());
        let date_time = Date::from_ymd(1970, 6, 30).unwrap();
        assert_eq!("2nd quarter", date_time.format("qqqq").unwrap());
        let date_time = Date::from_ymd(1970, 7, 1).unwrap();
        assert_eq!("3rd quarter", date_time.format("qqqq").unwrap());
        let date_time = Date::from_ymd(1970, 9, 30).unwrap();
        assert_eq!("3rd quarter", date_time.format("qqqq").unwrap());
        let date_time = Date::from_ymd(1970, 10, 1).unwrap();
        assert_eq!("4th quarter", date_time.format("qqqq").unwrap());
        let date_time = Date::from_ymd(1970, 12, 31).unwrap();
        assert_eq!("4th quarter", date_time.format("qqqq").unwrap());
    }

    #[test]
    fn format_month() {
        let date_time = Date::from_ymd(1970, 1, 1).unwrap();
        assert_eq!("1", date_time.format("M").unwrap());
        assert_eq!("01", date_time.format("MM").unwrap());
        assert_eq!("Jan", date_time.format("MMM").unwrap());
        assert_eq!("January", date_time.format("MMMM").unwrap());
        assert_eq!("J", date_time.format("MMMMM").unwrap());
        assert_eq!("January", date_time.format("MMMMMM").unwrap());

        let date_time = Date::from_ymd(1970, 6, 1).unwrap();
        assert_eq!("6", date_time.format("M").unwrap());
        assert_eq!("06", date_time.format("MM").unwrap());
        assert_eq!("Jun", date_time.format("MMM").unwrap());
        assert_eq!("June", date_time.format("MMMM").unwrap());
        assert_eq!("J", date_time.format("MMMMM").unwrap());
        assert_eq!("June", date_time.format("MMMMMM").unwrap());

        let date_time = Date::from_ymd(1970, 12, 1).unwrap();
        assert_eq!("12", date_time.format("M").unwrap());
        assert_eq!("12", date_time.format("MM").unwrap());
        assert_eq!("Dec", date_time.format("MMM").unwrap());
        assert_eq!("December", date_time.format("MMMM").unwrap());
        assert_eq!("D", date_time.format("MMMMM").unwrap());
        assert_eq!("December", date_time.format("MMMMMM").unwrap());
    }

    #[test]
    fn format_week() {
        let date_time = Date::from_ymd(1970, 1, 1).unwrap();
        assert_eq!("1", date_time.format("w").unwrap());
        assert_eq!("01", date_time.format("ww").unwrap());
        assert_eq!("01", date_time.format("www").unwrap());

        let date_time = Date::from_ymd(1971, 1, 1).unwrap();
        assert_eq!("53", date_time.format("w").unwrap());
        assert_eq!("53", date_time.format("ww").unwrap());
        assert_eq!("53", date_time.format("www").unwrap());

        let date_time = Date::from_ymd(1972, 1, 1).unwrap();
        assert_eq!("52", date_time.format("w").unwrap());
        assert_eq!("52", date_time.format("ww").unwrap());

        let date_time = Date::from_ymd(1973, 1, 1).unwrap();
        assert_eq!("1", date_time.format("w").unwrap());
        assert_eq!("01", date_time.format("ww").unwrap());

        let date_time = Date::from_ymd(1974, 1, 1).unwrap();
        assert_eq!("1", date_time.format("w").unwrap());
        assert_eq!("01", date_time.format("ww").unwrap());

        let date_time = Date::from_ymd(1975, 1, 1).unwrap();
        assert_eq!("1", date_time.format("w").unwrap());
        assert_eq!("01", date_time.format("ww").unwrap());

        let date_time = Date::from_ymd(1976, 1, 1).unwrap();
        assert_eq!("1", date_time.format("w").unwrap());
        assert_eq!("01", date_time.format("ww").unwrap());

        let date_time = Date::from_ymd(1977, 1, 1).unwrap();
        assert_eq!("53", date_time.format("w").unwrap());
        assert_eq!("53", date_time.format("ww").unwrap());

        let date_time = Date::from_ymd(1978, 1, 1).unwrap();
        assert_eq!("52", date_time.format("w").unwrap());
        assert_eq!("52", date_time.format("ww").unwrap());

        let date_time = Date::from_ymd(1979, 1, 1).unwrap();
        assert_eq!("1", date_time.format("w").unwrap());
        assert_eq!("01", date_time.format("ww").unwrap());

        let date_time = Date::from_ymd(1970, 12, 31).unwrap();
        assert_eq!("53", date_time.format("w").unwrap());
        assert_eq!("53", date_time.format("ww").unwrap());

        let date_time = Date::from_ymd(1971, 12, 31).unwrap();
        assert_eq!("52", date_time.format("w").unwrap());
        assert_eq!("52", date_time.format("ww").unwrap());

        let date_time = Date::from_ymd(1972, 12, 31).unwrap();
        assert_eq!("52", date_time.format("w").unwrap());
        assert_eq!("52", date_time.format("ww").unwrap());

        let date_time = Date::from_ymd(1973, 12, 31).unwrap();
        assert_eq!("1", date_time.format("w").unwrap());
        assert_eq!("01", date_time.format("ww").unwrap());

        let date_time = Date::from_ymd(1974, 12, 31).unwrap();
        assert_eq!("1", date_time.format("w").unwrap());
        assert_eq!("01", date_time.format("ww").unwrap());

        let date_time = Date::from_ymd(1975, 12, 31).unwrap();
        assert_eq!("1", date_time.format("w").unwrap());
        assert_eq!("01", date_time.format("ww").unwrap());

        let date_time = Date::from_ymd(1976, 12, 31).unwrap();
        assert_eq!("53", date_time.format("w").unwrap());
        assert_eq!("53", date_time.format("ww").unwrap());

        let date_time = Date::from_ymd(1977, 12, 31).unwrap();
        assert_eq!("52", date_time.format("w").unwrap());
        assert_eq!("52", date_time.format("ww").unwrap());

        let date_time = Date::from_ymd(1978, 12, 31).unwrap();
        assert_eq!("52", date_time.format("w").unwrap());
        assert_eq!("52", date_time.format("ww").unwrap());

        let date_time = Date::from_ymd(1979, 12, 31).unwrap();
        assert_eq!("1", date_time.format("w").unwrap());
        assert_eq!("01", date_time.format("ww").unwrap());

        let date_time = Date::from_ymd(2022, 3, 1).unwrap();
        assert_eq!("9", date_time.format("w").unwrap());
        assert_eq!("09", date_time.format("ww").unwrap());

        let date_time = Date::from_ymd(2032, 3, 1).unwrap();
        assert_eq!("10", date_time.format("w").unwrap());
        assert_eq!("10", date_time.format("ww").unwrap());
    }

    #[test]
    fn format_day() {
        let date_time = Date::from_ymd(1970, 1, 1).unwrap();
        assert_eq!("1", date_time.format("d").unwrap());
        assert_eq!("01", date_time.format("dd").unwrap());
        assert_eq!("01", date_time.format("ddd").unwrap());

        let date_time = Date::from_ymd(1970, 1, 15).unwrap();
        assert_eq!("15", date_time.format("d").unwrap());
        assert_eq!("15", date_time.format("dd").unwrap());
        assert_eq!("15", date_time.format("ddd").unwrap());

        let date_time = Date::from_ymd(1970, 1, 31).unwrap();
        assert_eq!("31", date_time.format("d").unwrap());
        assert_eq!("31", date_time.format("dd").unwrap());
        assert_eq!("31", date_time.format("ddd").unwrap());
    }

    #[test]
    fn format_year_day() {
        let date_time = Date::from_ymd(1970, 1, 1).unwrap();
        assert_eq!("1", date_time.format("D").unwrap());
        assert_eq!("01", date_time.format("DD").unwrap());
        assert_eq!("001", date_time.format("DDD").unwrap());
        assert_eq!("1", date_time.format("DDDD").unwrap());

        let date_time = Date::from_ymd(2020, 1, 24).unwrap();
        assert_eq!("24", date_time.format("D").unwrap());
        assert_eq!("24", date_time.format("DD").unwrap());
        assert_eq!("024", date_time.format("DDD").unwrap());
        assert_eq!("24", date_time.format("DDDD").unwrap());

        let date_time = Date::from_ymd(2020, 5, 15).unwrap();
        assert_eq!("136", date_time.format("D").unwrap());
        assert_eq!("136", date_time.format("DD").unwrap());
        assert_eq!("136", date_time.format("DDD").unwrap());
        assert_eq!("136", date_time.format("DDDD").unwrap());

        let date_time = Date::from_ymd(2022, 5, 15).unwrap();
        assert_eq!("135", date_time.format("D").unwrap());
        assert_eq!("135", date_time.format("DD").unwrap());
        assert_eq!("135", date_time.format("DDD").unwrap());
        assert_eq!("135", date_time.format("DDDD").unwrap());

        let date_time = Date::from_ymd(2300, 12, 31).unwrap();
        assert_eq!("365", date_time.format("D").unwrap());
        assert_eq!("365", date_time.format("DD").unwrap());
        assert_eq!("365", date_time.format("DDD").unwrap());
        assert_eq!("365", date_time.format("DDDD").unwrap());
    }

    #[test]
    fn format_wday() {
        let date_time = Date::from_ymd(1970, 1, 1).unwrap();
        assert_eq!("5", date_time.format("e").unwrap());
        assert_eq!("05", date_time.format("ee").unwrap());
        assert_eq!("Thu", date_time.format("eee").unwrap());
        assert_eq!("Thursday", date_time.format("eeee").unwrap());
        assert_eq!("T", date_time.format("eeeee").unwrap());
        assert_eq!("Th", date_time.format("eeeeee").unwrap());
        assert_eq!("4", date_time.format("eeeeeee").unwrap());
        assert_eq!("04", date_time.format("eeeeeeee").unwrap());
        assert_eq!("5", date_time.format("eeeeeeeee").unwrap());

        let date_time = Date::from_ymd(2020, 1, 1).unwrap();
        assert_eq!("4", date_time.format("e").unwrap());
        assert_eq!("04", date_time.format("ee").unwrap());
        assert_eq!("Wed", date_time.format("eee").unwrap());
        assert_eq!("Wednesday", date_time.format("eeee").unwrap());
        assert_eq!("W", date_time.format("eeeee").unwrap());
        assert_eq!("We", date_time.format("eeeeee").unwrap());
        assert_eq!("3", date_time.format("eeeeeee").unwrap());
        assert_eq!("03", date_time.format("eeeeeeee").unwrap());
        assert_eq!("4", date_time.format("eeeeeeeee").unwrap());

        let date_time = Date::from_ymd(2020, 5, 10).unwrap();
        assert_eq!("1", date_time.format("e").unwrap());
        assert_eq!("01", date_time.format("ee").unwrap());
        assert_eq!("Sun", date_time.format("eee").unwrap());
        assert_eq!("Sunday", date_time.format("eeee").unwrap());
        assert_eq!("S", date_time.format("eeeee").unwrap());
        assert_eq!("Su", date_time.format("eeeeee").unwrap());
        assert_eq!("7", date_time.format("eeeeeee").unwrap());
        assert_eq!("07", date_time.format("eeeeeeee").unwrap());
        assert_eq!("1", date_time.format("eeeeeeeee").unwrap());

        let date_time = Date::from_ymd(2020, 5, 11).unwrap();
        assert_eq!("2", date_time.format("e").unwrap());
        assert_eq!("02", date_time.format("ee").unwrap());
        assert_eq!("Mon", date_time.format("eee").unwrap());
        assert_eq!("Monday", date_time.format("eeee").unwrap());
        assert_eq!("M", date_time.format("eeeee").unwrap());
        assert_eq!("Mo", date_time.format("eeeeee").unwrap());
        assert_eq!("1", date_time.format("eeeeeee").unwrap());
        assert_eq!("01", date_time.format("eeeeeeee").unwrap());
        assert_eq!("2", date_time.format("eeeeeeeee").unwrap());
    }

    #[test]
    fn format_escape() {
        let date_time = Date::from_ymd(1970, 1, 1).unwrap();
        assert_eq!(
            "yyyMMdd19700101yyyMMdd",
            date_time.format("'yyyMMdd'yyyMMdd'yyyMMdd").unwrap()
        );
        assert_eq!(
            "yyyMMdd19700101yyyMMdd",
            date_time.format("'yyyMMdd'yyyMMdd'yyyMMdd'").unwrap()
        );
        assert_eq!(
            "yyyMMdd19700101yyyMMdd01",
            date_time.format("'yyyMMdd'yyyMMdd'yyyMMdd'dd").unwrap()
        );
        assert_eq!("yyyMMdd'dd", date_time.format("'yyyMMdd''dd").unwrap());
        assert_eq!(
            "yyyyMMdd19700101yyyMMdd'dd",
            date_time.format("'yyyyMMdd'yyyyMMdd'yyyMMdd''dd'").unwrap()
        );
        assert_eq!("''", date_time.format("''''").unwrap());
        assert_eq!("'01'", date_time.format("''dd''").unwrap());
        assert_eq!("'dd'", date_time.format("'''dd'''").unwrap());
        assert_eq!("''01''", date_time.format("''''dd''''").unwrap());
        assert_eq!("''dd''", date_time.format("'''''dd'''''").unwrap());
    }
}
