use crate::error::{Error, Result};
use crate::proto;
use crate::proto::peer_service_client::PeerServiceClient;
use crate::proto::ClusterConfig;
use crate::ring::HashRing;
use crate::store;
use crate::{Key, Version};
use log::trace;
use std::collections::HashMap;
use std::sync::Arc;
use tonic;

// RKV node address. host:port
pub type NodeAddr = String;

#[derive(Debug, Clone)]
pub struct Config {
    pub folder: String,
    pub address: NodeAddr,
    pub seed_nodes: Vec<NodeAddr>,
    pub cluster_config: ClusterConfig,
}
impl Config {
    pub fn from_args() -> Result<Config> {
        // TODO: parse
        Ok(Config {
            folder: "".to_string(),
            address: "127.0.0.1:8080".to_string(),
            seed_nodes: Vec::new(),
            cluster_config: default_cluster_config(),
        })
    }
}

pub struct Server {
    config: Config,
    ring: HashRing<NodeAddr>,
    store: Box<dyn store::Store>,
}

// TODO: Parallelize put/get/delete
impl Server {
    pub fn new(config: Config) -> Self {
        Self {
            ring: Self::make_ring(&config),
            config: config,
            store: Box::new(store::MemStore::new()),
        }
    }

    // TODO: Fetch peers from network
    fn make_ring(config: &Config) -> HashRing<NodeAddr> {
        let mut ring = HashRing::new(config.cluster_config.ring_replicas);
        ring.insert(config.address.clone());
        for peer in &config.seed_nodes {
            ring.insert(peer.clone());
        }
        ring
    }

    pub async fn describe_cluster(
        &self,
        req: proto::DescribeClusterRequest,
    ) -> Result<proto::DescribeClusterResponse> {
        Ok(proto::DescribeClusterResponse {
            cluster_config: Some(self.config.cluster_config.clone()),
        })
    }

    // TODO: Assign version if not given
    // TODO: Retry failed puts/hinted handoff
    pub async fn put(&self, req: proto::PutRequest) -> Result<proto::PutResponse> {
        self.check_key(&req.key)?;

        let replicas = self.find_replicas(&req.key)?;

        let mut results = Vec::new();
        for addr in replicas {
            results.push(self.remote_put(addr, req.clone()).await);
        }

        let (successes, failures): (Vec<_>, Vec<_>) =
            results.iter().partition(|result| result.is_ok());

        for result in failures {
            trace!("put error: {:?}", result);
        }

        let write_replicas = self.config.cluster_config.write_replicas as usize;
        if successes.len() < write_replicas {
            return Err(Error::TooFewReplicas);
        }

        Ok(proto::PutResponse { version: -1 })
    }

    pub async fn get(&self, req: proto::GetRequest) -> Result<proto::GetResponse> {
        self.check_key(&req.key)?;

        let replicas = self.find_replicas(&req.key)?;

        let mut results = Vec::new();
        for addr in replicas {
            results.push(self.remote_get(addr, req.clone()).await);
        }

        let (successes, failures): (Vec<_>, Vec<_>) =
            results.iter().partition(|result| result.is_ok());

        for result in failures {
            trace!("get error: {:?}", result);
        }

        // Find the most frequent version
        let mut version_counts: HashMap<Version, i32> = HashMap::new();
        for result in &successes {
            let count = version_counts
                .entry(result.as_ref().unwrap().version)
                .or_insert(0);
            *count += 1;
        }
        let (version, count) = version_counts
            .iter()
            .max_by_key(|(version, count)| (*count, *version))
            .map(|(version, count)| (*version, *count))
            .unwrap_or((-1, -1));

        let read_replicas = self.config.cluster_config.write_replicas;
        if count < read_replicas {
            return Err(Error::TooFewReplicas);
        }

        let response = successes
            .iter()
            .map(|result| result.as_ref().unwrap())
            .find(|response| response.version == version)
            .expect("no matching response");

        Ok(response.clone())
    }

