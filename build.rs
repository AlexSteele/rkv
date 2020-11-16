fn main() {
    tonic_build::configure()
        .compile(
            &[
                "proto/config.proto",
                "proto/rkv_service.proto",
                "proto/net_service.proto",
                "proto/storage_service.proto",
            ],
            &["proto"],
        )
        .unwrap();
}
