#[cfg(test)]
mod parse_tests {
    use astrolabe::Date;

    #[test]
    fn era() {
        parse_ok("AD2022-05-02", "Gyyyy-MM-dd");
        parse_ok("2022-AD05-02", "yyyy-GMM-dd");
        parse_ok("2022-05-02AD", "yyyy-MM-ddG");

        parse_ok("AD2022-05-02", "GGyyyy-MM-dd");
        parse_ok("2022-AD05-02", "yyyy-GGMM-dd");
        parse_ok("2022-05-02AD", "yyyy-MM-ddGG");

        parse_ok("AD2022-05-02", "GGGyyyy-MM-dd");
        parse_ok("2022-AD05-02", "yyyy-GGGMM-dd");
        parse_ok("2022-05-02AD", "yyyy-MM-ddGGG");

        parse_ok("Anno Domini2022-05-02", "GGGGyyyy-MM-dd");
        parse_ok("2022-Anno Domini05-02", "yyyy-GGGGMM-dd");
        parse_ok("2022-05-02Anno Domini", "yyyy-MM-ddGGGG");

        parse_ok("A2022-05-02", "GGGGGyyyy-MM-dd");
        parse_ok("2022-A05-02", "yyyy-GGGGGMM-dd");
        parse_ok("2022-05-02A", "yyyy-MM-ddGGGGG");

        parse_ok("Anno Domini2022-05-02", "GGGGGGyyyy-MM-dd");
        parse_ok("2022-Anno Domini05-02", "yyyy-GGGGGGMM-dd");
        parse_ok("2022-05-02Anno Domini", "yyyy-MM-ddGGGGGG");

        parse_ok("Anno Domini2022-05-02", "GGGGGGGyyyy-MM-dd");
        parse_ok("2022-Anno Domini05-02", "yyyy-GGGGGGGMM-dd");
        parse_ok("2022-05-02Anno Domini", "yyyy-MM-ddGGGGGGG");

        parse_ok("BC2022-05-02", "Gyyyy-MM-dd");
        parse_ok("2022-BC05-02", "yyyy-GMM-dd");
        parse_ok("2022-05-02BC", "yyyy-MM-ddG");

        parse_ok("BC2022-05-02", "GGyyyy-MM-dd");
        parse_ok("2022-BC05-02", "yyyy-GGMM-dd");
        parse_ok("2022-05-02BC", "yyyy-MM-ddGG");

        parse_ok("BC2022-05-02", "GGGyyyy-MM-dd");
        parse_ok("2022-BC05-02", "yyyy-GGGMM-dd");
        parse_ok("2022-05-02BC", "yyyy-MM-ddGGG");

        parse_ok("Before Christ2022-05-02", "GGGGyyyy-MM-dd");
        parse_ok("2022-Before Christ05-02", "yyyy-GGGGMM-dd");
        parse_ok("2022-05-02Before Christ", "yyyy-MM-ddGGGG");

        parse_ok("B2022-05-02", "GGGGGyyyy-MM-dd");
        parse_ok("2022-B05-02", "yyyy-GGGGGMM-dd");
        parse_ok("2022-05-02B", "yyyy-MM-ddGGGGG");

        parse_ok("Before Christ2022-05-02", "GGGGGGyyyy-MM-dd");
        parse_ok("2022-Before Christ05-02", "yyyy-GGGGGGMM-dd");
        parse_ok("2022-05-02Before Christ", "yyyy-MM-ddGGGGGG");

        parse_ok("Before Christ2022-05-02", "GGGGGGGyyyy-MM-dd");
        parse_ok("2022-Before Christ05-02", "yyyy-GGGGGGGMM-dd");
        parse_ok("2022-05-02Before Christ", "yyyy-MM-ddGGGGGGG");

        parse_err("AD", "G");
        parse_err("AD", "GGGG");
        parse_err("AD2022-05-02", "GGGGyyyy-MM-dd");
        parse_err("ADU2022-05-02", "Gyyyy-MM-dd");

        parse_err("", "G");
        parse_err("", "GGGGG");
    }

