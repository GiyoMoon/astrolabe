#[cfg(test)]
mod format_tests {
    use astrolabe::{Date, DateTime, Time};

    #[test]
    fn format_era() {
        let date = Date::from_ymd(1970, 1, 1).unwrap();
        assert_eq!("AD", date.format("G"));
        assert_eq!("AD", date.format("GG"));
        assert_eq!("AD", date.format("GGG"));
        assert_eq!("Anno Domini", date.format("GGGG"));
        assert_eq!("A", date.format("GGGGG"));
        assert_eq!("Anno Domini", date.format("GGGGGG"));
        assert_eq!("Anno Domini", date.format("GGGGGGG"));
        let date = Date::from_ymd(-1, 1, 1).unwrap();
        assert_eq!("BC", date.format("G"));
        assert_eq!("BC", date.format("GG"));
        assert_eq!("BC", date.format("GGG"));
        assert_eq!("Before Christ", date.format("GGGG"));
        assert_eq!("B", date.format("GGGGG"));
        assert_eq!("Before Christ", date.format("GGGGGG"));
        assert_eq!("Before Christ", date.format("GGGGGGG"));
    }

    #[test]
    fn format_year() {
        let date = Date::from_ymd(1970, 1, 1).unwrap();
        assert_eq!("1970", date.format("y"));
        assert_eq!("70", date.format("yy"));
        assert_eq!("1970", date.format("yyy"));
        assert_eq!("1970", date.format("yyyy"));
        assert_eq!("01970", date.format("yyyyy"));

        let date = Date::from_ymd(2000, 12, 31).unwrap();
        assert_eq!("2000", date.format("y"));
        assert_eq!("00", date.format("yy"));
        assert_eq!("2000", date.format("yyy"));
        assert_eq!("2000", date.format("yyyy"));
        assert_eq!("02000", date.format("yyyyy"));

        let date = Date::from_ymd(2345, 1, 1).unwrap();
        assert_eq!("2345", date.format("y"));
        assert_eq!("45", date.format("yy"));
        assert_eq!("2345", date.format("yyy"));
        assert_eq!("2345", date.format("yyyy"));
        assert_eq!("02345", date.format("yyyyy"));

        let date = Date::from_ymd(1, 1, 1).unwrap();
        assert_eq!("1", date.format("y"));
        assert_eq!("01", date.format("yy"));
        assert_eq!("001", date.format("yyy"));
        assert_eq!("0001", date.format("yyyy"));
        assert_eq!("00001", date.format("yyyyy"));

        let date = Date::from_ymd(-1, 1, 1).unwrap();
        assert_eq!("-1", date.format("y"));
        assert_eq!("-01", date.format("yy"));
        assert_eq!("-001", date.format("yyy"));
        assert_eq!("-0001", date.format("yyyy"));
        assert_eq!("-00001", date.format("yyyyy"));

        let date = Date::from_ymd(-2, 1, 1).unwrap();
        assert_eq!("-2", date.format("y"));
        assert_eq!("-02", date.format("yy"));
        assert_eq!("-002", date.format("yyy"));
        assert_eq!("-0002", date.format("yyyy"));
        assert_eq!("-00002", date.format("yyyyy"));
    }

