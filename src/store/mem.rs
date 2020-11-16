
use super::{Key, Value, Version, ValueVersion, Store};
use std::collections::HashMap;
use std::error::Error;
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

impl Store for MemStore {
    fn put(&mut self, key: Key, val: Value) -> Result<Version, Box<dyn Error>> {
        todo!();
    }
    fn get(&mut self, key: &Key) -> Result<Option<ValueVersion>, Box<dyn Error>> {
        todo!();
    }
    fn delete(&mut self, key: &Key) -> Result<Option<ValueVersion>, Box<dyn Error>> {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mem_store() {
        unimplemented!();
    }
}
