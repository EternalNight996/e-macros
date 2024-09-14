use serde::{Deserialize, Serialize};

#[e_macros::value]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TestEnumData {
    #[serde(rename = "t")]
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_enum_data_serde() {
        let v1 = TestEnumData::V1;
        let v2 = TestEnumData::V2("测试".to_string());
        let v3 = TestEnumData::V3(true, 3.14);
        let v4 = TestEnumData::V4 { x: 1, y: 2 };
        let v5 = TestEnumData::V5(json!({"key": "value"}));

        assert_eq!(v1.to_serde().unwrap(), r#""t""#);
        assert_eq!(v2.to_serde().unwrap(), r#"{"V2":"测试"}"#);
        assert_eq!(v3.to_serde().unwrap(), r#"{"V3":[true,3.14]}"#);
        assert_eq!(v4.to_serde().unwrap(), r#"{"V4":{"x":1,"y":2}}"#);
        assert_eq!(v5.to_serde().unwrap(), r#"{"V5":{"key":"value"}}"#);

        assert_eq!(TestEnumData::from_serde(json!("t")).unwrap(), v1);
        assert_eq!(TestEnumData::from_serde(json!({"V2": "测试"})).unwrap(), v2);
        assert_eq!(TestEnumData::from_serde(json!({"V3": [true, 3.14]})).unwrap(), v3);
        assert_eq!(TestEnumData::from_serde(json!({"V4": {"x": 1, "y": 2}})).unwrap(), v4);
        assert_eq!(TestEnumData::from_serde(json!({"V5": {"key": "value"}})).unwrap(), v5);
    }

    #[test]
    fn test_enum_serde() {
        let up = TestEnum::Up;
        let down = TestEnum::Down;
        let data = TestEnum::Data { data: TestEnumData::V1 };
        let next = TestEnum::Next(Box::new(TestEnum::Up));

        // 基本测试
        assert_eq!(up.to_serde().unwrap(), r#""Up""#);
        assert_eq!(down.to_serde().unwrap(), r#""Down""#);
        assert_eq!(data.to_serde().unwrap(), r#"{"Data":{"data":"t"}}"#);
        assert_eq!(next.to_serde().unwrap(), r#"{"Next":"Up"}"#);

        assert_eq!(TestEnum::from_serde(json!("Up")).unwrap(), up);
        assert_eq!(TestEnum::from_serde(json!("Down")).unwrap(), down);
        assert_eq!(TestEnum::from_serde(json!({"Data": {"data": "t"}})).unwrap(), data);
        assert_eq!(TestEnum::from_serde(json!({"Next": "Up"})).unwrap(), next);

        // 链接测试
        let nested_next = TestEnum::Next(Box::new(TestEnum::Next(Box::new(TestEnum::Down))));
        let expected_json = r#"{"Next":{"Next":"Down"}}"#;
        assert_eq!(nested_next.to_serde().unwrap(), expected_json);

        let deserialized = TestEnum::from_serde(json!({"Next": {"Next": "Down"}})).unwrap();
        assert_eq!(deserialized, nested_next);

        // 更复杂的链接测试
        let complex_enum = TestEnum::Next(Box::new(TestEnum::Data {
            data: TestEnumData::V3(true, 3.14)
        }));
        let complex_json = r#"{"Next":{"Data":{"data":{"V3":[true,3.14]}}}}"#;
        assert_eq!(complex_enum.to_serde().unwrap(), complex_json);

        let deserialized_complex = TestEnum::from_serde(json!( {
            "Next": {
                "Data": {
                    "data": {
                        "V3": [true, 3.14]
                    }
                }
            }
        })).unwrap();
        assert_eq!(deserialized_complex, complex_enum);
    }

    #[test]
    fn test_enum1_serde() {
        let up = TestEnum1::Up;
        let down = TestEnum1::Down;
        let c = TestEnum1::C(42);

        assert_eq!(up.to_serde().unwrap(), r#""Up""#);
        assert_eq!(down.to_serde().unwrap(), r#""Down""#);
        assert_eq!(c.to_serde().unwrap(), r#"{"C":42}"#);

        assert_eq!(TestEnum1::from_serde(json!("Up")).unwrap(), up);
        assert_eq!(TestEnum1::from_serde(json!("Down")).unwrap(), down);
        assert_eq!(TestEnum1::from_serde(json!({"C": 42})).unwrap(), c);
    }
}
