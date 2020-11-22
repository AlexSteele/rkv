use super::Store;
use crate::error::Result;
use crate::{Key, Value, ValueVersion, Version};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct MemStore {
    entries: Arc<Mutex<HashMap<Key, ValueVersion>>>,
}

impl MemStore {
    pub fn new() -> Self {
        Self {
            entries: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

// TODO: Fix versions
// TODO: lock().unwrap()??? Handle poisoned locks.
impl Store for MemStore {
    fn put(&self, key: Key, val: Value) -> Result<Version> {
        let mut entries = self.entries.lock().unwrap();
        entries.insert(key, (val, 0));
        Ok(0)
    }
    fn get(&self, key: &Key) -> Result<Option<ValueVersion>> {
        let entries = self.entries.lock().unwrap();
        Ok(entries.get(key).map(|(k, v)| (k.clone(), *v)))
    }
    fn delete(&self, key: &Key) -> Result<Option<ValueVersion>> {
        let mut entries = self.entries.lock().unwrap();
        Ok(entries.remove(key))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mem_store() {
        let mut store = MemStore::new();
        let version = store
            .put(Key("k".as_bytes().to_vec()), "v".as_bytes().to_vec())
            .unwrap();
        assert_eq!(version, 0);
        let (val, version) = store
            .get(&Key("k".as_bytes().to_vec()))
            .unwrap()
            .expect("missing value for key");
        assert_eq!((val, version), ("v".as_bytes().to_vec(), 0));
    }
}