    // TODO: Retry failed deletes
    pub async fn delete(&self, req: proto::DeleteRequest) -> Result<proto::DeleteResponse> {
        self.check_key(&req.key)?;

        let replicas = self.find_replicas(&req.key)?;

        let mut results = Vec::new();
        for addr in replicas {
            results.push(self.remote_delete(addr, req.clone()).await);
        }

        let (successes, failures): (Vec<_>, Vec<_>) =
            results.iter().partition(|result| result.is_ok());

        for result in failures {
            trace!("delete error: {:?}", result);
        }

        let write_replicas = self.config.cluster_config.write_replicas as usize;
        if successes.len() < write_replicas {
            return Err(Error::TooFewReplicas);
        }

        // TODO: return val
        Ok(proto::DeleteResponse { value: Vec::new() })
    }

    pub async fn direct_put(&self, req: proto::PutRequest) -> Result<proto::PutResponse> {
        self.check_key(&req.key)?;
        self.store
            .put(Key(req.key), req.value)
            .map(|version| proto::PutResponse { version })
    }

    pub async fn direct_get(&self, req: proto::GetRequest) -> Result<proto::GetResponse> {
        self.check_key(&req.key)?;
        self.store
            .get(&Key(req.key))
            .map(|result| result.unwrap_or((Vec::new(), -1)))
            .map(|(value, version)| proto::GetResponse { value, version })
    }

    pub async fn direct_delete(&self, req: proto::DeleteRequest) -> Result<proto::DeleteResponse> {
        self.check_key(&req.key)?;
        self.store
            .delete(&Key(req.key))
            .map(|result| result.unwrap_or((Vec::new(), -1)))
            .map(|(value, version)| proto::DeleteResponse { value })
    }

    pub async fn heartbeat(
        &self,
        req: proto::HeartbeatRequest,
    ) -> Result<proto::HeartbeatResponse> {
        Ok(proto::HeartbeatResponse {})
    }

    async fn remote_put(
        &self,
        addr: NodeAddr,
        req: proto::PutRequest,
    ) -> Result<proto::PutResponse> {
        if addr == self.config.address {
            return self.direct_put(req).await;
        }

        let mut client = PeerServiceClient::connect(addr).await?;
        let resp = client.direct_put(req).await?;
        Ok(resp.into_inner())
    }

    async fn remote_get(
        &self,
        addr: NodeAddr,
        req: proto::GetRequest,
    ) -> Result<proto::GetResponse> {
        if addr == self.config.address {
            return self.direct_get(req).await;
        }

        let mut client = PeerServiceClient::connect(addr).await?;
        let resp = client.direct_get(req).await?;
        Ok(resp.into_inner())
    }

    async fn remote_delete(
        &self,
        addr: NodeAddr,
        req: proto::DeleteRequest,
    ) -> Result<proto::DeleteResponse> {
        if addr == self.config.address {
            return self.direct_delete(req).await;
        }

        let mut client = PeerServiceClient::connect(addr).await?;
        let resp = client.direct_delete(req).await?;
        Ok(resp.into_inner())
    }

    fn find_replicas(&self, key: &Vec<u8>) -> Result<Vec<NodeAddr>> {
        let replication_factor = self.config.cluster_config.replication_factor as usize;
        let mut replicas = Vec::new();
        let mut iter = self.ring.successors(key);
        while let Some(addr) = iter.next() {
            if replicas.contains(addr) {
                continue;
            }
            replicas.push(addr.clone());
            if replicas.len() == replication_factor {
                break;
            }
        }
        if replicas.len() < replication_factor {
            return Err(Error::TooFewReplicas);
        }
        Ok(replicas)
    }

    fn check_key(&self, key: &Vec<u8>) -> Result<()> {
        if key.is_empty() {
            Err(Error::InvalidArgument("empty key".to_string()))
        } else {
            Ok(())
        }
    }
}

pub fn default_cluster_config() -> ClusterConfig {
    ClusterConfig {
        name: "default".to_string(),
        replication_factor: 3,
        read_replicas: 2,
        write_replicas: 2,
        ring_replicas: 8,
    }
}
