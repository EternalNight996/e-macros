<img src="../public/ico/white_64x64.ico" alt="e-macros"/>

### 📄 [中文](README.zh.md)  | 📄  [English](../README.md)
[![Test Status](https://github.com/rust-random/rand/workflows/Tests/badge.svg?event=push)](https://github.com/eternalnight996/e-macros/actions) [![Book](https://img.shields.io/badge/book-master-yellow.svg)](https://doc.rust-lang.org/book/) [![API](https://img.shields.io/badge/api-master-yellow.svg)](https://github.com/eternalnight996/e-macros) [![API](https://docs.rs/e-macros/badge.svg)](https://docs.rs/rand)
# ⚡ 这是什么?
**Rust 宏简化并加速枚举处理：轻松转换、快速索引和无痛序列化**

### 🛠️ 支持功能
<table style="background:#000">
  <tr>
    <th><h3 style="color:#fff">功能</h3></th>
    <th><h3 style="color:#fff">Windows 10</h3></th>
    <th><h3 style="color:#fff">Unix</h3></th>
    <th><h3 style="color:#fff">macOS</h3></th>
    <th><h3 style="color:#fff">描述</h3></th>
  </tr>
  <tr>
    <td><span style="color:#ccc">Enum</span></td>
    <td><h4 style="color:green">✓</h4></td>
    <td><h4 style="color:green">✓</h4></td>
    <td><h4 style="color:green">✓</h4></td>
    <td><span style="color:#ccc">高效的枚举操作，包括转换、索引和计数</span></td>
  </tr>
</table>

# ✨ 分支
```toml
```

# 📖 示例
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


## `💡!重要：`
```text
```

# 🚀 快速运行
```sh
#下载对象
git clone https://github.com/eternalnight996/e-macros
cd e-macros
#测试所有对象支持
cargo test
#基准测试结果将帮助您了解 e-macros 在不同场景下的性能特征。
cargo bench
```
---

## 📊 性能基准

以下是 `e-macros` 的性能基准结果：

| 方法 | 平均执行时间 |
|------|--------------|
| `TestEnum::to_string()` | 179.07 ns |
| `TestEnum::try_from()` (从字符串) | 3.0561 ns |
| `TestEnum::index()` | 1.3604 ns |
| `TestEnum::from()` | 10.437 ns |
| `TestEnum::value()` | 1.7382 ns |
| `TestEnum::try_from()` (从值) | 3.0647 ns |
| `TestEnum::variant_count()` | 217.48 ps |

这些测试结果表明：

- 大多数方法非常快，均在纳秒级完成。
- `to_string()` 方法相对较慢，这是因为涉及字符串创建。
- `variant_count()` 是最快的方法，仅需 217.48 皮秒。
- 其他方法如 `index()`、`value()` 和 `try_from()` 都极为高效，范围在 1 到 3 纳秒之间。

这些结果表明，`e-macros` 生成的枚举方法具有高效性，适用于对性能敏感的场景。

> 注意：这些测试是在特定硬件和环境下进行的。实际性能可能因系统不同而有所变化。

---

## 🦊 已运用项目
- **项目一**：描述项目一的功能和使用场景。
- **项目二**：描述项目二的功能和使用场景。
- **项目三**：描述项目三的功能和使用场景。

## 🔭 为什么需要这个库？
`e-macros` 旨在简化 Rust 枚举的处理过程，通过宏自动生成常用方法，减少手动编码工作量，提高开发效率。此外，其优化的性能使其成为高性能需求应用的绝佳选择。

---

## 🙋 参考项目与资料
- [Rust 官方文档](https://www.rust-lang.org/documentation.html)
- [Serde 文档](https://serde.rs/)
- [Cargo 用户指南](https://doc.rust-lang.org/cargo/)
- [e-macros 仓库](https://github.com/eternalnight996/e-macros)


# 📖 License协议


Rand 根据 MIT 许可证 的条款分发。

See [LICENSE-MIT](../LICENSE-MIT), and
[COPYRIGHT](../COPYRIGHT) for details.