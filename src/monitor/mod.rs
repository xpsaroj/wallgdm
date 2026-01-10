mod detect;

use crate::error::MonitorError;

#[derive(Debug)]
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

pub fn get_monitor_layout() -> Result<MonitorLayout, MonitorError> {
    detect::detect_monitors()
}
