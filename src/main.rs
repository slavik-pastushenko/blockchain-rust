use cliclack::spinner;

mod blockchain;

fn main() -> std::io::Result<()> {
    cliclack::clear_screen()?;

    let address: String = cliclack
        ::input("Address")
        .validate(|input: &String| {
            if input.is_empty() { Err("Please enter an address") } else { Ok(()) }
        })
        .interact()?;

    let difficulty: u32 = cliclack
        ::input("Difficulty")
        .default_input("2")
        .validate(|input: &String| {
            if input.is_empty() { Err("Please enter a difficulty") } else { Ok(()) }
        })
        .interact()?;

    let mut spinner = spinner();
    spinner.start("Generating a genesis block...");

    let mut chain = blockchain::Chain::new(address.trim().to_string(), difficulty);

    spinner.stop("✅ Blockchain was created successfully");

    loop {
        let action = cliclack
            ::select("💡 Select an action")
            .initial_value("add_transaction")
            .item("add_transaction", "Add a new transaction", "")
            .item("get_transaction", "Get a transaction", "")
            .item("get_transactions", "Get all transactions", "")
            .item("generate_block", "Generate a new block", "")
            .item("change_reward", "Change a reward", "")
            .item("change_difficulty", "Change a difficulty", "")
            .item("exit", "Exit", "")
            .interact()?;

        match action {
            "add_transaction" => {
                let sender: String = cliclack
                    ::input("Sender")
                    .validate(|input: &String| {
                        if input.is_empty() { Err("Please enter a sender") } else { Ok(()) }
                    })
                    .interact()?;

                let receiver: String = cliclack
                    ::input("Receiver")
                    .validate(|input: &String| {
                        if input.is_empty() { Err("Please enter a receiver") } else { Ok(()) }
                    })
                    .interact()?;

                let amount: f32 = cliclack
                    ::input("Amount")
                    .validate(|input: &String| {
                        if input.is_empty() { Err("Please enter an amount") } else { Ok(()) }
                    })
                    .interact()?;

                let confirm = cliclack::confirm("Confirm adding a transaction").interact()?;

                if confirm {
                    let res = chain.add_transaction(
                        sender.trim().to_string(),
                        receiver.trim().to_string(),
                        amount
                    );

                    match res {
                        true => println!("✅ Transaction was added successfully"),
                        false => println!("❌ Cannot add a transaction"),
                    }
                }
            }
            "get_transaction" => {
                let hash: String = cliclack
                    ::input("Transaction hash")
                    .validate(|input: &String| {
                        if input.is_empty() {
                            Err("Please enter a transaction hash")
                        } else {
                            Ok(())
                        }
                    })
                    .interact()?;

                let res = chain.get_transaction(hash);

                match res {
                    Some(trx) => println!("📦 {:?}", trx),
                    None => println!("❌ Transaction was not found"),
                }
            }
            "get_transactions" => {
                println!("📦 {:?}", chain.get_transactions());
            }
            "generate_block" => {
                let res = chain.generate_new_block();
                match res {
                    true => println!("✅ Block was generated successfully"),
                    false => println!("❌ Cannot generate a block"),
                }
            }
            "change_reward" => {
                let new_reward: String = cliclack
                    ::input("New reward")
                    .validate(|input: &String| {
                        if input.is_empty() { Err("Please enter a new reward") } else { Ok(()) }
                    })
                    .interact()?;

                let confirm = cliclack::confirm("Confirm changing a reward").interact()?;

                if confirm {
                    let res = chain.update_reward(new_reward.trim().parse().unwrap());

                    match res {
                        true => println!("✅ Reward was changed successfully"),
                        false => println!("❌ Cannot generate a reward"),
                    }
                }
            }
            "change_difficulty" => {
                let new_difficulty: u32 = cliclack
                    ::input("New difficulty")
                    .validate(|input: &String| {
                        if input.is_empty() { Err("Please enter a new difficulty") } else { Ok(()) }
                    })
                    .interact()?;

                let confirm = cliclack::confirm("Confirm changing a difficulty").interact()?;

                if confirm {
                    let res = chain.update_difficulty(new_difficulty);

                    match res {
                        true => println!("✅ Difficulty was changed successfully"),
                        false => println!("❌ Cannot generate a difficulty"),
                    }
                }
            }
            "exit" => {
                break;
            }
            _ => {
                break;
            }
        }
    }

    Ok(())
}
