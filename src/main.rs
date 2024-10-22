use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::thread;
use rand::Rng;
use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Wallet {
    id: String,
    balance: u32,
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
        let wallet = Wallet { id, balance: 0 };
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
        let amount: u32 = rand::thread_rng().gen_range(0..=90);
        self.balance += amount;
        amount
    }
    
    pub fn mine(&mut self) -> u32 {
        // Simulate mining
        let amount = 1; // Fixed amount for simplicity
        self.balance += amount;
        amount
    }

    pub fn get_balance(&self) -> u32 {
        self.balance
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
        println!("Type 'mine' to mine DiskCoin or 'airdrop' for a random amount of DiskCoin (0-90):");
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
            _ => println!("Unknown command. Please type 'mine' or 'airdrop'."),
        }
    }
}
