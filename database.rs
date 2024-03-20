// database.rs
use rocksdb::{DB, Options};
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("RocksDB error: {0}")]
    RocksDb(#[from] rocksdb::Error),
}

pub struct NeuronDb {
    db: DB,
}

impl NeuronDb {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, DatabaseError> {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        let db = DB::open(&opts, path)?;
        Ok(Self { db })
    }

    pub fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>, DatabaseError> {
        match self.db.get(key) {
            Ok(Some(value)) => Ok(Some(value.to_vec())),
            Ok(None) => Ok(None),
            Err(e) => Err(DatabaseError::from(e)),
        }
    }

    pub fn put(&self, key: &[u8], value: &[u8]) -> Result<(), DatabaseError> {
        self.db.put(key, value)?;
        Ok(())
    }

    pub fn delete(&self, key: &[u8]) -> Result<(), DatabaseError> {
        self.db.delete(key)?;
        Ok(())
    }
}
