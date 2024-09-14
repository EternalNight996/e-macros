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