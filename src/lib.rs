#![forbid(unsafe_code)]

use chrono::Utc;
use rand::Rng;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fmt::Write;
use std::iter;

/// Exchange of assets between two parties
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Transaction {
    /// Transaction hash
    pub hash: String,

    /// Transaction sender address
    pub from: String,

    /// Transaction receiver address
    pub to: String,

    /// Transaction fee
    pub fee: f64,

    /// Transaction amount
    pub amount: f64,

    /// Transaction timestamp
    pub timestamp: i64,
}

/// Identifier of a particular block on an entire blockchain
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BlockHeader {
    /// Timestamp at which a block was mined
    pub timestamp: i64,

    /// Integer to achieve the network's difficulty
    pub nonce: u32,

    /// Hash of a previous block
    pub previous_hash: String,

    /// Merkel root hash
    pub merkle: String,

    /// Current difficulty level of the network
    pub difficulty: f64,
}

/// Data storage in a blockchain
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Block {
    /// Information about the block and the miner
    pub header: BlockHeader,

    /// Total amount of transactions
    pub count: usize,

    /// An amount of transactions
    pub transactions: Vec<Transaction>,
}

/// Wallet
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Wallet {
    /// Unique email address associated with the wallet.
    pub email: String,

    /// Address uniquely identifying the wallet.
    pub address: String,

    /// The current balance of the wallet.
    pub balance: f64,
}

/// Blockchain
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Chain {
    /// Chain of blocks
    pub chain: Vec<Block>,

    /// List of transactions
    pub current_transactions: Vec<Transaction>,

    /// Current difficulty level of the network
    pub difficulty: f64,

    /// Blockchain genesis address
    pub address: String,

    /// Block reward
    pub reward: f64,

    /// Transaction fee
    pub fee: f64,

    /// A map to associate wallets with their corresponding addresses and balances.
    pub wallets: HashMap<String, Wallet>,
}

