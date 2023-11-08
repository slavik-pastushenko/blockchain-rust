#![forbid(unsafe_code)]

use chrono::Utc;
use serde_derive::Serialize;
use sha2::{Digest, Sha256};
use std::fmt::Write;

/// An exchange of assets between two parties
#[derive(Debug, Clone, Serialize)]
pub struct Transaction {
    /// A transaction hash
    pub hash: String,

    /// A transaction sender address
    pub from: String,

    /// A transaction receiver address
    pub to: String,

    /// A transaction amount
    pub amount: f32,

    /// A transaction timestamp
    pub timestamp: i64,
}

/// A identifier of a particular block on an entire blockchain
#[derive(Debug, Serialize)]
pub struct BlockHeader {
    /// A timestamp at which a block was mined
    pub timestamp: i64,

    /// An integer to achieve the network's difficulty
    pub nonce: u32,

    /// A hash of a previous block
    pub previous_hash: String,

    /// A Merkel root hash
    pub merkle: String,

    /// A current difficulty level of the network
    pub difficulty: u32,
}

/// A data storage in a blockchain
#[derive(Debug, Serialize)]
pub struct Block {
    /// Information about the block and the miner
    pub header: BlockHeader,

    /// A total amount of transactions
    pub count: usize,

    /// An amount of transactions
    pub transactions: Vec<Transaction>,
}

/// A blockchain
#[derive(Debug, Serialize)]
pub struct Chain {
    /// A chain of blocks
    pub chain: Vec<Block>,

    /// A list of transactions
    pub current_transactions: Vec<Transaction>,

    /// A current difficulty level of the network
    pub difficulty: u32,

    /// A blockchain genesis address
    pub address: String,

    /// A block reward
    pub reward: f32,
}

impl Chain {
    /// Initialize a new blockchain with the specified parameters.
    ///
    /// # Arguments
    /// - `address`: The address associated with the blockchain.
    /// - `difficulty`: The initial mining difficulty level of the network.
    /// - `reward`: The initial block reward for miners.
    ///
    /// # Returns
    /// A new `Chain` instance with the given parameters and a genesis block.
    pub fn new(address: String, difficulty: u32, reward: f32) -> Self {
        let mut chain = Chain {
            reward,
            address,
            difficulty,
            chain: Vec::new(),
            current_transactions: Vec::new(),
        };

        chain.generate_new_block();

        chain
    }

    /// Get a list of current transactions in the blockchain.
    ///
    /// # Returns
    /// A reference to a vector containing the current transactions.
    pub fn get_transactions(&mut self) -> &Vec<Transaction> {
        &self.current_transactions
    }

    /// Get a transaction by its hash.
    ///
    /// # Arguments
    /// - `hash`: The hash of the transaction to retrieve.
    ///
    /// # Returns
    /// An option containing a reference to the transaction if found, or `None` if not found.
    pub fn get_transaction(&self, hash: String) -> Option<&Transaction> {
        self.current_transactions
            .iter()
            .find(|&trx| trx.hash == hash)
    }

    /// Add a new transaction to the blockchain.
    ///
    /// # Arguments
    /// - `from`: The sender's address.
    /// - `to`: The receiver's address.
    /// - `amount`: The amount of the transaction.
    ///
    /// # Returns
    /// `true` if the transaction is successfully added to the current transactions.
    pub fn add_transaction(&mut self, from: String, to: String, amount: f32) -> bool {
        let timestamp = Utc::now().timestamp();
        let hash = Chain::hash(&(&from, &to, amount, timestamp));

        self.current_transactions.push(Transaction {
            to,
            from,
            hash,
            amount,
            timestamp: Utc::now().timestamp(),
        });

        true
    }

    /// Get the hash of the last block in the blockchain.
    ///
    /// # Returns
    /// The hash of the last block in the blockchain as a string.
    pub fn get_last_hash(&self) -> String {
        let block = match self.chain.last() {
            Some(block) => block,
            None => {
                return String::from_utf8(vec![48; 64]).unwrap();
            }
        };

        Chain::hash(&block.header)
    }

    /// Update the mining difficulty of the blockchain.
    ///
    /// # Arguments
    /// - `difficulty`: The new mining difficulty level.
    ///
    /// # Returns
    /// `true` if the difficulty is successfully updated.
    pub fn update_difficulty(&mut self, difficulty: u32) -> bool {
        self.difficulty = difficulty;

        true
    }

    /// Update the block reward.
    ///
    /// # Arguments
    /// - `reward`: The new block reward value.
    ///
    /// # Returns
    /// `true` if the reward is successfully updated.
    pub fn update_reward(&mut self, reward: f32) -> bool {
        self.reward = reward;

        true
    }

