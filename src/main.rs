use clap::Parser;
use wallgdm::commands::Cli;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    cli.run()?;

    Ok(())
}
