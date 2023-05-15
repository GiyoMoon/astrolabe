#[cfg(test)]
mod shared_tests {
    use astrolabe::Precision;

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
