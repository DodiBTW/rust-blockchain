use crate::blockchain::chain::Blockchain;
use crate::network::chain::chain_service_server::ChainService;
use tonic::{Request, Response, Status};
use crate::blockchain::block::Block;
use tokio::sync::Mutex;
use std::sync::Arc;
use crate::network::chain::*;
use crate::network::peer_manager::PeerManager;


#[derive(Debug, Clone)]
pub struct ChainHost {
    pub address: String,
    pub chain: Arc<Mutex<Blockchain>>,
    pub peer_manager: Arc<Mutex<PeerManager>>
}
#[tonic::async_trait]
impl ChainService for ChainHost{
    async fn ping(&self, _req: Request<Empty>) -> Result<Response<StringReply>, Status> {
        Ok(Response::new(StringReply { message: "pong".to_string() }))
    }
    async fn get_peers(&self, _req: Request<Empty>) -> Result<Response<PeerList>, Status> {
        let peer_manager = self.peer_manager.lock().await;
        let peers = peer_manager.get_peers();
        Ok(Response::new(PeerList { peers: peers }))
    }
    async fn get_chain(&self, _req: Request<Empty>) -> Result<Response<ProtoBlockchain>, Status> {
        let chain = self.chain.lock().await;
        let proto_blocks = chain.blocks
            .iter()
            .map(ProtoBlock::from)
            .collect();

        Ok(Response::new(ProtoBlockchain { blocks: proto_blocks }))
    }
    async fn receive_peer_add(&self, _req: Request<PeerAdd>) -> Result<Response<BoolReply>, Status> {
        println!("Received peer add request");
        let peer_add: PeerAdd = _req.into_inner();
        let mut peer_manager = self.peer_manager.lock().await;
        peer_manager.add_peer(peer_add.address.as_str(), false).await;
        println!("Peer {} added successfully!", peer_add.address);
        return Ok(Response::new(BoolReply { value: true }));
    }
    async fn receive_added_block(&self, _req : Request<ProtoBlock>) -> Result<Response<BoolReply>, Status>{
        // This is for blocks received by validators
        let proto_block : ProtoBlock = _req.into_inner();
        let mut chain = self.chain.lock().await;
        let new_block: Block = proto_block.into();
        let resp = chain.add_block(&new_block);
        return Ok(Response::new(BoolReply {value : resp}));
    }
    async fn receive_block_proposition(&self, _req: Request<ProtoBlock>) -> Result<Response<BoolReply>, Status>{
        // This is for blocks received by non validators
        let proto_block: ProtoBlock = _req.into_inner();
        let mut new_block: Block = proto_block.into();
        let mut chain = self.chain.lock().await;
        let last_block = chain.blocks.last().unwrap();
        if new_block.is_valid() {
            if new_block.prev_hash != last_block.hash {
                // maybe we can warn the client? not necessary but could be useful.
                new_block.prev_hash = last_block.hash.clone();
            }
            chain.add_block(&mut new_block);
            return Ok(Response::new(BoolReply {value : true}))
        }
        else{
            return Ok(Response::new(BoolReply {value : false}))
        }
    }
}