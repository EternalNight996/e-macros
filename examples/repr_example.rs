#[e_macros::value]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[repr(i8)]
pub enum TestEnumI8 {
    #[e(index = -128)]
    One,
    Two,
    Three,
    #[e(index = 126)]
    N1,
    N2,
    N3 = 100
}

fn main() {
    // Print the i8 value of each enum variant
    println!("TestEnumI8::One as i8: {}", TestEnumI8::One as i8);
    println!("TestEnumI8::Two as i8: {}", TestEnumI8::Two as i8);
    println!("TestEnumI8::Three as i8: {}", TestEnumI8::Three as i8);
    println!("TestEnumI8::N1 as i8: {}", TestEnumI8::N1 as i8);
    println!("TestEnumI8::N2 as i8: {}", TestEnumI8::N2 as i8);
    println!("TestEnumI8::N3 as i8: {}", TestEnumI8::N3 as i8);

    // Use the index() method to get the index of enum variants
    println!("\nUsing index() method:");
    println!("TestEnumI8::One.index(): {}", TestEnumI8::One.index());
    println!("TestEnumI8::Two.index(): {}", TestEnumI8::Two.index());
    println!("TestEnumI8::Three.index(): {}", TestEnumI8::Three.index());
    println!("TestEnumI8::N1.index(): {}", TestEnumI8::N1.index());
    println!("TestEnumI8::N2.index(): {}", TestEnumI8::N2.index());
    println!("TestEnumI8::N3.index(): {}", TestEnumI8::N3.index());
}