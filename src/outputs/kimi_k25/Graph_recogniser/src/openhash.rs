use std::sync::{PoisonError, RwLock, RwLockReadGuard, RwLockWriteGuard};

pub struct OpenHashTable {
buckets: RwLock<Vec<Option<(u64, u64)>>>,
}

impl OpenHashTable {
pub fn new(capacity: usize) -> Self {
Self {
buckets: RwLock::new(vec![None; capacity]),
}
}

pub fn read(&self) -> Result<RwLockReadGuard<Vec<Option<(u64, u64)>>>, PoisonError<RwLockReadGuard<Vec<Option<(u64, u64)>>>>> {
self.buckets.read()
}

pub fn write(&self) -> Result<RwLockWriteGuard<Vec<Option<(u64, u64)>>>, PoisonError<RwLockWriteGuard<Vec<Option<(u64, u64)>>>>> {
self.buckets.write()
}
}
