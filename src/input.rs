use anyhow::Result;
use cliclack::{input, intro, outro_note, select};
use console::style;

use dotkit::{Contract, CT};

/// Entry point
pub fn ask_input() -> Result<Contract> {
    intro(style(" DotKit ").on_white().black())?;

    // Build the contract struct
    let c = Contract::default()
        .name(&ask_name()?)
        .a_name(&ask_a_name()?)
        .a_email(&ask_a_email()?)
        .ct(ask_ct()?);

    outro_note(
        "Let's cook!🚀",
        "1. Open `lib.rs`\n2. Make some required changes\n3. Run `cargo contract build` to build the contract",
    )?;

    Ok(c)
}

/// Ask for contract/project name
fn ask_name() -> Result<String> {
    let name: String = input("Enter your project name:")
        .default_input("counter") // Default value
        .validate(|input: &String| {
            // Can not be empty
            if input.is_empty() {
                Err("Value is required!")
            } else {
                Ok(())
            }
        })
        .interact()?;

    Ok(name)
}

/// Ask for author name
fn ask_a_name() -> Result<String> {
    let a_name: String = input("Enter your name:")
        .default_input("[your_name]") // Default value
        .validate(|input: &String| {
            // Can not be empty
            if input.is_empty() {
                Err("Value is required!")
            } else {
                Ok(())
            }
        })
        .interact()?;

    Ok(a_name)
}

/// Ask for author email
fn ask_a_email() -> Result<String> {
    let a_email: String = input("Enter your email:")
        .default_input("[your_email@email.com]") // Default value
        .validate(|input: &String| {
            // Must be a valid email
            if input.contains("@") {
                Ok(())
            } else {
                Err("Invalid email address!")
            }
        })
        .interact()?;

    Ok(a_email)
}

/// Ask for contract type
fn ask_ct() -> Result<CT> {
    let typ = select("Pick a project type")
        .item(CT::Counter, "Counter Contract", "")
        .item(CT::Token, "Token Contract", "")
        .item(CT::NFT, "NFT Contract", "")
        .item(CT::Multisig, "Multisig Contract", "")
        .interact()?;

    Ok(typ)
}
