#[cfg(test)]
mod tests {
    use astrolabe::Date;

    #[test]
    fn format_era() {
        let date = Date::from_ymd(1970, 1, 1).unwrap();
        assert_eq!("AD", date.format("G").unwrap());
        assert_eq!("AD", date.format("GG").unwrap());
        assert_eq!("AD", date.format("GGG").unwrap());
        assert_eq!("Anno Domini", date.format("GGGG").unwrap());
        assert_eq!("A", date.format("GGGGG").unwrap());
        assert_eq!("Anno Domini", date.format("GGGGGG").unwrap());
        assert_eq!("Anno Domini", date.format("GGGGGGG").unwrap());
    }

    #[test]
    fn format_year() {
        let date = Date::from_ymd(1970, 1, 1).unwrap();
        assert_eq!("1970", date.format("y").unwrap());
        assert_eq!("70", date.format("yy").unwrap());
        assert_eq!("1970", date.format("yyy").unwrap());
        assert_eq!("1970", date.format("yyyy").unwrap());
        assert_eq!("01970", date.format("yyyyy").unwrap());

        let date = Date::from_ymd(2000, 12, 31).unwrap();
        assert_eq!("2000", date.format("y").unwrap());
        assert_eq!("00", date.format("yy").unwrap());
        assert_eq!("2000", date.format("yyy").unwrap());
        assert_eq!("2000", date.format("yyyy").unwrap());
        assert_eq!("02000", date.format("yyyyy").unwrap());

        let date = Date::from_ymd(2345, 1, 1).unwrap();
        assert_eq!("2345", date.format("y").unwrap());
        assert_eq!("45", date.format("yy").unwrap());
        assert_eq!("2345", date.format("yyy").unwrap());
        assert_eq!("2345", date.format("yyyy").unwrap());
        assert_eq!("02345", date.format("yyyyy").unwrap());
    }

    #[test]
    fn format_quarter() {
        let date = Date::from_ymd(1970, 1, 1).unwrap();
        assert_eq!("1", date.format("q").unwrap());
        assert_eq!("01", date.format("qq").unwrap());
        assert_eq!("Q1", date.format("qqq").unwrap());
        assert_eq!("1st quarter", date.format("qqqq").unwrap());
        assert_eq!("1", date.format("qqqqq").unwrap());
        assert_eq!("1", date.format("qqqqqq").unwrap());
        let date = Date::from_ymd(1970, 3, 31).unwrap();
        assert_eq!("1st quarter", date.format("qqqq").unwrap());
        let date = Date::from_ymd(1970, 4, 1).unwrap();
        assert_eq!("2nd quarter", date.format("qqqq").unwrap());
        let date = Date::from_ymd(1970, 6, 30).unwrap();
        assert_eq!("2nd quarter", date.format("qqqq").unwrap());
        let date = Date::from_ymd(1970, 7, 1).unwrap();
        assert_eq!("3rd quarter", date.format("qqqq").unwrap());
        let date = Date::from_ymd(1970, 9, 30).unwrap();
        assert_eq!("3rd quarter", date.format("qqqq").unwrap());
        let date = Date::from_ymd(1970, 10, 1).unwrap();
        assert_eq!("4th quarter", date.format("qqqq").unwrap());
        let date = Date::from_ymd(1970, 12, 31).unwrap();
        assert_eq!("4th quarter", date.format("qqqq").unwrap());
    }

    #[test]
    fn format_month() {
        let date = Date::from_ymd(1970, 1, 1).unwrap();
        assert_eq!("1", date.format("M").unwrap());
        assert_eq!("01", date.format("MM").unwrap());
        assert_eq!("Jan", date.format("MMM").unwrap());
        assert_eq!("January", date.format("MMMM").unwrap());
        assert_eq!("J", date.format("MMMMM").unwrap());
        assert_eq!("January", date.format("MMMMMM").unwrap());

        let date = Date::from_ymd(1970, 6, 1).unwrap();
        assert_eq!("6", date.format("M").unwrap());
        assert_eq!("06", date.format("MM").unwrap());
        assert_eq!("Jun", date.format("MMM").unwrap());
        assert_eq!("June", date.format("MMMM").unwrap());
        assert_eq!("J", date.format("MMMMM").unwrap());
        assert_eq!("June", date.format("MMMMMM").unwrap());

        let date = Date::from_ymd(1970, 12, 1).unwrap();
        assert_eq!("12", date.format("M").unwrap());
        assert_eq!("12", date.format("MM").unwrap());
        assert_eq!("Dec", date.format("MMM").unwrap());
        assert_eq!("December", date.format("MMMM").unwrap());
        assert_eq!("D", date.format("MMMMM").unwrap());
        assert_eq!("December", date.format("MMMMMM").unwrap());
    }