    #[test]
    fn year() {
        parse_ok("2022-05-02", "y-MM-dd");
        parse_ok("22-05-02", "yy-MM-dd");
        parse_ok("2022-05-02", "yyy-MM-dd");
        parse_ok("2022-05-02", "yyyy-MM-dd");
        parse_ok("02022-05-02", "yyyyy-MM-dd");

        parse_ok("05-2022-02", "MM-y-dd");
        parse_ok("05-22-02", "MM-yy-dd");
        parse_ok("05-2022-02", "MM-yyy-dd");
        parse_ok("05-2022-02", "MM-yyyy-dd");
        parse_ok("05-02022-02", "MM-yyyyy-dd");

        parse_ok("05-02-2022", "MM-dd-y");
        parse_ok("05-02-22", "MM-dd-yy");
        parse_ok("05-02-2022", "MM-dd-yyy");
        parse_ok("05-02-2022", "MM-dd-yyyy");
        parse_ok("05-02-02022", "MM-dd-yyyyy");

        parse_ok_custom("-1234-05-02", "y-MM-dd", "-1234/05/02");
        parse_ok_custom("-34-05-02", "yy-MM-dd", "-0034/05/02");
        parse_ok_custom("-1234-05-02", "yyy-MM-dd", "-1234/05/02");
        parse_ok_custom("-1234-05-02", "yyyy-MM-dd", "-1234/05/02");
        parse_ok_custom("-01234-05-02", "yyyyy-MM-dd", "-1234/05/02");

        parse_ok_custom("05--1234-02", "MM-y-dd", "-1234/05/02");
        parse_ok_custom("05--34-02", "MM-yy-dd", "-0034/05/02");
        parse_ok_custom("05--1234-02", "MM-yyy-dd", "-1234/05/02");
        parse_ok_custom("05--1234-02", "MM-yyyy-dd", "-1234/05/02");
        parse_ok_custom("05--01234-02", "MM-yyyyy-dd", "-1234/05/02");

        parse_ok_custom("05-02--1234", "MM-dd-y", "-1234/05/02");
        parse_ok_custom("05-02--34", "MM-dd-yy", "-0034/05/02");
        parse_ok_custom("05-02--1234", "MM-dd-yyy", "-1234/05/02");
        parse_ok_custom("05-02--1234", "MM-dd-yyyy", "-1234/05/02");
        parse_ok_custom("05-02--01234", "MM-dd-yyyyy", "-1234/05/02");

        parse_err("-1", "yy");
        parse_err("-aa", "yy");

        parse_err("1", "yy");
        parse_err("aa", "yy");

        parse_err("-", "y");

        parse_err("-", "yyyyy");
        parse_err("", "yyyyy");
    }

