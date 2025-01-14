use std::collections::HashMap;

/// Inserts a key-value pair into the hashmap or updates the value if the key exists.
pub fn insert_or_update(map: &mut HashMap<String, String>, key: String, value: String) {
    map.insert(key, value);
}

/// Retrieves the value associated with a key from the hashmap.
pub fn get_value(map: &HashMap<String, String>, key: String) -> Option<String> {
    map.get(&key).cloned()
}
