use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::Chain;

/// Exchange of assets between two parties.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Transaction {
    /// Transaction hash.
    pub hash: String,

    /// Transaction sender address.
    pub from: String,

    /// Transaction receiver address.
    pub to: String,

    /// Transaction fee.
    pub fee: f64,

    /// Transaction amount.
    pub amount: f64,

    /// Transaction timestamp.
    pub timestamp: i64,
}

impl Transaction {
    /// Create a new transaction.
    ///
    /// # Arguments
    ///
    /// - `from` - The transaction sender address.
    /// - `to` - The transaction receiver address.
    /// - `fee` - The transaction fee.
    /// - `amount` - The transaction amount.
    ///
    /// # Returns
    ///
    /// A new transaction with the given hash, sender, receiver, fee, amount, and timestamp.
    pub fn new(from: String, to: String, fee: f64, amount: f64) -> Self {
        let timestamp = Utc::now().timestamp();

        // Create a hash of the transaction
        let hash = Chain::hash(&(&from, &to, amount, timestamp));

        // Create a new transaction
        Transaction {
            hash,
            from,
            to,
            fee,
            amount,
            timestamp,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_transaction() {
        let from = "0x 1234".to_string();
        let to = "0x 5678".to_string();
        let fee = 0.1;
        let amount = 100.0;
        let transaction = Transaction::new(from.to_owned(), to.to_owned(), fee, amount);

        assert_eq!(transaction.from, from);
        assert_eq!(transaction.to, to);
        assert_eq!(transaction.fee, fee);
        assert_eq!(transaction.amount, amount);
    }
}
