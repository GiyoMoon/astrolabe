#[cfg(test)]
mod shared_tests {
    use astrolabe::{AstrolabeError, Offset, Precision};

    #[test]
    fn astrolabe_error() {
        let error = AstrolabeError::OutOfRange;
        // Debug
        println!("{error:?}");
        // Clone
        let clone = error.clone();
        // PartialEq
        assert!(error == clone);
    }

    #[test]
    fn offset() {
        let offset = Offset::East;
        // Debug
        println!("{offset:?}");
        // Clone
        let clone = offset.clone();
        // PartialEq
        assert!(offset == clone);
    }

    #[test]
    fn precision() {
        let precision = Precision::Seconds;
        // Debug
        println!("{precision:?}");
        // Clone
        let clone = precision.clone();
        // PartialEq
        assert!(precision == clone);
    }
}
