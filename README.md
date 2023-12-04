# Blockchain

A Rust crate provides an interface for interacting with a blockchain.

## Reference implementation

[![test](https://github.com/slavik-pastushenko/blockchain-rust/actions/workflows/test.yml/badge.svg)](https://github.com/slavik-pastushenko/blockchain-rust/actions/workflows/test.yml)
[![release](https://github.com/slavik-pastushenko/blockchain-rust/actions/workflows/release.yml/badge.svg?event=workflow_dispatch)](https://github.com/slavik-pastushenko/blockchain-rust/actions/workflows/release.yml)
[![docs](https://docs.rs/blockchain-cli/badge.svg)](https://docs.rs/blockchain-cli)
[![crate](https://img.shields.io/crates/v/blockchain-cli.svg)](https://crates.io/crates/blockchain-cli)
![downloads](https://img.shields.io/crates/d/blockchain-cli)
[![codecov](https://codecov.io/gh/slavik-pastushenko/blockchain-rust/graph/badge.svg?token=9EL0F6725A)](https://codecov.io/gh/slavik-pastushenko/blockchain-rust)

![Features](https://github.com/slavik-pastushenko/blockchain-rust/assets/16807375/28123ed1-aa79-40d7-a59a-3a0710acc381)

## Features

- `new(difficulty, reward, fee)`: Initialize a new blockchain with the specified parameters.
- `get_transactions()`: Get a list of current transactions in the blockchain.
- `get_transaction(hash)`: Get a transaction by its hash.
- `add_transaction(from, to, amount)`: Add a new transaction to the blockchain.
- `validate_transaction(from, amount)`: Validate a new transaction to the blockchain.
- `create_wallet(email)`: Create a new wallet with a unique email and an initial balance.
- `get_wallet_balance(address)`: Get a wallet's balance based on its address.
- `get_last_hash()`: Get the hash of the last block in the blockchain.
- `update_difficulty(difficulty)`: Update the mining difficulty of the blockchain.
- `update_reward(reward)`: Update the block reward.
- `update_fee(fee)`: Update the transaction fee.
- `generate_new_block()`: Generate a new block and append it to the blockchain.
- `get_merkle(transactions)`: Calculate the Merkle root hash for a list of transactions.
- `proof_of_work(header)`: Perform the proof-of-work process to mine a block.
- `hash(item)`: Calculate the SHA-256 hash of a serializable item.

## Options

| Option       | Data type    | Description                                                       |
|--------------|--------------|-------------------------------------------------------------------|
| `difficulty` | `f64`        | The initial mining difficulty level of the network.               |
| `reward`     | `f64`        | The initial block reward for miners.                              |
| `fee`        | `f64`        | The transaction fee.                                              |

## Safety

This crate uses `#![forbid(unsafe_code)]` to ensure everything is implemented in 100% safe Rust.

## Documentation

For more in-depth details, please refer to the full [documentation](https://docs.rs/blockchain-cli).

If you encounter any issues or have questions that are not addressed in the documentation, feel free to [submit an issue](https://github.com/slavik-pastushenko/blockchain-rust/issues).

## Examples

Explore the capabilities of this blockchain implementation through a set of examples:

- CLI for interacting with the blockchain: [see more](https://github.com/slavik-pastushenko/blockchain-rust/tree/main/examples/cli)
- API for interacting with the blockchain using axum: [see more](https://github.com/slavik-pastushenko/blockchain-rust/tree/main/examples/api-axum)

## Usage

```rust
use blockchain::Chain;

fn main() {
  // Initialise a new blockchain
  let mut chain = Chain::new(2, 100.0, 0.01);

  // Create a wallet for a sender
  let sender = chain.create_wallet(String::from("sender@mail.com"));
  
  // Create a wallet for a receiver
  let receiver = chain.create_wallet(String::from("receiver@mail.com"));

  // Add a transaction
  chain.add_transaction(sender, receiver, 1.25);

  // Get a transaction
  let transaction = chain.get_transaction(
    String::from("6e8c5dc01145016e5a979683ba7e13bafaf85e765490aa33c0bba1f41cf581ed")
  );

  match transaction {
    Some(trx) => println!("📦 Transaction: {:?}", trx),
    None => println!("❌ Transaction was not found"),
  }

  // Get all transactions
  let transactions = chain.get_transactions();
  println!("📦 Transactions: {:?}", transactions);

  // Others
}
```

## Contributing

Build the application:

```bash
cargo build
```

Test the application:

```bash
cargo test
```

Run the application:

```bash
cargo run
```

Run [clippy](https://github.com/rust-lang/rust-clippy):

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

Run [lint](https://github.com/rust-lang/rustfmt):

```bash
cargo fmt
```

Generate documentation in HTML format:

```bash
cargo doc --open
```
