use e_macros::value;

#[value]
#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
enum TestEnum {
    #[e(value = "one")]
    #[e(index = 111)]
    One,
    #[e(value = "two")]
    #[e(index = 2)]
    Two,
    #[e(value = "three")]
    Three,
    #[e(value = "three2")]
    Three2,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_method() {
        let cases = [
            (TestEnum::One, "one"),
            (TestEnum::Two, "two"),
            (TestEnum::Three, "three"),
            (TestEnum::Three2, "three2"),
        ];

        for (variant, expected) in cases {
            assert_eq!(variant.value(), expected, "Value method test failed for: {:?}", variant);
        }
    }

    #[test]
    fn test_index_method() {
        let cases = [
            (TestEnum::One, 111),
            (TestEnum::Two, 2),
            (TestEnum::Three, 0),  // Default index is 0
            (TestEnum::Three2, 0), // Default index is 0
        ];

        for (variant, expected) in cases {
            assert_eq!(variant.index(), expected, "Index method test failed for: {:?}", variant);
        }
    }

    #[test]
    fn test_enum_repr() {
        assert_eq!(std::mem::size_of::<TestEnum>(), 1, "Enum size should be 1 byte");
    }

    #[test]
    fn test_debug_and_partial_eq() {
        assert_eq!(format!("{:?}", TestEnum::One), "One", "Debug implementation test failed");

        assert_eq!(TestEnum::Two, TestEnum::Two, "Equality test failed");
        assert_ne!(TestEnum::Two, TestEnum::Three, "Inequality test failed");

        let variants = [
            TestEnum::One,
            TestEnum::Two,
            TestEnum::Three,
            TestEnum::Three2,
        ];
        for (i, v1) in variants.iter().enumerate() {
            for (j, v2) in variants.iter().enumerate() {
                if i == j {
                    assert_eq!(v1, v2, "Same variants should be equal: {:?}", v1);
                } else {
                    assert_ne!(v1, v2, "Different variants should not be equal: {:?} and {:?}", v1, v2);
                }
            }
        }
    }

    #[test]
    fn test_clone_and_copy() {
        let original = TestEnum::One;
        let cloned = original.clone();
        assert_eq!(original, cloned, "Should be equal after cloning");

        let copied = original;
        assert_eq!(original, copied, "Should be equal after copying");
    }

    #[test]
    fn test_default_index() {
        assert_eq!(TestEnum::Three.index(), 0, "Unspecified index variant should default to 0");
        assert_eq!(TestEnum::Three2.index(), 0, "Unspecified index variant should default to 0");
    }

    #[test]
    fn test_value_uniqueness() {
        let values = [
            TestEnum::One.value(),
            TestEnum::Two.value(),
            TestEnum::Three.value(),
            TestEnum::Three2.value(),
        ];
        let mut unique_values = values.to_vec();
        unique_values.sort();
        unique_values.dedup();
        assert_eq!(values.len(), unique_values.len(), "All values should be unique");
    }

    #[test]
    fn test_underlying_values() {
        assert!(TestEnum::One as u8 == 0, "One's underlying value should be 0");
        assert!(TestEnum::Two as u8 == 1, "Two's underlying value should be 1");
        assert!(TestEnum::Three as u8 == 2, "Three's underlying value should be 2");
        assert!(TestEnum::Three2 as u8 == 3, "Three2's underlying value should be 3");
    }

    #[test]
    fn test_index_values() {
        assert_eq!(TestEnum::One.index(), 111, "One's index value should be 111");
        assert_eq!(TestEnum::Two.index(), 2, "Two's index value should be 2");
        assert_eq!(TestEnum::Three.index(), 0, "Three's index value should default to 0");
        assert_eq!(TestEnum::Three2.index(), 0, "Three2's index value should default to 0");

        // Note: Three and Three2 have the same index value, which might not be ideal
        println!("Warning: Three and Three2 have the same index value, which might cause unexpected behavior");
    }

    #[test]
    fn test_index_and_value_relationship() {
        // Test the relationship between index values and underlying values
        assert!(
            TestEnum::One as u8 != TestEnum::One.index(),
            "One's underlying value and index value should be different"
        );
        assert!(
            TestEnum::Two as u8 != TestEnum::Two.index(),
            "Two's underlying value and index value should be different"
        );
        assert!(
            TestEnum::Three as u8 != TestEnum::Three.index(),
            "Three's underlying value and index value should be different"
        );
        assert!(
            TestEnum::Three2 as u8 != TestEnum::Three2.index(),
            "Three2's underlying value and index value should be different"
        );
    }
}
