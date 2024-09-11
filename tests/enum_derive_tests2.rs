// #[derive(e_macros::Enum, Debug, Clone, Default)]
// pub enum TestEnum {
//     #[default]
//     V1,
//     V2(String),
//     // V3(bool, f64),
//     // V4 {
//     //     x: u8,
//     //     y: u8,
//     // },
// }
// use std::convert::TryFrom;
// use std::str::FromStr;

// #[test]
// fn test_enum_functionality() {
//     let variants = [
//         (TestEnum::V1(42), "测试1", 0),
//         (TestEnum::V2("Hello".to_string()), "测试2", 1),
//         (TestEnum::V3(true, 3.14), "V3", 2),
//         (TestEnum::V4 { x: 1, y: 2 }, "V4", 3),
//     ];

//     for (variant, name, index) in variants.iter() {
//         // 测试 as_str
//         assert_eq!(variant.as_str(), *name);

//         // 测试 Display
//         assert_eq!(format!("{}", variant), *name);

//         // 测试 TryFrom<i32>
//         assert_eq!(TestEnum::try_from(*index).unwrap().as_str(), *name);

//         // 测试 Into<i32>
//         assert_eq!(Into::<i32>::into(variant.clone()), *index);
//     }

//     // 测试 FromStr 和 TryFrom<&str> 的特定情况
//     assert_eq!(TestEnum::from_str("测试1").unwrap().as_str(), "测试1");
//     assert_eq!(TestEnum::try_from("测试2").unwrap().as_str(), "测试2");

//     // 测试无效情况
//     assert!(TestEnum::from_str("Invalid").is_err());
//     assert!(TestEnum::try_from(100).is_err());
// }

// #[test]
// fn test_default() {
//     assert_eq!(TestEnum::default().as_str(), "V4");
// }

// #[test]
// fn test_variants() {
//     assert_eq!(TestEnum::ALL.len(), 4);
//     assert_eq!(
//         TestEnum::ALL.iter().map(|v| v.as_str()).collect::<Vec<_>>(),
//         vec!["测试1", "测试2", "V3", "V4"]
//     );
// }

// #[cfg(feature = "serde")]
// mod serde_tests {
//     use super::*;
//     use serde_json;

//     #[test]
//     fn test_serde() {
//         let variants = [
//             (TestEnum::V1(42), "\"测试1\""),
//             (TestEnum::V2("Hello".to_string()), "\"测试2\""),
//             (TestEnum::V3(true, 3.14), "\"V3\""),
//             (TestEnum::V4 { x: 1, y: 2 }, "\"V4\""),
//         ];

//         for (variant, json) in variants.iter() {
//             // 测试序列化
//             assert_eq!(serde_json::to_string(variant).unwrap(), *json);
//             // 测试反序列化
//             assert_eq!(serde_json::from_str::<TestEnum>(json).unwrap().as_str(), variant.as_str());
//         }

//         // 测试无效情况
//         assert!(serde_json::from_str::<TestEnum>("\"Invalid\"").is_err());
//         assert!(serde_json::from_str::<TestEnum>("42").is_err());
//         assert!(serde_json::from_str::<TestEnum>("null").is_err());
//     }
// }
