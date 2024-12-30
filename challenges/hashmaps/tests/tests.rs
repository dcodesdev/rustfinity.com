use hashmaps::*;
use std::collections::HashMap;

#[test]
fn test_insert_or_update() {
    let mut store = HashMap::new();

    // Insert a new key-value pair
    insert_or_update(&mut store, "name".to_string(), "Alice".to_string());
    assert_eq!(store.get("name"), Some(&"Alice".to_string()));

    // Update an existing key
    insert_or_update(&mut store, "name".to_string(), "Bob".to_string());
    assert_eq!(store.get("name"), Some(&"Bob".to_string()));
}

#[test]
fn test_get_value() {
    let mut store = HashMap::new();
    store.insert("city".to_string(), "Paris".to_string());

    // Retrieve existing key
    let value = get_value(&store, "city".to_string());
    assert_eq!(value, Some("Paris".to_string()));

    // Attempt to retrieve a non-existent key
    let missing = get_value(&store, "country".to_string());
    assert_eq!(missing, None);
}
