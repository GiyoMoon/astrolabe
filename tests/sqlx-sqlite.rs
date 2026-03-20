#[cfg(test)]
#[cfg(feature = "sqlx-sqlite")]
mod sqlx_sqlite_tests {
    use astrolabe::{Date, DateTime, Precision, Time, TimeUtilities};
    use sqlx::SqlitePool;

    #[sqlx::test(migrations = "./migrations-sqlite")]
    async fn datetime(db_pool: SqlitePool) {
        let datetime = DateTime::from_ymdhms(2024, 1, 2, 10, 30, 40)
            .unwrap()
            .set_nano(123_456_789)
            .unwrap();
        let result = sqlx::query!(
            r#"
            INSERT INTO astrolabe_tests (timestamp) VALUES (?)
            RETURNING timestamp as "timestamp: DateTime"
            "#,
            datetime
        )
        .fetch_one(&db_pool)
        .await
        .unwrap();

        assert_eq!(
            "2024-01-02T10:30:40.000000000Z",
            result.timestamp.unwrap().format_rfc3339(Precision::Nanos)
        );
    }

    #[sqlx::test(migrations = "./migrations-sqlite")]
    async fn date(db_pool: SqlitePool) {
        let date = Date::from_ymd(2024, 1, 2).unwrap();
        let result = sqlx::query!(
            r#"
            INSERT INTO astrolabe_tests (date) VALUES (?)
            RETURNING date as "date: Date"
            "#,
            date
        )
        .fetch_one(&db_pool)
        .await
        .unwrap();

        assert_eq!("2024/01/02", result.date.unwrap().format("yyyy/MM/dd"));
    }

    #[sqlx::test(migrations = "./migrations-sqlite")]
    async fn time(db_pool: SqlitePool) {
        let time = Time::from_hms(10, 30, 40)
            .unwrap()
            .set_nano(123_456_789)
            .unwrap();
        let result = sqlx::query!(
            r#"
            INSERT INTO astrolabe_tests (time) VALUES (?)
            RETURNING time as "time: Time"
            "#,
            time
        )
        .fetch_one(&db_pool)
        .await
        .unwrap();

        assert_eq!(
            "10:30:40:000000000",
            result.time.unwrap().format("HH:mm:ss:nnnnn")
        );
    }
}
