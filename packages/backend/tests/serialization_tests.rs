// Serialization tests for POE2 Overlord backend
// Tests for chrono timestamps, serde JSON, and serialization roundtrip

use chrono::Utc;

#[test]
fn test_chrono_serialization() {
    let now = Utc::now();
    let timestamp_str = now.to_rfc3339();

    // Should be able to parse the timestamp back
    let parsed = chrono::DateTime::parse_from_rfc3339(&timestamp_str);
    assert!(parsed.is_ok());

    let parsed = parsed.unwrap();
    // Allow for small differences due to precision
    assert!((parsed.timestamp() - now.timestamp()).abs() <= 1);
}

#[test]
fn test_serde_json_basic() {
    let data = serde_json::json!({
        "name": "Test",
        "value": 42,
        "active": true,
        "items": [1, 2, 3]
    });

    assert_eq!(data["name"], "Test");
    assert_eq!(data["value"], 42);
    assert_eq!(data["active"], true);
    assert_eq!(data["items"], serde_json::json!([1, 2, 3]));
}

#[test]
fn test_serde_serialization_roundtrip() {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct TestStruct {
        name: String,
        value: i32,
        active: bool,
    }

    let original = TestStruct {
        name: "Test".to_string(),
        value: 42,
        active: true,
    };

    // Serialize to JSON
    let json = serde_json::to_string(&original).unwrap();

    // Deserialize back
    let deserialized: TestStruct = serde_json::from_str(&json).unwrap();

    assert_eq!(original, deserialized);
}
