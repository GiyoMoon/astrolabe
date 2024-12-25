#[cfg(test)]
#[cfg(feature = "sqlx-postgres")]
mod sqlx_postgres_tests {
    use astrolabe::{Date, DateTime, Precision, Time, TimeUtilities};
    use sqlx::PgPool;

    #[sqlx::test]
    async fn datetime(db_pool: PgPool) {
        #[derive(sqlx::FromRow)]
        struct TimestampRow {
            timestamp: DateTime,
        }
        let datetime = DateTime::from_ymdhms(2024, 1, 2, 10, 30, 40)
            .unwrap()
            .set_nano(123_456_789)
            .unwrap();
        let insert_result: Result<TimestampRow, sqlx::Error> = sqlx::query_as(
            r#"
            INSERT INTO astrolabe_tests (timestamp) VALUES ($1)
            RETURNING timestamp
            "#,
        )
        .bind(datetime)
        .fetch_one(&db_pool)
        .await;

        assert!(insert_result.is_ok());

        let datetime = insert_result.unwrap().timestamp;

        assert_eq!(
            "2024-01-02T10:30:40.123456000Z",
            datetime.format_rfc3339(Precision::Nanos)
        );
    }

    #[sqlx::test]
    async fn date(db_pool: PgPool) {
        #[derive(sqlx::FromRow)]
        struct DateRow {
            date: Date,
        }
        let date = Date::from_ymd(2024, 1, 2).unwrap();
        let insert_result: Result<DateRow, sqlx::Error> = sqlx::query_as(
            r#"
            INSERT INTO astrolabe_tests (date) VALUES ($1)
            RETURNING date
            "#,
        )
        .bind(date)
        .fetch_one(&db_pool)
        .await;

        assert!(insert_result.is_ok());

        let date = insert_result.unwrap().date;

        assert_eq!("2024/01/02", date.format("yyyy/MM/dd"));
    }

    #[sqlx::test]
    async fn time(db_pool: PgPool) {
        #[derive(sqlx::FromRow)]
        struct TimeRow {
            time: Time,
        }
        let time = Time::from_hms(10, 30, 40)
            .unwrap()
            .set_nano(123_456_789)
            .unwrap();
        let insert_result: Result<TimeRow, sqlx::Error> = sqlx::query_as(
            r#"
            INSERT INTO astrolabe_tests (time) VALUES ($1)
            RETURNING time
            "#,
        )
        .bind(time)
        .fetch_one(&db_pool)
        .await;

        assert!(insert_result.is_ok());

        let time = insert_result.unwrap().time;

        assert_eq!("10:30:40:123456000", time.format("HH:mm:ss:nnnnn"));
    }
}
