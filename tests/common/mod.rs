use blockchain::Chain;

/// Setup a new blockchain.
///
/// # Returns
///
/// A new blockchain with a difficulty of 1.0, a reward of 100.0, and a fee of 0.1.
pub fn setup() -> Chain {
    Chain::new(1.0, 100.0, 0.1)
}
