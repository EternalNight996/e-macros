use serde::{Deserialize, Serialize};
use serde_json::json;

#[e_macros::value]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum TestEnumData {
    #[default]
    #[serde(rename = "name")]
    #[e(value = "一", index = 10)]
    V1,
    V2(String),
    V3(bool, f64),
    V4 {
        x: u8,
        y: u8,
    },
    V5(serde_json::Value),
}

#[e_macros::value]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum TestEnum {
    #[default]
    #[e(value = "上层")]
    Up,
    #[e(value = "下层")]
    Down,
    Data {
        data: TestEnumData,
    },
    Next(Box<TestEnum>),
}
