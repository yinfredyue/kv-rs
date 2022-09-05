use crate::{KvStoreError, KvsEngine, Result};

///
#[derive(Clone)]
pub struct SledKvsStore {
    store: sled::Db,
}

impl SledKvsStore {
    ///
    pub fn open(dir_path: &std::path::Path) -> Result<Self> {
        let store = sled::open(dir_path)?;
        Ok(SledKvsStore { store })
    }
}

impl KvsEngine for SledKvsStore {
    fn set(&self, key: String, value: String) -> Result<()> {
        self.store.insert(key.as_bytes(), value.as_bytes())?;
        self.store.flush()?; // important
        Ok(())
    }

    fn get(&self, key: String) -> Result<Option<String>> {
        let v = self
            .store
            .get(key.as_bytes())?
            .and_then(|v| Some(String::from_utf8(v.to_vec()).unwrap()));
        Ok(v)
    }

    fn remove(&self, key: String) -> Result<()> {
        let old_val = self.store.remove(key.as_bytes())?;
        self.store.flush()?; // important
        match old_val {
            None => Err(KvStoreError::RemoveNonexistingKey),
            Some(_) => Ok(()),
        }
    }
}
