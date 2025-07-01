use std::collections::HashMap;
use crate::network::peer_client::PeerClient;
use tokio::sync::Mutex;
use std::sync::Arc;
#[derive(Debug,Clone)]
pub struct PeerManager{
    pub peers : Vec<String>,
    pub inactive_pinged_peers: HashMap<String, u8>,
    pub max_strikes: u8,
    pub client: Arc<Mutex<PeerClient>>,
}

impl PeerManager {
    pub fn get_peers(&self) -> Vec<String> {
        return self.peers.clone();
    }
    pub async fn ping_peers(&mut self) {
        for peer in &self.peers {
            match self.client.lock().await.ping(peer).await {
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
    pub async fn add_peer(&mut self, peer_address: &str, mut propagate: bool) {
        {
            let self_address = self.client.lock().await.address.clone();
            if !self.peers.contains(&peer_address.to_string()) {
                if peer_address == self_address {
                    return;
                }
                self.peers.push(peer_address.to_string());
            } else {
                println!("Peer {} already exists.", peer_address);
                propagate = false;
            }
        };
        if propagate {
            let _ = self.client.lock().await.send_peer_add(&peer_address).await;
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