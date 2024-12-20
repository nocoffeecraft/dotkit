use anyhow::Result;
use cliclack::{input, intro, outro_note, select};
use console::style;

use dotkit::{Contract, CT};

pub fn ask_input() -> Result<Contract> {
    let mut c = Contract::default();

    intro(style(" DotKit ").on_white().black())?;
    c = c.name(&ask_name()?)
        .a_name(&ask_a_name()?)
        .a_email(&ask_a_email()?)
        .ct(ask_ct()?);

    outro_note(
        "Let's cook!🚀",
        "1. explian next steps\n2. next step\n3. okey done",
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
        .item("c", "Counter Contract", "")
        .item("t", "Token Contract", "")
        .item("n", "NFT Contract", "")
        .item("s", "Swap Contract", "")
        .item("v", "Voting Contract", "")
        .interact()?;

    let typ = match typ {
        "c" => CT::Counter,
        "t" => CT::Token,
        "n" => CT::NFT,
        "s" => CT::Swap,
        "v" => CT::Voting,
        _ => CT::Counter,
    };

    Ok(typ)
}
