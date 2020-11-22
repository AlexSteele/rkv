use super::{Error, Result};
use crate::proto;
use crate::proto::rkv_service_server::RkvServiceServer;
use crate::proto::ClusterConfig;
use crate::store;
use crate::{Key, Value};
use log::{trace};
use std::sync::Arc;
use tonic;

// host:port
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

// State shared between RkvService/PeerService
pub struct State {
    config: Config,
    store: Box<dyn store::Store>
}
impl State {
    pub fn new(config: Config) -> Self {
        Self {
            config: config,
            store: Box::new(store::MemStore::new()),
        }
    }
}

pub struct RkvService {
    state: Arc<State>
}
impl RkvService {
    pub fn new(state: Arc<State>) -> Self {
        Self { state }
    }
}

pub struct PeerService {
    state: Arc<State>
}
impl PeerService {
    pub fn new(state: Arc<State>) -> Self {
        Self { state }
    }
}

#[tonic::async_trait]
impl proto::rkv_service_server::RkvService for RkvService {
    async fn describe_cluster(
        &self,
        request: tonic::Request<proto::DescribeClusterRequest>,
    ) -> std::result::Result<tonic::Response<proto::DescribeClusterResponse>, tonic::Status> {
        trace!("describe_cluster");
        Ok(tonic::Response::new(proto::DescribeClusterResponse {
            cluster_config: Some(self.state.config.cluster_config.clone()),
        }))
    }

    async fn put(
        &self,
        request: tonic::Request<proto::PutRequest>,
    ) -> std::result::Result<tonic::Response<proto::PutResponse>, tonic::Status> {
        trace!("put");
        let req = request.into_inner();
        self.state.store
            .put(Key(req.key), req.value)
            .map(|version| tonic::Response::new(proto::PutResponse { version }))
            .map_err(|e| tonic::Status::new(tonic::Code::Internal, "storage error"))
    }

    async fn get(
        &self,
        request: tonic::Request<proto::GetRequest>,
    ) -> std::result::Result<tonic::Response<proto::GetResponse>, tonic::Status> {
        trace!("get");
        let req = request.into_inner();
        let result = self.state.store
            .get(&Key(req.key))
            .map_err(|e| tonic::Status::new(tonic::Code::Internal, "storage error"))?;
        let (value, version) = result.unwrap_or((Vec::new(), -1));
        Ok(tonic::Response::new(proto::GetResponse { value, version }))
    }

    async fn delete(
        &self,
        request: tonic::Request<proto::DeleteRequest>,
    ) -> std::result::Result<tonic::Response<proto::DeleteResponse>, tonic::Status> {
        trace!("delete");
        let req = request.into_inner();
        let result = self.state.store
            .delete(&Key(req.key))
            .map_err(|e| tonic::Status::new(tonic::Code::Internal, "storage error"))?;
        let (value, version) = result.unwrap_or((Vec::new(), -1));
        Ok(tonic::Response::new(proto::DeleteResponse { value }))
    }

    async fn heartbeat(
        &self,
        request: tonic::Request<proto::HeartbeatRequest>,
    ) -> std::result::Result<tonic::Response<proto::HeartbeatResponse>, tonic::Status> {
        trace!("heartbeat");
        Ok(tonic::Response::new(proto::HeartbeatResponse {}))
    }
}

#[tonic::async_trait]
impl proto::peer_service_server::PeerService for PeerService {
    async fn describe_cluster(
        &self,
        request: tonic::Request<proto::DescribeClusterRequest>,
    ) -> std::result::Result<tonic::Response<proto::DescribeClusterResponse>, tonic::Status> {
        todo!();
    }

    async fn direct_put(
        &self,
        request: tonic::Request<proto::PutRequest>,
    ) -> std::result::Result<tonic::Response<proto::PutResponse>, tonic::Status> {
        todo!();
    }

    async fn direct_get(
        &self,
        request: tonic::Request<proto::GetRequest>,
    ) -> std::result::Result<tonic::Response<proto::GetResponse>, tonic::Status> {
        todo!();
    }

    async fn direct_delete(
        &self,
        request: tonic::Request<proto::DeleteRequest>,
    ) -> std::result::Result<tonic::Response<proto::DeleteResponse>, tonic::Status> {
        todo!();
    }

    async fn join_network(
        &self,
        request: tonic::Request<proto::JoinNetworkRequest>,
    ) -> std::result::Result<tonic::Response<proto::JoinNetworkResponse>, tonic::Status> {
        todo!();
    }

    async fn leave_network(
        &self,
        request: tonic::Request<proto::LeaveNetworkRequest>,
    ) -> std::result::Result<tonic::Response<proto::LeaveNetworkResponse>, tonic::Status> {
        todo!();
    }

    async fn gossip(
        &self,
        request: tonic::Request<proto::GossipRequest>,
    ) -> std::result::Result<tonic::Response<proto::GossipResponse>, tonic::Status> {
        todo!();
    }

    async fn heartbeat(
        &self,
        request: tonic::Request<proto::HeartbeatRequest>,
    ) -> std::result::Result<tonic::Response<proto::HeartbeatResponse>, tonic::Status> {
        todo!();
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
