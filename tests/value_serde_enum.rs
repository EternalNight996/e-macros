// use serde::{Deserialize, Serialize};
// #[e_macros::value]
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub enum TestEnumData {
//     #[serde(rename = "name")]
//     V1,
//     V2(String),
//     V3(bool, f64),
//     V4 { x: u8, y: u8 },
//     V5(serde_json::Value),
// }

// #[e_macros::value]
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub enum TestEnum {
//     #[e(value = "上层")]
//     Up,
//     #[e(value = "下层")]
//     Down,
//     Data {
//         data: TestEnumData,
//     },
//     Next(Box<TestEnum>),
// }
