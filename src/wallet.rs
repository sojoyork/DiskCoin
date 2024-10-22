use rand::Rng; // Include this for generating a random ID
use sha2::{Sha256, Digest}; // Ensure you have this for hashing if needed
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::Path;

pub struct Wallet {
    pub id: String,
}

impl Wallet {
    pub fn create() -> Result<Self, Box<dyn std::error::Error>> {
        let wallet_id = Self::generate_wallet_id(); // This should now work

        // Save the wallet ID to a file
        let mut file = File::create("wallet.json")?; // Create the file
        writeln!(file, "{}", wallet_id)?; // Write the wallet ID to the file

        Ok(Wallet { id: wallet_id })
    }

    pub fn generate_wallet_id() -> String {
        let random_id: u64 = rand::thread_rng().gen(); // Generate a random u64
        let mut hasher = Sha256::new();
        hasher.update(random_id.to_string());
        format!("{:x}", hasher.finalize()) // Return the SHA256 hash as a hexadecimal string
    }

    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let path = Path::new("wallet.json");

        // Try to open the file
        let mut file = match File::open(&path) {
            Ok(file) => file,
            Err(e) => {
                println!("Failed to open wallet file: {}", e);
                // If the file doesn't exist, create a new wallet
                return Self::create(); // Call create to make a new wallet
            }
        };

        // Read the contents of the file into a string
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let wallet_id = contents.trim().to_string(); // Assuming the file contains just the wallet ID

        Ok(Wallet { id: wallet_id }) // Return the Wallet instance
    }
}
