# Blockchain CLI

## Reference implementation

[![test](https://github.com/slavik-pastushenko/blockchain-rust/actions/workflows/test.yml/badge.svg)](https://github.com/slavik-pastushenko/blockchain-rust/actions/workflows/test.yml)
[![docs](https://docs.rs/blockchain-cli/badge.svg)](https://docs.rs/blockchain-cli)
[![crate](https://img.shields.io/crates/v/blockchain-cli.svg)](https://crates.io/crates/blockchain-cli)
![Crates.io (recent)](https://img.shields.io/crates/dr/blockchain-cli)
![GitHub issues](https://img.shields.io/github/issues/slavik-pastushenko/blockchain-rust)
![GitHub](https://img.shields.io/github/license/slavik-pastushenko/blockchain-rust)

A Rust library provides a command-line interface (CLI) for interacting with blockchain.

## Features

- Create a blockchain
- Add a transaction
- Get a transaction
- Get all transactions
- Generate a block
- Change a reward
- Change a difficulty

## Usage

Run the following Cargo command in your project directory::

```bash
cargo add blockchain-cli
```

![Usage](https://github.com/slavik-pastushenko/blockchain-rust/assets/16807375/f9f15dbf-8594-4a1c-9d7a-675567a205da)

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
