
pub mod mem;

use std::error::Error;

#[derive(Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Key(pub Vec<u8>);
pub type Value = Vec<u8>;
pub type Version = i64;
pub type ValueVersion = (Value, Version);

pub trait Store: Send + Sync {
    fn put(&mut self, key: Key, val: Value) -> Result<Version, Box<dyn Error>>;
    fn get(&mut self, key: &Key) -> Result<Option<ValueVersion>, Box<dyn Error>>;
    fn delete(&mut self, key: &Key) -> Result<Option<ValueVersion>, Box<dyn Error>>;
}
