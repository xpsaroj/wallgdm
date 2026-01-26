//! Directory management for WallGDM.
//!
//! Provides helpers for creating and managing working directories used by commands
//! such as `set` and `list`.

use std::{
    fs,
    io::ErrorKind,
    path::{Path, PathBuf},
};

use crate::error::ConfigError;
use dirs::data_dir;

/// Working directories used by the `set` command.
#[derive(Debug)]
pub struct SetDirs {
    /// Temporary directory for theme extraction/modification
    pub theme_workdir: PathBuf,

    /// Directory where composed wallpaper images are saved
    pub wallpaper_output_dir: PathBuf,
}

impl SetDirs {
    /// Create and return a new `SetDirs` struct, ensuring directories exist.
    pub fn new() -> Result<Self, ConfigError> {
        Ok(Self {
            theme_workdir: theme_workdir()?,
            wallpaper_output_dir: wallpaper_output_dir()?,
        })
    }
}

/// Directories used by the `list` command.
#[derive(Debug)]
pub struct ListDirs {
    /// Base data directory for wallgdm
    pub data_dir: PathBuf,
}

impl ListDirs {
    /// Create and return a new `ListDirs` struct.
    pub fn new() -> Result<Self, ConfigError> {
        Ok(Self {
            data_dir: wallgdm_data_dir()?,
        })
    }
}

/// Directories used by the `status` command.
#[derive(Debug)]
pub struct StatusDirs {
    /// Base data directory for wallgdm
    pub data_dir: PathBuf,
}

impl StatusDirs {
    /// Create and return a new `StatusDirs` struct.
    pub fn new() -> Result<Self, ConfigError> {
        Ok(Self {
            data_dir: wallgdm_data_dir()?,
        })
    }
}

/// Temporary working directory for theme extraction/modification
pub fn theme_workdir() -> Result<PathBuf, ConfigError> {
    let path = std::env::temp_dir().join("wallgdm_theme_workdir");

    // Clean up old directory if it exists
    match fs::remove_dir_all("/tmp/wallgdm_theme_workdir") {
        Ok(_) => {}
        Err(e) if e.kind() == ErrorKind::NotFound => {}
        Err(_) => return Err(ConfigError::CreateDirFailed(path.into())),
    }

    ensure_exists(&path)?;
    Ok(path)
}

/// Directory to save composed wallpaper images
pub fn wallpaper_output_dir() -> Result<PathBuf, ConfigError> {
    let path = wallgdm_data_dir()?.join("images");
    ensure_exists(&path)?;
    Ok(path)
}

/// Base data directory for wallgdm
fn wallgdm_data_dir() -> Result<PathBuf, ConfigError> {
    let path = data_dir()
        .map(|dir| dir.join("wallgdm"))
        .unwrap_or_else(|| PathBuf::from("/usr/local/share/wallgdm"));

    ensure_exists(&path)?;
    Ok(path)
}

/// Ensures a directory exists, creating it if necessary
fn ensure_exists(path: &Path) -> Result<(), ConfigError> {
    fs::create_dir_all(path)
        .map_err(|_| ConfigError::CreateDirFailed(path.into()))
}
