use tokio::task;

use tokio::sync::Mutex;
use std::sync::Arc;

use crate::blockchain::chain::Blockchain;
use crate::network::peer_client::PeerClient;
use crate::network::peer_manager::PeerManager;

enum Action{
    AddBlock,
    PrintBlocks,
    CheckValidity,
    ClearConsole,
    PrintPeers,
    AddPeer,
    Exit,
}

pub async fn choose_menu() -> String{
    println!("What would you like to do? \n 1 - Add a block \n 2 - Print all blocks \n 3 - Check if our blockchain is valid \n 4 - Clear console \n 5 - Add peer address \n 6 - Print peers \n 0 - Exit");
    let choice = task::spawn_blocking(|| {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        input
    })
    .await
    .expect("Failed to read line asynchronously");

    choice
}

pub fn print_blockchain(blockchain: &Blockchain){
    for block in &blockchain.blocks {
        println!("Block {}: \n hash: {}, \n data: {}", block.index, block.hash, block.data);
    }
}

pub fn clear_console() {
    if cfg!(target_os = "windows") {
        std::process::Command::new("cmd")
            .args(&["/C", "cls"])
            .status()
            .expect("Failed to clear console");
    } else {
        std::process::Command::new("clear")
            .status()
            .expect("Failed to clear console");
    }
}

pub fn create_block(blockchain: &mut Blockchain){
    println!("What would you like the block to contain?");
    let mut data = String::new();
    std::io::stdin().read_line(&mut data).expect("Failed to read line");
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();
    blockchain.create_block(data.trim().to_string(), timestamp);
}

pub async fn user_choice(
    chain: &Arc<Mutex<Blockchain>>,
    peer_manager: &Arc<Mutex<PeerManager>>,
    peer_client: &Arc<Mutex<PeerClient>>,
) -> bool {
    let choice = choose_menu().await;
    match parse_choice(choice.trim()) {
    Some(action) => match action {
        Action::AddBlock =>{
            let mut blockchain_locked = chain.lock().await;
            create_block(&mut blockchain_locked);
            println!("Block added successfully!");
            let peer_manager_locked = peer_manager.lock().await;
            for peer in &peer_manager_locked.peers {
                let locked_peer_client = peer_client.lock().await;
                if let Err(e) = locked_peer_client.send_added_block(peer, blockchain_locked.blocks.last().unwrap().clone()).await {
                    println!("Failed to send added block to {}: {}", peer, e);
                } else {
                    println!("Added block sent to {}", peer);
                }
            }
        }
        Action::PrintBlocks => {
            let blockchain_locked = chain.lock().await;
            if blockchain_locked.blocks.is_empty() {
                println!("No blocks in the blockchain yet.");
            } else {
                print_blockchain(&blockchain_locked);
            }
        }
        Action::CheckValidity => {
            let blockchain_locked = chain.lock().await;
            if blockchain_locked.is_valid() {
                println!("Our blockchain is valid!");
            } else {
                println!("Our blockchain is NOT valid!!! that's bad!");
            }
        },
        Action::AddPeer => {
            println!("Enter the peer address to add:");
            let mut peer_address = String::new();
            std::io::stdin().read_line(&mut peer_address).expect("Failed to read line");
            let peer_address = peer_address.trim().to_string();
            let mut peer_manager_locked = peer_manager.lock().await;
            peer_manager_locked.add_peer(&peer_address, true).await;
            println!("Peer {} added successfully!", peer_address);
        },
        Action::PrintPeers => {
            let peer_manager_locked = peer_manager.lock().await;
            if peer_manager_locked.peers.is_empty() {
                println!("No peers available.");
            } else {
                println!("Current peers:");
                for peer in &peer_manager_locked.peers {
                    println!("{}", peer);
                }
            }
        },
        Action::ClearConsole => clear_console(),
        Action::Exit => return true,
    },
    None => println!("Invalid choice, please try again."),
}

    return false;
}
fn parse_choice(choice: &str) -> Option<Action> {
    match choice.trim() {
        "1" => Some(Action::AddBlock),
        "2" => Some(Action::PrintBlocks),
        "3" => Some(Action::CheckValidity),
        "4" => Some(Action::ClearConsole),
        "5" => Some(Action::AddPeer),
        "6" => Some(Action::PrintPeers),
        "0" => Some(Action::Exit),
        _ => None,
    }
}