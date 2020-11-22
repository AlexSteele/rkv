use crate::error::Result;
use crate::{Key, Value, ValueVersion, Version};

mod mem;
pub use mem::MemStore;

pub trait Store: Send + Sync {
    fn put(&self, key: Key, val: Value) -> Result<Version>;
    fn get(&self, key: &Key) -> Result<Option<ValueVersion>>;
    fn delete(&self, key: &Key) -> Result<Option<ValueVersion>>;
}
