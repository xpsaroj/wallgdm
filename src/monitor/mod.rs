//! Monitor detection and layout management for WallGDM.
//!
//! This module handles detection of connected monitors, parsing their geometry,
//! and applying DPI scaling to produce a `MonitorLayout` suitable for wallpaper composition.

mod detect;
mod transform;

use crate::error::MonitorError;

/// Represents a single monitor's geometry and position.
#[derive(Debug, Clone)]
pub struct Monitor {
    /// Monitor name as reported by the system.
    pub name: String,
    /// Width in pixels.
    pub width: u32,
    /// Height in pixels.
    pub height: u32,
    /// X position relative to the primary monitor.
    pub x: i32,
    /// Y position relative to the primary monitor.
    pub y: i32,
    /// Whether this monitor is the primary display.
    pub is_primary: bool,
}

/// Layout of all connected monitors.
#[derive(Debug)]
pub struct MonitorLayout {
    /// List of monitors.
    pub monitors: Vec<Monitor>,
    /// Total width of the combined monitor layout.
    pub total_width: u32,
    /// Total height of the combined monitor layout.
    pub total_height: u32,
}

/// Get the current monitor layout, applying the given scale factor.
///
/// # Parameters
/// - `scale`: DPI scaling factor to adjust monitor coordinates and sizes.
///
/// # Errors
/// Returns a `MonitorError` if monitor detection or scaling fails.
pub fn get_monitor_layout(scale: f32) -> Result<MonitorLayout, MonitorError> {
    log::info!("Getting monitor layout with scale factor: {}", scale);
    // Detect raw monitors
    let monitor_layout = detect::detect_monitors()?;
    for monitor in &monitor_layout.monitors {
        log::info!(
            "Detected monitor: {} ({}x{} at {}, {}){}",
            monitor.name,
            monitor.width,
            monitor.height,
            monitor.x,
            monitor.y,
            if monitor.is_primary { " [PRIMARY]" } else { "" }
        );
    }
    log::info!(
        "Total height: {}, width: {}",
        monitor_layout.total_height,
        monitor_layout.total_width
    );

    // Apply scaling
    let monitor_layout = transform::apply_scale(&monitor_layout, scale)?;
    for monitor in &monitor_layout.monitors {
        log::info!(
            "Scaled monitor: {} ({}x{} at {}, {}){}",
            monitor.name,
            monitor.width,
            monitor.height,
            monitor.x,
            monitor.y,
            if monitor.is_primary { " [PRIMARY]" } else { "" }
        );
    }
    log::info!(
        "Scaled total height: {}, width: {}",
        monitor_layout.total_height,
        monitor_layout.total_width
    );

    Ok(monitor_layout)
}
