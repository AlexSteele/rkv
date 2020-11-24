use log::*;
use rkv::proto::peer_service_server::PeerServiceServer;
use rkv::proto::rkv_service_server::RkvServiceServer;
use rkv::server::{Config, PeerService, RkvService, Server};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    stderrlog::new()
        .module(module_path!())
        .verbosity(5)
        .init()
        .unwrap();

    let config = Config::parse_from_args();
    let addr = config.address.parse()?;
    let server = Arc::new(Server::new(config));
    let rkv_service = RkvService {
        server: server.clone(),
    };
    let peer_service = PeerService { server: server };

    info!("starting rkv server at {}", addr);

    tonic::transport::Server::builder()
        .add_service(RkvServiceServer::new(rkv_service))
        .add_service(PeerServiceServer::new(peer_service))
        .serve(addr)
        .await?;

    Ok(())
}
