#[cfg(test)]
mod test {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_enum_value() {
        assert_eq!(TestEnumData::V1.value(), "一");
        assert_eq!(TestEnumData::V2("test".to_string()).value(), "V2");
        assert_eq!(TestEnumData::V3(true, 1.0).value(), "V3");
        assert_eq!(TestEnumData::V4 { x: 1, y: 2 }.value(), "V4");
        assert_eq!(TestEnumData::V5(json!({"key": "value"})).value(), "V5");

        assert_eq!(TestEnum::Up.value(), "上层");
        assert_eq!(TestEnum::Down.value(), "下层");
        assert_eq!(
            TestEnum::Data {
                data: TestEnumData::V1
            }
            .value(),
            "Data"
        );
        assert_eq!(TestEnum::Next(Box::new(TestEnum::Up)).value(), "Next");
    }

    #[test]
    fn test_enum_index() {
        assert_eq!(TestEnumData::V1.index(), 10);
        // 其他变体应该返回默认值，通常是0
        assert_eq!(TestEnumData::V2("test".to_string()).index(), 0);
    }

    #[test]
    fn test_enum_serialize() {
        let v1 = TestEnumData::V1;
        assert_eq!(serde_json::to_string(&v1).unwrap(), r#""name""#);

        let v2 = TestEnumData::V2("hello".to_string());
        assert_eq!(serde_json::to_string(&v2).unwrap(), r#"{"V2":"hello"}"#);

        let v3 = TestEnumData::V3(true, 3.14);
        assert_eq!(serde_json::to_string(&v3).unwrap(), r#"{"V3":[true,3.14]}"#);

        let v4 = TestEnumData::V4 { x: 1, y: 2 };
        assert_eq!(
            serde_json::to_string(&v4).unwrap(),
            r#"{"V4":{"x":1,"y":2}}"#
        );

        let v5 = TestEnumData::V5(json!({"key": "value"}));
        assert_eq!(
            serde_json::to_string(&v5).unwrap(),
            r#"{"V5":{"key":"value"}}"#
        );
    }

    #[test]
    fn test_enum_deserialize() {
        let v1: TestEnumData = serde_json::from_str(r#""name""#).unwrap();
        assert_eq!(v1, TestEnumData::V1);

        let v2: TestEnumData = serde_json::from_str(r#"{"V2":"hello"}"#).unwrap();
        assert_eq!(v2, TestEnumData::V2("hello".to_string()));

        let v3: TestEnumData = serde_json::from_str(r#"{"V3":[true,3.14]}"#).unwrap();
        assert_eq!(v3, TestEnumData::V3(true, 3.14));

        let v4: TestEnumData = serde_json::from_str(r#"{"V4":{"x":1,"y":2}}"#).unwrap();
        assert_eq!(v4, TestEnumData::V4 { x: 1, y: 2 });

        let v5: TestEnumData = serde_json::from_str(r#"{"V5":{"key":"value"}}"#).unwrap();
        assert_eq!(v5, TestEnumData::V5(json!({"key": "value"})));
    }

    #[test]
    fn test_enum_from_str() {
        // assert_eq!(TestEnum::from_str("上层").unwrap(), TestEnum::Up);
        // assert_eq!(TestEnum::from_str("下层").unwrap(), TestEnum::Down);

        // // 非单元变体应该返回错误
        // assert!(TestEnum::from_str("Data").is_err());
        // assert!(TestEnum::from_str("Next").is_err());

        // // 未知的变体也应该返回错误
        // assert!(TestEnum::from_str("Unknown").is_err());
    }

    #[test]
    fn test_test_enum_default() {
        assert_eq!(TestEnum::default(), TestEnum::Up);
    }
}

use serde::{Deserialize, Serialize};
use serde_json::json;

#[e_macros::value]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TestEnumData {
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
    Up = 0,
    #[e(value = "下层")]
    Down = 1,
    Data {
        data: TestEnumData,
    },
    Next(Box<TestEnum>),
}
#[e_macros::value]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum TestEnum1 {
    #[default]
    #[e(value = "上层")]
    Up,
    #[e(value = "下层")]
    Down,
    C(i32),
}
