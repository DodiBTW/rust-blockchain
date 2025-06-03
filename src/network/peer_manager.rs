use std::collections::HashMap;
use crate::network::peer_client::PeerClient;
#[derive(Debug,Clone)]
pub struct PeerManager{
    pub peers : Vec<String>,
    pub inactive_pinged_peers: HashMap<String, u8>,
    pub max_strikes: u8,
    pub client: PeerClient,
}

impl PeerManager {
    pub fn get_peers(&self) -> Vec<String> {
        return self.peers.clone();
    }
    pub async fn ping_peers(&mut self) {
        for peer in &self.peers {
            match self.client.ping(peer).await {
                Ok(_) => {
                    if self.inactive_pinged_peers.contains_key(peer){
                        self.inactive_pinged_peers.remove(peer);
                    }
                    continue;
                },
                Err(_) => {
                    println!("❌ No response from {}.", peer);
                    self.inactive_pinged_peers
                    .entry(peer.clone())
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
                },
            }
        }
        self.cleanup_dead_peers();
    }
    pub fn remove_peer(&mut self, peer_address: String){
        match self.peers.iter().position(|p| *p == peer_address){
            Some(x) => {
                self.peers.remove(x);
            }
            None => return,
        }
    }
    pub fn add_peer(&mut self, peer_address: String){
        if !self.peers.contains(&peer_address) {
            // Make sure it's not our address
            if peer_address == self.client.address {
                println!("Cannot add self as a peer.");
                return;
            }
            self.peers.push(peer_address);
        }
    }
    fn cleanup_dead_peers (&mut self){
        let removable : Vec<_> = self.inactive_pinged_peers.iter()
        .filter(|(_, count)| **count >= self.max_strikes)
        .map(|(peer, _)| peer.clone())
        .collect();
        for peer in removable {
            self.inactive_pinged_peers.remove(&peer);
            self.remove_peer(peer);
        }
    }
}