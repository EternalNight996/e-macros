#[cfg(test)]
mod tests {
    use std::convert::TryFrom;
    use std::str::FromStr as _;

    use serde::{Deserialize, Serialize};

    #[derive(e_macros::Enum, Debug, Clone, Copy, Default, Serialize, Deserialize)]
    pub enum TestEnum {
        #[ename("测试1")]
        Variant1,
        #[ename("测试2")]
        Variant2,
        Variant3,
        #[default]
        Variant4,
    }

    #[test]
    fn test_enum_functionality() {
        let variants = [
            (TestEnum::Variant1, "测试1", 0),
            (TestEnum::Variant2, "测试2", 1),
            (TestEnum::Variant3, "Variant3", 2),
            (TestEnum::Variant4, "Variant4", 3),
        ];

        for (variant, name, index) in variants.iter() {
            // Test as_str
            assert_eq!(variant.as_str(), *name);

            // Test Display
            assert_eq!(format!("{}", variant), *name);

            // Test FromStr
            assert_eq!(TestEnum::from_str(name).unwrap(), *variant);

            // Test TryFrom<i32>
            assert_eq!(TestEnum::try_from(*index).unwrap(), *variant);

            // Test Into<i32>
            assert_eq!(Into::<i32>::into(*variant), *index);

            // Test TryFrom<&str>
            assert_eq!(TestEnum::from_str(*name as &str).unwrap(), *variant);
        }

        // Test invalid cases
        assert!(TestEnum::from_str("Invalid").is_err());
        assert!(TestEnum::try_from(100).is_err());
        assert!(TestEnum::from_str("Invalid" as &str).is_err());
    }

    #[test]
    fn test_default() {
        assert_eq!(TestEnum::default(), TestEnum::Variant4);
    }

    #[test]
    fn test_variants() {
        assert_eq!(TestEnum::COUNT, 4);
        assert_eq!(
            TestEnum::ALL,
            [
                TestEnum::Variant1,
                TestEnum::Variant2,
                TestEnum::Variant3,
                TestEnum::Variant4
            ]
        );
    }

    #[test]
    fn test_from_i32() {
        assert_eq!(TestEnum::try_from(0).unwrap(), TestEnum::Variant1);
        assert_eq!(TestEnum::try_from(1).unwrap(), TestEnum::Variant2);
        assert_eq!(TestEnum::try_from(2).unwrap(), TestEnum::Variant3);
        assert_eq!(TestEnum::try_from(3).unwrap(), TestEnum::Variant4);
        assert_eq!(
            TestEnum::try_from(100).unwrap_or_default(),
            TestEnum::default()
        );
    }

    #[test]
    fn test_try_from_str() {
        assert_eq!(TestEnum::try_from("测试1").unwrap(), TestEnum::Variant1);
        assert_eq!(TestEnum::try_from("Variant1").unwrap(), TestEnum::Variant1);
        assert_eq!(TestEnum::try_from("测试2").unwrap(), TestEnum::Variant2);
        assert_eq!(TestEnum::try_from("Variant2").unwrap(), TestEnum::Variant2);
        assert_eq!(TestEnum::try_from("Variant3").unwrap(), TestEnum::Variant3);
        assert_eq!(TestEnum::try_from("Variant4").unwrap(), TestEnum::Variant4);
        assert!(TestEnum::try_from("Invalid").is_err());
    }

    #[cfg(feature = "serde")]
    mod serde_tests {
        use super::*;
        use serde_json;

        #[test]
        fn test_serde() {
            let variants = [
                (TestEnum::Variant1, "测试1", "\"测试1\""),
                (TestEnum::Variant2, "测试2", "\"测试2\""),
                (TestEnum::Variant3, "Variant3", "\"Variant3\""),
                (TestEnum::Variant4, "Variant4", "\"Variant4\""),
            ];

            for (variant, name, json) in variants.iter() {
                // Test serialization
                assert_eq!(serde_json::to_string(variant).unwrap(), *json);
                // Test deserialization
                assert_eq!(serde_json::from_str::<TestEnum>(json).unwrap(), *variant);
                // Err
                assert!(serde_json::from_str::<TestEnum>(name).is_err());
            }

            // Test invalid cases
            assert!(serde_json::from_str::<TestEnum>("\"Invalid\"").is_err());
            assert!(serde_json::from_str::<TestEnum>("42").is_err());
            assert!(serde_json::from_str::<TestEnum>("null").is_err());
        }
    }

    #[cfg(feature = "serde_json")]
    #[test]
    fn test_serde_json_value() {
        use serde_json::Value;

        let variants = [
            (
                TestEnum::Variant1,
                "测试1",
                Value::String("测试1".to_string()),
            ),
            (
                TestEnum::Variant2,
                "测试2",
                Value::String("测试2".to_string()),
            ),
            (
                TestEnum::Variant3,
                "Variant3",
                Value::String("Variant3".to_string()),
            ),
            (
                TestEnum::Variant4,
                "Variant4",
                Value::String("Variant4".to_string()),
            ),
        ];

        for (variant, _name, json_value) in variants.iter() {
            // Test From<&TestEnum> for serde_json::Value
            assert_eq!(Value::from(variant), *json_value);

            // Test TryFrom<serde_json::Value> for TestEnum
            assert_eq!(TestEnum::try_from(json_value.clone()).unwrap(), *variant);
        }

        // Test invalid cases
        assert!(TestEnum::try_from(Value::String("Invalid".to_string())).is_err());
        assert!(TestEnum::try_from(Value::Number(42.into())).is_err());
        assert!(TestEnum::try_from(Value::Null).is_err());
    }
}
