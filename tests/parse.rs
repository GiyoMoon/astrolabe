#[cfg(test)]
mod parse_tests {
    use astrolabe::{Date, DateTime, Time};

    #[test]
    fn era() {
        parse_ok_d("AD2022-05-02", "Gyyyy-MM-dd");
        parse_ok_d("2022-AD05-02", "yyyy-GMM-dd");
        parse_ok_d("2022-05-02AD", "yyyy-MM-ddG");

        parse_ok_d("AD2022-05-02", "GGyyyy-MM-dd");
        parse_ok_d("2022-AD05-02", "yyyy-GGMM-dd");
        parse_ok_d("2022-05-02AD", "yyyy-MM-ddGG");

        parse_ok_d("AD2022-05-02", "GGGyyyy-MM-dd");
        parse_ok_d("2022-AD05-02", "yyyy-GGGMM-dd");
        parse_ok_d("2022-05-02AD", "yyyy-MM-ddGGG");

        parse_ok_d("Anno Domini2022-05-02", "GGGGyyyy-MM-dd");
        parse_ok_d("2022-Anno Domini05-02", "yyyy-GGGGMM-dd");
        parse_ok_d("2022-05-02Anno Domini", "yyyy-MM-ddGGGG");

        parse_ok_d("A2022-05-02", "GGGGGyyyy-MM-dd");
        parse_ok_d("2022-A05-02", "yyyy-GGGGGMM-dd");
        parse_ok_d("2022-05-02A", "yyyy-MM-ddGGGGG");

        parse_ok_d("Anno Domini2022-05-02", "GGGGGGyyyy-MM-dd");
        parse_ok_d("2022-Anno Domini05-02", "yyyy-GGGGGGMM-dd");
        parse_ok_d("2022-05-02Anno Domini", "yyyy-MM-ddGGGGGG");

        parse_ok_d("Anno Domini2022-05-02", "GGGGGGGyyyy-MM-dd");
        parse_ok_d("2022-Anno Domini05-02", "yyyy-GGGGGGGMM-dd");
        parse_ok_d("2022-05-02Anno Domini", "yyyy-MM-ddGGGGGGG");

        parse_ok_d("BC2022-05-02", "Gyyyy-MM-dd");
        parse_ok_d("2022-BC05-02", "yyyy-GMM-dd");
        parse_ok_d("2022-05-02BC", "yyyy-MM-ddG");

        parse_ok_d("BC2022-05-02", "GGyyyy-MM-dd");
        parse_ok_d("2022-BC05-02", "yyyy-GGMM-dd");
        parse_ok_d("2022-05-02BC", "yyyy-MM-ddGG");

        parse_ok_d("BC2022-05-02", "GGGyyyy-MM-dd");
        parse_ok_d("2022-BC05-02", "yyyy-GGGMM-dd");
        parse_ok_d("2022-05-02BC", "yyyy-MM-ddGGG");

        parse_ok_d("Before Christ2022-05-02", "GGGGyyyy-MM-dd");
        parse_ok_d("2022-Before Christ05-02", "yyyy-GGGGMM-dd");
        parse_ok_d("2022-05-02Before Christ", "yyyy-MM-ddGGGG");

        parse_ok_d("B2022-05-02", "GGGGGyyyy-MM-dd");
        parse_ok_d("2022-B05-02", "yyyy-GGGGGMM-dd");
        parse_ok_d("2022-05-02B", "yyyy-MM-ddGGGGG");

        parse_ok_d("Before Christ2022-05-02", "GGGGGGyyyy-MM-dd");
        parse_ok_d("2022-Before Christ05-02", "yyyy-GGGGGGMM-dd");
        parse_ok_d("2022-05-02Before Christ", "yyyy-MM-ddGGGGGG");

        parse_ok_d("Before Christ2022-05-02", "GGGGGGGyyyy-MM-dd");
        parse_ok_d("2022-Before Christ05-02", "yyyy-GGGGGGGMM-dd");
        parse_ok_d("2022-05-02Before Christ", "yyyy-MM-ddGGGGGGG");

        parse_err_d("AD", "GGGG");
        parse_err_d("AD2022-05-02", "GGGGyyyy-MM-dd");
        parse_err_d("ADU2022-05-02", "Gyyyy-MM-dd");

        parse_err_d("", "G");
        parse_err_d("", "GGGGG");
    }

    #[test]
    fn year() {
        parse_ok_d("2022-05-02", "y-MM-dd");
        parse_ok_d("22-05-02", "yy-MM-dd");
        parse_ok_d("2022-05-02", "yyy-MM-dd");
        parse_ok_d("2022-05-02", "yyyy-MM-dd");
        parse_ok_d("02022-05-02", "yyyyy-MM-dd");

        parse_ok_d("05-2022-02", "MM-y-dd");
        parse_ok_d("05-22-02", "MM-yy-dd");
        parse_ok_d("05-2022-02", "MM-yyy-dd");
        parse_ok_d("05-2022-02", "MM-yyyy-dd");
        parse_ok_d("05-02022-02", "MM-yyyyy-dd");

        parse_ok_d("05-02-2022", "MM-dd-y");
        parse_ok_d("05-02-22", "MM-dd-yy");
        parse_ok_d("05-02-2022", "MM-dd-yyy");
        parse_ok_d("05-02-2022", "MM-dd-yyyy");
        parse_ok_d("05-02-02022", "MM-dd-yyyyy");

        parse_ok_custom_d("-1234-05-02", "y-MM-dd", "-1234/05/02");
        parse_ok_custom_d("-34-05-02", "yy-MM-dd", "-0034/05/02");
        parse_ok_custom_d("-1234-05-02", "yyy-MM-dd", "-1234/05/02");
        parse_ok_custom_d("-1234-05-02", "yyyy-MM-dd", "-1234/05/02");
        parse_ok_custom_d("-01234-05-02", "yyyyy-MM-dd", "-1234/05/02");

        parse_ok_custom_d("05--1234-02", "MM-y-dd", "-1234/05/02");
        parse_ok_custom_d("05--34-02", "MM-yy-dd", "-0034/05/02");
        parse_ok_custom_d("05--1234-02", "MM-yyy-dd", "-1234/05/02");
        parse_ok_custom_d("05--1234-02", "MM-yyyy-dd", "-1234/05/02");
        parse_ok_custom_d("05--01234-02", "MM-yyyyy-dd", "-1234/05/02");

        parse_ok_custom_d("05-02--1234", "MM-dd-y", "-1234/05/02");
        parse_ok_custom_d("05-02--34", "MM-dd-yy", "-0034/05/02");
        parse_ok_custom_d("05-02--1234", "MM-dd-yyy", "-1234/05/02");
        parse_ok_custom_d("05-02--1234", "MM-dd-yyyy", "-1234/05/02");
        parse_ok_custom_d("05-02--01234", "MM-dd-yyyyy", "-1234/05/02");

        parse_err_d("-1", "yy");
        parse_err_d("-aa", "yy");

        parse_err_d("1", "yy");
        parse_err_d("aa", "yy");

        parse_err_d("-", "y");

        parse_err_d("-", "yyyyy");
        parse_err_d("", "yyyyy");
    }