    #[test]
    fn format_quarter() {
        let date = Date::from_ymd(1970, 1, 1).unwrap();
        assert_eq!("1", date.format("q"));
        assert_eq!("01", date.format("qq"));
        assert_eq!("Q1", date.format("qqq"));
        assert_eq!("1st quarter", date.format("qqqq"));
        assert_eq!("1", date.format("qqqqq"));
        assert_eq!("1", date.format("qqqqqq"));
        let date = Date::from_ymd(1970, 4, 1).unwrap();
        assert_eq!("2", date.format("q"));
        assert_eq!("02", date.format("qq"));
        assert_eq!("Q2", date.format("qqq"));
        assert_eq!("2nd quarter", date.format("qqqq"));
        assert_eq!("2", date.format("qqqqq"));
        assert_eq!("2", date.format("qqqqqq"));
        let date = Date::from_ymd(1970, 7, 1).unwrap();
        assert_eq!("3", date.format("q"));
        assert_eq!("03", date.format("qq"));
        assert_eq!("Q3", date.format("qqq"));
        assert_eq!("3rd quarter", date.format("qqqq"));
        assert_eq!("3", date.format("qqqqq"));
        assert_eq!("3", date.format("qqqqqq"));
        let date = Date::from_ymd(1970, 10, 1).unwrap();
        assert_eq!("4", date.format("q"));
        assert_eq!("04", date.format("qq"));
        assert_eq!("Q4", date.format("qqq"));
        assert_eq!("4th quarter", date.format("qqqq"));
        assert_eq!("4", date.format("qqqqq"));
        assert_eq!("4", date.format("qqqqqq"));
        let date = Date::from_ymd(1970, 3, 31).unwrap();
        assert_eq!("1st quarter", date.format("qqqq"));
        let date = Date::from_ymd(1970, 4, 1).unwrap();
        assert_eq!("2nd quarter", date.format("qqqq"));
        let date = Date::from_ymd(1970, 6, 30).unwrap();
        assert_eq!("2nd quarter", date.format("qqqq"));
        let date = Date::from_ymd(1970, 7, 1).unwrap();
        assert_eq!("3rd quarter", date.format("qqqq"));
        let date = Date::from_ymd(1970, 9, 30).unwrap();
        assert_eq!("3rd quarter", date.format("qqqq"));
        let date = Date::from_ymd(1970, 10, 1).unwrap();
        assert_eq!("4th quarter", date.format("qqqq"));
        let date = Date::from_ymd(1970, 12, 31).unwrap();
        assert_eq!("4th quarter", date.format("qqqq"));
    }

    #[test]
    fn format_month() {
        let date = Date::from_ymd(1970, 1, 1).unwrap();
        assert_eq!("1", date.format("M"));
        assert_eq!("01", date.format("MM"));
        assert_eq!("Jan", date.format("MMM"));
        assert_eq!("January", date.format("MMMM"));
        assert_eq!("J", date.format("MMMMM"));
        assert_eq!("January", date.format("MMMMMM"));

        let date = Date::from_ymd(1970, 6, 1).unwrap();
        assert_eq!("6", date.format("M"));
        assert_eq!("06", date.format("MM"));
        assert_eq!("Jun", date.format("MMM"));
        assert_eq!("June", date.format("MMMM"));
        assert_eq!("J", date.format("MMMMM"));
        assert_eq!("June", date.format("MMMMMM"));

        let date = Date::from_ymd(1970, 12, 1).unwrap();
        assert_eq!("12", date.format("M"));
        assert_eq!("12", date.format("MM"));
        assert_eq!("Dec", date.format("MMM"));
        assert_eq!("December", date.format("MMMM"));
        assert_eq!("D", date.format("MMMMM"));
        assert_eq!("December", date.format("MMMMMM"));
    }

