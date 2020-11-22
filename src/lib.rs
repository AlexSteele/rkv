#[macro_use]
extern crate quick_error;

pub mod client;
pub(crate) mod error;
pub mod ring;
pub mod server;
pub mod store;
pub mod proto {
    tonic::include_proto!("rkv");
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Key(pub Vec<u8>);
pub type Value = Vec<u8>;
pub type Version = i64;
pub type ValueVersion = (Value, Version);
