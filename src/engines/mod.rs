use crate::Result;

pub use kv::KvStore;

/// A storage engine that can handle get, set and remove.
pub trait KvsEngine {
    /// set
    fn set(&mut self, key: String, value: String) -> Result<()>;
    /// get
    fn get(&mut self, key: String) -> Result<Option<String>>;
    /// remove
    fn remove(&mut self, key: String) -> Result<()>;
}

mod kv;
mod sled;
