pub mod chain_host;
pub mod peer_manager;
pub mod peer_client;
pub mod chain {
    tonic::include_proto!("chain");
}
