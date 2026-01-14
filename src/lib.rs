//! WallGDM: Set custom wallpapers for the GDM login screen.
//!
//! This crate provides commands, configuration management, and image processing
//! to update the GNOME Display Manager (GDM) login screen wallpaper.

pub mod commands;
pub mod error;

mod config;
mod image;
mod monitor;
mod theme;

use env_logger::Env;

/// Initialize logging for the crate.
///
/// By default, only warnings and errors are printed unless `RUST_LOG` is set.
/// Example:
/// ```bash
/// RUST_LOG=info wallgdm set --image path/to/image
/// ```
pub fn init_logging() {
    env_logger::Builder::from_env(Env::default().default_filter_or("warn"))
        .format_target(false) // don't show module paths in log output
        .format_timestamp(None) // optional: no timestamps, simpler output
        .init();
}
