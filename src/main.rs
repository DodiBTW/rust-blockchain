pub mod blockchain;
pub mod cli;
use blockchain::chain::Blockchain;
use cli::command::user_choice;

fn main() {
    println!("Hello, user! Let's create a blockchain.");
    let mut blockchain = Blockchain::new();
    loop{
        let end = user_choice(&mut blockchain);
        if end == true{
            break;
        }
    }
    println!("Thank you for participating!");
}
