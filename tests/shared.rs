#[cfg(test)]
mod shared_tests {
    use astrolabe::AstrolabeError;

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
}
