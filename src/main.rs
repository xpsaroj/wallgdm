use clap::Parser;
use wallgdm::{commands::Cli, error::Result};

fn main() -> Result<()> {
    let cli = Cli::parse();
    cli.run()?;

    Ok(())
}
