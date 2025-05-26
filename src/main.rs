pub mod blockchain;
pub mod cli;
pub mod network;
pub mod chain {
    tonic::include_proto!("chain");
}

use blockchain::chain::Blockchain;
use cli::command::user_choice;
use tonic::transport::Server;
use crate::network::chain::chain_service_server::{ChainServiceServer};
use std::{collections::HashMap, net::SocketAddr};
use crate::network::chain_host::ChainHost;


use tokio::sync::Mutex;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let host = "127.0.0.1";
    let port = 51100;
    let addr: SocketAddr = format!("{}:{}", host, port).parse()?;

    // Share blockchain state safely across threads/tasks
    let blockchain = Arc::new(Mutex::new(Blockchain::new()));
    let menu_blockchain = Arc::clone(&blockchain);


    // Spawn the gRPC server in background task
    let server_task = tokio::spawn(async move {
        let chain_host = ChainHost {
            address: addr.to_string(),
            peers: Vec::new(),
            chain: blockchain,
            inactive_pinged_peers: HashMap::new(),
        };

        println!("🔫 Server locked in at {}", addr);

        Server::builder()
            .add_service(ChainServiceServer::new(chain_host))
            .serve(addr)
            .await
            .unwrap();
    });

    loop {
        let mut locked_chain = menu_blockchain.lock().await;
        let end = user_choice(&mut locked_chain).await;
        drop(locked_chain);

        if end {
            break;
        }
    }

    println!("Thank you for participating!");

    server_task.abort();

    Ok(())
}
