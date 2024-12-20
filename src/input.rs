use anyhow::Result;
use cliclack::{input, intro, outro_note, select};
use console::style;

use dotkit::{Contract, CT};

pub fn ask_input() -> Result<Contract> {
    intro(style(" DotKit ").on_white().black())?;
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

fn ask_name() -> Result<String> {
    let name: String = input("Enter your project name:")
        .default_input("counter")
        .validate(|input: &String| {
            if input.is_empty() {
                Err("Value is required!")
            } else {
                Ok(())
            }
        })
        .interact()?;

    Ok(name)
}

fn ask_a_name() -> Result<String> {
    let a_name: String = input("Enter your name:")
        .default_input("[your_name]")
        .validate(|input: &String| {
            if input.is_empty() {
                Err("Value is required!")
            } else {
                Ok(())
            }
        })
        .interact()?;

    Ok(a_name)
}

fn ask_a_email() -> Result<String> {
    let a_email: String = input("Enter your email:")
        .default_input("[your_email@email.com]")
        .validate(|input: &String| {
            if input.contains("@") {
                Ok(())
            } else {
                Err("Invalid email address!")
            }
        })
        .interact()?;

    Ok(a_email)
}

fn ask_ct() -> Result<CT> {
    let typ = select("Pick a project type")
        .item(CT::Counter, "Counter Contract", "")
        .item(CT::Token, "Token Contract", "")
        .item(CT::NFT, "NFT Contract", "")
        .item(CT::Multisig, "Multisig Contract", "")
        .interact()?;

    Ok(typ)
}