    #[test]
    fn quarter() {
        parse_ok_d("12022-05-02", "qyyyy-MM-dd");
        parse_ok_d("012022-05-02", "qqyyyy-MM-dd");
        parse_ok_d("Q12022-05-02", "qqqyyyy-MM-dd");
        parse_ok_d("1st quarter2022-05-02", "qqqqyyyy-MM-dd");
        parse_ok_d("12022-05-02", "qqqqqyyyy-MM-dd");
        parse_ok_d("12022-05-02", "qqqqqqyyyy-MM-dd");

        parse_ok_d("2022-105-02", "yyyy-qMM-dd");
        parse_ok_d("2022-0105-02", "yyyy-qqMM-dd");
        parse_ok_d("2022-Q105-02", "yyyy-qqqMM-dd");
        parse_ok_d("2022-1st quarter05-02", "yyyy-qqqqMM-dd");
        parse_ok_d("2022-105-02", "yyyy-qqqqqMM-dd");
        parse_ok_d("2022-105-02", "yyyy-qqqqqqMM-dd");

        parse_ok_d("2022-05-021", "yyyy-MM-ddq");
        parse_ok_d("2022-05-0201", "yyyy-MM-ddqq");
        parse_ok_d("2022-05-02Q1", "yyyy-MM-ddqqq");
        parse_ok_d("2022-05-021st quarter", "yyyy-MM-ddqqqq");
        parse_ok_d("2022-05-021", "yyyy-MM-ddqqqqq");
        parse_ok_d("2022-05-021", "yyyy-MM-ddqqqqqq");

        parse_ok_d("22022-05-02", "qyyyy-MM-dd");
        parse_ok_d("022022-05-02", "qqyyyy-MM-dd");
        parse_ok_d("Q22022-05-02", "qqqyyyy-MM-dd");
        parse_ok_d("2nd quarter2022-05-02", "qqqqyyyy-MM-dd");
        parse_ok_d("22022-05-02", "qqqqqyyyy-MM-dd");
        parse_ok_d("22022-05-02", "qqqqqqyyyy-MM-dd");

        parse_ok_d("2022-205-02", "yyyy-qMM-dd");
        parse_ok_d("2022-0205-02", "yyyy-qqMM-dd");
        parse_ok_d("2022-Q205-02", "yyyy-qqqMM-dd");
        parse_ok_d("2022-2nd quarter05-02", "yyyy-qqqqMM-dd");
        parse_ok_d("2022-205-02", "yyyy-qqqqqMM-dd");
        parse_ok_d("2022-205-02", "yyyy-qqqqqqMM-dd");

        parse_ok_d("2022-05-022", "yyyy-MM-ddq");
        parse_ok_d("2022-05-0202", "yyyy-MM-ddqq");
        parse_ok_d("2022-05-02Q2", "yyyy-MM-ddqqq");
        parse_ok_d("2022-05-022nd quarter", "yyyy-MM-ddqqqq");
        parse_ok_d("2022-05-022", "yyyy-MM-ddqqqqq");
        parse_ok_d("2022-05-022", "yyyy-MM-ddqqqqqq");

        parse_ok_d("32022-05-02", "qyyyy-MM-dd");
        parse_ok_d("032022-05-02", "qqyyyy-MM-dd");
        parse_ok_d("Q32022-05-02", "qqqyyyy-MM-dd");
        parse_ok_d("3rd quarter2022-05-02", "qqqqyyyy-MM-dd");
        parse_ok_d("32022-05-02", "qqqqqyyyy-MM-dd");
        parse_ok_d("32022-05-02", "qqqqqqyyyy-MM-dd");

        parse_ok_d("2022-305-02", "yyyy-qMM-dd");
        parse_ok_d("2022-0305-02", "yyyy-qqMM-dd");
        parse_ok_d("2022-Q305-02", "yyyy-qqqMM-dd");
        parse_ok_d("2022-3rd quarter05-02", "yyyy-qqqqMM-dd");
        parse_ok_d("2022-305-02", "yyyy-qqqqqMM-dd");
        parse_ok_d("2022-305-02", "yyyy-qqqqqqMM-dd");

        parse_ok_d("2022-05-023", "yyyy-MM-ddq");
        parse_ok_d("2022-05-0203", "yyyy-MM-ddqq");
        parse_ok_d("2022-05-02Q3", "yyyy-MM-ddqqq");
        parse_ok_d("2022-05-023rd quarter", "yyyy-MM-ddqqqq");
        parse_ok_d("2022-05-023", "yyyy-MM-ddqqqqq");
        parse_ok_d("2022-05-023", "yyyy-MM-ddqqqqqq");

        parse_ok_d("42022-05-02", "qyyyy-MM-dd");
        parse_ok_d("042022-05-02", "qqyyyy-MM-dd");
        parse_ok_d("Q42022-05-02", "qqqyyyy-MM-dd");
        parse_ok_d("4th quarter2022-05-02", "qqqqyyyy-MM-dd");
        parse_ok_d("42022-05-02", "qqqqqyyyy-MM-dd");
        parse_ok_d("42022-05-02", "qqqqqqyyyy-MM-dd");

        parse_ok_d("2022-405-02", "yyyy-qMM-dd");
        parse_ok_d("2022-0405-02", "yyyy-qqMM-dd");
        parse_ok_d("2022-Q405-02", "yyyy-qqqMM-dd");
        parse_ok_d("2022-4th quarter05-02", "yyyy-qqqqMM-dd");
        parse_ok_d("2022-405-02", "yyyy-qqqqqMM-dd");
        parse_ok_d("2022-405-02", "yyyy-qqqqqqMM-dd");

        parse_ok_d("2022-05-024", "yyyy-MM-ddq");
        parse_ok_d("2022-05-0204", "yyyy-MM-ddqq");
        parse_ok_d("2022-05-02Q4", "yyyy-MM-ddqqq");
        parse_ok_d("2022-05-024th quarter", "yyyy-MM-ddqqqq");
        parse_ok_d("2022-05-024", "yyyy-MM-ddqqqqq");
        parse_ok_d("2022-05-024", "yyyy-MM-ddqqqqqq");

        parse_err_d("", "q");
        parse_err_d("", "qqq");
        parse_err_d("", "qqqq");
        parse_err_d("", "qqqqq");
    }

