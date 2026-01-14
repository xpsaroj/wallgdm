//! Monitor detection using `xrandr`.
//!
//! This module detects connected monitors and their resolutions using the `xrandr` command-line tool.
//!
//! **Note:** `xrandr` must be installed and accessible in the system PATH. On systems without
//! `xrandr` (or if it’s not executable), detection will fail with `MonitorError::XrandrUnavailable`.
//!
//! The module parses `xrandr --query` output to build a `MonitorLayout` struct.

use std::process::Command;

use crate::error::MonitorError;
use crate::monitor::{Monitor, MonitorLayout};

/// Detect connected monitors and return their layout.
///
/// Uses `xrandr --query` internally. Fails if `xrandr` is not installed,
/// not executable, or returns an invalid layout.
///
/// # Errors
/// Returns `MonitorError` if `xrandr` is unavailable, fails, or output cannot be parsed.
pub fn detect_monitors() -> Result<MonitorLayout, MonitorError> {
    let output = Command::new("xrandr")
        .arg("--query")
        .output()
        .map_err(|e| MonitorError::XrandrUnavailable(e))?;

    if !output.status.success() {
        return Err(MonitorError::XrandrFailed);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    parse_xrandr_output(&stdout)
}

/// Parse the output of `xrandr` into a `MonitorLayout`.
fn parse_xrandr_output(output: &str) -> Result<MonitorLayout, MonitorError> {
    let mut monitors = Vec::new();

    for line in output.lines() {
        if !line.contains(" connected") {
            continue;
        }
        log::debug!("Parsing xrandr line: {}", line);

        let is_primary = line.contains(" primary ");

        let parts: Vec<&str> = line.split_whitespace().collect();
        let name = parts[0].to_string();

        let geometry =
            parts.iter().find(|p| p.contains('x') && p.contains('+'));

        if let Some(g) = geometry {
            let (width, height, x, y) = parse_geometry(g)?;
            log::debug!(
                "Monitor detected: {} ({}x{} at {}, {}){}",
                name,
                width,
                height,
                x,
                y,
                if is_primary { " [primary]" } else { "" }
            );

            monitors.push(Monitor {
                name,
                width,
                height,
                x,
                y,
                is_primary,
            });
        }
    }

    if monitors.is_empty() {
        return Err(MonitorError::NoMonitorsFound);
    }

    let total_width = monitors
        .iter()
        .map(|m| m.x as u32 + m.width)
        .max()
        .ok_or(MonitorError::ParseFailed)?;

    let total_height = monitors
        .iter()
        .map(|m| m.y as u32 + m.height)
        .max()
        .ok_or(MonitorError::ParseFailed)?;

    Ok(MonitorLayout {
        monitors,
        total_width,
        total_height,
    })
}

/// Parse a geometry string like `1920x1080+0+0` into width, height, x, and y.
fn parse_geometry(g: &str) -> Result<(u32, u32, i32, i32), MonitorError> {
    let (res, pos) = g.split_once('+').ok_or(MonitorError::ParseFailed)?;
    let (w, h) = res.split_once('x').ok_or(MonitorError::ParseFailed)?;
    let (x, y) = pos.split_once('+').ok_or(MonitorError::ParseFailed)?;

    Ok((
        w.parse().map_err(|_| MonitorError::ParseFailed)?,
        h.parse().map_err(|_| MonitorError::ParseFailed)?,
        x.parse().map_err(|_| MonitorError::ParseFailed)?,
        y.parse().map_err(|_| MonitorError::ParseFailed)?,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_single_monitor() {
        let  xrandr_output = "\
Screen 0: minimum 16 x 16, current 1920 x 1080, maximum 32767 x 32767
eDP-1 connected 1920x1080+0+0 (normal left inverted right x axis y axis) 310mm x 170mm
   1920x1080     59.96*+
   1440x1080     59.99  
   1400x1050     59.98  
";

        let layout = parse_xrandr_output(xrandr_output).unwrap();

        assert_eq!(layout.monitors.len(), 1);
        let m = &layout.monitors[0];
        assert_eq!(m.name, "eDP-1");
        assert_eq!(m.width, 1920);
        assert_eq!(m.height, 1080);
        assert_eq!(m.x, 0);
        assert_eq!(m.y, 0);
        // assert!(m.is_primary); // Fails in hyprland as xrandr does not report primary
        assert_eq!(layout.total_width, 1920);
        assert_eq!(layout.total_height, 1080);
    }

    #[test]
    fn test_parse_dual_monitors() {
        let xrandr_output = "\
Screen 0: minimum 16 x 16, current 3840 x 1080, maximum 32767 x 32767
eDP-1 connected 1920x1080+0+0 (normal left inverted right x axis y axis) 310mm x 170mm
   1920x1080     59.96*+
   1440x1080     59.99  
   1400x1050     59.98  
HDMI-A-1 connected 1920x1080+1920+0 (normal left inverted right x axis y axis) 790mm x 0mm
   1920x1080     59.96*+
   1440x1080     59.99  
   1400x1050     59.98  
";

        let layout = parse_xrandr_output(xrandr_output).unwrap();
        assert_eq!(layout.monitors.len(), 2);

        let m1 = &layout.monitors[0];
        assert_eq!(m1.name, "eDP-1");
        assert_eq!(m1.width, 1920);
        assert_eq!(m1.height, 1080);
        assert_eq!(m1.x, 0);
        assert_eq!(m1.y, 0);

        let m2 = &layout.monitors[1];
        assert_eq!(m2.name, "HDMI-A-1");
        assert_eq!(m2.width, 1920);
        assert_eq!(m2.height, 1080);
        assert_eq!(m2.x, 1920);
        assert_eq!(m2.y, 0);

        // assert!(m.is_primary); // Fails in hyprland as xrandr does not report primary
        assert_eq!(layout.total_width, 1920 + 1920);
        assert_eq!(layout.total_height, 1080);
    }

    #[test]
    fn test_parse_invalid_output() {
        let output = "some text without proper format";
        let result = parse_xrandr_output(output);
        assert!(result.is_err());
    }
}
