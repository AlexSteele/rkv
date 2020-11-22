mod error;
mod server;

pub use error::{Error, Result};
pub use server::{Config, NodeAddr, State, RkvService, PeerService};
