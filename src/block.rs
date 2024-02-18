use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::{Chain, Transaction};

/// Identifier of a particular block on an entire blockchain.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BlockHeader {
    /// Timestamp at which a block was mined.
    pub timestamp: i64,

    /// Integer to achieve the network's difficulty.
    pub nonce: u32,

    /// Hash of a previous block.
    pub previous_hash: String,

    /// Merkel root hash.
    pub merkle: String,

    /// Current difficulty level of the network.
    pub difficulty: f64,
}

/// Data storage in a blockchain.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Block {
    /// Information about the block and the miner.
    pub header: BlockHeader,

    /// Total amount of transactions.
    pub count: usize,

    /// An amount of transactions.
    pub transactions: Vec<Transaction>,
}

impl Block {
    /// Create a new block.
    ///
    /// # Arguments
    ///
    /// - `previous_hash` - The hash of the previous block.
    /// - `difficulty` - The difficulty level of the network.
    ///
    /// # Returns
    ///
    /// A new block with the given previous hash and difficulty.
    pub fn new(previous_hash: String, difficulty: f64) -> Self {
        // Create a new block header
        let header = BlockHeader {
            nonce: 0,
            difficulty,
            previous_hash,
            merkle: String::new(),
            timestamp: Utc::now().timestamp(),
        };

        // Create a new block
        Block {
            header,
            count: 0,
            transactions: vec![],
        }
    }

    /// Perform the proof-of-work process to mine a block.
    ///
    /// # Arguments
    /// - `header`: A mutable reference to the block header to be mined.
    pub fn proof_of_work(header: &mut BlockHeader) {
        loop {
            let hash = Chain::hash(header);
            let slice = &hash[..header.difficulty as usize];

            match slice.parse::<u32>() {
                Ok(val) => {
                    if val != 0 {
                        header.nonce += 1;
                    } else {
                        break;
                    }
                }
                Err(_) => {
                    header.nonce += 1;

                    continue;
                }
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proof_of_work() {
        let mut block = Block::new("0".to_string(), 1.0);
        Block::proof_of_work(&mut block.header);

        assert_eq!(block.header.difficulty, 1.0);
        assert!(!block.header.previous_hash.is_empty());
    }

    #[test]
    fn test_new_block() {
        let block = Block::new("0".to_string(), 3.0);

        assert_eq!(block.count, 0);
        assert_eq!(block.transactions.len(), 0);
    }
}
