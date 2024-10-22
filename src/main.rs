mod blockandblockchain;
mod miner;
mod wallet;

use blockandblockchain::Blockchain; // Removed Block
use miner::Miner; // Ensure this is included
use wallet::Wallet;
use std::io;
use tokio::net::TcpStream;
use tokio::io::{AsyncWriteExt, BufWriter};

#[tokio::main]
async fn main() {
    // Create or load wallet
    let wallet_result = Wallet::load();
    let wallet = match wallet_result {
        Ok(wallet) => {
            println!("Logged in with Wallet ID: {}", wallet.id);
            wallet
        }
        Err(_) => {
            Wallet::create().expect("Failed to create wallet");
            Wallet::load().expect("Failed to load wallet")
        }
    };

    // Initialize blockchain and miner
    let blockchain = Blockchain::new();
    let mut miner = Miner::new(blockchain); // Create a Miner instance

    println!("Welcome to DiskCoin (DSCN)!");

    loop {
        println!("\nEnter command (mine/show/send/exit): ");
        let mut command = String::new();
        io::stdin().read_line(&mut command).expect("Failed to read line");
        let command = command.trim();

        match command {
            "mine" => {
                println!("Enter data for the new block: ");
                let mut data = String::new();
                io::stdin().read_line(&mut data).expect("Failed to read line");
                miner.mine(data.trim().to_string()); // Call mine on the instance
                println!("Block mined!");
            }
            "show" => {
                miner.blockchain.show_chain(); // Call show_chain on the instance
            }
            "send" => {
                println!("Enter recipient ID: ");
                let mut recipient_id = String::new();
                io::stdin().read_line(&mut recipient_id).expect("Failed to read line");

                println!("Enter amount to send: ");
                let mut amount_str = String::new();
                io::stdin().read_line(&mut amount_str).expect("Failed to read line");
                let amount: f64 = amount_str.trim().parse().expect("Invalid amount");

                // Send data
                if let Err(e) = send_data(recipient_id.trim(), &wallet.id, amount).await {
                    println!("Failed to send data: {}", e);
                }
            }
            "exit" => break,
            _ => println!("Unknown command!"),
        }
    }
}

// A function to send data over TCP
async fn send_data(to_id: &str, from_id: &str, amount: f64) -> Result<(), Box<dyn std::error::Error>> {
    // Construct the message
    let message = format!("Sent-dscn {}\nto: {}\nfrom: {}", amount, to_id, from_id);

    // Establish a connection to the recipient's address
    let recipient_ip = "127.0.0.1:8080"; // Replace with the actual recipient's address
    let stream = TcpStream::connect(recipient_ip).await?;

    let mut writer = BufWriter::new(stream);
    writer.write_all(message.as_bytes()).await?;
    writer.flush().await?;

    println!("Data sent: {}", message);
    Ok(())
}