    #[test]
    fn format_week() {
        let date = Date::from_ymd(1970, 1, 1).unwrap();
        assert_eq!("1", date.format("w"));
        assert_eq!("01", date.format("ww"));
        assert_eq!("01", date.format("www"));

        let date = Date::from_ymd(1971, 1, 1).unwrap();
        assert_eq!("53", date.format("w"));
        assert_eq!("53", date.format("ww"));
        assert_eq!("53", date.format("www"));

        let date = Date::from_ymd(1972, 1, 1).unwrap();
        assert_eq!("52", date.format("w"));
        assert_eq!("52", date.format("ww"));

        let date = Date::from_ymd(1973, 1, 1).unwrap();
        assert_eq!("1", date.format("w"));
        assert_eq!("01", date.format("ww"));

        let date = Date::from_ymd(1974, 1, 1).unwrap();
        assert_eq!("1", date.format("w"));
        assert_eq!("01", date.format("ww"));

        let date = Date::from_ymd(1975, 1, 1).unwrap();
        assert_eq!("1", date.format("w"));
        assert_eq!("01", date.format("ww"));

        let date = Date::from_ymd(1976, 1, 1).unwrap();
        assert_eq!("1", date.format("w"));
        assert_eq!("01", date.format("ww"));

        let date = Date::from_ymd(1977, 1, 1).unwrap();
        assert_eq!("53", date.format("w"));
        assert_eq!("53", date.format("ww"));

        let date = Date::from_ymd(1978, 1, 1).unwrap();
        assert_eq!("52", date.format("w"));
        assert_eq!("52", date.format("ww"));

        let date = Date::from_ymd(1979, 1, 1).unwrap();
        assert_eq!("1", date.format("w"));
        assert_eq!("01", date.format("ww"));

        let date = Date::from_ymd(1970, 12, 31).unwrap();
        assert_eq!("53", date.format("w"));
        assert_eq!("53", date.format("ww"));

        let date = Date::from_ymd(1971, 12, 31).unwrap();
        assert_eq!("52", date.format("w"));
        assert_eq!("52", date.format("ww"));

        let date = Date::from_ymd(1972, 12, 31).unwrap();
        assert_eq!("52", date.format("w"));
        assert_eq!("52", date.format("ww"));

        let date = Date::from_ymd(1973, 12, 31).unwrap();
        assert_eq!("1", date.format("w"));
        assert_eq!("01", date.format("ww"));

        let date = Date::from_ymd(1974, 12, 31).unwrap();
        assert_eq!("1", date.format("w"));
        assert_eq!("01", date.format("ww"));

        let date = Date::from_ymd(1975, 12, 31).unwrap();
        assert_eq!("1", date.format("w"));
        assert_eq!("01", date.format("ww"));

        let date = Date::from_ymd(1976, 12, 31).unwrap();
        assert_eq!("53", date.format("w"));
        assert_eq!("53", date.format("ww"));

        let date = Date::from_ymd(1977, 12, 31).unwrap();
        assert_eq!("52", date.format("w"));
        assert_eq!("52", date.format("ww"));

        let date = Date::from_ymd(1978, 12, 31).unwrap();
        assert_eq!("52", date.format("w"));
        assert_eq!("52", date.format("ww"));

        let date = Date::from_ymd(1979, 12, 31).unwrap();
        assert_eq!("1", date.format("w"));
        assert_eq!("01", date.format("ww"));

        let date = Date::from_ymd(2022, 3, 1).unwrap();
        assert_eq!("9", date.format("w"));
        assert_eq!("09", date.format("ww"));

        let date = Date::from_ymd(2032, 3, 1).unwrap();
        assert_eq!("10", date.format("w"));
        assert_eq!("10", date.format("ww"));
    }

    #[test]
    fn format_day() {
        let date = Date::from_ymd(1970, 1, 1).unwrap();
        assert_eq!("1", date.format("d"));
        assert_eq!("01", date.format("dd"));
        assert_eq!("01", date.format("ddd"));

        let date = Date::from_ymd(1970, 1, 15).unwrap();
        assert_eq!("15", date.format("d"));
        assert_eq!("15", date.format("dd"));
        assert_eq!("15", date.format("ddd"));

        let date = Date::from_ymd(1970, 1, 31).unwrap();
        assert_eq!("31", date.format("d"));
        assert_eq!("31", date.format("dd"));
        assert_eq!("31", date.format("ddd"));
    }

    #[test]
    fn format_year_day() {
        let date = Date::from_ymd(1970, 1, 1).unwrap();
        assert_eq!("1", date.format("D"));
        assert_eq!("01", date.format("DD"));
        assert_eq!("001", date.format("DDD"));
        assert_eq!("1", date.format("DDDD"));

        let date = Date::from_ymd(2020, 1, 24).unwrap();
        assert_eq!("24", date.format("D"));
        assert_eq!("24", date.format("DD"));
        assert_eq!("024", date.format("DDD"));
        assert_eq!("24", date.format("DDDD"));

        let date = Date::from_ymd(2020, 5, 15).unwrap();
        assert_eq!("136", date.format("D"));
        assert_eq!("136", date.format("DD"));
        assert_eq!("136", date.format("DDD"));
        assert_eq!("136", date.format("DDDD"));

        let date = Date::from_ymd(2022, 5, 15).unwrap();
        assert_eq!("135", date.format("D"));
        assert_eq!("135", date.format("DD"));
        assert_eq!("135", date.format("DDD"));
        assert_eq!("135", date.format("DDDD"));

        let date = Date::from_ymd(2300, 12, 31).unwrap();
        assert_eq!("365", date.format("D"));
        assert_eq!("365", date.format("DD"));
        assert_eq!("365", date.format("DDD"));
        assert_eq!("365", date.format("DDDD"));
    }

