use crate::error::{KvStoreError, Result};
use std::collections::HashMap;
use std::path;

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

    /// open a store
    pub fn open(path: &path::Path) -> Result<Self> {
        panic!()
    }

    /// set a value
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        self.store.insert(key, value);
        Ok(())
    }

    /// get a value
    pub fn get(&self, key: String) -> Result<Option<String>> {
        Ok(self.store.get(&key).cloned())
    }

    /// remove a value
    pub fn remove(&mut self, key: String) -> Result<()> {
        self.store.remove(&key);
        Ok(())
    }
}
