#[e_macros::value]
enum Foo {
    A,
}

#[e_macros::value]
#[repr(i64)]
enum Foo2 {
    A,
    B {},
    C(i32),
}

#[e_macros::value]
enum Foo4 {
    A,
}


#[e_macros::value]
enum Foo6 {
    A = 1,
    B,
    C,
}

#[e_macros::value]
enum Foo7 {
    A,
    B = 1000,
    C,
}

#[test]
fn t1() {
    #[e_macros::value]
    #[derive(Debug)]
    enum Foo10 {
        #[value("???")]
        A
    }
}
