/// Errors
use serde::{Deserialize, Serialize};

///
#[derive(Serialize, Deserialize, Debug)]
pub enum KvStoreError {
    ///
    IoError,
    ///
    SerdeError,
    ///
    RemoveNonexistingKey,
    ///
    SledError,
    ///
    WrongEngine
}

impl From<std::io::Error> for KvStoreError {
    fn from(_: std::io::Error) -> Self {
        Self::IoError
    }
}

impl From<serde_json::Error> for KvStoreError {
    fn from(_: serde_json::Error) -> Self {
        Self::SerdeError
    }
}

impl From<sled::Error> for KvStoreError {
    fn from(_ : sled::Error) -> Self {
        Self::SledError
    }
}

/// Avoid typing Result<T, KvStoreError>
pub type Result<T> = std::result::Result<T, KvStoreError>;
