//! Provides directory management for wallgdm.
//! Includes low-level helpers and per-command working directories structs.

use std::{
    fs,
    io::ErrorKind,
    path::{Path, PathBuf},
};

use crate::error::ConfigError;
use dirs::data_dir;

#[derive(Debug)]
pub struct SetDirs {
    pub theme_workdir: PathBuf,
    pub wallpaper_output_dir: PathBuf,
}

impl SetDirs {
    pub fn new() -> Result<Self, ConfigError> {
        Ok(Self {
            theme_workdir: theme_workdir()?,
            wallpaper_output_dir: wallpaper_output_dir()?,
        })
    }
}

#[derive(Debug)]
pub struct ListDirs {
    pub data_dir: PathBuf,
}

impl ListDirs {
    pub fn new() -> Result<Self, ConfigError> {
        Ok(Self {
            data_dir: wallgdm_data_dir()?,
        })
    }
}

/// Directory where temporary theme extraction / modification happens
pub fn theme_workdir() -> Result<PathBuf, ConfigError> {
    let path = std::env::temp_dir().join("wallgdm_theme_workdir");
    match fs::remove_dir_all("/tmp/wallgdm_theme_workdir") {
        Ok(_) => {}
        Err(e) if e.kind() == ErrorKind::NotFound => {}
        Err(_) => return Err(ConfigError::CreateDirFailed(path.into())),
    }
    let path = std::env::temp_dir().join("wallgdm_theme_workdir");
    ensure_exists(&path)?;
    Ok(path)
}

/// Directory where composed wallpaper images are saved
pub fn wallpaper_output_dir() -> Result<PathBuf, ConfigError> {
    let path = wallgdm_data_dir()?.join("images");
    ensure_exists(&path)?;
    Ok(path)
}

/// Base data directory for wallgdm
fn wallgdm_data_dir() -> Result<PathBuf, ConfigError> {
    let path = if let Some(dir) = data_dir() {
        dir.join("wallgdm")
    } else {
        // fallback if XDG_DATA_HOME / home dir is unavailable
        PathBuf::from("/usr/local/share/wallgdm")
    };

    ensure_exists(&path)?;
    Ok(path)
}

/// Ensures a directory exists, creating it if necessary
fn ensure_exists(path: &Path) -> Result<(), ConfigError> {
    fs::create_dir_all(path)
        .map_err(|_| ConfigError::CreateDirFailed(path.into()))
}
