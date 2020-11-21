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

pub struct Server {
    config: Config,
    store: Box<dyn store::Store>,
}

impl Server {
    pub fn new(config: Config) -> Self {
        Self {
            config: config,
            store: Box::new(store::MemStore::new()),
        }
    }
}

#[tonic::async_trait]
impl proto::rkv_service_server::RkvService for Server {
    async fn describe_cluster(
        &self,
        request: tonic::Request<proto::DescribeClusterRequest>,
    ) -> std::result::Result<tonic::Response<proto::DescribeClusterResponse>, tonic::Status> {
        trace!("describe_cluster");
        Ok(tonic::Response::new(proto::DescribeClusterResponse {
            cluster_config: Some(self.config.cluster_config.clone()),
        }))
    }

    async fn put(
        &self,
        request: tonic::Request<proto::PutRequest>,
    ) -> std::result::Result<tonic::Response<proto::PutResponse>, tonic::Status> {
        trace!("put");
        let req = request.into_inner();
        self.store
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
        let result = self
            .store
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
        let result = self
            .store
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

pub fn default_cluster_config() -> ClusterConfig {
    ClusterConfig {
        name: "default".to_string(),
        replication_factor: 3,
        read_replicas: 2,
        write_replicas: 2,
        ring_replicas: 8,
    }
}
