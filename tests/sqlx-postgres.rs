#[cfg(test)]
#[cfg(feature = "sqlx-postgres")]
mod sqlx_postgres_tests {
    use astrolabe::{Date, DateTime, Precision, Time, TimeUtilities};
    use sqlx::PgPool;

    #[sqlx::test]
    async fn datetime(db_pool: PgPool) {
        let datetime = DateTime::from_ymdhms(2024, 1, 2, 10, 30, 40)
            .unwrap()
            .set_nano(123_456_789)
            .unwrap();
        let insert_result = sqlx::query!(
            r#"
            INSERT INTO astrolabe_tests (timestamp) VALUES ($1)
            RETURNING timestamp as "timestamp: DateTime"
            "#,
            datetime as DateTime
        )
        .fetch_one(&db_pool)
        .await;

        assert!(insert_result.is_ok());

        let datetime = insert_result.unwrap().timestamp.unwrap();

        assert_eq!(
            "2024-01-02T10:30:40.123456000Z",
            datetime.format_rfc3339(Precision::Nanos)
        );
    }

    #[sqlx::test]
    async fn datetime_array(db_pool: PgPool) {
        let datetime_1 = DateTime::from_ymdhms(2024, 1, 2, 10, 30, 40)
            .unwrap()
            .set_nano(123_456_789)
            .unwrap();
        let datetime_2 = DateTime::from_ymdhms(1980, 1, 2, 10, 30, 40)
            .unwrap()
            .set_nano(123_456_789)
            .unwrap();
        let insert_result = sqlx::query!(
            r#"
            INSERT INTO astrolabe_tests (timestamps) VALUES ($1)
            RETURNING timestamps as "timestamps: Vec<DateTime>"
            "#,
            vec![datetime_1, datetime_2] as Vec<DateTime>
        )
        .fetch_one(&db_pool)
        .await;

        assert!(insert_result.is_ok());

        let datetimes = insert_result.unwrap().timestamps.unwrap();

        assert_eq!(
            "2024-01-02T10:30:40.123456000Z",
            datetimes[0].format_rfc3339(Precision::Nanos)
        );
        assert_eq!(
            "1980-01-02T10:30:40.123456000Z",
            datetimes[1].format_rfc3339(Precision::Nanos)
        );
    }

    #[sqlx::test]
    async fn date(db_pool: PgPool) {
        let date = Date::from_ymd(2024, 1, 2).unwrap();
        let insert_result = sqlx::query!(
            r#"
            INSERT INTO astrolabe_tests (date) VALUES ($1)
            RETURNING date as "date: Date"
            "#,
            date as Date
        )
        .fetch_one(&db_pool)
        .await;

        assert!(insert_result.is_ok());

        let date = insert_result.unwrap().date.unwrap();

        assert_eq!("2024/01/02", date.format("yyyy/MM/dd"));
    }

    #[sqlx::test]
    async fn date_array(db_pool: PgPool) {
        let date_1 = Date::from_ymd(2024, 1, 2).unwrap();
        let date_2 = Date::from_ymd(1980, 1, 2).unwrap();
        let insert_result = sqlx::query!(
            r#"
            INSERT INTO astrolabe_tests (dates) VALUES ($1)
            RETURNING dates as "dates: Vec<Date>"
            "#,
            vec![date_1, date_2] as Vec<Date>
        )
        .fetch_one(&db_pool)
        .await;

        assert!(insert_result.is_ok());

        let dates = insert_result.unwrap().dates.unwrap();

        assert_eq!("2024/01/02", dates[0].format("yyyy/MM/dd"));
        assert_eq!("1980/01/02", dates[1].format("yyyy/MM/dd"));
    }

    #[sqlx::test]
    async fn time(db_pool: PgPool) {
        let time = Time::from_hms(10, 30, 40)
            .unwrap()
            .set_nano(123_456_789)
            .unwrap();
        let insert_result = sqlx::query!(
            r#"
            INSERT INTO astrolabe_tests (time) VALUES ($1)
            RETURNING time as "time: Time"
            "#,
            time as Time
        )
        .fetch_one(&db_pool)
        .await;

        assert!(insert_result.is_ok());

        let time = insert_result.unwrap().time.unwrap();

        assert_eq!("10:30:40:123456000", time.format("HH:mm:ss:nnnnn"));
    }

    #[sqlx::test]
    async fn time_array(db_pool: PgPool) {
        let time_1 = Time::from_hms(10, 30, 40)
            .unwrap()
            .set_nano(123_456_789)
            .unwrap();
        let time_2 = Time::from_hms(23, 20, 35)
            .unwrap()
            .set_nano(123_456_789)
            .unwrap();
        let insert_result = sqlx::query!(
            r#"
            INSERT INTO astrolabe_tests (times) VALUES ($1)
            RETURNING times as "times: Vec<Time>"
            "#,
            vec![time_1, time_2] as Vec<Time>
        )
        .fetch_one(&db_pool)
        .await;

        assert!(insert_result.is_ok());

        let times = insert_result.unwrap().times.unwrap();

        assert_eq!("10:30:40:123456000", times[0].format("HH:mm:ss:nnnnn"));
        assert_eq!("23:20:35:123456000", times[1].format("HH:mm:ss:nnnnn"));
    }
}
