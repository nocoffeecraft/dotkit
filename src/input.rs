use anyhow::Result;
use cliclack::{input, intro, outro, select};

use dotkit::{Contract, CT};

pub fn ask_input() -> Result<()> {
    let mut contract = Contract::default();

    intro("")?;
    let name = ask_name()?;
    contract.name(&name);

    let tpy = ask_ct()?;
    contract.ct(tpy);

    outro("Let's cook!🚀")?;

    println!("{:?}", contract);
    Ok(())
}

fn ask_name() -> Result<String> {
    let name: String = input("Enter the project name:")
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
