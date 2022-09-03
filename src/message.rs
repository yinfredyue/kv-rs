use serde::{Deserialize, Serialize};
use crate::error::KvStoreError;

#[derive(Debug, Deserialize, Serialize)]
pub enum Request {
    Get{key : String},
    Set{key : String, value : String},
    Remove{key : String},
}

#[derive(Debug, Deserialize, Serialize)]
pub enum GetResponse {
    Ok(Option<String>),
    Err(KvStoreError)
}


#[derive(Debug, Deserialize, Serialize)]
pub enum SetResponse {
    Ok,
    Err(KvStoreError)
}


#[derive(Debug, Deserialize, Serialize)]
pub enum RemoveResponse {
    Ok,
    Err(KvStoreError)
}