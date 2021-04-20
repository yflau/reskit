
#[test]
fn test_map() {
    let v = serde_json::json!({
        "code": 200,
        "success": true,
        "caller": reskit_utils::caller!(),
        "payload": {
            "features": [
                "serde",
                "json"
            ]
        }
    });
    assert_eq!(v["caller"].as_str(), Some("json::test_map"));
}
