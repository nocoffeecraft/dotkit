use anyhow::Result;

mod input;
mod template;

fn main() -> Result<()> {
    let c = input::ask_input()?;
    template::template(&c);
    Ok(())
}
