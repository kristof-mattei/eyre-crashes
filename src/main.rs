use color_eyre::eyre::{Ok, Result, bail};

fn main() -> Result<()> {
    color_eyre::install()?;

    bail!("Does this crash?");

    #[expect(unreachable_code)]
    Ok(())
}