    #[test]
    fn quarter() {
        parse_ok("12022-05-02", "qyyyy-MM-dd");
        parse_ok("012022-05-02", "qqyyyy-MM-dd");
        parse_ok("Q12022-05-02", "qqqyyyy-MM-dd");
        parse_ok("1st quarter2022-05-02", "qqqqyyyy-MM-dd");
        parse_ok("12022-05-02", "qqqqqyyyy-MM-dd");
        parse_ok("12022-05-02", "qqqqqqyyyy-MM-dd");

        parse_ok("2022-105-02", "yyyy-qMM-dd");
        parse_ok("2022-0105-02", "yyyy-qqMM-dd");
        parse_ok("2022-Q105-02", "yyyy-qqqMM-dd");
        parse_ok("2022-1st quarter05-02", "yyyy-qqqqMM-dd");
        parse_ok("2022-105-02", "yyyy-qqqqqMM-dd");
        parse_ok("2022-105-02", "yyyy-qqqqqqMM-dd");

        parse_ok("2022-05-021", "yyyy-MM-ddq");
        parse_ok("2022-05-0201", "yyyy-MM-ddqq");
        parse_ok("2022-05-02Q1", "yyyy-MM-ddqqq");
        parse_ok("2022-05-021st quarter", "yyyy-MM-ddqqqq");
        parse_ok("2022-05-021", "yyyy-MM-ddqqqqq");
        parse_ok("2022-05-021", "yyyy-MM-ddqqqqqq");

        parse_ok("22022-05-02", "qyyyy-MM-dd");
        parse_ok("022022-05-02", "qqyyyy-MM-dd");
        parse_ok("Q22022-05-02", "qqqyyyy-MM-dd");
        parse_ok("2nd quarter2022-05-02", "qqqqyyyy-MM-dd");
        parse_ok("22022-05-02", "qqqqqyyyy-MM-dd");
        parse_ok("22022-05-02", "qqqqqqyyyy-MM-dd");

        parse_ok("2022-205-02", "yyyy-qMM-dd");
        parse_ok("2022-0205-02", "yyyy-qqMM-dd");
        parse_ok("2022-Q205-02", "yyyy-qqqMM-dd");
        parse_ok("2022-2nd quarter05-02", "yyyy-qqqqMM-dd");
        parse_ok("2022-205-02", "yyyy-qqqqqMM-dd");
        parse_ok("2022-205-02", "yyyy-qqqqqqMM-dd");

        parse_ok("2022-05-022", "yyyy-MM-ddq");
        parse_ok("2022-05-0202", "yyyy-MM-ddqq");
        parse_ok("2022-05-02Q2", "yyyy-MM-ddqqq");
        parse_ok("2022-05-022nd quarter", "yyyy-MM-ddqqqq");
        parse_ok("2022-05-022", "yyyy-MM-ddqqqqq");
        parse_ok("2022-05-022", "yyyy-MM-ddqqqqqq");

        parse_ok("32022-05-02", "qyyyy-MM-dd");
        parse_ok("032022-05-02", "qqyyyy-MM-dd");
        parse_ok("Q32022-05-02", "qqqyyyy-MM-dd");
        parse_ok("3rd quarter2022-05-02", "qqqqyyyy-MM-dd");
        parse_ok("32022-05-02", "qqqqqyyyy-MM-dd");
        parse_ok("32022-05-02", "qqqqqqyyyy-MM-dd");

        parse_ok("2022-305-02", "yyyy-qMM-dd");
        parse_ok("2022-0305-02", "yyyy-qqMM-dd");
        parse_ok("2022-Q305-02", "yyyy-qqqMM-dd");
        parse_ok("2022-3rd quarter05-02", "yyyy-qqqqMM-dd");
        parse_ok("2022-305-02", "yyyy-qqqqqMM-dd");
        parse_ok("2022-305-02", "yyyy-qqqqqqMM-dd");

        parse_ok("2022-05-023", "yyyy-MM-ddq");
        parse_ok("2022-05-0203", "yyyy-MM-ddqq");
        parse_ok("2022-05-02Q3", "yyyy-MM-ddqqq");
        parse_ok("2022-05-023rd quarter", "yyyy-MM-ddqqqq");
        parse_ok("2022-05-023", "yyyy-MM-ddqqqqq");
        parse_ok("2022-05-023", "yyyy-MM-ddqqqqqq");

        parse_ok("42022-05-02", "qyyyy-MM-dd");
        parse_ok("042022-05-02", "qqyyyy-MM-dd");
        parse_ok("Q42022-05-02", "qqqyyyy-MM-dd");
        parse_ok("4th quarter2022-05-02", "qqqqyyyy-MM-dd");
        parse_ok("42022-05-02", "qqqqqyyyy-MM-dd");
        parse_ok("42022-05-02", "qqqqqqyyyy-MM-dd");

        parse_ok("2022-405-02", "yyyy-qMM-dd");
        parse_ok("2022-0405-02", "yyyy-qqMM-dd");
        parse_ok("2022-Q405-02", "yyyy-qqqMM-dd");
        parse_ok("2022-4th quarter05-02", "yyyy-qqqqMM-dd");
        parse_ok("2022-405-02", "yyyy-qqqqqMM-dd");
        parse_ok("2022-405-02", "yyyy-qqqqqqMM-dd");

        parse_ok("2022-05-024", "yyyy-MM-ddq");
        parse_ok("2022-05-0204", "yyyy-MM-ddqq");
        parse_ok("2022-05-02Q4", "yyyy-MM-ddqqq");
        parse_ok("2022-05-024th quarter", "yyyy-MM-ddqqqq");
        parse_ok("2022-05-024", "yyyy-MM-ddqqqqq");
        parse_ok("2022-05-024", "yyyy-MM-ddqqqqqq");

        parse_err("", "q");
        parse_err("", "qqq");
        parse_err("", "qqqq");
        parse_err("", "qqqqq");
    }

