pub mod blockchain;
pub mod cli;
pub mod network;
pub mod chain {
    tonic::include_proto!("chain");
}

use blockchain::chain::Blockchain;
use cli::command::user_choice;
use network::peer_manager::PeerManager;
use tonic::transport::Server;
use crate::network::chain::chain_service_server::{ChainServiceServer};
use std::{collections::HashMap, net::SocketAddr};
use crate::network::chain_host::ChainHost;
use clap::Parser;

use tokio::sync::Mutex;
use std::sync::Arc;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct CliArgs {
    #[arg(short, long, default_value_t = false)]
    menu: bool,

    #[arg(short, long, default_value_t = 51100)]
    port: u16,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = CliArgs::parse();

    let host = "127.0.0.1";
    let port = args.port;
    let addr: SocketAddr = format!("{}:{}", host, port).parse()?;

    let max_strikes = 3; // How many pings before disconnecting an inactive peer
    let ping_delay = 2; // how many seconds before repinging

    let blockchain = Arc::new(Mutex::new(Blockchain::new()));
    let menu_blockchain = Arc::clone(&blockchain);
    let inactive :  HashMap<String, u8> =  HashMap::new();
    let peer_client = network::peer_client::PeerClient::new(addr.to_string());
    let peer_client_for_menu = Arc::new(Mutex::new(peer_client.clone()));
    let peer_manager  = Arc::new(Mutex::new(PeerManager { peers: vec![] , inactive_pinged_peers : inactive, max_strikes : max_strikes, client: peer_client }));
    let peer_manager_for_server = Arc::clone(&peer_manager);
    let peer_manager_for_ping = Arc::clone(&peer_manager);
    let peer_manager_for_menu = Arc::clone(&peer_manager);
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
    }

    Ok(())
}
