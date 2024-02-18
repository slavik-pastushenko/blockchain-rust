use std::{collections::HashMap, fmt::Write, iter};

use rand::Rng;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::{Block, Transaction, Wallet};

/// A blockchain.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Chain {
    /// Chain of blocks.
    pub chain: Vec<Block>,

    /// List of transactions.
    pub current_transactions: Vec<Transaction>,

    /// Current difficulty level of the network.
    pub difficulty: f64,

    /// Blockchain genesis address.
    pub address: String,

    /// Block reward.
    pub reward: f64,

    /// Transaction fee.
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
    /// # Arguments
    /// - `page`: The page number.
    /// - `size`: The number of transactions per page.
    ///
    /// # Returns
    /// A reference to a vector containing the current transactions for the specified page.
    pub fn get_transactions(&self, page: usize, size: usize) -> Vec<Transaction> {
        // Calculate the total number of pages
        let total_pages = (self.current_transactions.len() + size - 1) / size;

        // Return an empty vector if the page is greater than the total number of pages
        if page > total_pages {
            return Vec::new();
        }

        // Calculate the start and end indices for the transactions of the current page
        let start = page.saturating_sub(1) * size;
        let end = start + size;

        // Get the transactions for the current page
        self.current_transactions[start..end.min(self.current_transactions.len())].to_vec()
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
        let total = amount * self.fee;

        // Validate the transaction and create a new transaction if it is valid
        let transaction = match self.validate_transaction(&from, &to, total) {
            true => Transaction::new(from.to_owned(), to.to_owned(), self.fee, total),
            false => return false,
        };

        // Update sender's balance
        match self.wallets.get_mut(&from) {
            Some(wallet) => {
                wallet.balance -= total;

                // Add the transaction to the sender's transaction history
                wallet.transactions.push(transaction.hash.to_owned());
            }
            None => return false,
        };

        // Update receiver's balance
        match self.wallets.get_mut(&to) {
            Some(wallet) => {
                wallet.balance += amount;

                // Add the transaction to the receiver's transaction history
                wallet.transactions.push(transaction.hash.to_owned());
            }
            None => return false,
        };

        // Add the transaction to the current transactions
        self.current_transactions.push(transaction);

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
        let sender = match self.wallets.get(from) {
            Some(wallet) => wallet,
            None => return false,
        };

        // Validate if the receiver address is valid
        if self.wallets.get(to).is_none() {
            return false;
        }

        // Validate if sender can send the amount of the transaction
        if sender.balance < amount {
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

        let wallet = Wallet::new(email, address.to_owned(), 0.0);

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

    /// Get a wallet's transaction history based on its address.
    ///
    /// # Arguments
    /// - `address`: The unique wallet address.
    /// - `page`: The page number.
    /// - `size`: The number of transactions per page.
    ///
    /// # Returns
    /// The wallet transaction history for the specified page.
    pub fn get_wallet_transactions(
        &self,
        address: String,
        page: usize,
        size: usize,
    ) -> Option<Vec<Transaction>> {
        match self
            .wallets
            .get(&address)
            .map(|wallet| wallet.transactions.to_owned())
        {
            // Get the transaction history of the wallet
            Some(txs) => {
                let mut result = Vec::new();

                // Calculate the total number of pages
                let total_pages = (self.current_transactions.len() + size - 1) / size;

                // Return an empty vector if the page is greater than the total number of pages
                if page > total_pages {
                    return Some(result);
                }

                // Calculate the start and end indices for the transactions of the current page
                let start = page.saturating_sub(1) * size;
                let end = start + size;

                for tx in txs[start..end.min(txs.len())].iter() {
                    match self.get_transaction(tx.to_string()) {
                        Some(transaction) => result.push(transaction.to_owned()),
                        None => continue,
                    }
                }

                Some(result)
            }
            // Return None if the wallet is not found
            None => None,
        }
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
        // Create a new block
        let mut block = Block::new(self.get_last_hash(), self.difficulty);

        // Create a reward transaction
        let transaction = Transaction::new(
            "Root".to_string(),
            self.address.to_string(),
            self.fee,
            self.reward,
        );

        // Add the reward transaction to the block
        block.transactions.push(transaction);
        block.transactions.append(&mut self.current_transactions);

        // Update the block count and the Merkle root hash
        block.count = block.transactions.len();
        block.header.merkle = Chain::get_merkle(block.transactions.clone());

        // Perform the proof-of-work process
        Block::proof_of_work(&mut block.header);

        // Add the block to the blockchain
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

    #[test]
    fn test_generate_address() {
        let result = Chain::generate_address(42);

        assert_eq!(result.len(), 42);
    }
}
