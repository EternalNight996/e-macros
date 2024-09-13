#![allow(unused)]
use e_macros::value;
use std::fmt;

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

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
    }

    #[test]
    fn test_index() {
        assert_eq!(TestEnum::One.index(), 111);
        assert_eq!(TestEnum::Two.index(), 2);
        assert_eq!(TestEnum::Three.index(), 0); // 默认值
        assert_eq!(TestEnum::Three2.index(), 3);
        assert_eq!(TestEnum::Next.index(), 0); // 默认值
        assert_eq!(TestEnum::Next2.index(), 118);
        assert_eq!(TestEnum::EmptyValue.index(), 0); // 默认值
        assert_eq!(TestEnum::MaxI8Index.index(), 127);
    }

    #[test]
    fn test_derive_traits() {
        // 测试 Debug
        assert_eq!(format!("{:?}", TestEnum::One), "One");
        assert_eq!(format!("{:?}", TestEnum::Two), "Two");

        // 测试 PartialEq 和 Eq
        assert_eq!(TestEnum::One, TestEnum::One);
        assert_ne!(TestEnum::One, TestEnum::Two);

        // 测试 Clone 和 Copy
        let a = TestEnum::Three;
        let b = a;
        assert_eq!(a, b);

        // 测试 Hash
        let mut set = HashSet::new();
        set.insert(TestEnum::One);
        set.insert(TestEnum::Two);
        assert_eq!(set.len(), 2);
        assert!(set.contains(&TestEnum::One));
    }

    #[test]
    fn test_repr() {
        // 确保枚举使用 i8 表示
        assert_eq!(std::mem::size_of::<TestEnum>(), 1);
        assert_eq!(std::mem::align_of::<TestEnum>(), 1);
    }

    #[test]
    fn test_default_values() {
        // 测试没有显式设置 value 的情况
        assert_eq!(TestEnum::Next.value(), "Next");
        assert_eq!(TestEnum::Next.index(), 0);
    }

    #[test]
    fn test_combined_attributes() {
        // 测试在一个属性中同时设置 value 和 index
        assert_eq!(TestEnum::Two.value(), "two");
        assert_eq!(TestEnum::Two.index(), 2);

        // 测试属性顺序不同的情况
        assert_eq!(TestEnum::Three2.value(), "three2");
        assert_eq!(TestEnum::Three2.index(), 3);
    }

    #[test]
    fn test_edge_cases() {
        // 测试空字符串作为 value
        assert_eq!(TestEnum::EmptyValue.value(), "");

        // 测试 i8 的最大值作为 index
        assert_eq!(TestEnum::MaxI8Index.index(), 127);
    }

    #[test]
    fn test_exhaustive_matching() {
        // 确保所有枚举变体都被处理
        fn match_test(e: TestEnum) -> (&'static str, i8) {
            match e {
                TestEnum::One => ("one", 111),
                TestEnum::Two => ("two", 2),
                TestEnum::Three => ("测试", 0),
                TestEnum::Three2 => ("three2", 3),
                TestEnum::Next => ("Next", 0),
                TestEnum::Next2 => ("Next2", 118),
                TestEnum::EmptyValue => ("", 0),
                TestEnum::MaxI8Index => ("MaxI8Index", 127),
            }
        }

        assert_eq!(
            match_test(TestEnum::One),
            (TestEnum::One.value(), TestEnum::One.index())
        );
        assert_eq!(
            match_test(TestEnum::MaxI8Index),
            (TestEnum::MaxI8Index.value(), TestEnum::MaxI8Index.index())
        );
    }

    #[test]
    fn test_underlying_representation() {
        assert_eq!(TestEnum::One as u8, 0);
        assert_eq!(TestEnum::Two as u8, 1);
        assert_eq!(TestEnum::Three as u8, 2);
        assert_eq!(TestEnum::Three2 as u8, 3);
        assert_eq!(TestEnum::Next as u8, 4);
        assert_eq!(TestEnum::Next2 as u8, 5);
        assert_eq!(TestEnum::EmptyValue as u8, 6);
        assert_eq!(TestEnum::MaxI8Index as u8, 7);
    }

    #[test]
    fn test_display_impl() {
        assert_eq!(format!("{}", TestEnum::One), "one");
        assert_eq!(format!("{}", TestEnum::Two), "two");
        assert_eq!(format!("{}", TestEnum::Three), "测试");
        assert_eq!(format!("{}", TestEnum::Three2), "three2");
        assert_eq!(format!("{}", TestEnum::Next), "Next");
        assert_eq!(format!("{}", TestEnum::Next2), "Next2");
        assert_eq!(format!("{}", TestEnum::EmptyValue), "");
        assert_eq!(format!("{}", TestEnum::MaxI8Index), "MaxI8Index");
    }

    #[test]
    fn test_no_display_impl_for_no_debug() {
        #[allow(dead_code)]
        fn assert_not_display<T: Sized>()
        where
            T: fmt::Display,
        {
        }
        // 这行会导致编译错误，如果 TestEnumNoDebug 实现了 Display
        // assert_not_display::<TestEnumNoDebug>();
    }

    #[test]
    fn test_enum_no_debug_repr() {
        // 确保枚举使用 i128 表示
        assert_eq!(std::mem::size_of::<TestEnumNoDebug>(), 4);
        assert_eq!(std::mem::align_of::<TestEnumNoDebug>(), 4);
    }

    #[test]
    fn test_enum_repr() {
        // TestEnumNoDebug (repr(C))
        assert_eq!(
            std::mem::size_of::<TestEnumNoDebug>(),
            std::mem::size_of::<u32>()
        );
        assert_eq!(
            std::mem::align_of::<TestEnumNoDebug>(),
            std::mem::align_of::<u32>()
        );

        // TestEnumI8
        assert_eq!(std::mem::size_of::<TestEnumI8>(), 1);
        assert_eq!(std::mem::align_of::<TestEnumI8>(), 1);

        // TestEnumU8
        assert_eq!(std::mem::size_of::<TestEnumU8>(), 1);
        assert_eq!(std::mem::align_of::<TestEnumU8>(), 1);

        // TestEnumI16
        assert_eq!(std::mem::size_of::<TestEnumI16>(), 2);
        assert_eq!(std::mem::align_of::<TestEnumI16>(), 2);

        // TestEnumU16
        assert_eq!(std::mem::size_of::<TestEnumU16>(), 2);
        assert_eq!(std::mem::align_of::<TestEnumU16>(), 2);

        // TestEnumI32
        assert_eq!(std::mem::size_of::<TestEnumI32>(), 4);
        assert_eq!(std::mem::align_of::<TestEnumI32>(), 4);

        // TestEnumU32
        assert_eq!(std::mem::size_of::<TestEnumU32>(), 4);
        assert_eq!(std::mem::align_of::<TestEnumU32>(), 4);

        // TestEnumI64
        assert_eq!(std::mem::size_of::<TestEnumI64>(), 8);
        assert_eq!(std::mem::align_of::<TestEnumI64>(), 8);

        // TestEnumU64
        assert_eq!(std::mem::size_of::<TestEnumU64>(), 8);
        assert_eq!(std::mem::align_of::<TestEnumU64>(), 8);

        // TestEnumIsize
        assert_eq!(
            std::mem::size_of::<TestEnumIsize>(),
            std::mem::size_of::<isize>()
        );
        assert_eq!(
            std::mem::align_of::<TestEnumIsize>(),
            std::mem::align_of::<isize>()
        );

        // TestEnumUsize
        assert_eq!(
            std::mem::size_of::<TestEnumUsize>(),
            std::mem::size_of::<usize>()
        );
        assert_eq!(
            std::mem::align_of::<TestEnumUsize>(),
            std::mem::align_of::<usize>()
        );
    }
}

