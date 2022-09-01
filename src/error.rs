/// Errors

#[derive(Debug)]
pub enum KvStoreError {
    ///
    IoError,
    ///
    SerdeError,
    ///
    RemoveNonexistingKey,
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

/// Avoid typing Result<T, KvStoreError>
pub type Result<T> = std::result::Result<T, KvStoreError>;