    #[test]
    fn month() {
        parse_ok_d("5-2022-02", "M-yyyy-dd");
        parse_ok_d("05-2022-02", "MM-yyyy-dd");
        parse_ok_d("May-2022-02", "MMM-yyyy-dd");
        parse_ok_d("May-2022-02", "MMMM-yyyy-dd");
        parse_ok_d("M-05-2022-02", "MMMMM-MM-yyyy-dd");
        parse_ok_d("May-2022-02", "MMMMMM-yyyy-dd");

        parse_ok_d("2022-5-02", "yyyy-M-dd");
        parse_ok_d("2022-05-02", "yyyy-MM-dd");
        parse_ok_d("2022-May-02", "yyyy-MMM-dd");
        parse_ok_d("2022-May-02", "yyyy-MMMM-dd");
        parse_ok_d("2022-M-05-02", "yyyy-MMMMM-MM-dd");
        parse_ok_d("2022-May-02", "yyyy-MMMMMM-dd");

        parse_ok_d("2022-02-5", "yyyy-dd-M");
        parse_ok_d("2022-02-05", "yyyy-dd-MM");
        parse_ok_d("2022-02-May", "yyyy-dd-MMM");
        parse_ok_d("2022-02-May", "yyyy-dd-MMMM");
        parse_ok_d("2022-02-05-M", "yyyy-dd-MM-MMMMM");
        parse_ok_d("2022-02-May", "yyyy-dd-MMMMMM");

        parse_ok_custom_d("1-2022-02", "M-yyyy-dd", "2022/01/02");

        parse_err_d("", "M");
        parse_err_d("blabla", "MMM");
        parse_err_d("", "MMMMM");
        parse_err_d("blabla", "MMMMMM");
    }

    #[test]
    fn week() {
        parse_ok_d("1-2022-05-02", "w-yyyy-MM-dd");
        parse_ok_d("102022-05-02", "wyyyy-MM-dd");
        parse_ok_d("012022-05-02", "wwyyyy-MM-dd");

        parse_ok_d("2022-1-05-02", "yyyy-w-MM-dd");
        parse_ok_d("2022-1005-02", "yyyy-wMM-dd");
        parse_ok_d("2022-0105-02", "yyyy-wwMM-dd");

        parse_ok_d("2022-05-021", "yyyy-MM-ddw");
        parse_ok_d("2022-05-0210", "yyyy-MM-ddw");
        parse_ok_d("2022-05-0201", "yyyy-MM-ddww");

        parse_err_d("", "w");
        parse_err_d("", "ww");
    }

    #[test]
    fn day_of_month() {
        parse_ok_d("2-2022-05", "d-yyyy-MM");
        parse_ok_custom_d("22-2022-05", "d-yyyy-MM", "2022/05/22");
        parse_ok_d("02-2022-05", "dd-yyyy-MM");

        parse_ok_d("2022-2-05", "yyyy-d-MM");
        parse_ok_custom_d("2022-22-05", "yyyy-d-MM", "2022/05/22");
        parse_ok_d("2022-02-05", "yyyy-dd-MM");

        parse_ok_d("2022-05-2", "yyyy-MM-d");
        parse_ok_custom_d("2022-05-22", "yyyy-MM-d", "2022/05/22");
        parse_ok_d("2022-05-02", "yyyy-MM-dd");

        parse_err_d("a2", "d");
        parse_err_d("aa", "d");
        parse_err_d("aa", "dd");
    }

    #[test]
    fn day_of_year() {
        parse_ok_d("122-2022", "D-yyyy");
        parse_ok_custom_d("123-2020", "D-yyyy", "2020/05/02");
        parse_ok_d("122-2022", "DD-yyyy");
        parse_ok_d("122-2022", "DDD-yyyy");
        parse_ok_custom_d("1-2022", "D-yyyy", "2022/01/01");
        parse_ok_custom_d("01-2022", "DD-yyyy", "2022/01/01");
        parse_ok_custom_d("001-2022", "DDD-yyyy", "2022/01/01");
        parse_ok_custom_d("10-2022", "D-yyyy", "2022/01/10");
        parse_ok_custom_d("10-2022", "DD-yyyy", "2022/01/10");
        parse_ok_custom_d("010-2022", "DDD-yyyy", "2022/01/10");

        parse_ok_d("2022-122-1", "yyyy-D-w");
        parse_ok_custom_d("2020-123-1", "yyyy-D-w", "2020/05/02");
        parse_ok_d("2022-122-1", "yyyy-DD-w");
        parse_ok_d("2022-122-1", "yyyy-DDD-w");
        parse_ok_custom_d("2022-1-1", "yyyy-D-w", "2022/01/01");
        parse_ok_custom_d("2022-01-1", "yyyy-DD-w", "2022/01/01");
        parse_ok_custom_d("2022-001-1", "yyyy-DDD-w", "2022/01/01");
        parse_ok_custom_d("2022-10-1", "yyyy-D-w", "2022/01/10");
        parse_ok_custom_d("2022-10-1", "yyyy-DD-w", "2022/01/10");
        parse_ok_custom_d("2022-010-1", "yyyy-DDD-w", "2022/01/10");

        parse_ok_d("2022-122", "yyyy-D");
        parse_ok_custom_d("2020-123", "yyyy-D", "2020/05/02");
        parse_ok_d("2022-122", "yyyy-DD");
        parse_ok_d("2022-122", "yyyy-DDD");
        parse_ok_custom_d("2022-1", "yyyy-D", "2022/01/01");
        parse_ok_custom_d("2022-01", "yyyy-DD", "2022/01/01");
        parse_ok_custom_d("2022-001", "yyyy-DDD", "2022/01/01");
        parse_ok_custom_d("2022-10", "yyyy-D", "2022/01/10");
        parse_ok_custom_d("2022-10", "yyyy-DD", "2022/01/10");
        parse_ok_custom_d("2022-010", "yyyy-DDD", "2022/01/10");

        parse_err_d("", "DD");
        parse_err_d("", "DDD");
        parse_err_d("", "D");

        let date_time = DateTime::parse("122-2022", "D-yyyy").unwrap();
        assert_eq!("2022/05/02", date_time.format("yyyy/MM/dd"));
    }

