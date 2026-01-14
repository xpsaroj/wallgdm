//! Monitor layout transformation.
//!
//! This module adjusts monitor coordinates and resolutions based on a given scale factor.
//! The primary monitor is normalized and scaled, while secondary monitors are repositioned
//! relative to the primary monitor.  

use crate::error::MonitorError;
use crate::monitor::{Monitor, MonitorLayout};

/// Apply a scaling factor to the monitor layout.
///
/// The primary monitor is scaled according to `scale`, and all other monitors are
/// repositioned horizontally relative to the primary.  
/// Returns a new `MonitorLayout` with adjusted coordinates and dimensions.
///
/// # Errors
/// Returns `MonitorError::NoMonitorsFound` if the input layout has no monitors,
/// or `MonitorError::InvalidLayout` if the layout cannot be normalized.
pub fn apply_scale(
    layout: &MonitorLayout,
    scale: f32,
) -> Result<MonitorLayout, MonitorError> {
    if layout.monitors.is_empty() {
        return Err(MonitorError::NoMonitorsFound);
    }

    log::debug!("Applying scale factor {} to the layout", scale);

    // Normalize primary monitor
    let mut normalized = layout.monitors.clone();

    let primary_index =
        normalized.iter().position(|m| m.is_primary).unwrap_or(0); // fallback: first monitor

    for (i, m) in normalized.iter_mut().enumerate() {
        m.is_primary = i == primary_index;
    }

    let mut monitors = Vec::with_capacity(normalized.len());

    // Scale primary monitor
    let primary = &normalized[primary_index];
    monitors.push(Monitor {
        name: primary.name.clone(),
        width: ((primary.width as f32) / scale).round() as u32,
        height: ((primary.height as f32) / scale).round() as u32,
        x: 0,
        y: 0,
        is_primary: true,
    });

    // Adjust remaining monitors horizontally
    for m in normalized.iter().filter(|m| !m.is_primary) {
        let prev = monitors.last().ok_or(MonitorError::InvalidLayout)?;

        monitors.push(Monitor {
            name: m.name.clone(),
            width: m.width,
            height: m.height,
            x: prev.x + prev.width as i32,
            y: m.y,
            is_primary: false,
        });
    }

    let total_width = monitors
        .iter()
        .map(|m| m.x as u32 + m.width)
        .max()
        .ok_or(MonitorError::InvalidLayout)?;

    let total_height = monitors
        .iter()
        .map(|m| m.y as u32 + m.height)
        .max()
        .ok_or(MonitorError::InvalidLayout)?;

    log::debug!(
        "Scaled monitor layout: {:?}, total size: {}x{}",
        monitors,
        total_width,
        total_height
    );

    Ok(MonitorLayout {
        monitors,
        total_width,
        total_height,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_monitor_scaled_125_percent() {
        let layout = MonitorLayout {
            monitors: vec![Monitor {
                name: "eDP-1".into(),
                width: 1920,
                height: 1080,
                x: 0,
                y: 0,
                is_primary: true,
            }],
            total_width: 1920,
            total_height: 1080,
        };

        let scaled = apply_scale(&layout, 1.25).expect("scale failed");

        assert_eq!(scaled.monitors.len(), 1);
        assert_eq!(scaled.monitors[0].width, 1536);
        assert_eq!(scaled.monitors[0].height, 864);
        assert_eq!(scaled.total_width, 1536);
        assert_eq!(scaled.total_height, 864);
    }

    #[test]
    fn test_dual_monitors_primary_scaled() {
        let layout = MonitorLayout {
            monitors: vec![
                Monitor {
                    name: "eDP-1".into(),
                    width: 1920,
                    height: 1080,
                    x: 0,
                    y: 0,
                    is_primary: true,
                },
                Monitor {
                    name: "HDMI-A-1".into(),
                    width: 1920,
                    height: 1080,
                    x: 1920,
                    y: 0,
                    is_primary: false,
                },
            ],
            total_width: 3840,
            total_height: 1080,
        };

        let scaled = apply_scale(&layout, 1.25).expect("scale failed");

        assert_eq!(scaled.monitors.len(), 2);

        // primary scaled
        assert_eq!(scaled.monitors[0].width, 1536);
        assert_eq!(scaled.monitors[0].height, 864);

        // secondary repositioned
        assert_eq!(scaled.monitors[1].x, 1536);
        assert_eq!(scaled.monitors[1].width, 1920);

        assert_eq!(scaled.total_width, 1536 + 1920);
        assert_eq!(scaled.total_height, 1080);
    }

    #[test]
    fn test_primary_not_first() {
        let layout = MonitorLayout {
            monitors: vec![
                Monitor {
                    name: "HDMI-A-1".into(),
                    width: 1920,
                    height: 1080,
                    x: 1920,
                    y: 0,
                    is_primary: false,
                },
                Monitor {
                    name: "eDP-1".into(),
                    width: 1920,
                    height: 1080,
                    x: 0,
                    y: 0,
                    is_primary: true,
                },
            ],
            total_width: 3840,
            total_height: 1080,
        };

        let scaled = apply_scale(&layout, 1.25).expect("scale failed");

        assert_eq!(scaled.monitors[0].name, "eDP-1");
        assert_eq!(scaled.monitors[0].width, 1536);
        assert_eq!(scaled.monitors[1].x, 1536);
    }

    #[test]
    fn test_empty_layout_is_error() {
        let layout = MonitorLayout {
            monitors: vec![],
            total_width: 0,
            total_height: 0,
        };

        assert!(apply_scale(&layout, 1.25).is_err());
    }

    #[test]
    fn no_primary_does_not_duplicate_monitors() {
        let layout = MonitorLayout {
            monitors: vec![
                Monitor {
                    name: "eDP-1".into(),
                    width: 1920,
                    height: 1080,
                    x: 0,
                    y: 0,
                    is_primary: false,
                },
                Monitor {
                    name: "HDMI-A-1".into(),
                    width: 1920,
                    height: 1080,
                    x: 1920,
                    y: 0,
                    is_primary: false,
                },
            ],
            total_width: 3840,
            total_height: 1080,
        };

        let scaled = apply_scale(&layout, 1.25).expect("scale failed");

        assert_eq!(scaled.monitors.len(), 2);

        let primary_count =
            scaled.monitors.iter().filter(|m| m.is_primary).count();
        assert_eq!(primary_count, 1);
    }
}
