use crate::blockchain::chain::Blockchain;
use tonic::{Request, Response, Status};
use crate::blockchain::block::Block;
use std::collections::HashMap;

#[derive(Debug)]
pub struct GrpcHost {
    pub address: String,
    pub peers : Vec<String>,
    pub chain: Blockchain,
    pub inactive_pinged_peers: HashMap<String, u64>, // 5 pings unanswered --> remove from peers
}
impl GrpcHost{
    async fn ping(&self, _req : Request<()>) -> Result<Response<String>, Status> {
        Ok(Response::new("pong".to_string()))
    }
    async fn get_peers(&self, _req: Request<()>) -> Result<Response<Vec<String>>, Status> {
        Ok(Response::new(self.peers.clone()))
    }
    async fn get_chain(&self, _req: Request<()>) -> Result<Response<Blockchain>, Status> {
        Ok(Response::new(self.chain.clone()))
    }
    async fn receive_added_block(&mut self, _req : Request<Block>) -> Result<Response<bool>, Status>{
        // This is for blocks received by validators
        let new_block : Block = _req.into_inner();
        let resp = self.chain.add_block(&mut new_block);
        return Ok(Response::new(resp));
    }
    async fn receive_block_proposition(&mut self, _req: Request<Block>) -> Result<Response<bool>, Status>{
        // This is for blocks received by non validators
        let mut new_block = _req.into_inner();
        let last_block = self.chain.blocks.last().unwrap();
        if(new_block.is_valid()){
            if(new_block.prev_hash != last_block.hash){
                // maybe we can warn the client? not necessary but could be useful.
                new_block.prev_hash = last_block.hash.clone();
            }
            self.chain.add_block(new_block);
            return Ok(Response::new(true))
        }
        else{
            return Ok(Response::new(false))
        }
    }
}