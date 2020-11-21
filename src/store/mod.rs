mod mem;
use crate::{Key, Value, ValueVersion, Version};
pub use mem::MemStore;

use std::error::Error;

pub trait Store: Send + Sync {
    fn put(&self, key: Key, val: Value) -> Result<Version, Box<dyn Error>>;
    fn get(&self, key: &Key) -> Result<Option<ValueVersion>, Box<dyn Error>>;
    fn delete(&self, key: &Key) -> Result<Option<ValueVersion>, Box<dyn Error>>;
}
