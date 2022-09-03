use crate::error::{KvStoreError, Result};
use serde::{Deserialize, Serialize};
use serde_json::Deserializer;
use std::collections::HashMap;
use std::fs::File;
use std::{
    boxed::Box,
    fs,
    io::{Read, Write},
    os::unix::prelude::FileExt,
    path,
};

///
pub trait KvsEngine {
    ///
    fn set(&mut self, key: String, value: String) -> Result<()>;
    ///
    fn get(&mut self, key: String) -> Result<Option<String>>;
    ///
    fn remove(&mut self, key: String) -> Result<()>;
}

// Log entry written to file.
// Set is {key, Some(value)}. Remove is {key, None}.
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

/// Position of entry in the log file.
#[derive(Debug)]
struct EntryPos {
    offset: usize,
    size: usize,
}

// Statistics about log entries
#[derive(Debug)]
struct Stat {
    total: usize,
}

/// `KvStore` stores key-value pairs, using log-structured hashtable.  
/// The serialization format is JSON, for easy development & debugging.
#[derive(Debug)]
pub struct KvStore {
    // immutable
    dir_path: Box<path::PathBuf>,
    file_path: Box<path::PathBuf>,

    // mutable
    file: fs::File,
    mapping: HashMap<String, EntryPos>,
    stat: Stat,
}

impl KvStore {
    /// open a store
    pub fn open(dir_path: &path::Path) -> Result<Self> {
        let dir_path = Box::new(dir_path.to_owned());
        let file_path = Box::new(dir_path.join("data.json"));
        let mut file = Self::open_logfile(&file_path);
        let mapping = Self::mapping_from_log(&mut file)?;
        let store = KvStore {
            dir_path,
            file_path,
            file,
            mapping,
            stat: Stat { total: 0 },
        };
        Ok(store)
    }

    // parse an `Entry` from a file and metadata
    fn deserialize(file: &File, meta: &EntryPos) -> Result<Entry> {
        let EntryPos { offset, size } = meta;
        let mut buf = vec![0u8; *size];
        file.read_exact_at(&mut buf, *offset as u64)?;
        let entry: Entry = serde_json::from_slice(&buf)?;
        Ok(entry)
    }

    // Generate in-memory mapping by replaying a log file.
    fn mapping_from_log(file: &mut fs::File) -> Result<HashMap<String, EntryPos>> {
        let mut mapping = HashMap::new();
        if file.metadata()?.len() > 0 {
            let mut content = String::new();
            file.read_to_string(&mut content)?;
            let mut stream = Deserializer::from_str(&content).into_iter::<Entry>();

            let mut offset = 0;

            // `stream` is StreamDeserializer. We want to iterate over it
            // and call `stream.byte_offset()` when iterating. `byte_offset`
            // requires a reference.
            // Thus, when iterating it, we cannot consume/move `stream`, we
            // cannot use a mutable reference of `stream`. We can only
            // use a immutable reference for `stream`.
            // However, this is not supported.
            // 1. we cannot use `for v in stream`, because `for` loop is
            // just syntax sugar calling `into_iter` which consumes the value.
            // 2. we cannot use `for v in &mut stream` as it creates
            // mutable borrow.
            // 3. we cannot do `for entry in &stream`, error message:
            // ```
            // `&StreamDeserializer<...>` is not an iterator
            // the trait `Iterator` is not implemented for `&StreamDeserializer<...>`
            // ```
            //
            // The workaround is to use `next` method in a while let loop.
            // `stream.next()` only borrows input for the duration of its own
            // call, since the return value is owned.
            // https://www.reddit.com/r/rust/comments/2pqcgt/while_let_someitem_iteratornext/
            // https://github.com/rust-lang/rust/issues/8372
            while let Some(Ok(entry)) = stream.next() {
                let entry = entry;

                let new_offset = stream.byte_offset();
                let size = new_offset - offset;

                // This is an expensive way to get size. Can we do better?
                if entry.is_remove() {
                    mapping.remove(&entry.key);
                } else {
                    mapping.insert(entry.key, EntryPos { offset, size });
                }

                offset = new_offset;
            }
        }
        Ok(mapping)
    }

    // append some value to the log file, returning (offset, size).
    // Should only be called by `compact` and `append_entry`.
    fn append_file<T: Serialize>(file: &mut File, value: T) -> Result<(usize, usize)> {
        let serialized = serde_json::to_vec(&value)?;

        let size = serialized.len();
        let offset = file.metadata().unwrap().len() as usize;
        file.write_all(&serialized)?;

        Ok((offset, size))
    }

    // append an `Entry` to the log file. May compact. Update stat.
    fn append_entry(&mut self, entry: Entry) -> Result<(usize, usize)> {
        self.compact()?;

        let (offset, size) = Self::append_file(&mut self.file, entry)?;
        self.stat.total += 1;

        Ok((offset, size))
    }

    // open a file to be used a log file, with proper flags
    fn open_logfile(path: &path::Path) -> fs::File {
        fs::OpenOptions::new()
            .create(true) // open if existing, otherwise create
            .read(true)
            .write(true)
            .append(true)
            .open(&path)
            .unwrap()
    }

    // may compact log file
    fn compact(&mut self) -> Result<()> {
        if (self.mapping.len() as f32) / (self.stat.total as f32) < 0.4 {
            // write new log file and create new mapping
            let new_log_path = self.dir_path.join("compacted.json");
            let mut new_mapping = HashMap::new();
            {
                let mut new_log = Self::open_logfile(&new_log_path);
                for (key, meta) in (self.mapping).iter_mut() {
                    let entry = Self::deserialize(&self.file, meta)?;
                    let (offset, size) = Self::append_file(&mut new_log, entry)?;
                    new_mapping.insert(key.to_owned(), EntryPos { offset, size });
                }
            }

            // update fields
            fs::rename(new_log_path, self.file_path.as_path())?;
            self.file = Self::open_logfile(self.file_path.as_path());
            self.mapping = new_mapping;
            self.stat = Stat { total: 0 };
        }
        Ok(())
    }
}

impl KvsEngine for KvStore {
    fn set(&mut self, key: String, value: String) -> Result<()> {
        let entry = Entry {
            key: key.to_owned(),
            value: Some(value),
        };
        let (offset, size) = self.append_entry(entry)?;
        self.mapping.insert(key, EntryPos { offset, size });
        Ok(())
    }

    fn get(&mut self, key: String) -> Result<Option<String>> {
        match self.mapping.get(&key) {
            None => Ok(None),
            Some(meta) => {
                let entry = Self::deserialize(&self.file, meta)?;
                Ok(entry.value)
            }
        }
    }

    fn remove(&mut self, key: String) -> Result<()> {
        if self.mapping.get(&key).is_none() {
            return Err(KvStoreError::RemoveNonexistingKey);
        }

        let entry = Entry {
            key: key.to_owned(),
            value: None,
        };
        self.append_entry(entry)?;
        self.mapping.remove(&key);
        Ok(())
    }
}