    #[test]
    fn format_wday() {
        let date = Date::from_ymd(1970, 1, 1).unwrap();
        assert_eq!("5", date.format("e"));
        assert_eq!("05", date.format("ee"));
        assert_eq!("Thu", date.format("eee"));
        assert_eq!("Thursday", date.format("eeee"));
        assert_eq!("T", date.format("eeeee"));
        assert_eq!("Th", date.format("eeeeee"));
        assert_eq!("4", date.format("eeeeeee"));
        assert_eq!("04", date.format("eeeeeeee"));
        assert_eq!("5", date.format("eeeeeeeee"));

        let date = Date::from_ymd(2020, 1, 1).unwrap();
        assert_eq!("4", date.format("e"));
        assert_eq!("04", date.format("ee"));
        assert_eq!("Wed", date.format("eee"));
        assert_eq!("Wednesday", date.format("eeee"));
        assert_eq!("W", date.format("eeeee"));
        assert_eq!("We", date.format("eeeeee"));
        assert_eq!("3", date.format("eeeeeee"));
        assert_eq!("03", date.format("eeeeeeee"));
        assert_eq!("4", date.format("eeeeeeeee"));

        let date = Date::from_ymd(2020, 5, 10).unwrap();
        assert_eq!("1", date.format("e"));
        assert_eq!("01", date.format("ee"));
        assert_eq!("Sun", date.format("eee"));
        assert_eq!("Sunday", date.format("eeee"));
        assert_eq!("S", date.format("eeeee"));
        assert_eq!("Su", date.format("eeeeee"));
        assert_eq!("7", date.format("eeeeeee"));
        assert_eq!("07", date.format("eeeeeeee"));
        assert_eq!("1", date.format("eeeeeeeee"));

        let date = Date::from_ymd(2020, 5, 11).unwrap();
        assert_eq!("2", date.format("e"));
        assert_eq!("02", date.format("ee"));
        assert_eq!("Mon", date.format("eee"));
        assert_eq!("Monday", date.format("eeee"));
        assert_eq!("M", date.format("eeeee"));
        assert_eq!("Mo", date.format("eeeeee"));
        assert_eq!("1", date.format("eeeeeee"));
        assert_eq!("01", date.format("eeeeeeee"));
        assert_eq!("2", date.format("eeeeeeeee"));
    }

    #[test]
    fn format_period() {
        let time = Time::from_hms(0, 0, 0).unwrap();
        assert_eq!("AM", time.format("a"));
        assert_eq!("AM", time.format("aa"));
        assert_eq!("am", time.format("aaa"));
        assert_eq!("a.m.", time.format("aaaa"));
        assert_eq!("a", time.format("aaaaa"));
        assert_eq!("am", time.format("aaaaaa"));
        assert_eq!("midnight", time.format("b"));
        assert_eq!("midnight", time.format("bb"));
        assert_eq!("midnight", time.format("bbb"));
        assert_eq!("midnight", time.format("bbbb"));
        assert_eq!("mi", time.format("bbbbb"));
        assert_eq!("midnight", time.format("bbbbbb"));

        let time = Time::from_hms(12, 0, 0).unwrap();
        assert_eq!("PM", time.format("a"));
        assert_eq!("PM", time.format("aa"));
        assert_eq!("pm", time.format("aaa"));
        assert_eq!("p.m.", time.format("aaaa"));
        assert_eq!("p", time.format("aaaaa"));
        assert_eq!("pm", time.format("aaaaaa"));
        assert_eq!("noon", time.format("b"));
        assert_eq!("noon", time.format("bb"));
        assert_eq!("noon", time.format("bbb"));
        assert_eq!("noon", time.format("bbbb"));
        assert_eq!("n", time.format("bbbbb"));
        assert_eq!("noon", time.format("bbbbbb"));

        let time = Time::from_hms(1, 0, 0).unwrap();
        assert_eq!("AM", time.format("a"));
        assert_eq!("AM", time.format("aa"));
        assert_eq!("am", time.format("aaa"));
        assert_eq!("a.m.", time.format("aaaa"));
        assert_eq!("a", time.format("aaaaa"));
        assert_eq!("am", time.format("aaaaaa"));
        assert_eq!("AM", time.format("b"));
        assert_eq!("AM", time.format("bb"));
        assert_eq!("am", time.format("bbb"));
        assert_eq!("a.m.", time.format("bbbb"));
        assert_eq!("a", time.format("bbbbb"));
        assert_eq!("am", time.format("bbbbbb"));

        let time = Time::from_hms(13, 0, 0).unwrap();
        assert_eq!("PM", time.format("a"));
        assert_eq!("PM", time.format("aa"));
        assert_eq!("pm", time.format("aaa"));
        assert_eq!("p.m.", time.format("aaaa"));
        assert_eq!("p", time.format("aaaaa"));
        assert_eq!("pm", time.format("aaaaaa"));
        assert_eq!("PM", time.format("b"));
        assert_eq!("PM", time.format("bb"));
        assert_eq!("pm", time.format("bbb"));
        assert_eq!("p.m.", time.format("bbbb"));
        assert_eq!("p", time.format("bbbbb"));
        assert_eq!("pm", time.format("bbbbbb"));
    }

