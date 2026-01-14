//! Command-line interface definitions and dispatch logic for WallGDM.
//!
//! This module defines the top-level CLI structure using `clap` and
//! routes subcommands (`set`, `install`, `revert`, `status`, `list`)
//! to their respective handlers.
//!
//! It acts as the central entry point for user-facing commands and
//! ensures consistent error propagation to the application root.

use std::path::PathBuf;

use crate::error::Result;
use clap::{Parser, Subcommand};

mod install;
mod list;
mod revert;
mod set;
mod status;

/// Command line interface for WallGDM.
///
/// Provides subcommands to set, install, revert, list, or check the status
/// of GDM login screen wallpapers.
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
    /// Set a new wallpaper for the GDM login screen
    Set(set::SetArgs),

    /// Install a custom GResource file for the login screen
    Install { gresource: PathBuf },

    /// Revert to the previous wallpaper
    Revert,

    /// Show the current wallpaper status
    Status,

    /// List available wallpapers or themes
    List,
}

impl Cli {
    /// Execute the selected command.
    ///
    /// Returns a `Result` which is:
    /// - `Ok(())` on success
    /// - `Err(WallGdmError)` if any command-specific or system error occurs
    ///
    /// Errors should be handled at the top level.
    pub fn run(self) -> Result<()> {
        match self.command {
            Commands::Set(args) => set::run(args),
            Commands::Install { gresource } => install::run(&gresource),
            Commands::Revert => revert::run(),
            Commands::Status => status::run(),
            Commands::List => list::run(),
        }
    }
}
