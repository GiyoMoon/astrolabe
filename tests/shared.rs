#[cfg(test)]
mod shared_tests {
    use astrolabe::{Offset, Precision};

    #[test]
    fn offset() {
        let offset = Offset::East;
        // Debug
        println!("{:?}", offset);
        // Clone
        #[allow(clippy::redundant_clone)]
        let clone = offset.clone();
        // PartialEq
        assert!(offset == clone);
    }

    #[test]
    fn precision() {
        let precision = Precision::Seconds;
        // Debug
        println!("{:?}", precision);
        // Clone
        #[allow(clippy::redundant_clone)]
        let clone = precision.clone();
        // PartialEq
        assert!(precision == clone);
    }
}