    /// Generate a new block and append it to the blockchain.
    ///
    /// # Returns
    /// `true` if a new block is successfully generated and added to the blockchain.
    pub fn generate_new_block(&mut self) -> bool {
        let header = BlockHeader {
            timestamp: Utc::now().timestamp(),
            nonce: 0,
            previous_hash: self.get_last_hash(),
            merkle: String::new(),
            difficulty: self.difficulty,
        };

        let timestamp = Utc::now().timestamp();
        let to = self.address.clone();
        let from = String::from("Root");
        let hash = Chain::hash(&(&from, &to, self.reward, timestamp));

        let reward_trans = Transaction {
            to,
            from,
            hash,
            timestamp,
            amount: self.reward,
        };

        let mut block = Block {
            header,
            count: 0,
            transactions: vec![],
        };

        block.transactions.push(reward_trans);
        block.transactions.append(&mut self.current_transactions);

        block.count = block.transactions.len();
        block.header.merkle = Chain::get_merkle(block.transactions.clone());

        Chain::proof_of_work(&mut block.header);

        self.chain.push(block);

        true
    }

    /// Calculate the Merkle root hash for a list of transactions.
    ///
    /// # Arguments
    /// - `transactions`: A vector of transactions for which the Merkle root hash is calculated.
    ///
    /// # Returns
    /// The Merkle root hash as a string.
    pub fn get_merkle(transactions: Vec<Transaction>) -> String {
        let mut merkle = Vec::new();

        for t in &transactions {
            let hash = Chain::hash(t);
            merkle.push(hash);
        }

        if merkle.len() % 2 == 1 {
            let last = merkle.last().cloned().unwrap();
            merkle.push(last);
        }

        while merkle.len() > 1 {
            let mut h1 = merkle.remove(0);
            let h2 = merkle.remove(0);

            h1.push_str(&h2);

            let nh = Chain::hash(&h1);
            merkle.push(nh);
        }

        merkle.pop().unwrap()
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

    /// Calculate the SHA-256 hash of a serializable item.
    ///
    /// # Arguments
    /// - `item`: A serializable item to be hashed.
    ///
    /// # Returns
    /// The SHA-256 hash of the item as a string.
    pub fn hash<T: serde::Serialize>(item: &T) -> String {
        let input = serde_json::to_string(&item).unwrap();
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        let res = hasher.finalize();
        let vec_res = res.to_vec();

        let mut result = String::new();

        for b in vec_res.as_slice() {
            write!(&mut result, "{:x}", b).expect("Unable to write");
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> Chain {
        Chain::new("Address".to_string(), 1, 100.0)
    }

    #[test]
    fn test_add_transaction() {
        let mut chain = setup();

        let result = chain.add_transaction("Sender".to_string(), "Receiver".to_string(), 10.0);

        assert!(result);
        assert_eq!(chain.current_transactions.len(), 1);
    }

    #[test]
    fn test_get_transaction_found() {
        let mut chain = setup();

        chain.add_transaction("Sender".to_string(), "Receiver".to_string(), 10.0);

        let transaction = chain.get_transaction(chain.current_transactions[0].hash.clone());

        assert!(transaction.is_some());
        assert_eq!(transaction.unwrap().from, "Sender");
        assert_eq!(transaction.unwrap().to, "Receiver");
    }

    #[test]
    fn test_get_transaction_not_found() {
        let chain = setup();

        let transaction = chain.get_transaction("NonExistentHash".to_string());

        assert!(transaction.is_none());
    }

    #[test]
    fn test_get_transactions() {
        let mut chain = setup();
        chain.add_transaction("Sender1".to_string(), "Receiver1".to_string(), 10.0);
        chain.add_transaction("Sender2".to_string(), "Receiver2".to_string(), 20.0);

        let transactions = chain.get_transactions();

        assert_eq!(transactions.len(), 2);
        assert_eq!(transactions[0].from, "Sender1");
        assert_eq!(transactions[1].from, "Sender2");
    }

    #[test]
    fn test_get_transactions_not_found() {
        let mut chain = setup();

        let transactions = chain.get_transactions();

        assert!(transactions.is_empty());
    }

    #[test]
    fn test_get_last_hash() {
        let chain = setup();
        let hash = chain.get_last_hash();

        assert!(!hash.is_empty());
    }

    #[test]
    fn test_update_difficulty() {
        let mut chain = setup();

        let result = chain.update_difficulty(4);

        assert!(result);
        assert_eq!(chain.difficulty, 4);
    }

    #[test]
    fn test_update_reward() {
        let mut chain = setup();

        let result = chain.update_reward(50.0);

        assert!(result);
        assert_eq!(chain.reward, 50.0);
    }

    #[test]
    fn test_generate_new_block() {
        let mut chain = setup();

        let result = chain.generate_new_block();

        assert!(result);
        assert_eq!(chain.chain.len(), 2);
    }
}