    #[test]
    fn month() {
        parse_ok("5-2022-02", "M-yyyy-dd");
        parse_ok("05-2022-02", "MM-yyyy-dd");
        parse_ok("May-2022-02", "MMM-yyyy-dd");
        parse_ok("May-2022-02", "MMMM-yyyy-dd");
        parse_ok("M-05-2022-02", "MMMMM-MM-yyyy-dd");
        parse_ok("May-2022-02", "MMMMMM-yyyy-dd");

        parse_ok("2022-5-02", "yyyy-M-dd");
        parse_ok("2022-05-02", "yyyy-MM-dd");
        parse_ok("2022-May-02", "yyyy-MMM-dd");
        parse_ok("2022-May-02", "yyyy-MMMM-dd");
        parse_ok("2022-M-05-02", "yyyy-MMMMM-MM-dd");
        parse_ok("2022-May-02", "yyyy-MMMMMM-dd");

        parse_ok("2022-02-5", "yyyy-dd-M");
        parse_ok("2022-02-05", "yyyy-dd-MM");
        parse_ok("2022-02-May", "yyyy-dd-MMM");
        parse_ok("2022-02-May", "yyyy-dd-MMMM");
        parse_ok("2022-02-05-M", "yyyy-dd-MM-MMMMM");
        parse_ok("2022-02-May", "yyyy-dd-MMMMMM");

        parse_ok_custom("1-2022-02", "M-yyyy-dd", "2022/01/02");

        parse_err("", "M");
        parse_err("blabla", "MMM");
        parse_err("", "MMMMM");
        parse_err("blabla", "MMMMMM");
    }

    #[test]
    fn week() {
        parse_ok("1-2022-05-02", "w-yyyy-MM-dd");
        parse_ok("102022-05-02", "wyyyy-MM-dd");
        parse_ok("012022-05-02", "wwyyyy-MM-dd");

        parse_ok("2022-1-05-02", "yyyy-w-MM-dd");
        parse_ok("2022-1005-02", "yyyy-wMM-dd");
        parse_ok("2022-0105-02", "yyyy-wwMM-dd");

        parse_ok("2022-05-021", "yyyy-MM-ddw");
        parse_ok("2022-05-0210", "yyyy-MM-ddw");
        parse_ok("2022-05-0201", "yyyy-MM-ddww");

        parse_err("", "w");
        parse_err("", "ww");
    }

    #[test]
    fn day_of_month() {
        parse_ok("2-2022-05", "d-yyyy-MM");
        parse_ok_custom("22-2022-05", "d-yyyy-MM", "2022/05/22");
        parse_ok("02-2022-05", "dd-yyyy-MM");

        parse_ok("2022-2-05", "yyyy-d-MM");
        parse_ok_custom("2022-22-05", "yyyy-d-MM", "2022/05/22");
        parse_ok("2022-02-05", "yyyy-dd-MM");

        parse_ok("2022-05-2", "yyyy-MM-d");
        parse_ok_custom("2022-05-22", "yyyy-MM-d", "2022/05/22");
        parse_ok("2022-05-02", "yyyy-MM-dd");

        parse_err("a2", "d");
        parse_err("aa", "d");
        parse_err("aa", "dd");
    }

