use tokio::task;

use crate::blockchain::chain::Blockchain;

enum Action{
    AddBlock,
    PrintBlocks,
    CheckValidity,
    ClearConsole,
    Exit,
}

pub async fn choose_menu() -> String{
    println!("What would you like to do? \n 1 - Add a block, \n 2 - Print all blocks, \n 3 - Check if our blockchain is valid, \n 4 - Clear console\n 5 - Exit");
    let choice = task::spawn_blocking(|| {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        input
    }   
    )
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

pub async fn user_choice(blockchain : &mut Blockchain) -> bool {
    let choice = choose_menu().await;
    match parse_choice(choice.trim()) {
    Some(action) => match action {
        Action::AddBlock => create_block(blockchain),
        Action::PrintBlocks => print_blockchain(blockchain),
        Action::CheckValidity => {
            if blockchain.is_valid() {
                println!("Our blockchain is valid!");
            } else {
                println!("Our blockchain is NOT valid!!! that's bad!");
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
        "5" => Some(Action::Exit),
        _ => None,
    }
}