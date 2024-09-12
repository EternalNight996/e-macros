#[e_macros::value]
#[derive(Debug, PartialEq, Eq, Default)]
#[repr(u128)]
enum Foo {
    #[value("v1")]
    A = 10,
    // #[value("v2")]
    B = 20,
    C = 30
}

#[test]
fn test_debug_impl() {
    let a = format!("{:?}", Foo::A);
    let a1 = format!("{:?}", Foo::new(0));
    let unknown = format!("{:?}", Foo::from(1));
    println!("value -> {} A {:?}", Foo::A.index(), Foo::A);
    println!("{a:?}\n {a1}\n {unknown}");
    assert_eq!(a, "A");
    assert_eq!(a1, "Foo(0)");
    assert_eq!(unknown, "Foo(1)");
}

#[test]
fn test_value_method() {
    assert_eq!(Foo::A.index(), 10);
    assert_eq!(Foo::B.index(), 20);
    assert_eq!(Foo::C.index(), 30);
}

#[test]
fn test_new_method() {
    let foo = Foo::new(10);
    assert_eq!(foo, Foo::A);
    let foo = Foo::new(20);
    assert_eq!(foo, Foo::B);
    let foo = Foo::new(30);
    assert_eq!(foo, Foo::C);
}

#[test]
fn test_from_method() {
    let foo: Foo = 10.into();
    assert_eq!(foo, Foo::A);
    let foo: Foo = 20.into();
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
