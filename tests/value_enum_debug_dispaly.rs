#[e_macros::value]
#[derive(Debug, PartialEq, Default)]
#[repr(u8)]
enum TestEnum {
    #[default]
    #[e(value = "test", index = 1)]
    Nil,
    #[e(index = 20)]
    Cons { value: i32, next: Box<TestEnum> },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_debug_and_display() {
        let list = TestEnum::Cons {
            value: 1,
            next: Box::new(TestEnum::Cons {
                value: 2,
                next: Box::new(TestEnum::Nil),
            }),
        };

        // 测试 Debug
        assert_eq!(
            format!("{:?}", list),
            "Cons { value: 1, next: Cons { value: 2, next: Nil } }"
        );

        // 测试 Display
        assert_eq!(format!("{}", list), "Cons");

        // 测试 Nil 的 Debug 和 Display
        assert_eq!(format!("{:?}", TestEnum::Nil), "Nil");
        assert_eq!(format!("{}", TestEnum::Nil), "test");
    }
}
