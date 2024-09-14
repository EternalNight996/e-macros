#[e_macros::value]
#[derive(Debug, PartialEq, Default)]
#[repr(u8)]
enum TestEnum {
    #[default]
    #[e(value = "test", index = 1)]
    One,
    #[e(index = 20)]
    Two,
    #[e(index = 255)]
    Three,
    Custom(i32),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value() {
        assert_eq!(TestEnum::One.value(), "test");
        assert_eq!(TestEnum::Two.value(), "Two");
        assert_eq!(TestEnum::Three.value(), "Three");
        assert_eq!(TestEnum::Custom(42).value(), "Custom");
    }

    #[test]
    fn test_index() {
        assert_eq!(TestEnum::One.index(), 1);
        assert_eq!(TestEnum::Two.index(), 20);
        assert_eq!(TestEnum::Three.index(), 255);
        assert_eq!(TestEnum::Custom(42).index(), 255);
    }

    #[test]
    fn test_try_from_u8() {
        assert_eq!(TestEnum::try_from(1u8), Ok(TestEnum::One));
        assert_eq!(TestEnum::try_from(20u8), Ok(TestEnum::Two));
        assert_eq!(TestEnum::try_from(255u8), Ok(TestEnum::Three));
        assert!(TestEnum::try_from(0u8).is_err());
    }

    #[test]
    fn test_try_from_str() {
        assert_eq!(TestEnum::try_from("test"), Ok(TestEnum::One));
        assert_eq!(TestEnum::try_from("Two"), Ok(TestEnum::Two));
        assert_eq!(TestEnum::try_from("Three"), Ok(TestEnum::Three));
        assert!(TestEnum::try_from("Custom").is_err());
        assert!(TestEnum::try_from("InvalidVariant").is_err());
    }

    #[test]
    fn test_variant_count() {
        assert_eq!(TestEnum::variant_count(), 4);
    }
}
