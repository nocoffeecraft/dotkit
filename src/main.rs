use anyhow::Result;

mod input;

fn main() -> Result<()> {
    input::ask_input()?;
    Ok(())
}