    #[test]
    fn wday() {
        parse_ok_d("12022-05-02", "eyyyy-MM-dd");
        parse_ok_d("012022-05-02", "eeyyyy-MM-dd");
        parse_ok_d("Sun2022-05-02", "eeeyyyy-MM-dd");
        parse_ok_d("Sunday2022-05-02", "eeeeyyyy-MM-dd");
        parse_ok_d("S2022-05-02", "eeeeeyyyy-MM-dd");
        parse_ok_d("Su2022-05-02", "eeeeeeyyyy-MM-dd");
        parse_ok_d("12022-05-02", "eeeeeeeyyyy-MM-dd");
        parse_ok_d("012022-05-02", "eeeeeeeeyyyy-MM-dd");

        parse_ok_d("2022-105-02", "yyyy-eMM-dd");
        parse_ok_d("2022-0105-02", "yyyy-eeMM-dd");
        parse_ok_d("2022-Sun05-02", "yyyy-eeeMM-dd");
        parse_ok_d("2022-Sunday05-02", "yyyy-eeeeMM-dd");
        parse_ok_d("2022-S05-02", "yyyy-eeeeeMM-dd");
        parse_ok_d("2022-Su05-02", "yyyy-eeeeeeMM-dd");
        parse_ok_d("2022-105-02", "yyyy-eeeeeeeMM-dd");
        parse_ok_d("2022-0105-02", "yyyy-eeeeeeeeMM-dd");

        parse_ok_d("2022-05-021", "yyyy-MM-dde");
        parse_ok_d("2022-05-0201", "yyyy-MM-ddee");
        parse_ok_d("2022-05-02Sun", "yyyy-MM-ddeee");
        parse_ok_d("2022-05-02Sunday", "yyyy-MM-ddeeee");
        parse_ok_d("2022-05-02S", "yyyy-MM-ddeeeee");
        parse_ok_d("2022-05-02Su", "yyyy-MM-ddeeeeee");
        parse_ok_d("2022-05-021", "yyyy-MM-ddeeeeeee");
        parse_ok_d("2022-05-0201", "yyyy-MM-ddeeeeeeee");

        parse_err_d("", "e");
        parse_err_d("", "ee");
        parse_err_d("blabla", "eeee");
        parse_err_d("", "eeeeee");
    }

    #[test]
    fn period_a() {
        parse_ok_t("PM123201", "ahhmmss");
        parse_ok_t("PM123201", "aahhmmss");
        parse_ok_t("pm123201", "aaahhmmss");
        parse_ok_t("p.m.123201", "aaaahhmmss");
        parse_ok_t("p123201", "aaaaahhmmss");
        parse_ok_t("pm123201", "aaaaaahhmmss");
        parse_ok_custom_t("PM013201", "ahhmmss", "13:32:01", "HH:mm:ss");
        parse_ok_custom_t("PM013201", "aahhmmss", "13:32:01", "HH:mm:ss");
        parse_ok_custom_t("pm013201", "aaahhmmss", "13:32:01", "HH:mm:ss");
        parse_ok_custom_t("p.m.013201", "aaaahhmmss", "13:32:01", "HH:mm:ss");
        parse_ok_custom_t("p013201", "aaaaahhmmss", "13:32:01", "HH:mm:ss");
        parse_ok_custom_t("pm013201", "aaaaaahhmmss", "13:32:01", "HH:mm:ss");
        parse_ok_custom_t("AM123201", "ahhmmss", "00:32:01", "HH:mm:ss");
        parse_ok_custom_t("AM123201", "aahhmmss", "00:32:01", "HH:mm:ss");
        parse_ok_custom_t("am123201", "aaahhmmss", "00:32:01", "HH:mm:ss");
        parse_ok_custom_t("a.m.123201", "aaaahhmmss", "00:32:01", "HH:mm:ss");
        parse_ok_custom_t("a123201", "aaaaahhmmss", "00:32:01", "HH:mm:ss");
        parse_ok_custom_t("am123201", "aaaaaahhmmss", "00:32:01", "HH:mm:ss");
        parse_ok_custom_t("AM013201", "ahhmmss", "01:32:01", "HH:mm:ss");
        parse_ok_custom_t("AM013201", "aahhmmss", "01:32:01", "HH:mm:ss");
        parse_ok_custom_t("am013201", "aaahhmmss", "01:32:01", "HH:mm:ss");
        parse_ok_custom_t("a.m.013201", "aaaahhmmss", "01:32:01", "HH:mm:ss");
        parse_ok_custom_t("a013201", "aaaaahhmmss", "01:32:01", "HH:mm:ss");
        parse_ok_custom_t("am013201", "aaaaaahhmmss", "01:32:01", "HH:mm:ss");

        parse_ok_t("12PM3201", "hhammss");
        parse_ok_t("12PM3201", "hhaammss");
        parse_ok_t("12pm3201", "hhaaammss");
        parse_ok_t("12p.m.3201", "hhaaaammss");
        parse_ok_t("12p3201", "hhaaaaammss");
        parse_ok_t("12pm3201", "hhaaaaaammss");
        parse_ok_custom_t("12AM3201", "hhammss", "00:32:01", "HH:mm:ss");
        parse_ok_custom_t("12AM3201", "hhaammss", "00:32:01", "HH:mm:ss");
        parse_ok_custom_t("12am3201", "hhaaammss", "00:32:01", "HH:mm:ss");
        parse_ok_custom_t("12a.m.3201", "hhaaaammss", "00:32:01", "HH:mm:ss");
        parse_ok_custom_t("12a3201", "hhaaaaammss", "00:32:01", "HH:mm:ss");
        parse_ok_custom_t("12am3201", "hhaaaaaammss", "00:32:01", "HH:mm:ss");

        parse_ok_t("123201PM", "hhmmssa");
        parse_ok_t("123201PM", "hhmmssaa");
        parse_ok_t("123201pm", "hhmmssaaa");
        parse_ok_t("123201p.m.", "hhmmssaaaa");
        parse_ok_t("123201p", "hhmmssaaaaa");
        parse_ok_t("123201pm", "hhmmssaaaaaa");
        parse_ok_custom_t("123201AM", "hhmmssa", "00:32:01", "HH:mm:ss");
        parse_ok_custom_t("123201AM", "hhmmssaa", "00:32:01", "HH:mm:ss");
        parse_ok_custom_t("123201am", "hhmmssaaa", "00:32:01", "HH:mm:ss");
        parse_ok_custom_t("123201a.m.", "hhmmssaaaa", "00:32:01", "HH:mm:ss");
        parse_ok_custom_t("123201a", "hhmmssaaaaa", "00:32:01", "HH:mm:ss");
        parse_ok_custom_t("123201am", "hhmmssaaaaaa", "00:32:01", "HH:mm:ss");

        parse_err_t("blab1", "aaaah");
        parse_err_t("123", "aaaa");
        parse_err_t("", "aaaaa");
        parse_err_t("m", "aaaaa");
        parse_err_t("a", "a");
        parse_err_t("ab", "a");

        let date_time = DateTime::parse("2022-05-02 12pm", "yyyy-MM-dd hhaaa").unwrap();
        assert_eq!("2022/05/02 12", date_time.format("yyyy/MM/dd HH"));

        let date_time = DateTime::parse("2022-05-02 12am", "yyyy-MM-dd hhaaa").unwrap();
        assert_eq!("2022/05/02 00", date_time.format("yyyy/MM/dd HH"));
    }

