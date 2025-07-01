use crate::network::chain::chain_service_client::ChainServiceClient;
use crate::network::chain::Empty;
use crate::blockchain::block::Block;
#[derive(Debug, Clone)]
pub struct PeerClient{
    pub address: String,
}

impl PeerClient {
    pub fn new(address: String) -> Self {
        PeerClient { address }
    }

    pub async fn ping(&self, peer_addr: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut client = ChainServiceClient::connect(format!("http://{}", peer_addr)).await?;

        let request = tonic::Request::new(Empty {});

        let response = client.ping(request).await?;
        if response.into_inner().message == "pong" {
            Ok(())
        } else {
            println!("Ping failed to {}.", peer_addr);
            Err("Ping response was not pong".into())
        }
    }

    pub async fn get_peers(&self, peer_addr: &str) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
        let mut client = ChainServiceClient::connect(format!("http://{}", peer_addr)).await?;

        let request = tonic::Request::new(Empty {});

        let response = client.get_peers(request).await?;
        let peers = response.into_inner().peers;
        Ok(peers)
    }
    pub async fn get_chain(&self, peer_addr: &str) -> Result<Vec<Block>, Box<dyn std::error::Error + Send + Sync>> {
        let mut client = ChainServiceClient::connect(format!("http://{}", peer_addr)).await?;
        let request = tonic::Request::new(Empty {});
        let response = client.get_chain(request).await?;
        let proto_blocks = response.into_inner().blocks;
        // Convert ProtoBlock to Block
        let blocks = proto_blocks.into_iter().map(Block::from).collect();
        Ok(blocks)
    }
    pub async fn send_added_block(
        &self,
        peer_addr: &str,
        block: crate::blockchain::block::Block,
    ) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        let mut client = ChainServiceClient::connect(format!("http://{}", peer_addr)).await?;

        let proto_block: crate::network::chain::ProtoBlock = (&block).into();
        let request = tonic::Request::new(proto_block);

        let response = client.receive_added_block(request).await?;
        let result = response.into_inner().value;
        println!("✅ Block added to {}: {}", peer_addr, result);
        Ok(result)
    }
    pub async fn send_block_proposition(
        &self,
        peer_addr: &str,
        block: crate::blockchain::block::Block,
    ) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        let mut client = ChainServiceClient::connect(format!("http://{}", peer_addr)).await?;

        let proto_block: crate::network::chain::ProtoBlock = (&block).into();
        let request = tonic::Request::new(proto_block);

        let response = client.receive_block_proposition(request).await?;
        let result = response.into_inner().value;
        println!("✅ Block proposition sent to {}: {}", peer_addr, result);
        Ok(result)
    }
    pub async fn send_peer_add(
        &self,
        peer_addr: &str,
    ) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        println!("Adding peer at address: {}", peer_addr);
        let mut client = ChainServiceClient::connect(format!("http://{}", peer_addr)).await?;

        let request = tonic::Request::new(crate::network::chain::PeerAdd { address: self.address.clone() });

        let response = client.receive_peer_add(request).await?;
        let result = response.into_inner().value;
        println!("✅ Peer added to {}: {}", peer_addr, result);
        Ok(result)
    }
}