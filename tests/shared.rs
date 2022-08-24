#[cfg(test)]
mod shared_tests {
    use astrolabe::{Offset, Precision};

    #[test]
    fn offset() {
        let offset = Offset::East;
        // Debug
        println!("{:?}", offset);
        // Clone
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
        let clone = precision.clone();
        // PartialEq
        assert!(precision == clone);
    }
}
