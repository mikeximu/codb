use crate::core::{ReadOptions, WriteOptions, DBStats};
use crate::core::error::DBError;

pub trait DB: Send + Sync {
    // -------- Core KV --------

    fn get(
        &self,
        key: &[u8],
        opts: Option<&ReadOptions>,
    ) -> Result<Vec<u8>, DBError>;

    fn put(
        &self,
        key: &[u8],
        value: &[u8],
        opts: Option<&WriteOptions>,
    ) -> Result<(), DBError>;

    fn delete(
        &self,
        key: &[u8],
        opts: Option<&WriteOptions>,
    ) -> Result<(), DBError>;

    fn has(
        &self,
        key: &[u8],
        opts: Option<&ReadOptions>,
    ) -> Result<bool, DBError>;

    // -------- Admin --------

    fn stats(&self) -> DBStats;

    fn size(&self) -> i64;

    // -------- Lifecycle --------

    fn close(&self) -> Result<(), DBError>;

    fn is_closed(&self) -> bool;

    fn ping(&self) -> Result<(), DBError>;
}