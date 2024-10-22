use sha2::{Sha256, Digest};

#[derive(Debug)]
pub struct Block {
    pub index: u64,
    pub previous_hash: String,
    pub timestamp: u64,
    pub data: String,
    pub hash: String,
}

pub struct Blockchain {
    pub chain: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        let mut blockchain = Blockchain { chain: Vec::new() };
        // Create the genesis block
        blockchain.create_block("Genesis Block".to_string());
        blockchain
    }

    pub fn create_block(&mut self, data: String) -> &Block {
        let index = self.chain.len() as u64;
        let previous_hash = if index == 0 {
            String::from("0")
        } else {
            self.chain[index as usize - 1].hash.clone()
        };
        let timestamp = get_timestamp();
        let hash = calculate_hash(index, &previous_hash, timestamp, &data);

        let block = Block {
            index,
            previous_hash,
            timestamp,
            data,
            hash,
        };

        self.chain.push(block);
        self.chain.last().unwrap()
    }

    pub fn show_chain(&self) {
        for block in &self.chain {
            println!("{:?}", block);
        }
    }
}

// Function to get the current timestamp
fn get_timestamp() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
}

// Function to calculate the hash of a block
fn calculate_hash(index: u64, previous_hash: &str, timestamp: u64, data: &str) -> String {
    let input = format!("{}{}{}{}", index, previous_hash, timestamp, data);
    let mut hasher = Sha256::new();
    hasher.update(input);
    format!("{:x}", hasher.finalize())
}
