use std::convert::TryFrom;

#[e_macros::value]
#[derive(Debug, PartialEq)]
enum Status {
    #[e(value = "OK", index = 200)]
    Ok,
    #[e(value = "NOT_FOUND", index = 404)]
    NotFound,
    #[e(value = "ERROR2", index = 500)]
    Error2(String),
}

fn main() {
    // From string
    let status_from_str = Status::try_from("NOT_FOUND").unwrap();
    println!("From string: {:?}", status_from_str);

    // From index
    let status_from_index = Status::try_from(500).is_ok();
    println!("From index: {:?}", status_from_index);

    // To string
    let status = Status::Error2("Server error".to_string());
    println!("To string: {}", status.to_string());

    // To index
    println!("To index: {}", status.index());
}