    #[test]
    fn day_of_year() {
        parse_ok("122-2022", "D-yyyy");
        parse_ok_custom("123-2020", "D-yyyy", "2020/05/02");
        parse_ok("122-2022", "DD-yyyy");
        parse_ok("122-2022", "DDD-yyyy");
        parse_ok_custom("1-2022", "D-yyyy", "2022/01/01");
        parse_ok_custom("01-2022", "DD-yyyy", "2022/01/01");
        parse_ok_custom("001-2022", "DDD-yyyy", "2022/01/01");
        parse_ok_custom("10-2022", "D-yyyy", "2022/01/10");
        parse_ok_custom("10-2022", "DD-yyyy", "2022/01/10");
        parse_ok_custom("010-2022", "DDD-yyyy", "2022/01/10");

        parse_ok("2022-122-1", "yyyy-D-w");
        parse_ok_custom("2020-123-1", "yyyy-D-w", "2020/05/02");
        parse_ok("2022-122-1", "yyyy-DD-w");
        parse_ok("2022-122-1", "yyyy-DDD-w");
        parse_ok_custom("2022-1-1", "yyyy-D-w", "2022/01/01");
        parse_ok_custom("2022-01-1", "yyyy-DD-w", "2022/01/01");
        parse_ok_custom("2022-001-1", "yyyy-DDD-w", "2022/01/01");
        parse_ok_custom("2022-10-1", "yyyy-D-w", "2022/01/10");
        parse_ok_custom("2022-10-1", "yyyy-DD-w", "2022/01/10");
        parse_ok_custom("2022-010-1", "yyyy-DDD-w", "2022/01/10");

        parse_ok("2022-122", "yyyy-D");
        parse_ok_custom("2020-123", "yyyy-D", "2020/05/02");
        parse_ok("2022-122", "yyyy-DD");
        parse_ok("2022-122", "yyyy-DDD");
        parse_ok_custom("2022-1", "yyyy-D", "2022/01/01");
        parse_ok_custom("2022-01", "yyyy-DD", "2022/01/01");
        parse_ok_custom("2022-001", "yyyy-DDD", "2022/01/01");
        parse_ok_custom("2022-10", "yyyy-D", "2022/01/10");
        parse_ok_custom("2022-10", "yyyy-DD", "2022/01/10");
        parse_ok_custom("2022-010", "yyyy-DDD", "2022/01/10");

        parse_err("", "DD");
        parse_err("", "DDD");
        parse_err("", "D");
    }

    #[test]
    fn wday() {
        parse_ok("12022-05-02", "eyyyy-MM-dd");
        parse_ok("012022-05-02", "eeyyyy-MM-dd");
        parse_ok("Sun2022-05-02", "eeeyyyy-MM-dd");
        parse_ok("Sunday2022-05-02", "eeeeyyyy-MM-dd");
        parse_ok("S2022-05-02", "eeeeeyyyy-MM-dd");
        parse_ok("Su2022-05-02", "eeeeeeyyyy-MM-dd");
        parse_ok("12022-05-02", "eeeeeeeyyyy-MM-dd");
        parse_ok("012022-05-02", "eeeeeeeeyyyy-MM-dd");

        parse_ok("2022-105-02", "yyyy-eMM-dd");
        parse_ok("2022-0105-02", "yyyy-eeMM-dd");
        parse_ok("2022-Sun05-02", "yyyy-eeeMM-dd");
        parse_ok("2022-Sunday05-02", "yyyy-eeeeMM-dd");
        parse_ok("2022-S05-02", "yyyy-eeeeeMM-dd");
        parse_ok("2022-Su05-02", "yyyy-eeeeeeMM-dd");
        parse_ok("2022-105-02", "yyyy-eeeeeeeMM-dd");
        parse_ok("2022-0105-02", "yyyy-eeeeeeeeMM-dd");

        parse_ok("2022-05-021", "yyyy-MM-dde");
        parse_ok("2022-05-0201", "yyyy-MM-ddee");
        parse_ok("2022-05-02Sun", "yyyy-MM-ddeee");
        parse_ok("2022-05-02Sunday", "yyyy-MM-ddeeee");
        parse_ok("2022-05-02S", "yyyy-MM-ddeeeee");
        parse_ok("2022-05-02Su", "yyyy-MM-ddeeeeee");
        parse_ok("2022-05-021", "yyyy-MM-ddeeeeeee");
        parse_ok("2022-05-0201", "yyyy-MM-ddeeeeeeee");

        parse_err("", "e");
        parse_err("", "ee");
        parse_err("blabla", "eeee");
        parse_err("", "eeeeee");
    }

