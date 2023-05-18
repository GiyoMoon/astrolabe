#[cfg(test)]
#[cfg(feature = "cron")]
mod cron_tests {
    use std::str::FromStr;

    use astrolabe::CronSchedule;

    #[test]
    fn parse() {
        // Test valid cron expressions
        assert!(CronSchedule::parse("* * * * *").is_ok());
        assert!(CronSchedule::parse("0 0 1 1 0").is_ok());
        assert!(CronSchedule::parse("*/1 */1 */1 */1 */1").is_ok());
        assert!(CronSchedule::parse("0,59 0,23 1,31 1,12 0,6").is_ok());
        assert!(CronSchedule::parse("0-59 0-23 1-31 1-12 0-6").is_ok());
        assert!(CronSchedule::parse("* * * jan,FEB,mAr mon,THU,wEd").is_ok());
        assert!(CronSchedule::parse("* * * jan-dec sun-sat").is_ok());
        assert!(CronSchedule::parse("* * * * 7").is_ok());
        assert!(CronSchedule::parse(
            "* * * jan,feb,mar,apr,may,jun,jul,aug,sep,oct,nov,dec sun,mon,tue,wed,thu,fri,sat"
        )
        .is_ok());
        assert!(CronSchedule::from_str("* * * * *").is_ok());

        // Test invalid cron expressions
        assert!(CronSchedule::parse("").is_err());
        assert!(CronSchedule::parse("* * * * * *").is_err());
        assert!(CronSchedule::parse("a a a a a").is_err());
        assert!(CronSchedule::parse("60 * * * *").is_err());
        assert!(CronSchedule::parse("* 24 * * *").is_err());
        assert!(CronSchedule::parse("* * 0 * *").is_err());
        assert!(CronSchedule::parse("* * 32 * *").is_err());
        assert!(CronSchedule::parse("* * * 0 *").is_err());
        assert!(CronSchedule::parse("* * * 13 *").is_err());
        assert!(CronSchedule::parse("* * * * 8").is_err());
        assert!(CronSchedule::parse("*/0 * * * *").is_err());
        assert!(CronSchedule::parse("*/bla * * * *").is_err());
        assert!(CronSchedule::parse("*/256 * * * *").is_err());
        assert!(CronSchedule::parse("1-0 * * * *").is_err());
        assert!(CronSchedule::parse("0-60 * * * *").is_err());
        assert!(CronSchedule::parse("256 * * * *").is_err());
        assert!(CronSchedule::parse("256-0 * * * *").is_err());
        assert!(CronSchedule::parse("0-256 * * * *").is_err());
        assert!(CronSchedule::parse("-60 * * * *").is_err());
        assert!(CronSchedule::parse("0- * * * *").is_err());
        assert!(CronSchedule::parse("* * * bla *").is_err());
        assert!(CronSchedule::parse("* * * * bla").is_err());
        assert!(CronSchedule::from_str("").is_err());
    }

    #[test]
    fn debug() {
        let schedule = CronSchedule::parse("0 0 1 1 0").unwrap();
        assert_eq!("CronSchedule { minutes: {0}, hours: {0}, days_of_month: {1}, months: {1}, days_of_week: {0}, last_schedule: None }", format!("{:?}", schedule));
    }

    #[test]
    fn clone() {
        let schedule = CronSchedule::parse("* * * * *").unwrap();
        #[allow(clippy::redundant_clone)]
        let _ = schedule.clone();
    }
}