    #[test]
    fn period_b() {
        parse_ok_t("noon123201", "bhhmmss");
        parse_ok_t("noon123201", "bbhhmmss");
        parse_ok_t("noon123201", "bbbhhmmss");
        parse_ok_t("noon123201", "bbbbhhmmss");
        parse_ok_t("n123201", "bbbbbhhmmss");
        parse_ok_t("noon123201", "bbbbbbhhmmss");
        parse_ok_custom_t("PM013201", "bhhmmss", "13:32:01", "HH:mm:ss");
        parse_ok_custom_t("PM013201", "bbhhmmss", "13:32:01", "HH:mm:ss");
        parse_ok_custom_t("pm013201", "bbbhhmmss", "13:32:01", "HH:mm:ss");
        parse_ok_custom_t("p.m.013201", "bbbbhhmmss", "13:32:01", "HH:mm:ss");
        parse_ok_custom_t("p013201", "bbbbbhhmmss", "13:32:01", "HH:mm:ss");
        parse_ok_custom_t("pm013201", "bbbbbbhhmmss", "13:32:01", "HH:mm:ss");
        parse_ok_custom_t("midnight123201", "bhhmmss", "00:32:01", "HH:mm:ss");
        parse_ok_custom_t("midnight123201", "bbhhmmss", "00:32:01", "HH:mm:ss");
        parse_ok_custom_t("midnight123201", "bbbhhmmss", "00:32:01", "HH:mm:ss");
        parse_ok_custom_t("midnight123201", "bbbbhhmmss", "00:32:01", "HH:mm:ss");
        parse_ok_custom_t("mi123201", "bbbbbhhmmss", "00:32:01", "HH:mm:ss");
        parse_ok_custom_t("midnight123201", "bbbbbbhhmmss", "00:32:01", "HH:mm:ss");
        parse_ok_custom_t("AM013201", "bhhmmss", "01:32:01", "HH:mm:ss");
        parse_ok_custom_t("AM013201", "bbhhmmss", "01:32:01", "HH:mm:ss");
        parse_ok_custom_t("am013201", "bbbhhmmss", "01:32:01", "HH:mm:ss");
        parse_ok_custom_t("a.m.013201", "bbbbhhmmss", "01:32:01", "HH:mm:ss");
        parse_ok_custom_t("a013201", "bbbbbhhmmss", "01:32:01", "HH:mm:ss");
        parse_ok_custom_t("am013201", "bbbbbbhhmmss", "01:32:01", "HH:mm:ss");

        parse_ok_t("12noon3201", "hhbmmss");
        parse_ok_t("12noon3201", "hhbmmss");
        parse_ok_t("12noon3201", "hhbbbmmss");
        parse_ok_t("12noon3201", "hhbbbbmmss");
        parse_ok_t("12n3201", "hhbbbbbmmss");
        parse_ok_t("12noon3201", "hhbbbbbbmmss");
        parse_ok_custom_t("12midnight3201", "hhbmmss", "00:32:01", "HH:mm:ss");
        parse_ok_custom_t("12midnight3201", "hhbbmmss", "00:32:01", "HH:mm:ss");
        parse_ok_custom_t("12midnight3201", "hhbbbmmss", "00:32:01", "HH:mm:ss");
        parse_ok_custom_t("12midnight3201", "hhbbbbmmss", "00:32:01", "HH:mm:ss");
        parse_ok_custom_t("12mi3201", "hhbbbbbmmss", "00:32:01", "HH:mm:ss");
        parse_ok_custom_t("12midnight3201", "hhbbbbbbmmss", "00:32:01", "HH:mm:ss");

        parse_ok_t("123201noon", "hhmmssb");
        parse_ok_t("123201noon", "hhmmssbb");
        parse_ok_t("123201noon", "hhmmssbbb");
        parse_ok_t("123201noon", "hhmmssbbbb");
        parse_ok_t("123201n", "hhmmssbbbbb");
        parse_ok_t("123201noon", "hhmmssbbbbbb");
        parse_ok_custom_t("123201midnight", "hhmmssb", "00:32:01", "HH:mm:ss");
        parse_ok_custom_t("123201midnight", "hhmmssbb", "00:32:01", "HH:mm:ss");
        parse_ok_custom_t("123201midnight", "hhmmssbbb", "00:32:01", "HH:mm:ss");
        parse_ok_custom_t("123201midnight", "hhmmssbbbb", "00:32:01", "HH:mm:ss");
        parse_ok_custom_t("123201mi", "hhmmssbbbbb", "00:32:01", "HH:mm:ss");
        parse_ok_custom_t("123201midnight", "hhmmssbbbbbb", "00:32:01", "HH:mm:ss");

        parse_err_t("blab1", "bbbbh");
        parse_err_t("123", "bbbb");
        parse_err_t("", "bbbbb");
        parse_err_t("m", "bbbbb");
        parse_err_t("a", "b");
        parse_err_t("ab", "b");
    }

    #[test]
    fn hour_h_lower() {
        parse_ok_custom_t("11-3201-PM", "h-mmss-a", "23:32:01", "HH:mm:ss");

        parse_ok_custom_t("4-3201-PM", "h-mmss-a", "16:32:01", "HH:mm:ss");
        parse_ok_t("123201-PM", "hmmss-a");
        parse_ok_t("123201-PM", "hhmmss-a");

        parse_ok_custom_t("32-4-01-PM", "mm-h-ss-a", "16:32:01", "HH:mm:ss");
        parse_ok_t("321201-PM", "mmhss-a");
        parse_ok_t("321201-PM", "mmhhss-a");

        parse_ok_custom_t("32-01-PM-4", "mm-ss-a-h", "16:32:01", "HH:mm:ss");
        parse_ok_t("3201-PM12", "mmss-ah");
        parse_ok_t("3201-PM12", "mmss-ahh");

        parse_ok_t("123201-PM", "hhhmmss-a");

        parse_err_t("a", "h");
        parse_err_t("a1", "h");
        parse_err_t("aa", "hh");
    }

    #[test]
    fn hour_h_upper() {
        parse_ok_custom_t("4-3201", "H-mmss", "04:32:01", "HH:mm:ss");
        parse_ok_t("123201", "Hmmss");
        parse_ok_t("123201", "HHmmss");

        parse_ok_custom_t("32-4-01", "mm-H-ss", "04:32:01", "HH:mm:ss");
        parse_ok_t("321201", "mmHss");
        parse_ok_t("321201", "mmHHss");

        parse_ok_custom_t("32-01-4", "mm-ss-H", "04:32:01", "HH:mm:ss");
        parse_ok_t("320112", "mmssH");
        parse_ok_t("320112", "mmssHH");

        parse_ok_t("123201", "HHHmmss");

        parse_err_t("a", "H");
        parse_err_t("a1", "H");
        parse_err_t("aa", "HH");
    }

