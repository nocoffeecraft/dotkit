// 1. Ask for email, full name
//
// 2. ask for name, symbol etc for erc20

use anyhow::Result;

mod input;
mod template;

fn main() -> Result<()> {
    let c = input::ask_input()?;
    template::template(&c);
    Ok(())
}
