use rkv::proto::rkv_service_client::RkvServiceClient;
use rkv::proto::*;
use tokio::prelude::*;

#[tokio::test]
async fn test_rkv() {
    let mut client = RkvServiceClient::connect("http://127.0.0.1:8080")
        .await
        .unwrap();

    let resp = client
        .describe_cluster(DescribeClusterRequest {})
        .await
        .unwrap();

    assert_eq!(resp.into_inner().cluster_config.is_some(), true);

    let resp = client
        .put(PutRequest {
            key: "k0".as_bytes().to_vec(),
            value: "v0".as_bytes().to_vec(),
            version: -1,
        })
        .await
        .unwrap();

    let resp = client
        .get(GetRequest {
            key: "k0".as_bytes().to_vec(),
        })
        .await
        .unwrap();

    assert_eq!(resp.into_inner().value, "v0".as_bytes().to_vec());
}
