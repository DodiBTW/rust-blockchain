use std::collections::HashMap;
use crate::network::peer_client::*;

use super::peer_client::ping;
#[derive(Debug,Clone)]
pub struct PeerManager{
    pub peers : Vec<String>,
    pub inactive_pinged_peers: HashMap<String, u8>,
}

impl PeerManager{
    pub fn get_peers(&self) -> Vec<String>{
        return self.peers.clone();
    }
    pub async fn ping_peers(&mut self) {
        for peer in &self.peers {
            match ping(peer).await {
                Ok(_) => return,
                Err(e) => {
                    println!("❌ No response from {}: {:?}", peer, e);
                    self.inactive_pinged_peers
                    .entry(peer.clone())
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
                },
            }
        }
    }
    pub fn remove_peer(&mut self, peer_address: String){
        match self.peers.iter().position(|p| *p == peer_address){
            Some(x) => {
                self.peers.remove(x);
            }
            None => return,
        }
    }
}