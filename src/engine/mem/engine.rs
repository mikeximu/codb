
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use dashmap::DashMap;

use crate::core::{DB, DBStats, DBError};
use crate::core::options::{ReadOptions, WriteOptions};


pub struct Engine {
    data: DashMap<Vec<u8>, Vec<u8>>,
    closed: AtomicBool,
    key_count: AtomicU64,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            data: DashMap::new(),
            closed: AtomicBool::new(false),
            key_count: AtomicU64::new(0),
        }
    }

    fn ensure_open(&self) -> Result<(), DBError> {
        if self.closed.load(Ordering::SeqCst) { Err(DBError::Closed) } else { Ok(()) }
    }
}

impl DB for Engine {
    fn get(&self, key: &[u8], _: Option<&ReadOptions>) -> Result<Vec<u8>, DBError> {
        self.ensure_open()?;
        self.data.get(key).map(|v| v.clone()).ok_or(DBError::NotFound)
    }

    fn put(&self, key: &[u8], value: &[u8], _: Option<&WriteOptions>) -> Result<(), DBError> {
        self.ensure_open()?;
        let existed = self.data.insert(key.to_vec(), value.to_vec()).is_some();
        if !existed { self.key_count.fetch_add(1, Ordering::SeqCst); }
        Ok(())
    }

    fn delete(&self, key: &[u8], _: Option<&WriteOptions>) -> Result<(), DBError> {
        self.ensure_open()?;
        if self.data.remove(key).is_some() { self.key_count.fetch_sub(1, Ordering::SeqCst); }
        Ok(())
    }

    fn has(&self, key: &[u8], _: Option<&ReadOptions>) -> Result<bool, DBError> {
        self.ensure_open()?;
        Ok(self.data.contains_key(key))
    }

    fn stats(&self) -> DBStats {
        DBStats { key_count: self.key_count.load(Ordering::SeqCst) }
    }

    fn size(&self) -> i64 {
        self.data.iter().map(|e| e.key().len() + e.value().len()).sum::<usize>() as i64
    }

    fn close(&self) -> Result<(), DBError> { self.closed.store(true, Ordering::SeqCst); Ok(()) }
    fn is_closed(&self) -> bool { self.closed.load(Ordering::SeqCst) }
    fn ping(&self) -> Result<(), DBError> { self.ensure_open() }
}
