use std::{
    fs,
    io::ErrorKind,
    path::{Path, PathBuf},
};

use crate::error::ConfigError;
use dirs::data_dir;

/// Directory where temporary theme extraction / modification happens
pub fn theme_workdir() -> Result<PathBuf, ConfigError> {
    let path = std::env::temp_dir().join("wallgdm_theme_workdir");
    match fs::remove_dir_all("/tmp/wallgdm_theme_workdir") {
        Ok(_) => {}
        Err(e) if e.kind() == ErrorKind::NotFound => {}
        Err(_) => return Err(ConfigError::CreateDirFailed(path.to_path_buf())),
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

/// Full path for the composed wallpaper image file
pub fn wallpaper_file_path() -> Result<PathBuf, ConfigError> {
    Ok(wallpaper_output_dir()?.join("composed_wallpaper.png"))
}

/// Gnome shell theme gresource location
pub const GNOME_SHELL_THEME_RESOURCE: &str =
    "/usr/share/gnome-shell/gnome-shell-theme.gresource";

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
        .map_err(|_| ConfigError::CreateDirFailed(path.to_path_buf()))
}
