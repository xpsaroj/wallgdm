use clap::Parser;
use wallgdm::commands::Cli;

fn main() {
    let cli = Cli::parse();

    if let Err(err) = cli.run() {
        eprintln!("error: {}", err);
        std::process::exit(1);
    }
}
