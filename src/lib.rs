#![deny(missing_docs)]

//! A simple key/value store

pub use crate::engine::{KvStore, KvsEngine};
pub use crate::error::{KvStoreError, Result};
pub use crate::server::KvServer;
pub use crate::client::KvClient;

mod error;
mod kv;
mod engine;
mod server;
mod client;
mod message;