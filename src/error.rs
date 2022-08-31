/// Errors

#[derive(Debug)]
pub enum KvStoreError {}

/// Avoid typing Result<T, KvStoreError>
pub type Result<T> = std::result::Result<T, KvStoreError>;