    #[test]
    fn hour() {
        let time = Time::from_hms(0, 0, 0).unwrap();
        assert_eq!("12", time.format("h"));
        assert_eq!("12", time.format("hh"));
        assert_eq!("0", time.format("H"));
        assert_eq!("00", time.format("HH"));
        assert_eq!("0", time.format("K"));
        assert_eq!("00", time.format("KK"));
        assert_eq!("24", time.format("k"));
        assert_eq!("24", time.format("kk"));

        let time = Time::from_hms(1, 0, 0).unwrap();
        assert_eq!("1", time.format("h"));
        assert_eq!("01", time.format("hh"));
        assert_eq!("01", time.format("hhh"));
        assert_eq!("1", time.format("H"));
        assert_eq!("01", time.format("HH"));
        assert_eq!("01", time.format("HHH"));
        assert_eq!("1", time.format("K"));
        assert_eq!("01", time.format("KK"));
        assert_eq!("01", time.format("KKK"));
        assert_eq!("1", time.format("k"));
        assert_eq!("01", time.format("kk"));
        assert_eq!("01", time.format("kkk"));

        let time = Time::from_hms(2, 0, 0).unwrap();
        assert_eq!("2", time.format("h"));
        assert_eq!("02", time.format("hh"));
        assert_eq!("2", time.format("H"));
        assert_eq!("02", time.format("HH"));
        assert_eq!("2", time.format("K"));
        assert_eq!("02", time.format("KK"));
        assert_eq!("2", time.format("k"));
        assert_eq!("02", time.format("kk"));

        let time = Time::from_hms(12, 0, 0).unwrap();
        assert_eq!("12", time.format("h"));
        assert_eq!("12", time.format("hh"));
        assert_eq!("12", time.format("H"));
        assert_eq!("12", time.format("HH"));
        assert_eq!("0", time.format("K"));
        assert_eq!("00", time.format("KK"));
        assert_eq!("12", time.format("k"));
        assert_eq!("12", time.format("kk"));

        let time = Time::from_hms(15, 0, 0).unwrap();
        assert_eq!("3", time.format("h"));
        assert_eq!("03", time.format("hh"));
        assert_eq!("15", time.format("H"));
        assert_eq!("15", time.format("HH"));
        assert_eq!("3", time.format("K"));
        assert_eq!("03", time.format("KK"));
        assert_eq!("15", time.format("k"));
        assert_eq!("15", time.format("kk"));
    }

    #[test]
    fn minute() {
        let time = Time::from_hms(0, 0, 0).unwrap();
        assert_eq!("0", time.format("m"));
        assert_eq!("00", time.format("mm"));
        assert_eq!("00", time.format("mmm"));
        let time = Time::from_hms(0, 30, 0).unwrap();
        assert_eq!("30", time.format("m"));
        assert_eq!("30", time.format("mm"));
        assert_eq!("30", time.format("mmm"));
        let time = Time::from_hms(0, 59, 0).unwrap();
        assert_eq!("59", time.format("m"));
        assert_eq!("59", time.format("mm"));
        assert_eq!("59", time.format("mmm"));
    }