    #[test]
    fn escape() {
        parse_ok("yyyMMdd2022-05-02yyyMMdd", "'yyyMMdd'yyy-MM-dd'yyyMMdd");
        parse_ok("yyyMMdd2022-05-02yyyMMdd", "'yyyMMdd'yyy-MM-dd'yyyMMdd'");
        parse_ok(
            "yyyMMdd2022-05-01yyyMMdd02",
            "'yyyMMdd'yyy-MM-dd'yyyMMdd'dd",
        );
        parse_ok("yyyMMdd'dd2022-05-02", "'yyyMMdd''dd'yyy-MM-dd");
        parse_ok(
            "yyyyMMdd2022-05-02yyyMMdd'dd",
            "'yyyyMMdd'yyyy-MM-dd'yyyMMdd''dd'",
        );
        parse_ok("''2022-05-02", "''''yyy-MM-dd");
        parse_ok("'02'2022-05", "''dd''yyy-MM");
        parse_ok("''02''2022-05", "''''dd''''yyy-MM");
        parse_ok("''dd''2022-05-02", "'''''dd'''''yyy-MM-dd");
        parse_err("", "");
        parse_ok("test2022-05-02", "te's'tyyy-MM-dd");

        // TODO: Time tests
        // assert_eq!("HHmmss123201HHmmss", time.format("'HHmmss'HHmmss'HHmmss"));
        // assert_eq!("HHmmss123201HHmmss", time.format("'HHmmss'HHmmss'HHmmss'"));
        // assert_eq!(
        //     "HHmmss123201HHmmss01",
        //     time.format("'HHmmss'HHmmss'HHmmss'ss")
        // );
        // assert_eq!("HHmmss'ss", time.format("'HHmmss''ss"));
        // assert_eq!(
        //     "HHmmss123201HHmmss'ss",
        //     time.format("'HHmmss'HHmmss'HHmmss''ss'")
        // );
        // assert_eq!("''", time.format("''''"));
        // assert_eq!("'01'", time.format("''ss''"));
        // assert_eq!("'ss'", time.format("'''ss'''"));
        // assert_eq!("''01''", time.format("''''ss''''"));
        // assert_eq!("''ss''", time.format("'''''ss'''''"));
    }

    #[test]
    fn other() {
        parse_err("2022-13-02", "yyyy-MM-dd");
        parse_err("2022-366", "yyyy-D");
        parse_err("2020-367", "yyyy-D");
        parse_err("2022-0", "yyyy-D");
        parse_err("5879611-194", "yyyy-D");
        parse_err("-5879611-173", "yyyy-D");
        parse_ok_custom("5879611-193", "yyyy-D", "5879611/07/12");
        parse_ok_custom("-5879611-174", "yyyy-D", "-5879611/06/23");
        parse_ok_custom("-4-1", "yyyy-D", "-0004/01/01");

        parse_err("0-1", "y-D");
        parse_err("-5879612-1", "y-D");
        parse_err("5879612-1", "y-D");
        parse_err("2020-0", "y-D");
        parse_err("-", "--");
    }

    fn parse_ok(string: &str, format: &str) {
        let date = Date::parse(string, format).unwrap();
        assert_eq!("2022/05/02", date.format("yyyy/MM/dd"));
    }

    fn parse_ok_custom(string: &str, format: &str, result: &str) {
        let date = Date::parse(string, format).unwrap();
        assert_eq!(result, date.format("yyyy/MM/dd"));
    }

    fn parse_err(string: &str, format: &str) {
        let date = Date::parse(string, format);
        assert!(date.is_err());
    }
}
