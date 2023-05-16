#[cfg(test)]
mod errors_tests {
    use astrolabe::DateTime;
    use std::{
        collections::hash_map::DefaultHasher,
        hash::{Hash, Hasher},
    };

    #[test]
    fn out_of_range() {
        let out_of_range = DateTime::from_ymd(1, 0, 1).unwrap_err();
        // Debug
        println!("{:?}", out_of_range);
        // From<AstrolabeError> for String
        println!("{}", String::from(out_of_range.clone()));
        // From<&AstrolabeError> for String
        println!("{}", String::from(&out_of_range));
        // Display
        println!("{}", out_of_range);
        // Hash
        let mut hasher = DefaultHasher::new();
        out_of_range.hash(&mut hasher);
        println!("{:x}", hasher.finish());

        let clone = out_of_range.clone();
        assert!(out_of_range == clone);

        let custom_out_of_range = DateTime::from_ymd(0, 1, 1).unwrap_err();
        // Display
        println!("{}", custom_out_of_range);

        let conditional_out_of_range = DateTime::from_ymd(5_879_611, 12, 31).unwrap_err();
        // Display
        println!("{}", conditional_out_of_range);
    }

    #[test]
    fn invalid_format() {
        let invalid_format = DateTime::parse_rfc3339("test").unwrap_err();
        // Debug
        println!("{:?}", invalid_format);
        // Display
        println!("{}", invalid_format);
        // From<AstrolabeError> for String
        println!("{}", String::from(invalid_format.clone()));
        // From<AstrolabeError> for String
        println!("{}", String::from(&invalid_format));
        // Hash
        let mut hasher = DefaultHasher::new();
        invalid_format.hash(&mut hasher);
        println!("{:x}", hasher.finish());

        let clone = invalid_format.clone();
        assert!(invalid_format == clone);
    }
}
