//! Implements the 'status' command functionality.
//!
//! This module provides functionality to check and display the current
//! status of the GDM login screen wallpaper.

use crate::config::StatusDirs;
use crate::error::Result;
use crate::error::StatusError;
use crate::monitor::detect_monitors;

pub fn run() -> Result<()> {
    log::info!("Checking GDM wallpaper status");

    // Prepare working directories
    let dirs = StatusDirs::new().map_err(StatusError::from)?;

    let monitor_layout = detect_monitors().map_err(StatusError::from)?;
    
    println!("Detected Monitors:");
    for monitor in &monitor_layout.monitors {
        println!(
            "  {}: {}x{} at ({}, {}){}",
            monitor.name,
            monitor.width,
            monitor.height,
            monitor.x,
            monitor.y,
            if monitor.is_primary { " [PRIMARY]" } else { "" }
        );
    }

    Ok(())
}
