use std::collections::HashMap;

pub trait KeyValueStore {
    type Key;
    type Value;

    fn set(&mut self, key: Self::Key, value: Self::Value);
    fn get(&self, key: &Self::Key) -> Option<&Self::Value>;
}

pub struct InMemoryStore<K, V> {
    pub storage: HashMap<K, V>,
}

impl<K, V> KeyValueStore for InMemoryStore<K, V>
where
    K: std::cmp::Eq + std::hash::Hash,
{
    type Key = K;
    type Value = V;

    fn set(&mut self, key: Self::Key, value: Self::Value) {
        self.storage.insert(key, value);
    }

    fn get(&self, key: &Self::Key) -> Option<&Self::Value> {
        self.storage.get(key)
    }
}
