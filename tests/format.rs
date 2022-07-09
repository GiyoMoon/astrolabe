#[cfg(feature = "format")]
#[cfg(test)]
mod tests {
    use astrolabe::DateTime;

    #[test]
    fn format_year() {
        let date_time = DateTime::from_ymd(1970, 1, 1).unwrap();
        assert_eq!("1970", date_time.format("y").unwrap());
        assert_eq!("70", date_time.format("yy").unwrap());
        assert_eq!("1970", date_time.format("yyy").unwrap());
        assert_eq!("1970", date_time.format("yyyy").unwrap());
        assert_eq!("01970", date_time.format("yyyyy").unwrap());

        let date_time = DateTime::from_ymd(2000, 12, 31).unwrap();
        assert_eq!("2000", date_time.format("y").unwrap());
        assert_eq!("00", date_time.format("yy").unwrap());
        assert_eq!("2000", date_time.format("yyy").unwrap());
        assert_eq!("2000", date_time.format("yyyy").unwrap());
        assert_eq!("02000", date_time.format("yyyyy").unwrap());

        let date_time = DateTime::from_ymd(2345, 1, 1).unwrap();
        assert_eq!("2345", date_time.format("y").unwrap());
        assert_eq!("45", date_time.format("yy").unwrap());
        assert_eq!("2345", date_time.format("yyy").unwrap());
        assert_eq!("2345", date_time.format("yyyy").unwrap());
        assert_eq!("02345", date_time.format("yyyyy").unwrap());
    }

    #[test]
    fn format_month() {
        let date_time = DateTime::from_ymd(1970, 1, 1).unwrap();
        assert_eq!("1", date_time.format("M").unwrap());
        assert_eq!("01", date_time.format("MM").unwrap());
        assert_eq!("Jan", date_time.format("MMM").unwrap());
        assert_eq!("January", date_time.format("MMMM").unwrap());
        assert_eq!("J", date_time.format("MMMMM").unwrap());
        assert_eq!("January", date_time.format("MMMMMM").unwrap());

        let date_time = DateTime::from_ymd(1970, 6, 1).unwrap();
        assert_eq!("6", date_time.format("M").unwrap());
        assert_eq!("06", date_time.format("MM").unwrap());
        assert_eq!("Jun", date_time.format("MMM").unwrap());
        assert_eq!("June", date_time.format("MMMM").unwrap());
        assert_eq!("J", date_time.format("MMMMM").unwrap());
        assert_eq!("June", date_time.format("MMMMMM").unwrap());

        let date_time = DateTime::from_ymd(1970, 12, 1).unwrap();
        assert_eq!("12", date_time.format("M").unwrap());
        assert_eq!("12", date_time.format("MM").unwrap());
        assert_eq!("Dec", date_time.format("MMM").unwrap());
        assert_eq!("December", date_time.format("MMMM").unwrap());
        assert_eq!("D", date_time.format("MMMMM").unwrap());
        assert_eq!("December", date_time.format("MMMMMM").unwrap());
    }

    #[test]
    fn format_day() {
        let date_time = DateTime::from_ymd(1970, 1, 1).unwrap();
        assert_eq!("1", date_time.format("d").unwrap());
        assert_eq!("01", date_time.format("dd").unwrap());
        assert_eq!("01", date_time.format("ddd").unwrap());

        let date_time = DateTime::from_ymd(1970, 1, 15).unwrap();
        assert_eq!("15", date_time.format("d").unwrap());
        assert_eq!("15", date_time.format("dd").unwrap());
        assert_eq!("15", date_time.format("ddd").unwrap());

        let date_time = DateTime::from_ymd(1970, 1, 31).unwrap();
        assert_eq!("31", date_time.format("d").unwrap());
        assert_eq!("31", date_time.format("dd").unwrap());
        assert_eq!("31", date_time.format("ddd").unwrap());
    }

