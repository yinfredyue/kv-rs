#![deny(missing_docs)]

//! A simple key/value store

pub use crate::engine::{KvStore, KvsEngine};
pub use crate::error::{KvStoreError, Result};

mod error;
mod kv;
mod engine;
