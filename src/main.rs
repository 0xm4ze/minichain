mod block;
mod blockchain;

use blockchain::Blockchain;
use std::io::{self, Write};

fn main() {
    let mut blockchain = Blockchain::new();

    loop {
        println!("\n--- MiniChain CLI ---");
        println!("1. Add Block");
        println!("2. Show Chain");
        println!("3. Validate Chain");
        println!("4. Exit");

        print!("> ");
        io::stdout().flush().unwrap();
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                let mut data = String::new();
                println!("Enter data for the block:");
                io::stdin().read_line(&mut data).unwrap();
                blockchain.add_block(data.trim().to_string());
            }
            "2" => {
                for block in &blockchain.chain {
                    println!("{:#?}", block);
                }
            }
            "3" => {
                if blockchain.is_valid() {
                    println!("✅ Blockchain is valid!");
                } else {
                    println!("❌ Blockchain is NOT valid!");
                }
            }
            "4" => break,
            _ => println!("Invalid choice"),
        }
    }
}
