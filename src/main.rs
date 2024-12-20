use anyhow::Result;

/// Take inputs from users
mod input;

/// Render templates
mod template;

fn main() -> Result<()> {
    let c = input::ask_input()?;
    template::template(&c);
    Ok(())
}
