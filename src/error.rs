//! Error types used throughout wallgdm.
//!
//! Errors are layered:
//! - `WallGdmError` represents top-level command failures
//! - Command-specific errors (`SetError`, `RevertError`, etc.) describe user intent failures
//! - Module errors (`MonitorError`, `SystemError`) describe low-level system failures

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
    #[error("set command failed")]
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
}

/// Errors that can occur while setting a new GDM wallpaper.
#[derive(Error, Debug)]
pub enum SetError {
    /// The provided image path is invalid or is not readable.
    #[error("invalid image path")]
    InvalidImagePath,

    /// Failed to detect the system monitor layout.
    #[error("monitor error")]
    Monitor(#[from] MonitorError),

    /// A system-level operation failed (permissions, commands, etc.).
    #[error("system error")]
    System(#[from] SystemError),

    /// Failed to extract or modify the GNOME Shell theme.
    #[error("failed to extract gnome shell theme")]
    ThemeExtractionFailed,

    /// The provided image format is not supported.
    #[error("unsupported image format")]
    UnsupportedImageFormat,
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

/// Errors related to monitor detection.
#[derive(Error, Debug)]
pub enum MonitorError {
    /// The system monitor layout could not be detected.
    #[error("failed to detect monitors")]
    DetectionFailed,
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
