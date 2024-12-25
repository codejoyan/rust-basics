use std::sync::atomic::AtomicUsize;

use crossbeam_skiplist::SkipMap;
use crossbeam_skiplist::map::Entry;

use anyhow::Result;

pub struct MemTable {
    map: Arc<SkipMap<Bytes, Bytes>>,
    // remember to change this to Option<Wal>
    wal: Option<Bytes>,
    id: usize,
    approximate_size: Arc<AtomicUsize>,
}

impl MemTable {
    pub fn create(id: usize) -> Self {
        Self {
            id,
            map: Arc::new(SkipMap::new()),
            wal: None,
            approximate_size: Arc::new(AtomicUsize::new(0)),
        }
    }

    pub fn get(&self, key: &[u8]) -> Option<Bytes> {
        self.map.get(key).map(|e| e.value.clone())
    }
}