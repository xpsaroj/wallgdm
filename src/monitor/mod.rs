mod detect;
mod transform;

use crate::error::MonitorError;

#[derive(Debug, Clone)]
pub struct Monitor {
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub x: i32,
    pub y: i32,
    pub is_primary: bool,
}

#[derive(Debug)]
pub struct MonitorLayout {
    pub monitors: Vec<Monitor>,
    pub total_width: u32,
    pub total_height: u32,
}

pub fn get_monitor_layout(scale: f32) -> Result<MonitorLayout, MonitorError> {
    let monitor_layout = detect::detect_monitors()?;
    println!("Raw monitor layout: {:#?}", monitor_layout);

    let monitor_layout = transform::apply_scale(&monitor_layout, scale)?;
    println!("Scaled monitor layout: {:#?}", monitor_layout);

    Ok(monitor_layout)
}
