use crate::error::{KvStoreError, Result};
use serde::{Deserialize, Serialize};
use serde_json::Deserializer;
use std::collections::HashMap;
use std::io::Read;
use std::{fs, io::Write, os::unix::prelude::FileExt, path};

#[derive(Debug, Serialize, Deserialize)]
struct Entry {
    key: String,
    value: Option<String>,
}

impl Entry {
    fn is_remove(&self) -> bool {
        self.value.is_none()
    }
}

// metadata about serialized object (for deserialization)
#[derive(Debug)]
struct SerMeta {
    offset: usize,
    size: usize,
}

/// `KvStore` stores key-value pairs
#[derive(Debug)]
pub struct KvStore {
    file: fs::File,
    mapping: HashMap<String, SerMeta>,
}

impl KvStore {
    /// open a store
    pub fn open(dir_path: &path::Path) -> Result<Self> {
        let file_path = dir_path.join("data.json");
        let mut file = fs::OpenOptions::new()
            .create(true) // open if existing, otherwise create
            .read(true)
            .write(true)
            .append(true)
            .open(&file_path)
            .unwrap();

        let mapping = Self::mapping_from_log(&mut file)?;
        let store = KvStore { file, mapping };
        Ok(store)
    }

    /// set a value
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let entry = Entry {
            key: key.to_owned(),
            value: Some(value),
        };
        let (offset, size) = self.append_log(entry)?;
        self.mapping.insert(key, SerMeta { offset, size });
        Ok(())
    }

    /// get a value
    pub fn get(&self, key: String) -> Result<Option<String>> {
        match self.mapping.get(&key) {
            None => Ok(None),
            Some(SerMeta { offset, size }) => {
                let mut buf = vec![0u8; *size];
                self.file.read_exact_at(&mut buf, *offset as u64)?;
                let entry: Entry = serde_json::from_slice(&buf)?;
                Ok(entry.value)
            }
        }
    }

    /// remove a value
    pub fn remove(&mut self, key: String) -> Result<()> {
        if self.mapping.get(&key).is_none() {
            return Err(KvStoreError::RemoveNonexistingKey);
        }

        let entry = Entry {
            key: key.to_owned(),
            value: None,
        };
        let (offset, size) = self.append_log(entry)?;
        self.mapping.insert(key, SerMeta { offset, size });
        Ok(())
    }

    fn mapping_from_log(file: &mut fs::File) -> Result<HashMap<String, SerMeta>> {
        let mut mapping = HashMap::new();
        if file.metadata()?.len() > 0 {
            let mut content = String::new();
            file.read_to_string(&mut content)?;
            let stream = Deserializer::from_str(&content).into_iter::<Entry>();

            let mut offset = 0;
            for entry in stream {
                let entry = entry?;

                // This is an expensive way to get size. Can we do better?
                let size = serde_json::to_vec(&entry).unwrap().len();
                if entry.is_remove() {
                    mapping.remove(&entry.key);
                } else {
                    mapping.insert(entry.key, SerMeta { offset, size });
                }

                offset += size;
            }
        }
        Ok(mapping)
    }

    fn append_log(&mut self, entry: Entry) -> Result<(usize, usize)> {
        let serialized = serde_json::to_string(&entry)?;

        let size = serialized.as_bytes().len();
        let offset = self.file.metadata().unwrap().len() as usize;
        self.file.write(serialized.as_bytes())?;

        Ok((offset, size))
    }
}
