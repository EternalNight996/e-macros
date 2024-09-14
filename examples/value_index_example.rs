use e_macros::value;

#[value]
#[derive(Debug, PartialEq)]
enum Color {
    #[e(value = "RED", index = 0)]
    Red,
    #[e(value = "GREEN", index = 1)]
    Green,
    #[e(value = "BLUE", index = 2)]
    Blue,
}

fn main() {
    let color = Color::Green;

    println!("Color value: {}", color.value());
    println!("Color index: {}", color.index());

    let from_value = Color::try_from("BLUE").unwrap();
    println!("From value: {:?}", from_value);

    let from_index = Color::try_from(0).unwrap();
    println!("From index: {:?}", from_index);

    println!("Variant count: {}", Color::variant_count());
}