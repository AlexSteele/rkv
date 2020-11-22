use log::*;
use rkv::proto::rkv_service_server::RkvServiceServer;
use rkv::proto::peer_service_server::PeerServiceServer;
use rkv::server::{Config, State, RkvService, PeerService};
use tokio::prelude::*;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    stderrlog::new()
        .module(module_path!())
        .verbosity(5)
        .init()
        .unwrap();

    let config = Config::from_args()?;
    let addr = config.address.parse()?;
    let state = Arc::new(State::new(config));
    let rkv_service = RkvService::new(state.clone());
    let peer_service = PeerService::new(state);

    info!("starting rkv server at {}", addr);

    tonic::transport::Server::builder()
        .add_service(RkvServiceServer::new(rkv_service))
        .add_service(PeerServiceServer::new(peer_service))
        .serve(addr)
        .await?;

    Ok(())
}