#[value]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[repr(i8)]
enum TestEnum {
    #[e(value = "one")]
    #[e(index = 111)]
    One,
    #[e(value = "two", index = 2)]
    Two,
    #[e(value = "测试")]
    Three,
    #[e(index = 3, value = "three2")]
    Three2,
    Next,
    #[e(index = 118)]
    Next2,
    #[e(value = "")]
    EmptyValue,
    #[e(index = 127)]
    MaxI8Index,
}


#[value]
#[derive(PartialEq, Eq, Clone, Copy, Hash)]
#[repr(C)]
enum TestEnumNoDebug {
    #[allow(unused)]
    One,
}

#[value]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[repr(i8)]
enum TestEnumI8 {
    One,
    Two,
}

#[value]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[repr(u8)]
enum TestEnumU8 {
    One,
    Two,
}

#[value]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[repr(i16)]
enum TestEnumI16 {
    One,
    Two,
}

#[value]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[repr(u16)]
enum TestEnumU16 {
    One,
    Two,
}

#[value]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[repr(i32)]
enum TestEnumI32 {
    One,
    Two,
}

#[value]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[repr(u32)]
enum TestEnumU32 {
    One,
    Two,
}

#[value]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[repr(i64)]
enum TestEnumI64 {
    One,
    Two,
}

#[value]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[repr(u64)]
enum TestEnumU64 {
    One,
    Two,
}

#[value]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[repr(isize)]
enum TestEnumIsize {
    One,
    Two,
}

#[value]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[repr(usize)]
enum TestEnumUsize {
    One,
    Two,
}
