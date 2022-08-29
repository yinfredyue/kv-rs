#![deny(missing_docs)]

//! A simple key/value store

use std::collections::HashMap;

/// `KvStore` stores key-value pairs in memory
/// Example:
/// ```rust
/// # use kvs::KvStore;
/// let mut store = KvStore::new();
/// store.set("key".to_owned(), "value".to_owned());
/// let val = store.get("key".to_owned());
/// assert_eq!(val, Some("value".to_owned()));
/// ```
pub struct KvStore {
    store: HashMap<String, String>,
}

impl KvStore {
    /// create a new store
    pub fn new() -> Self {
        KvStore {
            store: HashMap::new(),
        }
    }

    /// set a value
    pub fn set(&mut self, key: String, value: String) {
        self.store.insert(key, value);
    }

    /// get a value
    pub fn get(&self, key: String) -> Option<String> {
        self.store.get(&key).cloned()
    }

    /// remove a value
    pub fn remove(&mut self, key: String) {
        self.store.remove(&key);
    }
}
