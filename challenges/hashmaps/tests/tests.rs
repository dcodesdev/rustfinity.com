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

    // Insert another key-value pair
    insert_or_update(&mut store, "age".to_string(), "25".to_string());
    assert_eq!(store.get("age"), Some(&"25".to_string()));

    // Update another key
    insert_or_update(&mut store, "age".to_string(), "30".to_string());
    assert_eq!(store.get("age"), Some(&"30".to_string()));
}

#[test]
fn test_get_value() {
    let mut store = HashMap::new();
    store.insert("city".to_string(), "Paris".to_string());
    store.insert("country".to_string(), "France".to_string());

    // Retrieve existing key
    let value = get_value(&store, "city".to_string());
    assert_eq!(value, Some("Paris".to_string()));

    // Attempt to retrieve a non-existent key
    let missing = get_value(&store, "population".to_string());
    assert_eq!(missing, None);

    // Retrieve another existing key
    let value = get_value(&store, "country".to_string());
    assert_eq!(value, Some("France".to_string()));
}

#[test]
fn test_empty_map() {
    let store: HashMap<String, String> = HashMap::new();

    // Attempt to retrieve a value from an empty map
    let value = get_value(&store, "key".to_string());
    assert_eq!(value, None);
}

#[test]
fn test_overwrite_behavior() {
    let mut store = HashMap::new();

    // Insert a value and overwrite it multiple times
    insert_or_update(&mut store, "key".to_string(), "Value1".to_string());
    assert_eq!(store.get("key"), Some(&"Value1".to_string()));

    insert_or_update(&mut store, "key".to_string(), "Value2".to_string());
    assert_eq!(store.get("key"), Some(&"Value2".to_string()));

    insert_or_update(&mut store, "key".to_string(), "Value3".to_string());
    assert_eq!(store.get("key"), Some(&"Value3".to_string()));
}

#[test]
fn test_large_map() {
    let mut store = HashMap::new();

    // Insert a large number of key-value pairs
    for i in 0..1000 {
        let key = format!("key{}", i);
        let value = format!("value{}", i);
        insert_or_update(&mut store, key.clone(), value.clone());
        assert_eq!(store.get(&key), Some(&value));
    }

    // Verify all keys are present
    for i in 0..1000 {
        let key = format!("key{}", i);
        let value = format!("value{}", i);
        assert_eq!(get_value(&store, key), Some(value));
    }
}

#[test]
fn test_unicode_keys_and_values() {
    let mut store = HashMap::new();

    // Insert and retrieve keys and values with Unicode characters
    insert_or_update(&mut store, "ключ".to_string(), "значение".to_string());
    assert_eq!(
        get_value(&store, "ключ".to_string()),
        Some("значение".to_string())
    );

    insert_or_update(&mut store, "emoji".to_string(), "😊".to_string());
    assert_eq!(
        get_value(&store, "emoji".to_string()),
        Some("😊".to_string())
    );
}
