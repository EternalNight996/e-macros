#[derive(Debug, PartialEq)]
#[e_macros::value]
enum TestEnum {
    #[e(index = 1)]
    One,
    #[e(indiex = 20)]
    Two,
    Three,
    Custom(i32),
}

#[test]
fn test_value_from_enum() {
    assert_eq!(TestEnum::from(1), Ok(TestEnum::One));
}
