#[cfg(test)]
#[cfg(feature = "serde")]
mod serde_tests {
    use astrolabe::{Date, DateTime, Time};
    use serde_test::{assert_de_tokens_error, assert_tokens, Token};

    #[test]
    fn time() {
        let time = Time::from_hms(12, 32, 10).unwrap();

        assert_tokens(&time, &[Token::String("12:32:10")]);

        assert_de_tokens_error::<Time>(
            &[Token::I32(0)],
            "invalid type: integer `0`, expected a formatted date string in the format `HH:mm:ss`",
        );
    }

    #[test]
    fn date() {
        let date = Date::from_ymd(2022, 5, 2).unwrap();

        assert_tokens(&date, &[Token::String("2022-05-02")]);

        assert_de_tokens_error::<Date>(
            &[Token::I32(0)],
            "invalid type: integer `0`, expected a formatted date string in the format `yyyy-MM-dd`",
        );
    }

    #[test]
    fn date_time() {
        let date_time = DateTime::from_ymdhms(2022, 5, 2, 12, 32, 10).unwrap();

        assert_tokens(&date_time, &[Token::String("2022-05-02T12:32:10Z")]);

        assert_de_tokens_error::<DateTime>(
            &[Token::I32(0)],
            "invalid type: integer `0`, expected an RFC 3339 formatted date string",
        );
    }
}
