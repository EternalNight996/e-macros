use e_macros::Enum;
use serde::{Deserialize, Serialize};

#[derive(Enum, Debug, Clone, Serialize, Deserialize)]
pub enum TestEnumData {
    V1,
    V2(String),
    V3(bool, f64),
    V4 { x: u8, y: u8 },
    V5(serde_json::Value),
}

#[derive(Enum, Debug, Clone, Serialize, Deserialize)]
pub enum TestEnum {
    #[ename("上层")]
    Up,
    #[ename("下层")]
    Down,
    Data {
        data: TestEnumData,
    },
    Next(Box<TestEnum>),
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{from_str, json, to_string};

    #[test]
    fn test_test_enum_data() {
        let v1 = TestEnumData::V1;
        let v2 = TestEnumData::V2("测试".to_string());
        let v3 = TestEnumData::V3(true, 3.14);
        let v4 = TestEnumData::V4 { x: 1, y: 2 };
        let v5 = TestEnumData::V5(json!({"key": "value"}));

        assert_eq!(format!("{:?}", v1), "V1");
        assert_eq!(format!("{:?}", v2), "V2(\"测试\")");
        assert_eq!(format!("{:?}", v3), "V3(true, 3.14)");
        assert_eq!(format!("{:?}", v4), "V4 { x: 1, y: 2 }");
        assert_eq!(
            format!("{:?}", v5),
            "V5(Object {\"key\": String(\"value\")})"
        );
    }

    #[test]
    fn test_test_enum() {
        let up = TestEnum::Up;
        let down = TestEnum::Down;
        let data = TestEnum::Data {
            data: TestEnumData::V1,
        };
        let next = TestEnum::Next(Box::new(TestEnum::Up));

        assert_eq!(up.as_str(), "上层");
        assert_eq!(down.as_str(), "下层");
        assert_eq!(format!("{:?}", data), "Data { data: V1 }");
        assert_eq!(format!("{:?}", next), "Next(Up)");
    }

    #[test]
    fn test_enum_clone() {
        let original = TestEnum::Data {
            data: TestEnumData::V2("克隆测试".to_string()),
        };
        let cloned = original.clone();

        assert_eq!(format!("{:?}", original), format!("{:?}", cloned));
    }

    #[test]
    fn test_serialization_deserialization() {
        // 创建一个复杂的 TestEnum 实例
        let original = TestEnum::Next(Box::new(TestEnum::Data {
            data: TestEnumData::V5(json!({
                "name": "张三",
                "age": 30,
                "hobbies": ["读书", "旅游"]
            })),
        }));

        // 序列化
        let serialized = to_string(&original).expect("序列化失败");
        println!("序列化结果: {}", serialized);

        // 反序列化
        let deserialized: TestEnum = from_str(&serialized).expect("反序列化失败");

        // 验证反序列化后的结果是否与原始数据相同
        assert_eq!(format!("{:?}", original), format!("{:?}", deserialized));

        // 测试 ename 方法是否正常工作
        if let TestEnum::Next(boxed) = deserialized {
            if let TestEnum::Data { data } = *boxed {
                if let TestEnumData::V5(value) = data {
                    assert_eq!(value["name"], "张三");
                    assert_eq!(value["age"], 30);
                    assert_eq!(value["hobbies"][0], "读书");
                    assert_eq!(value["hobbies"][1], "旅游");
                } else {
                    panic!("反序列化后的数据结构不正确");
                }
            } else {
                panic!("反序列化后的数据结构不正确");
            }
        } else {
            panic!("反序列化后的数据结构不正确");
        }
    }
}
