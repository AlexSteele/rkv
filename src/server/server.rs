use std::error::Error;
use crate::proto::ClusterConfig;

pub type NodeAddr = String;

#[derive(Debug, Clone)]
pub struct Config {
    folder: String,
    address: NodeAddr,
    seed_nodes: Vec<NodeAddr>,
    cluster_config: ClusterConfig,
}

impl Config {
    fn from_args() -> Result<Config, Box<dyn Error>> {
        todo!();
    }
}

pub struct Server {
    config: Config,
}

impl Server {
    pub fn new(config: Config) -> Self {
        Self {
            config: config,
        }
    }
    pub fn start() -> Result<(), Box<dyn Error>> {
        todo!();
    }
    pub fn stop() -> Result<(), Box<dyn Error>> {
        todo!();
    }
}
