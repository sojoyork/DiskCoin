use std::fs::File;
use std::io::{self, Read}; // Removed Write and thread as they are unused
use rand::Rng;
use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Clone, Serialize, Deserialize)]
pub struct Wallet {
    id: String,
    balance: u32,
    last_airdrop: Option<DateTime<Utc>>, // Field to track the last airdrop
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
        let wallet = Wallet { id, balance: 0, last_airdrop: None }; // Initialize last_airdrop to None
        let file = File::create("wallet.json")?;
        serde_json::to_writer(file, &wallet)?;
        Ok(wallet)
    }

    fn generate_wallet_id() -> String {
        let mut hasher = Sha256::new();
        hasher.update(format!("{}", rand::random::<u64>()));
        format!("{:x}", hasher.finalize())
    }

    pub fn airdrop(&mut self) -> Result<u32, &'static str> {
        let now = Utc::now();
        let one_week = chrono::Duration::weeks(1);
        
        // Check if airdrop can occur
        if let Some(last) = self.last_airdrop {
            if now - last < one_week {
                return Err("Airdrop can only be claimed once a week.");
            }
        }

        let amount: u32 = rand::thread_rng().gen_range(0..=90);
        self.balance += amount;
        self.last_airdrop = Some(now); // Update last airdrop time
        self.save().expect("Failed to save wallet after airdrop");
        Ok(amount)
    }

    pub fn mine(&mut self, difficulty: usize) -> u32 {
        // Simulate mining with proof of work
        let mut nonce = 0;
        let prefix = "0".repeat(difficulty);

        while nonce < u32::MAX {
            let input = format!("{}{}", self.id, nonce); // Unique input for the wallet
            let hash = Sha256::digest(input.as_bytes());
            let hash_hex = format!("{:x}", hash);

            if hash_hex.starts_with(&prefix) {
                let amount = 1; // Reward for mining
                self.balance += amount;
                self.save().expect("Failed to save wallet after mining");
                return amount; // Return the amount mined
            }
            nonce += 1;
        }

        0 // Should not reach here
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

    loop {
        println!("Type 'mine', 'airdrop', 'show', or 'balance':");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim() {
            "mine" => {
                let difficulty = 4; // Example difficulty level (4 leading zeros)
                let amount = wallet.mine(difficulty);
                println!("You mined {} DiskCoin! Current balance: {}", amount, wallet.get_balance());
            }
            "airdrop" => {
                match wallet.airdrop() {
                    Ok(amount) => {
                        println!("You received {} DiskCoin from airdrop! Current balance: {}", amount, wallet.get_balance());
                    }
                    Err(e) => {
                        println!("{}", e); // Print error message if airdrop is not allowed
                    }
                }
            }
            "show" | "balance" => {
                println!("Current balance: {}", wallet.get_balance());
            }
            _ => println!("Unknown command. Please type 'mine', 'airdrop', 'show', or 'balance'."),
        }
    }
}
