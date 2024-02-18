use serde::{Deserialize, Serialize};

/// A wallet that holds a balance of a cryptocurrency.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Wallet {
    /// Unique email address associated with the wallet.
    pub email: String,

    /// Address uniquely identifying the wallet.
    pub address: String,

    /// The current balance of the wallet.
    pub balance: f64,

    /// A history of transactions associated with the wallet.
    pub transactions: Vec<String>,
}

impl Wallet {
    /// Create a new wallet.
    ///
    /// # Arguments
    ///
    /// * `email` - The email address associated with the wallet.
    /// * `address` - The address uniquely identifying the wallet.
    /// * `balance` - The current balance of the wallet.
    ///
    /// # Returns
    ///
    /// A new wallet with the given email, address, and balance.
    pub fn new(email: String, address: String, balance: f64) -> Self {
        Wallet {
            email,
            address,
            balance,
            transactions: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_wallet() {
        let email = "email".to_string();
        let address = "0x 1234".to_string();
        let balance = 100.0;
        let wallet = Wallet::new(email.to_owned(), address.to_owned(), balance);

        assert_eq!(wallet.email, email);
        assert_eq!(wallet.address, address);
        assert_eq!(wallet.balance, balance);
        assert!(wallet.transactions.is_empty());
    }
}
