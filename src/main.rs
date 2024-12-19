// 1. Ask for email, full name
//
// 2. ask for name, symbol etc for erc20

use anyhow::Result;

mod input;

fn main() -> Result<()> {
    input::ask_input()?;
    Ok(())
}
