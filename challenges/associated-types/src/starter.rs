use std::collections::HashMap;

pub trait KeyValueStore {
    type Key;
    type Value;

    fn set(&mut self, key: Self::Key, value: Self::Value);
    fn get(&self, key: &Self::Key) -> Option<&Self::Value>;
}

pub struct InMemoryStore<K, V> {
    // Your code here...
}

impl<K, V> KeyValueStore for InMemoryStore<K, V> {
    type Key = K;
    type Value = V;

    fn set(&mut self, key: Self::Key, value: Self::Value) {
        // Implement here...
    }

    fn get(&self, key: &Self::Key) -> Option<&Self::Value> {
        // Implement here...
    }
}

// Example usage
pub fn main() {
    let mut store: InMemoryStore<String, String> = InMemoryStore {
        // Initialize your store here...
    };

    store.set("name".to_string(), "Rust".to_string());
    assert_eq!(store.get(&"name".to_string()), Some(&"Rust".to_string()));

    store.set("language".to_string(), "Rust".to_string());
    assert_eq!(
        store.get(&"language".to_string()),
        Some(&"Rust".to_string())
    );

    assert_eq!(store.get(&"non_existent".to_string()), None);
}
