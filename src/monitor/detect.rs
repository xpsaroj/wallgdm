use std::process::Command;

use crate::error::MonitorError;
use crate::monitor::{Monitor, MonitorLayout};

pub fn detect_monitors()
-> Result<MonitorLayout, MonitorError> {
    let output = Command::new("xrandr")
        .arg("--query")
        .output()
        .map_err(|_| MonitorError::DetectionFailed)?;

    if !output.status.success() {
        return Err(MonitorError::DetectionFailed);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    parse_xrandr_output(&stdout)
}

fn parse_xrandr_output(
    output: &str,
) -> Result<MonitorLayout, MonitorError> {
    let mut monitors = Vec::new();

    for line in output.lines() {
        if !line.contains(" connected") {
            continue;
        }

        let is_primary = line.contains(" primary ");

        let parts: Vec<&str> =
            line.split_whitespace().collect();
        let name = parts[0].to_string();

        let geometry = parts
            .iter()
            .find(|p| p.contains('x') && p.contains('+'));

        if let Some(g) = geometry {
            let (width, height, x, y) = parse_geometry(g)?;

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
        return Err(MonitorError::DetectionFailed);
    }

    let total_width = monitors
        .iter()
        .map(|m| m.x as u32 + m.width)
        .max()
        .ok_or(MonitorError::DetectionFailed)?;

    let total_height = monitors
        .iter()
        .map(|m| m.y as u32 + m.height)
        .max()
        .ok_or(MonitorError::DetectionFailed)?;

    Ok(MonitorLayout {
        monitors,
        total_width,
        total_height,
    })
}

fn parse_geometry(
    g: &str,
) -> Result<(u32, u32, i32, i32), MonitorError> {
    let (res, pos) = g
        .split_once('+')
        .ok_or(MonitorError::DetectionFailed)?;
    let (w, h) = res
        .split_once('x')
        .ok_or(MonitorError::DetectionFailed)?;
    let (x, y) = pos
        .split_once('+')
        .ok_or(MonitorError::DetectionFailed)?;

    Ok((
        w.parse()
            .map_err(|_| MonitorError::DetectionFailed)?,
        h.parse()
            .map_err(|_| MonitorError::DetectionFailed)?,
        x.parse()
            .map_err(|_| MonitorError::DetectionFailed)?,
        y.parse()
            .map_err(|_| MonitorError::DetectionFailed)?,
    ))
}
