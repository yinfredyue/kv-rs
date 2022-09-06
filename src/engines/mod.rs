use crate::Result;

pub use kv::KvStore;
pub use crate::engines::sled::SledKvsStore;

/// A storage engine that can handle get, set and remove.
/// 
/// - Why requiring the Clone trait?
///   To be passed into multiple threads.
/// - Why changing from `&mut self` to `&self`?
///   This means we need to wrap the data structure in a Mutex. With an
///   immutable mutex, we can get a mutable reference to the value inside.
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
