//! Error types used throughout wallgdm.
//!
//! Errors are layered:
//! - `WallGdmError` represents top-level command failures
//! - Command-specific errors (`SetError`, `RevertError`, etc.) describe user intent failures
//! - Module errors (`MonitorError`, `SystemError`) describe low-level system failures

use std::path::PathBuf;
use thiserror::Error;

/// Result type used across the project.
pub type Result<T> = std::result::Result<T, WallGdmError>;

/// Top-level error type returned by CLI commands.
///
/// This wraps command-specific errors and is intended to be the
/// primary error type surfaced to the user.
#[derive(Error, Debug)]
pub enum WallGdmError {
    /// Setting the wallpaper failed.
    #[error("set command failed: {0}")]
    Set(#[from] SetError),

    /// Reverting to the previous wallpaper failed.
    #[error("revert command failed")]
    Revert(#[from] RevertError),

    /// Fetching the wallpaper status failed.
    #[error("status command failed")]
    Status(#[from] StatusError),

    /// Listing available wallpapers failed.
    #[error("list command failed")]
    List(#[from] ListError),

    /// Installing a new GDM theme failed.
    #[error("install command failed")]
    Install,
}

/// Errors that can occur while setting a new GDM wallpaper.
#[derive(Error, Debug)]
pub enum SetError {
    /// The provided image path could not be processed.
    #[error("image processing error:\n {0}")]
    Image(#[from] ImageError),

    /// Failed to detect the system monitor layout.
    #[error("monitor detection error:\n {0}")]
    Monitor(#[from] MonitorError),

    /// Theme extraction or modification failed.
    #[error("theme extraction/modification error:\n {0}")]
    Theme(#[from] ThemeError),

    /// Configuration or directory operation failed.
    #[error("configuration error:\n {0}")]
    Config(#[from] ConfigError),
}

/// Errors that can occur while reverting to the previous GDM wallpaper.
#[derive(Error, Debug)]
pub enum RevertError {
    /// No previously applied wallpaper was found.
    #[error("no previous wallpaper to revert to")]
    NoPreviousWallpaper,

    /// Failed to detect the system monitor layout.
    #[error("monitor error")]
    Monitor(#[from] MonitorError),

    /// A system-level operation failed.
    #[error("system error")]
    System(#[from] SystemError),
}

/// Errors that can occur while fetching the GDM wallpaper status.
#[derive(Error, Debug)]
pub enum StatusError {
    /// Failed to detect the system monitor layout.
    #[error("monitor error")]
    Monitor(#[from] MonitorError),

    /// A system-level operation failed.
    #[error("system error")]
    System(#[from] SystemError),
}

/// Errors that can occur while listing available GDM wallpapers.
#[derive(Error, Debug)]
pub enum ListError {
    /// Wallpaper storage directory was not found.
    #[error("wallpaper storage directory not found")]
    DirectoryNotFound,

    /// A system-level operation failed.
    #[error("system error")]
    System(#[from] SystemError),

    /// Failed to read wallpaper metadata or files.
    #[error("failed to read wallpaper list: {0}")]
    ReadError(String),
}

#[derive(Error, Debug)]
pub enum MonitorError {
    /// Failed to execute `xrandr` (not found or not executable)
    #[error("failed to execute xrandr")]
    XrandrUnavailable(#[source] std::io::Error),

    /// `xrandr` ran but returned a non-zero exit status
    #[error("xrandr command failed")]
    XrandrFailed,

    /// Output from xrandr could not be parsed
    #[error("failed to parse monitor layout")]
    ParseFailed,

    /// No connected monitors were detected
    #[error("no connected monitors detected")]
    NoMonitorsFound,

    /// Monitor layout is internally inconsistent
    #[error("invalid monitor layout")]
    InvalidLayout,
}

/// Errors related to system-level operations.
#[derive(Error, Debug)]
pub enum SystemError {
    /// The operation requires elevated permissions.
    #[error("permission denied")]
    PermissionDenied,

    /// A required system command failed to execute.
    #[error("system command failed: {0}")]
    CommandFailed(String),
}

/// Errors related to image processing.
#[derive(Error, Debug)]
pub enum ImageError {
    /// The provided image path is invalid or unreadable.
    #[error("image path does not exist or is unreadable: {path}")]
    InvalidImagePath { path: PathBuf },

    /// The image format is not supported.
    #[error("unsupported image format")]
    UnsupportedImageFormat,

    /// Failed to load the image from the specified path.
    #[error("failed to load image '{path}': {source}")]
    ImageLoadFailed {
        path: PathBuf,
        #[source]
        source: image::ImageError,
    },

    /// Failed to save the processed image.
    #[error("failed to save image to '{path}': {source}")]
    ImageSaveFailed {
        path: PathBuf,
        #[source]
        source: image::ImageError,
    },
}

/// Errors related to theme extraction and modification.
#[derive(Error, Debug)]
pub enum ThemeError {
    /// Required external command failed or is unavailable
    #[error("required command failed: {0}")]
    CommandFailed(&'static str),

    /// File system operation failed
    #[error("file system operation failed")]
    Filesystem,

    /// Failed to extract the GNOME Shell theme resources
    #[error("failed to extract gnome shell theme")]
    ThemeExtractionFailed,

    /// Failed to modify theme CSS
    #[error("failed to modify theme CSS")]
    CssModificationFailed,

    /// Failed to generate gresource XML
    #[error("failed to generate gresource XML")]
    GresourceXmlGenerationFailed,

    /// Failed to compile theme resources
    #[error("failed to compile theme resources")]
    ThemeCompilationFailed,

    /// Failed to install theme
    #[error("theme installation failed")]
    ThemeInstallationFailed,
}

/// Error type for config / directory operations
#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("failed to create directory: {0}")]
    CreateDirFailed(PathBuf),
}

/// Errors related to theme installation.
#[derive(Error, Debug)]
pub enum ThemeInstallError {
    /// Failed to install the theme.
    #[error("theme installation failed")]
    InstallationFailed,
}
