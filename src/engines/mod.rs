use crate::Result;

pub use kv::KvStore;
pub use crate::engines::sled::SledKvsStore;

/// A storage engine that can handle get, set and remove.
pub trait KvsEngine: Clone + Send + 'static {
    /// set
    fn set(&self, key: String, value: String) -> Result<()>;
    /// get
    fn get(&self, key: String) -> Result<Option<String>>;
    /// remove
    fn remove(&self, key: String) -> Result<()>;
}

mod kv;
mod sled;