impl Chain {
    /// Initialize a new blockchain with the specified parameters.
    ///
    /// # Arguments
    /// - `difficulty`: The initial mining difficulty level of the network.
    /// - `reward`: The initial block reward for miners.
    /// - `fee`: The transaction fee.
    ///
    /// # Returns
    /// A new `Chain` instance with the given parameters and a genesis block.
    pub fn new(difficulty: f64, reward: f64, fee: f64) -> Self {
        let mut chain = Chain {
            fee,
            reward,
            difficulty,
            chain: Vec::new(),
            wallets: HashMap::new(),
            current_transactions: Vec::new(),
            address: Chain::generate_address(42),
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
    pub fn add_transaction(&mut self, from: String, to: String, amount: f64) -> bool {
        let total_amount = amount * self.fee;

        // Validate the transaction
        if !self.validate_transaction(&from, &to, total_amount) {
            return false;
        }

        let timestamp = Utc::now().timestamp();
        let hash = Chain::hash(&(&from, &to, total_amount, timestamp));

        // Update balances
        let sender = self.wallets.get_mut(&from).unwrap();
        sender.balance -= total_amount;

        let receiver = self.wallets.get_mut(&to).unwrap();
        receiver.balance += amount;

        self.current_transactions.push(Transaction {
            to,
            from,
            hash,
            timestamp,
            fee: self.fee,
            amount: total_amount,
        });

        true
    }

    /// Validate a transaction.
    ///
    /// # Arguments
    /// - `from`: The sender's address.
    /// - `to`: The receiver's address.
    /// - `amount`: The amount of the transaction.
    ///
    /// # Returns
    /// `true` if the transaction is valid, `false` otherwise.
    pub fn validate_transaction(&self, from: &str, to: &str, amount: f64) -> bool {
        // Validate if the sender is not the root
        if from == "Root" {
            return false;
        }

        // Validate that sender and receiver addresses are different
        if from == to {
            return false;
        }

        // Validate if the amount is non-negative
        if amount <= 0.0 {
            return false;
        }

        // Validate if sender and receiver addresses are valid
        let sender = self.wallets.get(from);
        let receiver = self.wallets.get(to);

        if sender.is_none() || receiver.is_none() {
            // One or both wallets not found
            return false;
        }

        // Validate if sender can send the amount of the transaction
        if sender.unwrap().balance < amount {
            return false;
        }

        true
    }

    /// Create a new wallet with a unique email and an initial balance.
    ///
    /// # Arguments
    /// - `email`: The unique user email.
    ///
    /// # Returns
    /// The newly created wallet address.
    pub fn create_wallet(&mut self, email: String) -> String {
        let address = Chain::generate_address(42);

        let wallet = Wallet {
            email,
            balance: 0.0,
            address: address.clone(),
        };

        self.wallets.insert(address.to_string(), wallet);

        address
    }

    /// Get a wallet's balance based on its address.
    ///
    /// # Arguments
    /// - `address`: The unique wallet address.
    ///
    /// # Returns
    /// The wallet balance.
    pub fn get_wallet_balance(&self, address: String) -> Option<f64> {
        self.wallets.get(&address).map(|wallet| wallet.balance)
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
    pub fn update_difficulty(&mut self, difficulty: f64) -> bool {
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
    pub fn update_reward(&mut self, reward: f64) -> bool {
        self.reward = reward;

        true
    }

    /// Update the transaction fee.
    ///
    /// # Arguments
    /// - `fee`: The new transaction fee value.
    ///
    /// # Returns
    /// `true` if the transaction fee is successfully updated.
    pub fn update_fee(&mut self, fee: f64) -> bool {
        self.fee = fee;

        true
    }

    /// Generate a new block and append it to the blockchain.
    ///
    /// # Returns
    /// `true` if a new block is successfully generated and added to the blockchain.
    pub fn generate_new_block(&mut self) -> bool {
        let timestamp = Utc::now().timestamp();
        let header = BlockHeader {
            nonce: 0,
            timestamp,
            merkle: String::new(),
            difficulty: self.difficulty,
            previous_hash: self.get_last_hash(),
        };

        let to = self.address.clone();
        let from = String::from("Root");
        let hash = Chain::hash(&(&from, &to, self.reward, timestamp));

        let reward_trans = Transaction {
            to,
            from,
            hash,
            timestamp,
            fee: self.fee,
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

    /// Generates a random alphanumeric string of a specified length.
    ///
    /// # Arguments
    /// - `length`: The length of the generated string.
    ///
    /// # Returns
    /// A `String` containing the generated alphanumeric string.
    fn generate_address(length: usize) -> String {
        let mut rng = rand::thread_rng();

        let address: String = iter::repeat(())
            .map(|()| rng.sample(rand::distributions::Alphanumeric) as char)
            .take(length)
            .collect();

        address
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> Chain {
        Chain::new(1.0, 100.0, 0.1)
    }

    #[test]
    fn test_add_transaction() {
        let mut chain = setup();

        let from = chain.create_wallet("s@mail.com".to_string());
        let to = chain.create_wallet("r@mail.com".to_string());

        let sender = chain.wallets.get_mut(&from).unwrap();
        sender.balance += 20.0;

        let result = chain.add_transaction(from, to, 10.0);

        assert!(result);
        assert_eq!(chain.current_transactions.len(), 1);
    }

    #[test]
    fn test_add_transaction_validation_failed() {
        let mut chain = setup();
        let from = chain.create_wallet("s@mail.com".to_string());
        let to = chain.create_wallet("r@mail.com".to_string());

        let sender = chain.wallets.get_mut(&from).unwrap();
        sender.balance += 20.0;

        let result = chain.add_transaction(from, to, 0.0);

        assert!(!result);
        assert!(chain.current_transactions.is_empty());
    }

    #[test]
    fn test_validate_transaction() {
        let mut chain = setup();
        let from = chain.create_wallet("s@mail.com".to_string());
        let to = chain.create_wallet("r@mail.com".to_string());

        let sender = chain.wallets.get_mut(&from).unwrap();
        sender.balance += 20.0;

        let result = chain.validate_transaction(&from, &to, 10.0);

        assert!(result);
    }

    #[test]
    fn test_validate_transaction_failed_by_invalid_amount() {
        let mut chain = setup();
        let from = chain.create_wallet("s@mail.com".to_string());
        let to = chain.create_wallet("r@mail.com".to_string());

        let sender = chain.wallets.get_mut(&from).unwrap();
        sender.balance += 20.0;

        let result = chain.validate_transaction(&from, &to, -1.0);

        assert!(!result);
    }

    #[test]
    fn test_validate_transaction_failed_by_invalid_sender() {
        let mut chain = setup();
        let _ = chain.create_wallet("s@mail.com".to_string());
        let to = chain.create_wallet("r@mail.com".to_string());

        let result = chain.validate_transaction("invalid", &to, 1.0);

        assert!(!result);
    }

    #[test]
    fn test_validate_transaction_failed_by_invalid_receiver() {
        let mut chain = setup();
        let from = chain.create_wallet("s@mail.com".to_string());
        let _ = chain.create_wallet("r@mail.com".to_string());

        let sender = chain.wallets.get_mut(&from).unwrap();
        sender.balance += 20.0;

        let result = chain.validate_transaction(&from, "invalid", 1.0);

        assert!(!result);
    }

    #[test]
    fn test_validate_transaction_failed_by_same_addresses() {
        let chain = setup();

        let result = chain.validate_transaction("address", "address", 1.0);

        assert!(!result);
    }

    #[test]
    fn test_validate_transaction_failed_by_invalid_sender_balance() {
        let mut chain = setup();
        let from = chain.create_wallet("s@mail.com".to_string());
        let to = chain.create_wallet("r@mail.com".to_string());

        let result = chain.validate_transaction(&from, &to, 1.0);

        assert!(!result);
    }

    #[test]
    fn test_validate_transaction_failed_by_root() {
        let chain = setup();

        let result = chain.validate_transaction("Root", "Receiver", 0.01);

        assert!(!result);
    }

    #[test]
    fn test_get_transaction() {
        let mut chain = setup();
        let from = chain.create_wallet("s@mail.com".to_string());
        let to = chain.create_wallet("r@mail.com".to_string());

        let sender = chain.wallets.get_mut(&from).unwrap();
        sender.balance += 20.0;

        chain.add_transaction(from.clone(), to.clone(), 10.0);

        let transaction = chain.get_transaction(chain.current_transactions[0].hash.clone());

        assert!(transaction.is_some());
        assert_eq!(transaction.unwrap().from, from);
        assert_eq!(transaction.unwrap().to, to);
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
        let from = chain.create_wallet("s@mail.com".to_string());
        let to = chain.create_wallet("r@mail.com".to_string());

        let sender = chain.wallets.get_mut(&from).unwrap();
        sender.balance += 20.0;

        chain.add_transaction(from.clone(), to.clone(), 10.0);
        chain.add_transaction(to.clone(), from.clone(), 20.0);

        let transactions = chain.get_transactions();

        assert_eq!(transactions.len(), 2);
        assert_eq!(transactions[0].from, from);
        assert_eq!(transactions[1].from, to);
    }

    #[test]
    fn test_get_transactions_not_found() {
        let mut chain = setup();

        let transactions = chain.get_transactions();

        assert!(transactions.is_empty());
    }

    #[test]
    fn test_create_wallet() {
        let mut chain = setup();

        let result = chain.create_wallet("s@mail.com".to_string());

        assert_eq!(result.len(), 42);
    }

    #[test]
    fn test_get_wallet_balance() {
        let mut chain = setup();
        let address = chain.create_wallet("s@mail.com".to_string());

        let result = chain.get_wallet_balance(address);

        assert!(result.is_some());
    }

    #[test]
    fn test_get_wallet_balance_not_found() {
        let chain = setup();

        let result = chain.get_wallet_balance("address".to_string());

        assert!(result.is_none());
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

        let result = chain.update_difficulty(4.0);

        assert!(result);
        assert_eq!(chain.difficulty, 4.0);
    }

    #[test]
    fn test_update_reward() {
        let mut chain = setup();

        let result = chain.update_reward(50.0);

        assert!(result);
        assert_eq!(chain.reward, 50.0);
    }

    #[test]
    fn test_update_fee() {
        let mut chain = setup();

        let result = chain.update_fee(0.02);

        assert!(result);
        assert_eq!(chain.fee, 0.02);
    }

    #[test]
    fn test_generate_new_block() {
        let mut chain = setup();

        let result = chain.generate_new_block();

        assert!(result);
        assert_eq!(chain.chain.len(), 2);
    }

    #[test]
    fn test_generate_address() {
        let result = Chain::generate_address(42);

        assert_eq!(result.len(), 42);
    }
}
