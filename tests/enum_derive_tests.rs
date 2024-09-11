#[cfg(test)]
mod tests {
    use std::convert::TryFrom;
    use std::str::FromStr;

    use e_macros::*;

    // Ensure TestEnum is defined only once
    #[derive(Enum, Clone, Copy, Debug)]
    pub enum TestEnum {
        Variant1,
        Variant2,
        Variant3,
    }

    #[test]
    fn test_as_str() {
        assert_eq!(TestEnum::Variant1.as_str(), "Variant1");
        assert_eq!(TestEnum::Variant2.as_str(), "Variant2");
        assert_eq!(TestEnum::Variant3.as_str(), "Variant3");
    }

    #[test]
    fn test_from_str() {
        assert_eq!(TestEnum::from_str("Variant1").unwrap(), TestEnum::Variant1);
        assert_eq!(TestEnum::from_str("Variant2").unwrap(), TestEnum::Variant2);
        assert_eq!(TestEnum::from_str("Variant3").unwrap(), TestEnum::Variant3);
        assert!(TestEnum::from_str("Invalid").is_err());
    }

    #[test]
    fn test_try_from_i32() {
        assert_eq!(TestEnum::try_from(0).unwrap(), TestEnum::Variant1);
        assert_eq!(TestEnum::try_from(1).unwrap(), TestEnum::Variant2);
        assert_eq!(TestEnum::try_from(2).unwrap(), TestEnum::Variant3);
        assert!(TestEnum::try_from(100).is_err());
    }

    #[test]
    fn test_into_i32() {
        let v1: i32 = TestEnum::Variant1.into();
        let v2: i32 = TestEnum::Variant2.into();
        let v3: i32 = TestEnum::Variant3.into();
        assert_eq!(v1, 0);
        assert_eq!(v2, 1);
        assert_eq!(v3, 2);
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", TestEnum::Variant1), "Variant1");
        assert_eq!(format!("{}", TestEnum::Variant2), "Variant2");
        assert_eq!(format!("{}", TestEnum::Variant3), "Variant3");
    }
}
