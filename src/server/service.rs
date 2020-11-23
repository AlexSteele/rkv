// GRPC service wrappers
use super::Server;
use crate::error::Result;
use crate::proto;
use log::trace;
use std::sync::Arc;
use tonic;

pub struct RkvService {
    pub server: Arc<Server>,
}
pub struct PeerService {
    pub server: Arc<Server>,
}

#[tonic::async_trait]
impl proto::rkv_service_server::RkvService for RkvService {
    async fn describe_cluster(
        &self,
        request: tonic::Request<proto::DescribeClusterRequest>,
    ) -> std::result::Result<tonic::Response<proto::DescribeClusterResponse>, tonic::Status> {
        trace!("describe_cluster");
        map_response(self.server.describe_cluster(request.into_inner()).await)
    }

    async fn put(
        &self,
        request: tonic::Request<proto::PutRequest>,
    ) -> std::result::Result<tonic::Response<proto::PutResponse>, tonic::Status> {
        trace!("put");
        map_response(self.server.put(request.into_inner()).await)
    }

    async fn get(
        &self,
        request: tonic::Request<proto::GetRequest>,
    ) -> std::result::Result<tonic::Response<proto::GetResponse>, tonic::Status> {
        trace!("get");
        map_response(self.server.get(request.into_inner()).await)
    }

    async fn delete(
        &self,
        request: tonic::Request<proto::DeleteRequest>,
    ) -> std::result::Result<tonic::Response<proto::DeleteResponse>, tonic::Status> {
        trace!("delete");
        map_response(self.server.delete(request.into_inner()).await)
    }

    async fn heartbeat(
        &self,
        request: tonic::Request<proto::HeartbeatRequest>,
    ) -> std::result::Result<tonic::Response<proto::HeartbeatResponse>, tonic::Status> {
        trace!("heartbeat");
        map_response(self.server.heartbeat(request.into_inner()).await)
    }
}

#[tonic::async_trait]
impl proto::peer_service_server::PeerService for PeerService {
    async fn describe_cluster(
        &self,
        request: tonic::Request<proto::DescribeClusterRequest>,
    ) -> std::result::Result<tonic::Response<proto::DescribeClusterResponse>, tonic::Status> {
        trace!("describe_cluster");
        map_response(self.server.describe_cluster(request.into_inner()).await)
    }

    async fn direct_put(
        &self,
        request: tonic::Request<proto::PutRequest>,
    ) -> std::result::Result<tonic::Response<proto::PutResponse>, tonic::Status> {
        trace!("direct_put");
        map_response(self.server.direct_put(request.into_inner()).await)
    }

    async fn direct_get(
        &self,
        request: tonic::Request<proto::GetRequest>,
    ) -> std::result::Result<tonic::Response<proto::GetResponse>, tonic::Status> {
        trace!("direct_put");
        map_response(self.server.direct_get(request.into_inner()).await)
    }

    async fn direct_delete(
        &self,
        request: tonic::Request<proto::DeleteRequest>,
    ) -> std::result::Result<tonic::Response<proto::DeleteResponse>, tonic::Status> {
        trace!("direct_delete");
        map_response(self.server.direct_delete(request.into_inner()).await)
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
        trace!("heartbeat");
        map_response(self.server.heartbeat(request.into_inner()).await)
    }
}

// TODO: Fix response error
fn map_response<T>(resp: Result<T>) -> std::result::Result<tonic::Response<T>, tonic::Status> {
    resp.map(|result| tonic::Response::new(result))
        .map_err(|e| tonic::Status::new(tonic::Code::Internal, format!("{:?}", e)))
}
