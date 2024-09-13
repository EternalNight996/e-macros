#[derive(Debug, PartialEq, Default)]
#[e_macros::value]
enum TestEnum {
    #[default]
    #[e(value = "test", index = 1)]
    One,
    #[e(index = 20)]
    Two,
    Three,
    Custom(i32),
}

#[test]
fn test_value_from_enum() {
    assert_eq!(TestEnum::from(1), Ok(TestEnum::One));
    assert_eq!(TestEnum::from(20), Ok(TestEnum::Two));
    assert_eq!(TestEnum::from(3), Ok(TestEnum::Three));
    assert_eq!(TestEnum::from(3), Ok(TestEnum::Custom(0)));
    assert_eq!(TestEnum::from(0), Err("Invalid value"));
    assert_eq!(TestEnum::from(100), Err("Invalid value"));
}

#[test]
fn test_enum_to_value() {
    assert_eq!(TestEnum::One.value(), "One");
    assert_eq!(TestEnum::Two.value(), "Two");
    assert_eq!(TestEnum::Three.value(), "Three");
    assert_eq!(TestEnum::Custom(42).value(), "Custom");
}

#[test]
fn test_enum_to_index() {
    assert_eq!(TestEnum::One.index(), 1);
    assert_eq!(TestEnum::Two.index(), 20);
    assert_eq!(TestEnum::Three.index(), 2);
    assert_eq!(TestEnum::Custom(42).index(), 3);
}

// #[test]
// fn test_variant_count() {
//     assert_eq!(TestEnum::varia nt_count(), 4);
// }

#[test]
fn test_display() {
    assert_eq!(format!("{:?}", TestEnum::One), "One");
    // assert_eq!(format!("{}", TestEnum::Two), "Two");
    assert_eq!(format!("{}", TestEnum::Three), "Three");
    // assert_eq!(format!("{}", TestEnum::Custom(42)), "Custom");
}
