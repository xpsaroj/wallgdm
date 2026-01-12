use crate::error::MonitorError;
use crate::monitor::{self, Monitor, MonitorLayout};

pub fn apply_scale(
    layout: &MonitorLayout,
    scale: f32,
) -> Result<MonitorLayout, MonitorError> {
    let monitors: Vec<Monitor> = layout
        .monitors
        .iter()
        .map(|m| Monitor {
            name: m.name.clone(),
            width: ((m.width as f32) * scale).round() as u32,
            height: ((m.height as f32) * scale).round() as u32,
            x: m.x,
            y: m.y,
            is_primary: m.is_primary,
        })
        .collect();

    // let monitors = vec![
    //     Monitor {
    //         name: layout.monitors[0].name.clone(),
    //         width: ((layout.monitors[0].width as f32) * scale).round() as u32,
    //         height: layout.monitors[0].height,
    //         x: (layout.monitors[0].x as f32 / scale).round() as i32,
    //         y: (layout.monitors[0].y as f32 / scale).round() as i32,
    //         is_primary: layout.monitors[0].is_primary,
    //     },
    //     Monitor {
    //         name: layout.monitors[1].name.clone(),
    //         width: layout.monitors[1].width,
    //         height: layout.monitors[1].height,
    //         x: layout.monitors[1].x,
    //         y: layout.monitors[1].y,
    //         is_primary: layout.monitors[1].is_primary,
    //     },
    // ];

    let total_width = monitors
        .iter()
        .map(|m| m.x as u32 + m.width)
        .max()
        .ok_or(MonitorError::InvalidData)?;

    let total_height = monitors
        .iter()
        .map(|m| m.y as u32 + m.height)
        .max()
        .ok_or(MonitorError::InvalidData)?;

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
    fn test_apply_scale_125_percent() {
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

        let scaled = apply_scale(&layout, 1.25)
            .expect("failed to apply scale to the monitor layout.");

        assert_eq!(scaled.monitors[0].width, 1536);
        assert_eq!(scaled.monitors[1].x, 1536);
        assert_eq!(scaled.total_width, 3072);
    }
}
