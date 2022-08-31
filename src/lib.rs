#![deny(missing_docs)]

//! A simple key/value store

pub use crate::kv::KvStore;
pub use crate::error::{KvStoreError, Result};

mod error;
mod kv;