    #[test]
    fn hour_k_upper() {
        parse_ok_t("0-32-01-PM", "K-mm-ss-a");
        parse_ok_t("003201-PM", "Kmmss-a");
        parse_ok_t("003201-PM", "KKmmss-a");

        parse_ok_t("32-0-01-PM", "mm-K-ss-a");
        parse_ok_t("320001-PM", "mmKss-a");
        parse_ok_t("320001-PM", "mmKKss-a");

        parse_ok_t("3201-PM0", "mmss-aK");
        parse_ok_t("3201-PM00", "mmss-aK");
        parse_ok_t("3201-PM00", "mmss-aKK");

        parse_ok_t("003201-PM", "KKKmmss-a");

        parse_err_t("a", "K");
        parse_err_t("a1", "K");
        parse_err_t("aa", "KK");
    }

    #[test]
    fn hour_k_lower() {
        parse_ok_custom_t("4-3201", "k-mmss", "04:32:01", "HH:mm:ss");
        parse_ok_t("123201", "kmmss");
        parse_ok_t("123201", "kkmmss");

        parse_ok_custom_t("32-4-01", "mm-k-ss", "04:32:01", "HH:mm:ss");
        parse_ok_t("321201", "mmkss");
        parse_ok_t("321201", "mmkkss");

        parse_ok_custom_t("32-01-4", "mm-ss-k", "04:32:01", "HH:mm:ss");
        parse_ok_t("320112", "mmssk");
        parse_ok_t("320112", "mmsskk");

        parse_ok_t("123201", "kkkmmss");

        parse_ok_custom_t("24-3201", "k-mmss", "00:32:01", "HH:mm:ss");
        parse_ok_custom_t("24-3201", "kk-mmss", "00:32:01", "HH:mm:ss");

        parse_err_t("a", "k");
        parse_err_t("a1", "k");
        parse_err_t("aa", "kk");
    }

    #[test]
    fn minute() {
        parse_ok_custom_t("5-1201", "m-HHss", "12:05:01", "HH:mm:ss");
        parse_ok_t("321201", "mHHss");
        parse_ok_t("321201", "mmHHss");

        parse_ok_custom_t("125-01", "HHm-ss", "12:05:01", "HH:mm:ss");
        parse_ok_t("123201", "HHmss");
        parse_ok_t("123201", "HHmmss");

        parse_ok_custom_t("12015", "HHssm", "12:05:01", "HH:mm:ss");
        parse_ok_t("120132", "HHssm");
        parse_ok_t("120132", "HHssmm");

        parse_err_t("a", "m");
        parse_err_t("a1", "m");
        parse_err_t("aa", "mm");
    }

    #[test]
    fn second() {
        parse_ok_t("1-1232", "s-HHmm");
        parse_ok_t("011232", "sHHmm");
        parse_ok_t("011232", "ssHHmm");

        parse_ok_t("121-32", "HHs-mm");
        parse_ok_t("120132", "HHsmm");
        parse_ok_t("120132", "HHsmm");

        parse_ok_t("12321", "HHmms");
        parse_ok_t("123201", "HHmms");
        parse_ok_t("123201", "HHmmss");

        parse_err_t("a", "s");
        parse_err_t("a1", "s");
        parse_err_t("aa", "ss");
    }

    #[test]
    fn subsecond() {
        let time = Time::parse("2123201", "nHHmmss").unwrap();
        assert_eq!("12:32:01 200000000", time.format("HH:mm:ss nnnnn"));
        let time = Time::parse("22123201", "nnHHmmss").unwrap();
        assert_eq!("12:32:01 220000000", time.format("HH:mm:ss nnnnn"));
        let time = Time::parse("222123201", "nnnHHmmss").unwrap();
        assert_eq!("12:32:01 222000000", time.format("HH:mm:ss nnnnn"));
        let time = Time::parse("222222123201", "nnnnHHmmss").unwrap();
        assert_eq!("12:32:01 222222000", time.format("HH:mm:ss nnnnn"));
        let time = Time::parse("222222222123201", "nnnnnHHmmss").unwrap();
        assert_eq!("12:32:01 222222222", time.format("HH:mm:ss nnnnn"));

        parse_err_t("a", "n");
        parse_err_t("a", "nn");
        parse_err_t("a", "nnn");
        parse_err_t("a", "nnnn");
        parse_err_t("a", "nnnnn");

        let date_time = DateTime::parse("22022-05-02", "nyyyy-MM-dd").unwrap();
        assert_eq!("2022/05/02 200000000", date_time.format("yyyy/MM/dd nnnnn"));
        let date_time = DateTime::parse("222022-05-02", "nnyyyy-MM-dd").unwrap();
        assert_eq!("2022/05/02 220000000", date_time.format("yyyy/MM/dd nnnnn"));
        let date_time = DateTime::parse("2222022-05-02", "nnnyyyy-MM-dd").unwrap();
        assert_eq!("2022/05/02 222000000", date_time.format("yyyy/MM/dd nnnnn"));
        let date_time = DateTime::parse("2222222022-05-02", "nnnnyyyy-MM-dd").unwrap();
        assert_eq!("2022/05/02 222222000", date_time.format("yyyy/MM/dd nnnnn"));
        let date_time = DateTime::parse("2222222222022-05-02", "nnnnnyyyy-MM-dd").unwrap();
        assert_eq!("2022/05/02 222222222", date_time.format("yyyy/MM/dd nnnnn"));
    }

