use crate::error::Result;
use clap::{Parser, Subcommand};

pub mod list;
pub mod revert;
pub mod set;
pub mod status;

/// Command line interface structure
#[derive(Parser, Debug)]
#[command(
    name = "wallgdm",
    version,
    about = "A wallpaper manager for GNOME login screen with multi-monitor support",
    long_about = None,
    arg_required_else_help = true,
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

/// Available commands
#[derive(Subcommand, Debug)]
pub enum Commands {
    Set(set::SetArgs),
    Revert,
    Status,
    List,
}

impl Cli {
    /// Execute the selected command
    pub fn run(self) -> Result<()> {
        match self.command {
            Commands::Set(args) => set::run(args),
            Commands::Revert => revert::run(),
            Commands::Status => status::run(),
            Commands::List => list::run(),
        }
    }
}
