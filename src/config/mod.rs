//! Configuration and filesystem layout for WallGDM.
//!
//! This module provides constants and directory helpers used across
//! commands to locate data directories, temporary workspaces, and
//! GNOME Shell resources.

mod constants;
mod dirs;
mod state;

pub use constants::*;
pub use dirs::*;
pub use state::*;