    #[test]
    fn format_hour() {
        let date_time = DateTime::from_ymdhms(1970, 1, 1, 0, 0, 0).unwrap();
        assert_eq!("12", date_time.format("h").unwrap());
        assert_eq!("12", date_time.format("hh").unwrap());
        assert_eq!("12", date_time.format("hhh").unwrap());
        assert_eq!("0", date_time.format("H").unwrap());
        assert_eq!("00", date_time.format("HH").unwrap());
        assert_eq!("00", date_time.format("HHH").unwrap());
        assert_eq!("0", date_time.format("K").unwrap());
        assert_eq!("00", date_time.format("KK").unwrap());
        assert_eq!("00", date_time.format("KKK").unwrap());
        assert_eq!("24", date_time.format("k").unwrap());
        assert_eq!("24", date_time.format("kk").unwrap());
        assert_eq!("24", date_time.format("kkk").unwrap());

        let date_time = DateTime::from_ymdhms(1970, 1, 1, 1, 0, 0).unwrap();
        assert_eq!("1", date_time.format("h").unwrap());
        assert_eq!("01", date_time.format("hh").unwrap());
        assert_eq!("01", date_time.format("hhh").unwrap());
        assert_eq!("1", date_time.format("H").unwrap());
        assert_eq!("01", date_time.format("HH").unwrap());
        assert_eq!("01", date_time.format("HHH").unwrap());
        assert_eq!("1", date_time.format("K").unwrap());
        assert_eq!("01", date_time.format("KK").unwrap());
        assert_eq!("01", date_time.format("KKK").unwrap());
        assert_eq!("1", date_time.format("k").unwrap());
        assert_eq!("01", date_time.format("kk").unwrap());
        assert_eq!("01", date_time.format("kkk").unwrap());

        let date_time = DateTime::from_ymdhms(1970, 12, 31, 12, 0, 0).unwrap();
        assert_eq!("12", date_time.format("h").unwrap());
        assert_eq!("12", date_time.format("hh").unwrap());
        assert_eq!("12", date_time.format("hhh").unwrap());
        assert_eq!("12", date_time.format("H").unwrap());
        assert_eq!("12", date_time.format("HH").unwrap());
        assert_eq!("12", date_time.format("HHH").unwrap());
        assert_eq!("0", date_time.format("K").unwrap());
        assert_eq!("00", date_time.format("KK").unwrap());
        assert_eq!("00", date_time.format("KKK").unwrap());
        assert_eq!("12", date_time.format("k").unwrap());
        assert_eq!("12", date_time.format("kk").unwrap());
        assert_eq!("12", date_time.format("kkk").unwrap());

        let date_time = DateTime::from_ymdhms(1970, 12, 31, 15, 0, 0).unwrap();
        assert_eq!("3", date_time.format("h").unwrap());
        assert_eq!("03", date_time.format("hh").unwrap());
        assert_eq!("03", date_time.format("hhh").unwrap());
        assert_eq!("15", date_time.format("H").unwrap());
        assert_eq!("15", date_time.format("HH").unwrap());
        assert_eq!("15", date_time.format("HHH").unwrap());
        assert_eq!("3", date_time.format("K").unwrap());
        assert_eq!("03", date_time.format("KK").unwrap());
        assert_eq!("03", date_time.format("KKK").unwrap());
        assert_eq!("15", date_time.format("k").unwrap());
        assert_eq!("15", date_time.format("kk").unwrap());
        assert_eq!("15", date_time.format("kkk").unwrap());

        let date_time = DateTime::from_ymdhms(1970, 12, 31, 23, 0, 0).unwrap();
        assert_eq!("11", date_time.format("h").unwrap());
        assert_eq!("11", date_time.format("hh").unwrap());
        assert_eq!("11", date_time.format("hhh").unwrap());
        assert_eq!("23", date_time.format("H").unwrap());
        assert_eq!("23", date_time.format("HH").unwrap());
        assert_eq!("23", date_time.format("HHH").unwrap());
        assert_eq!("11", date_time.format("K").unwrap());
        assert_eq!("11", date_time.format("KK").unwrap());
        assert_eq!("11", date_time.format("KKK").unwrap());
        assert_eq!("23", date_time.format("k").unwrap());
        assert_eq!("23", date_time.format("kk").unwrap());
        assert_eq!("23", date_time.format("kkk").unwrap());
    }

    #[test]
    fn format_minute() {
        let date_time = DateTime::from_ymdhms(1970, 1, 1, 0, 0, 0).unwrap();
        assert_eq!("0", date_time.format("m").unwrap());
        assert_eq!("00", date_time.format("mm").unwrap());
        assert_eq!("00", date_time.format("mmm").unwrap());

        let date_time = DateTime::from_ymdhms(1970, 1, 1, 0, 59, 0).unwrap();
        assert_eq!("59", date_time.format("m").unwrap());
        assert_eq!("59", date_time.format("mm").unwrap());
        assert_eq!("59", date_time.format("mmm").unwrap());
    }

    #[test]
    fn format_second() {
        let date_time = DateTime::from_ymdhms(1970, 1, 1, 0, 0, 0).unwrap();
        assert_eq!("0", date_time.format("s").unwrap());
        assert_eq!("00", date_time.format("ss").unwrap());
        assert_eq!("00", date_time.format("sss").unwrap());

        let date_time = DateTime::from_ymdhms(1970, 1, 1, 0, 0, 59).unwrap();
        assert_eq!("59", date_time.format("s").unwrap());
        assert_eq!("59", date_time.format("ss").unwrap());
        assert_eq!("59", date_time.format("sss").unwrap());
    }

    #[test]
    fn format_rfc_3339() {
        let date_time = DateTime::from_ymdhms(1970, 1, 1, 0, 0, 0).unwrap();
        assert_eq!(
            "1970-01-01T00:00:00+00:00",
            date_time.format("yyyy-MM-ddTHH:mm:ss+00:00").unwrap()
        );

        let date_time = DateTime::from_ymdhms(1970, 1, 1, 0, 0, 0).unwrap();
        assert_eq!(
            "1970-01-01T00:00:00+00:00",
            date_time
                .format("yyyy'-'MM'-'dd'T'HH':'mm':'ss'+00:00'")
                .unwrap()
        );

        let date_time = DateTime::from_ymdhms(1970, 1, 2, 3, 4, 5).unwrap();
        assert_eq!(
            "1970-01-02T03:04:05+00:00",
            date_time.format("yyyy-MM-ddTHH:mm:ss+00:00").unwrap()
        );

        let date_time = DateTime::from_ymdhms(2000, 12, 31, 23, 59, 59).unwrap();
        assert_eq!(
            "2000-12-31T23:59:59+00:00",
            date_time.format("yyyy-MM-ddTHH:mm:ss+00:00").unwrap()
        );
    }
}