    #[test]
    fn format_week() {
        let date = Date::from_ymd(1970, 1, 1).unwrap();
        assert_eq!("1", date.format("w").unwrap());
        assert_eq!("01", date.format("ww").unwrap());
        assert_eq!("01", date.format("www").unwrap());

        let date = Date::from_ymd(1971, 1, 1).unwrap();
        assert_eq!("53", date.format("w").unwrap());
        assert_eq!("53", date.format("ww").unwrap());
        assert_eq!("53", date.format("www").unwrap());

        let date = Date::from_ymd(1972, 1, 1).unwrap();
        assert_eq!("52", date.format("w").unwrap());
        assert_eq!("52", date.format("ww").unwrap());

        let date = Date::from_ymd(1973, 1, 1).unwrap();
        assert_eq!("1", date.format("w").unwrap());
        assert_eq!("01", date.format("ww").unwrap());

        let date = Date::from_ymd(1974, 1, 1).unwrap();
        assert_eq!("1", date.format("w").unwrap());
        assert_eq!("01", date.format("ww").unwrap());

        let date = Date::from_ymd(1975, 1, 1).unwrap();
        assert_eq!("1", date.format("w").unwrap());
        assert_eq!("01", date.format("ww").unwrap());

        let date = Date::from_ymd(1976, 1, 1).unwrap();
        assert_eq!("1", date.format("w").unwrap());
        assert_eq!("01", date.format("ww").unwrap());

        let date = Date::from_ymd(1977, 1, 1).unwrap();
        assert_eq!("53", date.format("w").unwrap());
        assert_eq!("53", date.format("ww").unwrap());

        let date = Date::from_ymd(1978, 1, 1).unwrap();
        assert_eq!("52", date.format("w").unwrap());
        assert_eq!("52", date.format("ww").unwrap());

        let date = Date::from_ymd(1979, 1, 1).unwrap();
        assert_eq!("1", date.format("w").unwrap());
        assert_eq!("01", date.format("ww").unwrap());

        let date = Date::from_ymd(1970, 12, 31).unwrap();
        assert_eq!("53", date.format("w").unwrap());
        assert_eq!("53", date.format("ww").unwrap());

        let date = Date::from_ymd(1971, 12, 31).unwrap();
        assert_eq!("52", date.format("w").unwrap());
        assert_eq!("52", date.format("ww").unwrap());

        let date = Date::from_ymd(1972, 12, 31).unwrap();
        assert_eq!("52", date.format("w").unwrap());
        assert_eq!("52", date.format("ww").unwrap());

        let date = Date::from_ymd(1973, 12, 31).unwrap();
        assert_eq!("1", date.format("w").unwrap());
        assert_eq!("01", date.format("ww").unwrap());

        let date = Date::from_ymd(1974, 12, 31).unwrap();
        assert_eq!("1", date.format("w").unwrap());
        assert_eq!("01", date.format("ww").unwrap());

        let date = Date::from_ymd(1975, 12, 31).unwrap();
        assert_eq!("1", date.format("w").unwrap());
        assert_eq!("01", date.format("ww").unwrap());

        let date = Date::from_ymd(1976, 12, 31).unwrap();
        assert_eq!("53", date.format("w").unwrap());
        assert_eq!("53", date.format("ww").unwrap());

        let date = Date::from_ymd(1977, 12, 31).unwrap();
        assert_eq!("52", date.format("w").unwrap());
        assert_eq!("52", date.format("ww").unwrap());

        let date = Date::from_ymd(1978, 12, 31).unwrap();
        assert_eq!("52", date.format("w").unwrap());
        assert_eq!("52", date.format("ww").unwrap());

        let date = Date::from_ymd(1979, 12, 31).unwrap();
        assert_eq!("1", date.format("w").unwrap());
        assert_eq!("01", date.format("ww").unwrap());

        let date = Date::from_ymd(2022, 3, 1).unwrap();
        assert_eq!("9", date.format("w").unwrap());
        assert_eq!("09", date.format("ww").unwrap());

        let date = Date::from_ymd(2032, 3, 1).unwrap();
        assert_eq!("10", date.format("w").unwrap());
        assert_eq!("10", date.format("ww").unwrap());
    }

    #[test]
    fn format_day() {
        let date = Date::from_ymd(1970, 1, 1).unwrap();
        assert_eq!("1", date.format("d").unwrap());
        assert_eq!("01", date.format("dd").unwrap());
        assert_eq!("01", date.format("ddd").unwrap());

        let date = Date::from_ymd(1970, 1, 15).unwrap();
        assert_eq!("15", date.format("d").unwrap());
        assert_eq!("15", date.format("dd").unwrap());
        assert_eq!("15", date.format("ddd").unwrap());

        let date = Date::from_ymd(1970, 1, 31).unwrap();
        assert_eq!("31", date.format("d").unwrap());
        assert_eq!("31", date.format("dd").unwrap());
        assert_eq!("31", date.format("ddd").unwrap());
    }

