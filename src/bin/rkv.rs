use log::*;
use rkv::proto::rkv_service_server::RkvServiceServer;
use rkv::server::{Config, Server};
use tokio::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    stderrlog::new()
        .module(module_path!())
        .verbosity(5)
        .init()
        .unwrap();

    let config = Config::from_args()?;
    let addr = config.address.parse()?;
    let server = Server::new(config);

    info!("starting rkv server at {}", addr);

    tonic::transport::Server::builder()
        .add_service(RkvServiceServer::new(server))
        .serve(addr)
        .await?;

    Ok(())
}
