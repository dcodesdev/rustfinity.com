use associated_types::*;
use std::collections::HashMap;

#[test]
fn test_set_and_get() {
    let mut store: InMemoryStore<String, String> = InMemoryStore {
        storage: HashMap::new(),
    };

    store.set("key1".to_string(), "value1".to_string());
    assert_eq!(store.get(&"key1".to_string()), Some(&"value1".to_string()));

    store.set("key2".to_string(), "value2".to_string());
    assert_eq!(store.get(&"key2".to_string()), Some(&"value2".to_string()));
}

#[test]
fn test_non_existent_key() {
    let store: InMemoryStore<String, String> = InMemoryStore {
        storage: HashMap::new(),
    };

    assert_eq!(store.get(&"missing".to_string()), None);
}

#[test]
fn test_overwrite_value() {
    let mut store: InMemoryStore<String, String> = InMemoryStore {
        storage: HashMap::new(),
    };

    store.set("key".to_string(), "value1".to_string());
    store.set("key".to_string(), "value2".to_string());
    assert_eq!(store.get(&"key".to_string()), Some(&"value2".to_string()));
}

#[test]
fn test_different_key_value_types() {
    let mut store: InMemoryStore<u32, f64> = InMemoryStore {
        storage: HashMap::new(),
    };

    store.set(1, 10.5);
    store.set(2, 20.0);

    assert_eq!(store.get(&1), Some(&10.5));
    assert_eq!(store.get(&2), Some(&20.0));
    assert_eq!(store.get(&3), None);
}

#[test]
fn test_large_store() {
    let mut store: InMemoryStore<u64, String> = InMemoryStore {
        storage: HashMap::new(),
    };

    for i in 0..1000 {
        store.set(i, format!("value{}", i));
    }

    for i in 0..1000 {
        assert_eq!(store.get(&i), Some(&format!("value{}", i)));
    }

    assert_eq!(store.get(&1001), None);
}

#[test]
fn test_empty_store() {
    let store: InMemoryStore<String, String> = InMemoryStore {
        storage: HashMap::new(),
    };

    assert_eq!(store.get(&"anything".to_string()), None);
}

#[test]
fn test_store_with_complex_key() {
    #[derive(Eq, PartialEq, Hash, Debug, Clone)]
    struct ComplexKey {
        id: u32,
        name: String,
    }

    let mut store: InMemoryStore<ComplexKey, String> = InMemoryStore {
        storage: HashMap::new(),
    };

    let key1 = ComplexKey {
        id: 1,
        name: "key1".to_string(),
    };
    let key2 = ComplexKey {
        id: 2,
        name: "key2".to_string(),
    };

    store.set(key1.clone(), "value1".to_string());
    store.set(key2.clone(), "value2".to_string());

    assert_eq!(store.get(&key1), Some(&"value1".to_string()));
    assert_eq!(store.get(&key2), Some(&"value2".to_string()));
}

#[test]
fn test_store_remove_key_behavior() {
    let mut store: InMemoryStore<String, String> = InMemoryStore {
        storage: HashMap::new(),
    };

    store.set("key".to_string(), "value".to_string());
    assert_eq!(store.get(&"key".to_string()), Some(&"value".to_string()));

    store.storage.remove(&"key".to_string());
    assert_eq!(store.get(&"key".to_string()), None);
}
