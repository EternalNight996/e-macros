<img src="public/ico/white_64x64.ico" alt="e-macros"/>

### 📄 [中文](docs/README.zh.md)  | 📄  [English](README.md)

[![Test Status](https://github.com/rust-random/rand/workflows/Tests/badge.svg?event=push)](https://github.com/eternalnight996/e-macros/actions) [![Book](https://img.shields.io/badge/book-master-yellow.svg)](https://doc.rust-lang.org/book/) [![API](https://img.shields.io/badge/api-master-yellow.svg)](https://github.com/eternalnight996/e-macros) [![API](https://docs.rs/e-macros/badge.svg)](https://docs.rs/rand)
## ⚡ What this is?
**Rust macros to simplify and accelerate enum handling: effortless conversion, fast indexing, and painless serialization**

### 🛠️ Support Features
<table style="background:#000">
  <tr>
    <th><h3 style="color:#fff">Feature</h3></th>
    <th><h3 style="color:#fff">Windows 10</h3></th>
    <th><h3 style="color:#fff">Unix</h3></th>
    <th><h3 style="color:#fff">macOS</h3></th>
    <th><h3 style="color:#fff">Description</h3></th>
  </tr>
  <tr>
    <td><span style="color:#ccc">Enum</span></td>
    <td><h4 style="color:green">✓</h4></td>
    <td><h4 style="color:green">✓</h4></td>
    <td><h4 style="color:green">✓</h4></td>
    <td><span style="color:#ccc">Efficient enum operations including conversion, indexing, and counting</span></td>
  </tr>
</table>



# 📖 Example
```toml
[dependencies]
e-macros = "0.2"
```
#### 🔢 Base Exmaple
```rust
#[e_macros::value]
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
```

####  🔢 about serde exmaple
```rust
use e_macros::value;
use serde::{Serialize, Deserialize};

#[value]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
enum ApiStatus {
    #[e(value = "OK", index = 200)]
    Ok,
    #[e(value = "NOT_FOUND", index = 404)]
    NotFound(String),
    #[e(value = "SERVER_ERROR", index = 500)]
    ServerError { message: String },
}

fn main() {
    let status = ApiStatus::NotFound("Resource not available".to_string());

    // Standard serialization
    let json = serde_json::to_string(&status).unwrap();
    println!("Standard serialized: {}", json);

    // Standard deserialization
    let deserialized: ApiStatus = serde_json::from_str(&json).unwrap();
    println!("Standard deserialized: {:?}", deserialized);

    // Custom serialization
    let custom_json = status.to_serde().unwrap();
    println!("Custom serialized: {}", custom_json);

    // Custom deserialization
    let custom_deserialized = ApiStatus::from_serde(serde_json::json!({
        "ServerError": { "message": "Internal server error" }
    })).unwrap();
    println!("Custom deserialized: {:?}", custom_deserialized);
}
```

#### 🔢 about debug and display exmaple
```rust
// Define the LinkedList enum
#[e_macros::value]
#[derive(Debug, PartialEq)]
enum LinkedList {
    #[e(value = "cons")]
    Cons(i32, Box<LinkedList>),
    #[e(value = "nil")]
    Nil,
}

fn main() {
    // Create a linked list instance
    let list = LinkedList::Cons(
        1,
        Box::new(LinkedList::Cons(
            2,
            Box::new(LinkedList::Cons(3, Box::new(LinkedList::Nil))),
        )),
    );
    // Print different formats of the linked list
    println!("LinkedList Debug: {:?}", list);
    println!("LinkedList Display: {}", list);
    println!("LinkedList Pretty Debug: {:#?}", list);
}
```

#### 🔢 About repr limit example
```rust
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
```

# ✨ Features
```toml
```

## `💡!important：`
```text
```

# 🚀 fast running
```sh
# Donwloading the object
git clone https://github.com/eternalnight996/e-macros
cd e-macros
# test all object support
cargo test
# The benchmark results will help you understand the performance characteristics of e-macros in different scenarios.
cargo bench
```


# 📊 Performance Benchmarks

Here are the performance benchmark results for `e-macros`:

| Method | Average Execution Time |
|--------|------------------------|
| `TestEnum::to_string()` | 179.07 ns |
| `TestEnum::try_from()` (from string) | 3.0561 ns |
| `TestEnum::index()` | 1.3604 ns |
| `TestEnum::from()` | 10.437 ns |
| `TestEnum::value()` | 1.7382 ns |
| `TestEnum::try_from()` (from value) | 3.0647 ns |
| `TestEnum::variant_count()` | 217.48 ps |

These test results indicate:

- Most methods are very fast, completing in nanoseconds.
- The `to_string()` method is relatively slower, which is expected as it involves string creation.
- `variant_count()` is the fastest method, taking only 217.48 picoseconds.
- Other methods like `index()`, `value()`, and `try_from()` are highly efficient, ranging from 1 to 3 nanoseconds.

These results demonstrate that the enum methods generated by `e-macros` are highly efficient and suitable for use in performance-sensitive scenarios.

> Note: These tests were conducted on specific hardware and environment. Actual performance may vary depending on the system.

# 🦊 Applied Projects
- **Project One**: Description of Project One's features and use cases.
- **Project Two**: Description of Project Two's features and use cases.
- **Project Three**: Description of Project Three's features and use cases.

---

## 🔭 Why Do You Need This Library?

`e-macros` aims to simplify the handling of enums in Rust by automatically generating commonly used methods through macros, reducing the amount of manual coding required and increasing development efficiency. Additionally, its optimized performance makes it an excellent choice for applications with high-performance requirements.

---

# 🙋 Reference items and materials
- [Rust Official Documentation](https://www.rust-lang.org/documentation.html)
- [Serde Documentation](https://serde.rs/)
- [Cargo User Guide](https://doc.rust-lang.org/cargo/)
- [e-macros Repository](https://github.com/eternalnight996/e-macros)

# 📖 License

Rand is distributed under the terms of both the MIT license and the
Apache License (Version 2.0).

See [LICENSE-MIT](LICENSE-MIT), and
[COPYRIGHT](COPYRIGHT) for details.
