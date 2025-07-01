pub mod blockchain;
pub mod cli;
pub mod network;
pub mod chain {
    tonic::include_proto!("chain");
}
use rand::Rng;
use blockchain::chain::Blockchain;
use cli::command::user_choice;
use network::peer_manager::PeerManager;
use tonic::transport::Server;
use crate::network::chain::chain_service_server::{ChainServiceServer};
use std::{collections::HashMap, net::SocketAddr};
use crate::network::chain_host::ChainHost;
use rand::distributions::Alphanumeric;
use clap::Parser;
use tokio::sync::Mutex;
use std::sync::Arc;
use std::time::Instant;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct CliArgs {
    #[arg(short, long, default_value_t = false)]
    menu: bool,

    #[arg(short, long, default_value_t = 51100)]
    port: u16,

    #[arg(short, long, default_value_t = false)]
    benchmark: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = CliArgs::parse();

    let host = "127.0.0.1";
    let port = args.port;
    let addr: SocketAddr = format!("{}:{}", host, port).parse()?;

    let max_strikes = 3; // How many pings before disconnecting an inactive peer
    let ping_delay = 10; // how many seconds before repinging

    let benchmark_data_size = 1024; // size of each block in bytes
    let benchmark_amount = 10000; // how many blocks to create in the benchmark
    let benchmark_peers = ["127.0.0.1:6000"]; // Peers to benchmark.

    let sync_delay = 5; // how many seconds before syncing the blockchain with peers

    let blockchain = Arc::new(Mutex::new(Blockchain::new()));
    let menu_blockchain = Arc::clone(&blockchain);
    let inactive: HashMap<String, u8> = HashMap::new();
    let peer_client = Arc::new(Mutex::new(network::peer_client::PeerClient::new(addr.to_string())));
    let peer_client_for_menu = Arc::clone(&peer_client);
    let peer_manager = Arc::new(Mutex::new(PeerManager {
        peers: vec![],
        inactive_pinged_peers: inactive,
        max_strikes: max_strikes,
        client: Arc::clone(&peer_client),
    }));
    let peer_manager_for_server = Arc::clone(&peer_manager);
    let peer_manager_for_ping = Arc::clone(&peer_manager);
    let peer_manager_for_menu = Arc::clone(&peer_manager);
    let blockchain_for_chain_sync = Arc::clone(&blockchain);
    let peer_manager_for_chain_sync = Arc::clone(&peer_manager);
    let peer_client_for_chain_sync = Arc::clone(&peer_client_for_menu);
    let peer_manager_for_network_sync = Arc::clone(&peer_manager);
    let peer_client_for_network_sync = Arc::clone(&peer_client_for_menu);
    if args.benchmark {
        for peer in benchmark_peers {
            peer_manager.lock().await.add_peer(peer, true).await;
        }
    }
    let server_task = tokio::spawn(async move {
        let chain_host = ChainHost {
            address: addr.to_string(),
            chain: blockchain,
            peer_manager: peer_manager_for_server,
        };

        println!("Server locked in at {}", addr);

        Server::builder()
            .add_service(ChainServiceServer::new(chain_host))
            .serve(addr)
            .await
            .unwrap();
    });

    let ping_task = {
        tokio::spawn(async move {
            loop {
                {
                    let mut pm = peer_manager_for_ping.lock().await;
                    pm.ping_peers().await;
                }
                tokio::time::sleep(tokio::time::Duration::from_secs(ping_delay)).await;
            }
        })
    };

    let network_sync_task = {
        tokio::spawn(async move {
            loop {
                {
                    let peers = peer_manager_for_network_sync.lock().await.get_peers();
                    for peer in &peers {
                        let received_peers = peer_client_for_network_sync.lock().await.get_peers(peer).await;
                        if let Ok(received_peers) = received_peers {
                            for new_peer in received_peers {
                                if !peer_manager_for_network_sync.lock().await.peers.contains(&new_peer) {
                                    peer_manager_for_network_sync.lock().await.add_peer(&new_peer, true).await;
                                }
                            }
                        } else {
                            println!("Failed to get peers from {}", peer);
                        }
                    }
                }
                tokio::time::sleep(tokio::time::Duration::from_secs(sync_delay)).await;
            }
        })
    };

    let chain_sync_task = {
        tokio::spawn(async move {
            loop {
                {
                    let peers = peer_manager_for_chain_sync.lock().await.get_peers();

                    // Sync with peers
                    for peer in peers {
                        if let Ok(peers_chain) = peer_client_for_chain_sync.lock().await.get_chain(&peer).await {
                            // If chain longer and valid, replace current chain
                            if peers_chain.len() > blockchain_for_chain_sync.lock().await.blocks.len() && peers_chain.iter().all(|block| block.is_valid()) {
                                println!("Syncing with peer: {}", peer);
                                blockchain_for_chain_sync.lock().await.blocks = peers_chain;
                            }
                        }
                    }
                }
                tokio::time::sleep(tokio::time::Duration::from_secs(sync_delay)).await;
            }
        })
    };


    if args.benchmark {
        let mut handles = vec![];
        println!("Starting benchmark: sending {} blocks of {} bytes each", benchmark_amount, benchmark_data_size);
        let blockchain = Arc::clone(&menu_blockchain);
        let handle = tokio::spawn(async move {
            let start = Instant::now();
            let mut total_size = 0;

            for _ in 0..benchmark_amount {
            let data: String = rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(benchmark_data_size)
                .map(char::from)
                .collect();
            let timestamp = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs();
            blockchain.lock().await.create_block(data, timestamp);
            total_size += benchmark_data_size;
            }

            let duration = start.elapsed();
            let speed = total_size as f64 / duration.as_secs_f64();
            println!("Benchmark: {} bytes/sec", speed);
        });
        handles.push(handle);

        futures::future::join_all(handles).await;
    }


    if args.menu {
        loop {
            let end = user_choice(
                &menu_blockchain,
                &peer_manager_for_menu,
                &peer_client_for_menu
            ).await;

            if end {
                break;
            }
        }
        // Clean shutdown
        server_task.abort();
        ping_task.abort();
        println!("Thank you for participating!");
    } else {
        let _ = server_task.await;
        ping_task.abort();
        chain_sync_task.abort();
        network_sync_task.abort();
    }
    

    

    Ok(())
}
