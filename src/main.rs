use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use rand::Rng;
use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Wallet {
    id: String,
    balance: u32,
    last_airdrop: Option<u64>, // Timestamp for last airdrop
}

impl Wallet {
    pub fn load() -> Result<Wallet, std::io::Error> {
        // Load wallet from file
        let file = File::open("wallet.json")?;
        let wallet: Wallet = serde_json::from_reader(file)?;
        Ok(wallet)
    }

    pub fn create() -> Result<Wallet, std::io::Error> {
        let id = Self::generate_wallet_id();
        let wallet = Wallet { id, balance: 0, last_airdrop: None };
        let file = File::create("wallet.json")?;
        serde_json::to_writer(file, &wallet)?;
        Ok(wallet)
    }

    fn generate_wallet_id() -> String {
        let mut hasher = Sha256::new();
        hasher.update(format!("{}", rand::random::<u64>()));
        format!("{:x}", hasher.finalize())
    }

    pub fn airdrop(&mut self) -> u32 {
        let current_time = chrono::Utc::now().timestamp() as u64;
        if self.last_airdrop.is_none() || current_time - self.last_airdrop.unwrap() >= 604800 { // 1 week in seconds
            let amount: u32 = rand::thread_rng().gen_range(0..=90);
            self.balance += amount;
            self.last_airdrop = Some(current_time);
            self.save().expect("Failed to save wallet after airdrop");
            amount
        } else {
            println!("Airdrop can only be claimed once a week.");
            0
        }
    }

    pub fn mine(&mut self) -> u32 {
        let mut nonce = 0;
        let difficulty = 4; // Difficulty level (number of leading zeros)
        let target = "0000"; // Target hash prefix for difficulty

        loop {
            // Create a string to hash
            let block_data = format!("{}{}", self.id, nonce);
            let hash = format!("{:x}", Sha256::digest(block_data.as_bytes()));

            // Check if the hash meets the difficulty
            if &hash[..difficulty] == target {
                self.balance += 1; // Increase balance for a successful mine
                self.save().expect("Failed to save wallet after mining");
                println!("Successfully mined! Nonce: {}, Hash: {}", nonce, hash);
                return 1; // Return the mined amount
            }
            nonce += 1; // Increment nonce for next attempt
        }
    }

    pub fn get_balance(&self) -> u32 {
        self.balance
    }

    fn save(&self) -> Result<(), std::io::Error> {
        let file = File::create("wallet.json")?;
        serde_json::to_writer(file, self)?;
        Ok(())
    }
}

#[derive(Clone)]
pub struct Block {
    index: u32,
    // Add other fields as needed
}

#[derive(Clone)]
pub struct Blockchain {
    chain: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Blockchain {
        Blockchain { chain: vec![] }
    }

    pub fn get_chain(&self) -> &Vec<Block> {
        &self.chain
    }
}

fn main() {
    // Load or create the wallet
    let mut wallet = Wallet::load().unwrap_or_else(|err| {
        println!("Failed to load wallet: {}", err);
        Wallet::create().expect("Failed to create a new wallet")
    });
    println!("Wallet loaded with ID: {:?}, Current balance: {}", wallet.id, wallet.get_balance());

    // Initialize the blockchain
    let blockchain = Blockchain::new();
    println!("Blockchain initialized.");

    loop {
        println!("Type 'mine', 'airdrop', 'show', or 'balance':");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim() {
            "mine" => {
                let amount = wallet.mine();
                println!("You mined {} DiskCoin! Current balance: {}", amount, wallet.get_balance());
            }
            "airdrop" => {
                let amount = wallet.airdrop();
                println!("You received {} DiskCoin from airdrop! Current balance: {}", amount, wallet.get_balance());
            }
            "show" | "balance" => {
                println!("Current balance: {}", wallet.get_balance());
            }
            _ => println!("Unknown command. Please type 'mine', 'airdrop', 'show', or 'balance'."),
        }
    }
}
