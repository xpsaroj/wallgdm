//! State management for the application, including saving and loading configuration.
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

use crate::{error::ConfigError, monitor::MonitorLayout};

/// Represents the current state of the application, including wallpaper configuration
/// and monitor layout.
#[derive(Debug, Serialize, Deserialize)]
pub struct AppState {
    pub image_path: PathBuf,
    pub blur: u32,
    pub scale: f32,
    pub monitor_layout: MonitorLayout,
}

impl AppState {
    pub fn new(
        image_path: PathBuf,
        blur: u32,
        scale: f32,
        monitor_layout: MonitorLayout,
    ) -> Self {
        Self {
            image_path,
            blur,
            scale,
            monitor_layout,
        }
    }

    /// Save the current state to a JSON file at the specified path.
    pub fn save(&self, path: &PathBuf) -> Result<(), ConfigError> {
        let json = serde_json::to_string_pretty(self)
            .map_err(|_| ConfigError::SerializeFailed)?;
        fs::write(path, json)
            .map_err(|_| ConfigError::WriteFailed(path.into()))?;
        Ok(())
    }

    /// Load the current state from a JSON file at the specified path.
    pub fn load(path: &PathBuf) -> Result<Self, ConfigError> {
        let json = fs::read_to_string(path)
            .map_err(|_| ConfigError::DeserializeFailed)?;

        let state: Self = serde_json::from_str(&json)
            .map_err(|_| ConfigError::DeserializeFailed)?;

        Ok(state)
    }
}