    #[test]
    fn offset() {
        let time = Time::parse("123201Z", "HHmmssX").unwrap();
        assert_eq!("12:32:01 +00:00", time.format("HH:mm:ss xxxxx"));
        let time = Time::parse("123201+01", "HHmmssX").unwrap();
        assert_eq!("13:32:01 +01:00", time.format("HH:mm:ss xxxxx"));
        let time = Time::parse("123201+0101", "HHmmssX").unwrap();
        assert_eq!("13:33:01 +01:01", time.format("HH:mm:ss xxxxx"));
        let time = Time::parse("123201-01", "HHmmssX").unwrap();
        assert_eq!("11:32:01 -01:00", time.format("HH:mm:ss xxxxx"));
        let time = Time::parse("123201-0101", "HHmmssX").unwrap();
        assert_eq!("11:31:01 -01:01", time.format("HH:mm:ss xxxxx"));

        let time = Time::parse("123201Z", "HHmmssXX").unwrap();
        assert_eq!("12:32:01 +00:00", time.format("HH:mm:ss xxxxx"));
        let time = Time::parse("123201+0100", "HHmmssXX").unwrap();
        assert_eq!("13:32:01 +01:00", time.format("HH:mm:ss xxxxx"));
        let time = Time::parse("123201+0101", "HHmmssXX").unwrap();
        assert_eq!("13:33:01 +01:01", time.format("HH:mm:ss xxxxx"));
        let time = Time::parse("123201-0100", "HHmmssXX").unwrap();
        assert_eq!("11:32:01 -01:00", time.format("HH:mm:ss xxxxx"));
        let time = Time::parse("123201-0101", "HHmmssXX").unwrap();
        assert_eq!("11:31:01 -01:01", time.format("HH:mm:ss xxxxx"));

        let time = Time::parse("123201Z", "HHmmssXXX").unwrap();
        assert_eq!("12:32:01 +00:00", time.format("HH:mm:ss xxxxx"));
        let time = Time::parse("123201+01:00", "HHmmssXXX").unwrap();
        assert_eq!("13:32:01 +01:00", time.format("HH:mm:ss xxxxx"));
        let time = Time::parse("123201+01:01", "HHmmssXXX").unwrap();
        assert_eq!("13:33:01 +01:01", time.format("HH:mm:ss xxxxx"));
        let time = Time::parse("123201-01:00", "HHmmssXXX").unwrap();
        assert_eq!("11:32:01 -01:00", time.format("HH:mm:ss xxxxx"));
        let time = Time::parse("123201-01:01", "HHmmssXXX").unwrap();
        assert_eq!("11:31:01 -01:01", time.format("HH:mm:ss xxxxx"));

        let time = Time::parse("123201Z", "HHmmssXXXX").unwrap();
        assert_eq!("12:32:01 +00:00", time.format("HH:mm:ss xxxxx"));
        let time = Time::parse("123201+0100", "HHmmssXXXX").unwrap();
        assert_eq!("13:32:01 +01:00", time.format("HH:mm:ss xxxxx"));
        let time = Time::parse("123201+0101", "HHmmssXXXX").unwrap();
        assert_eq!("13:33:01 +01:01", time.format("HH:mm:ss xxxxx"));
        let time = Time::parse("123201+010101", "HHmmssXXXX").unwrap();
        assert_eq!("13:33:02 +01:01:01", time.format("HH:mm:ss xxxxx"));
        let time = Time::parse("123201-0100", "HHmmssXXXX").unwrap();
        assert_eq!("11:32:01 -01:00", time.format("HH:mm:ss xxxxx"));
        let time = Time::parse("123201-0101", "HHmmssXXXX").unwrap();
        assert_eq!("11:31:01 -01:01", time.format("HH:mm:ss xxxxx"));
        let time = Time::parse("123201-010101", "HHmmssXXXX").unwrap();
        assert_eq!("11:31:00 -01:01:01", time.format("HH:mm:ss xxxxx"));

        let time = Time::parse("123201Z", "HHmmssXXXXX").unwrap();
        assert_eq!("12:32:01 +00:00", time.format("HH:mm:ss xxxxx"));
        let time = Time::parse("123201+01:00", "HHmmssXXXXX").unwrap();
        assert_eq!("13:32:01 +01:00", time.format("HH:mm:ss xxxxx"));
        let time = Time::parse("123201+01:01", "HHmmssXXXXX").unwrap();
        assert_eq!("13:33:01 +01:01", time.format("HH:mm:ss xxxxx"));
        let time = Time::parse("123201+01:01:01", "HHmmssXXXXX").unwrap();
        assert_eq!("13:33:02 +01:01:01", time.format("HH:mm:ss xxxxx"));
        let time = Time::parse("123201-01:00", "HHmmssXXXXX").unwrap();
        assert_eq!("11:32:01 -01:00", time.format("HH:mm:ss xxxxx"));
        let time = Time::parse("123201-01:01", "HHmmssXXXXX").unwrap();
        assert_eq!("11:31:01 -01:01", time.format("HH:mm:ss xxxxx"));
        let time = Time::parse("123201-01:01:01", "HHmmssXXXXX").unwrap();
        assert_eq!("11:31:00 -01:01:01", time.format("HH:mm:ss xxxxx"));

        let time = Time::parse("123201+00:00", "HHmmssxxx").unwrap();
        assert_eq!("12:32:01 +00:00", time.format("HH:mm:ss xxxxx"));
        let time = Time::parse("123201+01:00", "HHmmssxxx").unwrap();
        assert_eq!("13:32:01 +01:00", time.format("HH:mm:ss xxxxx"));
        let time = Time::parse("123201+01:01", "HHmmssxxx").unwrap();
        assert_eq!("13:33:01 +01:01", time.format("HH:mm:ss xxxxx"));
        let time = Time::parse("123201-01:00", "HHmmssxxx").unwrap();
        assert_eq!("11:32:01 -01:00", time.format("HH:mm:ss xxxxx"));
        let time = Time::parse("123201-01:01", "HHmmssxxx").unwrap();
        assert_eq!("11:31:01 -01:01", time.format("HH:mm:ss xxxxx"));

        parse_err_t("", "X");
        parse_err_t("a", "X");
        parse_err_t("+aa", "X");
        parse_err_t("+010a", "X");

        parse_err_t("+010a", "XX");

        parse_err_t("+01", "XXX");
        parse_err_t("+01", "xxx");
        parse_err_t("+01:aa", "XXX");

        parse_err_t("+010a", "XXXX");
        parse_err_t("+010a01", "XXXX");
        parse_err_t("+01010a", "XXXX");

        parse_err_t("+01:0a:01", "XXXXX");
        parse_err_t("+01:01:0a", "XXXXX");
        parse_err_t("+01", "XXXXX");
        parse_err_t("+01:0a", "XXXXX");

        parse_err_t("1+99:00", "hXXX");
        parse_err_t("99+01:00", "hXXX");

        let date_time = DateTime::parse("2022-05-02 +01:00", "yyyy-MM-dd xxx").unwrap();
        assert_eq!("2022/05/02 +01:00", date_time.format("yyyy/MM/dd xxx"));
    }

