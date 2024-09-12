#[e_macros::value]
#[derive(Debug, PartialEq, Eq, Default)]
#[repr(i128)]
enum Foo {
    A = 10,
    B = -200,
    #[value("vC")]
    C = 30,
    D(i32)
}

#[test]
fn test_value_method() {
    assert_eq!(Foo::A.index(), 10);
    assert_eq!(Foo::B.index(), -200);
    assert_eq!(Foo::C.index(), 30);
}

#[test]
fn test_new_method() {
    let foo = Foo::new(10);
    assert_eq!(foo, Foo::A);
    let foo = Foo::new(-200);
    assert_eq!(foo, Foo::B);
    let foo = Foo::new(30);
    assert_eq!(foo, Foo::C);
}

#[test]
fn test_from_method() {
    let foo: Foo = 10.into();
    assert_eq!(foo, Foo::A);
    let foo: Foo = (-200).into();
    assert_eq!(foo, Foo::B);
    let foo: Foo = 30.into();
    assert_eq!(foo, Foo::C);
}

#[test]
fn test_default_impl() {
    let default_foo = Foo::default();
    assert_eq!(default_foo, Foo::A); // Assuming Foo::A is the default
}

#[test]
fn test_debug_output() {
    let debug_a = format!("{:?}", Foo::A);
    let debug_b = format!("{:?}", Foo::B);
    let debug_c = format!("{:?}", Foo::C);
    assert_eq!(debug_a, "A");
    assert_eq!(debug_b, "B");
    assert_eq!(debug_c, "C");
}

// 新增测试用例
#[test]
fn test_value_string() {
    assert_eq!(Foo::A.value(), "A");
    assert_eq!(Foo::B.value(), "B");
    assert_eq!(Foo::C.value(), "vC");
}
