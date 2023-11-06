# Blockchain CLI

A Rust library provides a command-line interface (CLI) for interacting with blockchain.

## Reference implementation

[![test](https://github.com/slavik-pastushenko/blockchain-rust/actions/workflows/test.yml/badge.svg)](https://github.com/slavik-pastushenko/blockchain-rust/actions/workflows/test.yml)
[![docs](https://docs.rs/blockchain-cli/badge.svg)](https://docs.rs/blockchain-cli)
[![crate](https://img.shields.io/crates/v/blockchain-cli.svg)](https://crates.io/crates/blockchain-cli)
![Crates.io (recent)](https://img.shields.io/crates/dr/blockchain-cli)
![GitHub](https://img.shields.io/github/license/slavik-pastushenko/blockchain-rust)

![Features](https://github.com/slavik-pastushenko/blockchain-rust/assets/16807375/f9f15dbf-8594-4a1c-9d7a-675567a205da)

## Features

- `new(address, difficulty, reward)`: Initialize a new blockchain with the specified parameters.
- `get_transactions()`: Get a list of current transactions in the blockchain.
- `get_transaction(hash)`: Get a transaction by its hash.
- `add_transaction(from, to, amount)`: Add a new transaction to the blockchain.
- `get_last_hash()`: Get the hash of the last block in the blockchain.
- `update_difficulty(difficulty)`: Update the mining difficulty of the blockchain.
- `update_reward(reward)`: Update the block reward.
- `generate_new_block()`: Generate a new block and append it to the blockchain.
- `get_merkle(transactions)`: Calculate the Merkle root hash for a list of transactions.
- `proof_of_work(header)`: Perform the proof-of-work process to mine a block.
- `hash(item)`: Calculate the SHA-256 hash of a serializable item.

## Options

| Option       | Description                                                       |
|--------------|-------------------------------------------------------------------|
| `address`    | The address associated with the blockchain.                       |
| `difficulty` | The initial mining difficulty level of the network.               |
| `reward`     | The initial block reward for miners.                              |

## Safety

This crate uses `#![forbid(unsafe_code)]` to ensure everything is implemented in 100% safe Rust.

## Usage

Run the following Cargo command in your project directory::

```bash
cargo add blockchain-cli
```

```rust
extern crate blockchain;

use blockchain::Chain;

fn main() {
  // Initialise a new blockchain
  let mut chain = Chain::new(
    String::from("bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh"),
    2,
    100.0
  );

  // Add a transaction
  chain.add_transaction(
    String::from("mxwgXGHxtjmGJ1cFebRW9emcV2vV1aPGfk"),
    String::from("n2zet2T3KNRjD69oF9ZquLsigH1ZBJcraR"),
    1.25
  );

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

- Build an application:

```bash
cargo build
```

- Test an application:

```bash
cargo test
```

- Run an application:

```bash
cargo run
```

- Run [clippy](https://github.com/rust-lang/rust-clippy):

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

- Generate documentation in HTML format:

```bash
cargo doc --open
```
