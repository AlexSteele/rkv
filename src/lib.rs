pub mod ring;
pub mod server;
pub mod store;
pub mod proto {
    tonic::include_proto!("rkv");
}
