use tonic::transport::Channel;
use crate::network::chain::chain_service_client::ChainServiceClient;
use crate::network::chain::Empty;

pub async fn ping(peer_addr: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut client = ChainServiceClient::connect(format!("http://{}", peer_addr)).await?;

    let request = tonic::Request::new(Empty {});

    let response = client.ping(request).await?;
    if response.into_inner().message == "pong" {
        println!("👊 Ping to {} succeeded", peer_addr);
        Ok(())
    } else {
        Err("Ping response was not pong".into())
    }
}