    #[test]
    fn second() {
        let time = Time::from_hms(0, 0, 0).unwrap();
        assert_eq!("0", time.format("s"));
        assert_eq!("00", time.format("ss"));
        assert_eq!("00", time.format("sss"));
        let time = Time::from_hms(0, 0, 30).unwrap();
        assert_eq!("30", time.format("s"));
        assert_eq!("30", time.format("ss"));
        assert_eq!("30", time.format("sss"));
        let time = Time::from_hms(0, 0, 59).unwrap();
        assert_eq!("59", time.format("s"));
        assert_eq!("59", time.format("ss"));
        assert_eq!("59", time.format("sss"));

        let date_time = DateTime::from_hms(0, 0, 0).unwrap();
        assert_eq!("0", date_time.format("s"));
        assert_eq!("00", date_time.format("ss"));
        assert_eq!("00", date_time.format("sss"));
        let date_time = DateTime::from_hms(0, 0, 30).unwrap();
        assert_eq!("30", date_time.format("s"));
        assert_eq!("30", date_time.format("ss"));
        assert_eq!("30", date_time.format("sss"));
        let date_time = DateTime::from_hms(0, 0, 59).unwrap();
        assert_eq!("59", date_time.format("s"));
        assert_eq!("59", date_time.format("ss"));
        assert_eq!("59", date_time.format("sss"));
    }

    #[test]
    fn format_escape() {
        let date = Date::from_ymd(1970, 1, 1).unwrap();
        assert_eq!(
            "yyyMMdd19700101yyyMMdd",
            date.format("'yyyMMdd'yyyMMdd'yyyMMdd")
        );
        assert_eq!(
            "yyyMMdd19700101yyyMMdd",
            date.format("'yyyMMdd'yyyMMdd'yyyMMdd'")
        );
        assert_eq!(
            "yyyMMdd19700101yyyMMdd01",
            date.format("'yyyMMdd'yyyMMdd'yyyMMdd'dd")
        );
        assert_eq!("yyyMMdd'dd", date.format("'yyyMMdd''dd"));
        assert_eq!(
            "yyyyMMdd19700101yyyMMdd'dd",
            date.format("'yyyyMMdd'yyyyMMdd'yyyMMdd''dd'")
        );
        assert_eq!("''", date.format("''''"));
        assert_eq!("'01'", date.format("''dd''"));
        assert_eq!("'dd'", date.format("'''dd'''"));
        assert_eq!("''01''", date.format("''''dd''''"));
        assert_eq!("''dd''", date.format("'''''dd'''''"));

        let time = Time::from_hms(12, 32, 1).unwrap();
        assert_eq!("HHmmss123201HHmmss", time.format("'HHmmss'HHmmss'HHmmss"));
        assert_eq!("HHmmss123201HHmmss", time.format("'HHmmss'HHmmss'HHmmss'"));
        assert_eq!(
            "HHmmss123201HHmmss01",
            time.format("'HHmmss'HHmmss'HHmmss'ss")
        );
        assert_eq!("HHmmss'ss", time.format("'HHmmss''ss"));
        assert_eq!(
            "HHmmss123201HHmmss'ss",
            time.format("'HHmmss'HHmmss'HHmmss''ss'")
        );
        assert_eq!("''", time.format("''''"));
        assert_eq!("'01'", time.format("''ss''"));
        assert_eq!("'ss'", time.format("'''ss'''"));
        assert_eq!("''01''", time.format("''''ss''''"));
        assert_eq!("''ss''", time.format("'''''ss'''''"));
        assert_eq!("test", time.format("te's't"));

        let date_time = DateTime::from_ymd(1970, 1, 1).unwrap();
        assert_eq!(
            "yyyMMdd19700101yyyMMdd",
            date_time.format("'yyyMMdd'yyyMMdd'yyyMMdd")
        );
        assert_eq!(
            "yyyMMdd19700101yyyMMdd",
            date_time.format("'yyyMMdd'yyyMMdd'yyyMMdd'")
        );
        assert_eq!(
            "yyyMMdd19700101yyyMMdd01",
            date_time.format("'yyyMMdd'yyyMMdd'yyyMMdd'dd")
        );
        assert_eq!("yyyMMdd'dd", date_time.format("'yyyMMdd''dd"));
        assert_eq!(
            "yyyyMMdd19700101yyyMMdd'dd",
            date_time.format("'yyyyMMdd'yyyyMMdd'yyyMMdd''dd'")
        );
        assert_eq!("''", date_time.format("''''"));
        assert_eq!("'01'", date_time.format("''dd''"));
        assert_eq!("'dd'", date_time.format("'''dd'''"));
        assert_eq!("''01''", date_time.format("''''dd''''"));
        assert_eq!("''dd''", date_time.format("'''''dd'''''"));

        assert_eq!("", date_time.format(""));
    }
}
