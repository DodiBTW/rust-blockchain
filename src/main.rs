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
use std::net::SocketAddr;
use crate::network::chain_host::ChainHost;
use clap::Parser;

use tokio::sync::Mutex;
use std::sync::Arc;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct CliArgs {
    #[arg(short, long, default_value_t = false)]
    menu: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = CliArgs::parse();
    let host = "127.0.0.1";
    let port = 51100;
    let addr: SocketAddr = format!("{}:{}", host, port).parse()?;
    let blockchain = Arc::new(Mutex::new(Blockchain::new()));
    let menu_blockchain = Arc::clone(&blockchain);
    let peer_manager : Arc<PeerManager> = Arc::new(PeerManager { peers: vec![] });
    let server_task = tokio::spawn(async move {
        let chain_host = ChainHost {
            address: addr.to_string(),
            chain: blockchain,
            peer_manager: peer_manager,
        };

        println!("🔫 Server locked in at {}", addr);

        Server::builder()
            .add_service(ChainServiceServer::new(chain_host))
            .serve(addr)
            .await
            .unwrap();
    });
    if args.menu {
        loop {
            let mut locked_chain = menu_blockchain.lock().await;
            let end = user_choice(&mut locked_chain).await;
            drop(locked_chain);

            if end {
                break;
            }
            
        }
        server_task.abort();
        println!("Thank you for participating!");
    }
    else{
        let _ = server_task.await; // Keep server running infinitely
    }

    Ok(())
}
