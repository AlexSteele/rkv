fn main() {
    tonic_build::configure()
        .compile(
            &[
                "proto/config.proto",
                "proto/rkv_service.proto",
                "proto/peer_service.proto",
            ],
            &["proto"],
        )
        .unwrap();
}
