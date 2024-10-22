use crate::blockandblockchain::{Blockchain, Block};

pub struct Miner {
    pub blockchain: Blockchain,
}

impl Miner {
    pub fn new(blockchain: Blockchain) -> Self {
        Miner { blockchain }
    }

    pub fn mine(&mut self, data: String) {
        self.blockchain.create_block(data);
    }
}
