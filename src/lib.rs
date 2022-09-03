#![deny(missing_docs)]

//! A simple key/value store

pub use crate::client::KvClient;
pub use crate::engine::{KvStore, KvsEngine};
pub use crate::error::{KvStoreError, Result};
pub use crate::server::KvServer;

mod client;
mod engine;
mod error;
mod kv;
mod message;
mod server;
