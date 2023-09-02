extern crate chrono;
extern crate serde;
extern crate serde_json;
extern crate sha2;

use chrono::Utc;
use serde_derive::Serialize;
use sha2::{ Sha256, Digest };
use std::fmt::Write;

/// An exchange of assets between two parties
#[derive(Debug, Clone, Serialize)]
pub struct Transaction {
    /// A transaction hash
    hash: String,

    /// A transaction sender address
    from: String,

    /// A transaction receiver address
    to: String,

    /// A transaction amount
    amount: f32,

    /// A transaction timestamp
    timestamp: i64,
}

/// A identifier of a particular block on an entire blockchain
#[derive(Debug, Serialize)]
pub struct BlockHeader {
    /// A timestamp at which a block was mined
    timestamp: i64,

    /// An integer to achieve the network's difficulty
    nonce: u32,

    /// A hash of a previous block
    previous_hash: String,

    /// A Merkel root hash
    merkle: String,

    /// A current difficulty level of the network
    difficulty: u32,
}

/// A data storage in a blockchain
#[derive(Debug, Serialize)]
pub struct Block {
    /// Information about the block and the miner
    header: BlockHeader,

    /// A total amount of transactions
    count: u32,

    /// An amount of transactions
    transactions: Vec<Transaction>,
}

/// A blockchain
#[derive(Debug, Serialize)]
pub struct Chain {
    /// A chain of blocks
    chain: Vec<Block>,

    /// A list of transactions
    current_transactions: Vec<Transaction>,

    /// A current difficulty level of the network
    difficulty: u32,

    /// A blockchain genesis address
    address: String,

    /// A block reward
    reward: f32,
}

impl Chain {
    pub fn new(address: String, difficulty: u32) -> Chain {
        let mut chain = Chain {
            chain: Vec::new(),
            current_transactions: Vec::new(),
            difficulty,
            address,
            reward: 100.0,
        };

        chain.generate_new_block();

        chain
    }

    pub fn get_transactions(&mut self) -> &Vec<Transaction> {
        &self.current_transactions
    }

    pub fn get_transaction(&self, hash: String) -> Option<&Transaction> {
        self.current_transactions.iter().find(|&trx| trx.hash == hash)
    }

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

    pub fn get_last_hash(&self) -> String {
        let block = match self.chain.last() {
            Some(block) => block,
            None => {
                return String::from_utf8(vec![48; 64]).unwrap();
            }
        };

        Chain::hash(&block.header)
    }

    pub fn update_difficulty(&mut self, difficulty: u32) -> bool {
        self.difficulty = difficulty;

        true
    }

    pub fn update_reward(&mut self, reward: f32) -> bool {
        self.reward = reward;

        true
    }

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

        block.count = block.transactions.len() as u32;
        block.header.merkle = Chain::get_merkle(block.transactions.clone());

        Chain::proof_of_work(&mut block.header);

        self.chain.push(block);

        true
    }

    fn get_merkle(transactions: Vec<Transaction>) -> String {
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
            let mut h2 = merkle.remove(0);

            h1.push_str(&mut h2);

            let nh = Chain::hash(&h1);
            merkle.push(nh);
        }
        merkle.pop().unwrap()
    }

    pub fn proof_of_work(header: &mut BlockHeader) {
        loop {
            let hash = Chain::hash(header);
            let slice = &hash[..header.difficulty as usize];

            match slice.parse::<u32>() {
                Ok(val) => {
                    if val != 0 {
                        header.nonce += 1;
                    } else {
                        println!("Block hash: {:?}", hash);

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
        Chain::new("Address".to_string(), 1)
    }

    #[test]
    fn test_add_transaction() {
        let mut chain = setup();

        let result = chain.add_transaction("Sender".to_string(), "Receiver".to_string(), 10.0);

        assert_eq!(result, true);
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

        assert_eq!(transactions.len(), 0);
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

        assert_eq!(result, true);
        assert_eq!(chain.difficulty, 4);
    }

    #[test]
    fn test_update_reward() {
        let mut chain = setup();

        let result = chain.update_reward(50.0);

        assert_eq!(result, true);
        assert_eq!(chain.reward, 50.0);
    }

    #[test]
    fn test_generate_new_block() {
        let mut chain = setup();

        let result = chain.generate_new_block();

        assert_eq!(result, true);
        assert_eq!(chain.chain.len(), 2);
    }
}