    #[test]
    fn escape() {
        parse_ok_d("yyyMMdd2022-05-02yyyMMdd", "'yyyMMdd'yyy-MM-dd'yyyMMdd");
        parse_ok_d("yyyMMdd2022-05-02yyyMMdd", "'yyyMMdd'yyy-MM-dd'yyyMMdd'");
        parse_ok_d(
            "yyyMMdd2022-05-01yyyMMdd02",
            "'yyyMMdd'yyy-MM-dd'yyyMMdd'dd",
        );
        parse_ok_d("yyyMMdd'dd2022-05-02", "'yyyMMdd''dd'yyy-MM-dd");
        parse_ok_d(
            "yyyyMMdd2022-05-02yyyMMdd'dd",
            "'yyyyMMdd'yyyy-MM-dd'yyyMMdd''dd'",
        );
        parse_ok_d("''2022-05-02", "''''yyy-MM-dd");
        parse_ok_d("'02'2022-05", "''dd''yyy-MM");
        parse_ok_d("''02''2022-05", "''''dd''''yyy-MM");
        parse_ok_d("''dd''2022-05-02", "'''''dd'''''yyy-MM-dd");
        parse_ok_d("test2022-05-02", "te's'tyyy-MM-dd");

        parse_ok_t("HHmmss123201HHmmss", "'HHmmss'HHmmss'HHmmss");
        parse_ok_t("HHmmss123201HHmmss", "'HHmmss'HHmmss'HHmmss'");
        parse_ok_t("HHmmss123201HHmmss01", "'HHmmss'HHmmss'HHmmss'ss");
        parse_ok_t("123201HHmmss'ss", "HHmmss'HHmmss''ss");
        parse_ok_t("HHmmss123201HHmmss'ss", "'HHmmss'HHmmss'HHmmss''ss'");
        parse_ok_t("''123201", "''''HHmmss");
        parse_ok_t("01'01'123201", "ss''ss''HHmm");
        parse_ok_t("01'02'1232", "ss'''ss'''HHmm");
        parse_ok_t("01''01''1232", "ss''''ss''''HHmm");
        parse_ok_t("01''02''1232", "ss'''''ss'''''HHmm");

        parse_ok_dt("yyyMMdd2022-05-02yyyMMdd", "'yyyMMdd'yyy-MM-dd'yyyMMdd");
        parse_ok_dt("yyyMMdd2022-05-02yyyMMdd", "'yyyMMdd'yyy-MM-dd'yyyMMdd'");
        parse_ok_dt(
            "yyyMMdd2022-05-01yyyMMdd02",
            "'yyyMMdd'yyy-MM-dd'yyyMMdd'dd",
        );
        parse_ok_dt("yyyMMdd'dd2022-05-02", "'yyyMMdd''dd'yyy-MM-dd");
        parse_ok_dt(
            "yyyyMMdd2022-05-02yyyMMdd'dd",
            "'yyyyMMdd'yyyy-MM-dd'yyyMMdd''dd'",
        );
        parse_ok_dt("''2022-05-02", "''''yyy-MM-dd");
        parse_ok_dt("'02'2022-05", "''dd''yyy-MM");
        parse_ok_dt("''02''2022-05", "''''dd''''yyy-MM");
        parse_ok_dt("''dd''2022-05-02", "'''''dd'''''yyy-MM-dd");
        parse_ok_dt("test2022-05-02", "te's'tyyy-MM-dd");
    }

    #[test]
    fn other() {
        parse_err_d("2022-13-02", "yyyy-MM-dd");
        parse_err_d("2022-366", "yyyy-D");
        parse_err_d("2020-367", "yyyy-D");
        parse_err_d("2022-0", "yyyy-D");
        parse_err_d("5879611-194", "yyyy-D");
        parse_err_d("-5879611-173", "yyyy-D");
        parse_ok_custom_d("5879611-193", "yyyy-D", "5879611/07/12");
        parse_ok_custom_d("-5879611-174", "yyyy-D", "-5879611/06/23");
        parse_ok_custom_d("-4-1", "yyyy-D", "-0004/01/01");

        parse_err_d("0-1", "y-D");
        parse_err_d("-5879612-1", "y-D");
        parse_err_d("5879612-1", "y-D");
        parse_err_d("2020-0", "y-D");
        parse_err_d("-", "--");
        parse_err_t("-", "--");

        let date_time = DateTime::parse("2022-05-02 12:32:01", "yyyy-MM-dd HH:mm:ss").unwrap();
        assert_eq!(
            "2022-05-02 12:32:01",
            date_time.format("yyyy-MM-dd HH:mm:ss")
        );
        let date_time = DateTime::parse("'2022-05-02 12:32:01", "''yyyy-MM-dd HH:mm:ss").unwrap();
        assert_eq!(
            "2022-05-02 12:32:01",
            date_time.format("yyyy-MM-dd HH:mm:ss")
        );

        assert!(DateTime::parse("a", "y").is_err());
        assert!(DateTime::parse("a", "h").is_err());
        assert!(DateTime::parse("", "-").is_err());

        parse_err_t("99", "h");

        assert!(DateTime::parse("2022-14-02", "yyyy-MM-dd").is_err());
        assert!(DateTime::parse("2022-400", "yyyy-DDD").is_err());

        assert!(DateTime::parse("2022-M05-02 24", "yyyy-M-dd HH").is_err());
        assert!(DateTime::parse("2022-05-02 +24:00", "yyyy-MM-dd xxx").is_err());
        assert!(DateTime::parse("2022-05-02 24", "yyyy-MM-dd HH").is_err());
    }

    #[test]
    fn minimal() {
        parse_ok_custom_t("", "", "00:00:00 000000000", "HH:mm:ss nnnnn");
        parse_ok_custom_t("12", "HH", "12:00:00 000000000", "HH:mm:ss nnnnn");

        parse_ok_custom_d("", "", "0001/01/01");
        parse_ok_custom_d("2022", "yyyy", "2022/01/01");
        parse_ok_custom_d("05", "MM", "0001/05/01");
        parse_ok_custom_d("02", "dd", "0001/01/02");

        let date_time = DateTime::parse("", "").unwrap();
        assert_eq!(
            "0001/01/01 00:00:00 000000000",
            date_time.format("yyyy/MM/dd HH:mm:ss nnnnn")
        );
        let date_time = DateTime::parse("2022", "yyyy").unwrap();
        assert_eq!(
            "2022/01/01 00:00:00 000000000",
            date_time.format("yyyy/MM/dd HH:mm:ss nnnnn")
        );
        let date_time = DateTime::parse("05", "MM").unwrap();
        assert_eq!(
            "0001/05/01 00:00:00 000000000",
            date_time.format("yyyy/MM/dd HH:mm:ss nnnnn")
        );
    }

    fn parse_ok_d(string: &str, format: &str) {
        let date = Date::parse(string, format).unwrap();
        assert_eq!("2022/05/02", date.format("yyyy/MM/dd"));
    }

    fn parse_ok_custom_d(string: &str, format: &str, result: &str) {
        let date = Date::parse(string, format).unwrap();
        assert_eq!(result, date.format("yyyy/MM/dd"));
    }

    fn parse_err_d(string: &str, format: &str) {
        let date = Date::parse(string, format);
        assert!(date.is_err());
    }

    fn parse_ok_t(string: &str, format: &str) {
        let time = Time::parse(string, format).unwrap();
        assert_eq!("12:32:01", time.format("HH:mm:ss"));
    }

    fn parse_ok_dt(string: &str, format: &str) {
        let date_time = DateTime::parse(string, format).unwrap();
        assert_eq!("2022/05/02", date_time.format("yyyy/MM/dd"));
    }

    fn parse_ok_custom_t(string: &str, format: &str, result: &str, result_format: &str) {
        let time = Time::parse(string, format).unwrap();
        assert_eq!(result, time.format(result_format));
    }

    fn parse_err_t(string: &str, format: &str) {
        let time = Time::parse(string, format);
        assert!(time.is_err());
    }
}
