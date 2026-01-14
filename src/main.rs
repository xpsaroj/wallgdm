//! Main entry point for `wallgdm`.
//! Handles CLI parsing, logger initialization, and running the selected command.

use clap::Parser;
use colored::Colorize;
use wallgdm::{commands::Cli, init_logging};

fn main() {
    // Initialize logger (logs can be filtered via RUST_LOG)
    init_logging();

    // Parse CLI arguments
    let cli = Cli::parse();

    // Run the CLI and handle errors
    if let Err(err) = cli.run() {
        eprintln!("{}: {}", "error".red().bold(), err);
        std::process::exit(1);
    }
}
