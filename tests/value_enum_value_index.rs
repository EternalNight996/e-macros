#![allow(unused)]
use e_macros::value;
use std::fmt;
#[value]
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[repr(i8)]
enum TestEnum {
    #[e(value = "one")]
    #[e(index = 111)]
    One,
    #[e(value = "two", index = 2)]
    Two,
    #[e(value = "测试")]
    Three,
    #[e(index = 50, value = "three2")]
    Three2,
    Next,
    #[e(index = 118)]
    Next2,
    #[e(value = "")]
    EmptyValue,
    #[e(index = 127)]
    MaxI8Index,
    #[e(value = "Object")]
    Vlaue {
        index: i32,
        value: String,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_variant_count() {
        assert_eq!(TestEnum::variant_count(), 9);
    }
    #[test]
    fn test_index() {
        assert_eq!(TestEnum::One.index(), 111);
        assert_eq!(TestEnum::Two.index(), 2);
        assert_eq!(TestEnum::Three.index(), 3);
        assert_eq!(TestEnum::Three2.index(), 50);
        assert_eq!(TestEnum::Next.index(), 51);
        assert_eq!(TestEnum::Next2.index(), 118);
        assert_eq!(TestEnum::EmptyValue.index(), 119);
        assert_eq!(TestEnum::MaxI8Index.index(), 127);
        assert_eq!(
            TestEnum::Vlaue {
                index: 0,
                value: "".to_string()
            }
            .index(),
            127
        );
    }

    #[test]
    fn test_value() {
        assert_eq!(TestEnum::One.value(), "one");
        assert_eq!(TestEnum::Two.value(), "two");
        assert_eq!(TestEnum::Three.value(), "测试");
        assert_eq!(TestEnum::Three2.value(), "three2");
        assert_eq!(TestEnum::Next.value(), "Next");
        assert_eq!(TestEnum::Next2.value(), "Next2");
        assert_eq!(TestEnum::EmptyValue.value(), "");
        assert_eq!(TestEnum::MaxI8Index.value(), "MaxI8Index");
        assert_eq!(
            TestEnum::Vlaue {
                index: 0,
                value: "".to_string()
            }
            .value(),
            "Object"
        );
    }

    #[test]
    fn test_custom_index_and_value() {
        assert_eq!(TestEnum::One.index(), 111);
        assert_eq!(TestEnum::One.value(), "one");
        assert_eq!(TestEnum::Two.index(), 2);
        assert_eq!(TestEnum::Two.value(), "two");
    }

    #[test]
    fn test_default_index_assignment() {
        assert_eq!(TestEnum::Three.index(), 3);
        assert_eq!(TestEnum::Next.index(), 51);
    }

    #[test]
    fn test_struct_variant() {
        let value = TestEnum::Vlaue {
            index: 42,
            value: "test".to_string(),
        };
        assert_eq!(value.index(), 127);
        assert_eq!(value.value(), "Object");
    }

    #[test]
    fn test_max_i8_index() {
        assert_eq!(TestEnum::MaxI8Index.index(), 127);
    }

    #[test]
    fn test_empty_value() {
        assert_eq!(TestEnum::EmptyValue.value(), "");
    }

    #[test]
    fn test_non_ascii_value() {
        assert_eq!(TestEnum::Three.value(), "测试");
    }
}
