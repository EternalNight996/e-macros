use e_macros::value;

#[value]
#[derive(PartialEq, Eq, Clone, Copy, Hash)]
enum TestEnumNoDebug {
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

macro_rules! test_repr_limits {
    ($name:ident, $type:ty) => {
        #[test]
        fn $name() {
            #[value]
            #[repr($type)]
            enum TestEnum {
                Min,
                Max,
                Other,
            }

            assert_eq!(TestEnum::Min.index(), 1);
            assert_eq!(TestEnum::Max.index(), 2);
            assert_eq!(TestEnum::Other.index(), 3);
        }
    };
}
macro_rules! test_repr_edge_cases_large {
    ($name:ident, $type:ty, $max:expr) => {
        #[test]
        fn $name() {
            #[value]
            #[repr($type)]
            enum TestEnum {
                Min = 1,
                Max = $max,
            }

            assert_eq!(TestEnum::Min.index(), 1);
            assert!(TestEnum::Max.index() > 1);
            assert!(TestEnum::Max.index() <= $max);
        }
    };
}

macro_rules! test_repr_edge_cases {
    ($name:ident, $type:ty, $max:expr) => {
        #[test]
        fn $name() {
            #[value]
            #[repr($type)]
            enum TestEnum {
                Min = 1,
                Max = $max,
            }

            assert_eq!(TestEnum::Min.index(), 1);
            assert!(TestEnum::Max.index() > 1);
        }
    };
}
#[cfg(test)]
mod tests {
    use super::*;
    
    test_repr_edge_cases!(test_repr_i8_edge, i8, 127);
    test_repr_edge_cases!(test_repr_u8_edge, u8, 255);
    test_repr_edge_cases!(test_repr_i16_edge, i16, 32767);
    test_repr_edge_cases!(test_repr_u16_edge, u16, 65535);
    test_repr_edge_cases!(test_repr_i32_edge, i32, 2147483647);
    test_repr_edge_cases!(test_repr_u32_edge, u32, 4294967295);

    test_repr_edge_cases_large!(test_repr_i64_edge, i64, i64::MAX);
    test_repr_edge_cases_large!(test_repr_u64_edge, u64, u64::MAX);
    test_repr_edge_cases_large!(test_repr_isize_edge, isize, isize::MAX);
    test_repr_edge_cases_large!(test_repr_usize_edge, usize, usize::MAX);
    
    #[test]
    fn test_repr_isize_limits() {
        #[value]
        #[repr(isize)]
        enum TestEnum {
            Min,
            Max,
            Other,
        }

        assert_eq!(TestEnum::Min.index(), 1);
        assert_eq!(TestEnum::Max.index(), 2);
        assert_eq!(TestEnum::Other.index(), 3);
    }

    #[test]
    fn test_repr_usize_limits() {
        #[value]
        #[repr(usize)]
        enum TestEnum {
            Min,
            Max,
            Other,
        }

        assert_eq!(TestEnum::Min.index(), 1);
        assert_eq!(TestEnum::Max.index(), 2);
        assert_eq!(TestEnum::Other.index(), 3);
    }

    test_repr_limits!(test_repr_i8_limits, i8);
    test_repr_limits!(test_repr_u8_limits, u8);
    test_repr_limits!(test_repr_i16_limits, i16);
    test_repr_limits!(test_repr_u16_limits, u16);
    test_repr_limits!(test_repr_i32_limits, i32);
    test_repr_limits!(test_repr_u32_limits, u32);
    test_repr_limits!(test_repr_i64_limits, i64);
    test_repr_limits!(test_repr_u64_limits, u64);

    #[test]
    fn test_no_debug() {
        let _ = TestEnumNoDebug::One;
        // 如果这个编译通过，说明没有 Debug 实现也可以
    }
    #[test]
    fn test_variant_count() {
        assert_eq!(TestEnumNoDebug::variant_count(), 1);
        assert_eq!(TestEnumI8::variant_count(), 2);
        assert_eq!(TestEnumU8::variant_count(), 2);
        assert_eq!(TestEnumI16::variant_count(), 2);
        assert_eq!(TestEnumU16::variant_count(), 2);
        assert_eq!(TestEnumI32::variant_count(), 2);
        assert_eq!(TestEnumU32::variant_count(), 2);
        assert_eq!(TestEnumI64::variant_count(), 2);
        assert_eq!(TestEnumU64::variant_count(), 2);
        assert_eq!(TestEnumIsize::variant_count(), 2);
        assert_eq!(TestEnumUsize::variant_count(), 2);
    }

    #[test]
    fn test_different_repr_types() {
        assert_eq!(TestEnumI8::One.index(), 1i8);
        assert_eq!(TestEnumU8::One.index(), 1u8);
        assert_eq!(TestEnumI16::One.index(), 1i16);
        assert_eq!(TestEnumU16::One.index(), 1u16);
        assert_eq!(TestEnumI32::One.index(), 1i32);
        assert_eq!(TestEnumU32::One.index(), 1u32);
        assert_eq!(TestEnumI64::One.index(), 1i64);
        assert_eq!(TestEnumU64::One.index(), 1u64);
        assert_eq!(TestEnumIsize::One.index(), 1isize);
        assert_eq!(TestEnumUsize::One.index(), 1usize);
    }
}
