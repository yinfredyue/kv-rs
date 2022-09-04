#![deny(missing_docs)]

//! A simple key/value store

pub use crate::client::KvClient;
pub use crate::engines::{KvStore, KvsEngine};
pub use crate::error::{KvStoreError, Result};
pub use crate::server::KvServer;

mod client;
mod error;
mod message;
mod server;
mod engines;