    #[test]
    fn format_year_day() {
        let date = Date::from_ymd(1970, 1, 1).unwrap();
        assert_eq!("1", date.format("D").unwrap());
        assert_eq!("01", date.format("DD").unwrap());
        assert_eq!("001", date.format("DDD").unwrap());
        assert_eq!("1", date.format("DDDD").unwrap());

        let date = Date::from_ymd(2020, 1, 24).unwrap();
        assert_eq!("24", date.format("D").unwrap());
        assert_eq!("24", date.format("DD").unwrap());
        assert_eq!("024", date.format("DDD").unwrap());
        assert_eq!("24", date.format("DDDD").unwrap());

        let date = Date::from_ymd(2020, 5, 15).unwrap();
        assert_eq!("136", date.format("D").unwrap());
        assert_eq!("136", date.format("DD").unwrap());
        assert_eq!("136", date.format("DDD").unwrap());
        assert_eq!("136", date.format("DDDD").unwrap());

        let date = Date::from_ymd(2022, 5, 15).unwrap();
        assert_eq!("135", date.format("D").unwrap());
        assert_eq!("135", date.format("DD").unwrap());
        assert_eq!("135", date.format("DDD").unwrap());
        assert_eq!("135", date.format("DDDD").unwrap());

        let date = Date::from_ymd(2300, 12, 31).unwrap();
        assert_eq!("365", date.format("D").unwrap());
        assert_eq!("365", date.format("DD").unwrap());
        assert_eq!("365", date.format("DDD").unwrap());
        assert_eq!("365", date.format("DDDD").unwrap());
    }

    #[test]
    fn format_wday() {
        let date = Date::from_ymd(1970, 1, 1).unwrap();
        assert_eq!("5", date.format("e").unwrap());
        assert_eq!("05", date.format("ee").unwrap());
        assert_eq!("Thu", date.format("eee").unwrap());
        assert_eq!("Thursday", date.format("eeee").unwrap());
        assert_eq!("T", date.format("eeeee").unwrap());
        assert_eq!("Th", date.format("eeeeee").unwrap());
        assert_eq!("4", date.format("eeeeeee").unwrap());
        assert_eq!("04", date.format("eeeeeeee").unwrap());
        assert_eq!("5", date.format("eeeeeeeee").unwrap());

        let date = Date::from_ymd(2020, 1, 1).unwrap();
        assert_eq!("4", date.format("e").unwrap());
        assert_eq!("04", date.format("ee").unwrap());
        assert_eq!("Wed", date.format("eee").unwrap());
        assert_eq!("Wednesday", date.format("eeee").unwrap());
        assert_eq!("W", date.format("eeeee").unwrap());
        assert_eq!("We", date.format("eeeeee").unwrap());
        assert_eq!("3", date.format("eeeeeee").unwrap());
        assert_eq!("03", date.format("eeeeeeee").unwrap());
        assert_eq!("4", date.format("eeeeeeeee").unwrap());

        let date = Date::from_ymd(2020, 5, 10).unwrap();
        assert_eq!("1", date.format("e").unwrap());
        assert_eq!("01", date.format("ee").unwrap());
        assert_eq!("Sun", date.format("eee").unwrap());
        assert_eq!("Sunday", date.format("eeee").unwrap());
        assert_eq!("S", date.format("eeeee").unwrap());
        assert_eq!("Su", date.format("eeeeee").unwrap());
        assert_eq!("7", date.format("eeeeeee").unwrap());
        assert_eq!("07", date.format("eeeeeeee").unwrap());
        assert_eq!("1", date.format("eeeeeeeee").unwrap());

        let date = Date::from_ymd(2020, 5, 11).unwrap();
        assert_eq!("2", date.format("e").unwrap());
        assert_eq!("02", date.format("ee").unwrap());
        assert_eq!("Mon", date.format("eee").unwrap());
        assert_eq!("Monday", date.format("eeee").unwrap());
        assert_eq!("M", date.format("eeeee").unwrap());
        assert_eq!("Mo", date.format("eeeeee").unwrap());
        assert_eq!("1", date.format("eeeeeee").unwrap());
        assert_eq!("01", date.format("eeeeeeee").unwrap());
        assert_eq!("2", date.format("eeeeeeeee").unwrap());
    }

    #[test]
    fn format_escape() {
        let date = Date::from_ymd(1970, 1, 1).unwrap();
        assert_eq!(
            "yyyMMdd19700101yyyMMdd",
            date.format("'yyyMMdd'yyyMMdd'yyyMMdd").unwrap()
        );
        assert_eq!(
            "yyyMMdd19700101yyyMMdd",
            date.format("'yyyMMdd'yyyMMdd'yyyMMdd'").unwrap()
        );
        assert_eq!(
            "yyyMMdd19700101yyyMMdd01",
            date.format("'yyyMMdd'yyyMMdd'yyyMMdd'dd").unwrap()
        );
        assert_eq!("yyyMMdd'dd", date.format("'yyyMMdd''dd").unwrap());
        assert_eq!(
            "yyyyMMdd19700101yyyMMdd'dd",
            date.format("'yyyyMMdd'yyyyMMdd'yyyMMdd''dd'").unwrap()
        );
        assert_eq!("''", date.format("''''").unwrap());
        assert_eq!("'01'", date.format("''dd''").unwrap());
        assert_eq!("'dd'", date.format("'''dd'''").unwrap());
        assert_eq!("''01''", date.format("''''dd''''").unwrap());
        assert_eq!("''dd''", date.format("'''''dd'''''").unwrap());
    }
}
