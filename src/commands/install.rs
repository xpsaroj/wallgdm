//! Install a compiled GNOME Shell theme resource for the GDM login screen.
//!
//! ⚠️ This command performs system-wide file replacement under
//! `/usr/share/gnome-shell` and **must be run as root**.
//!
//! The original GNOME Shell theme resource is backed up once as
//! `gnome-shell-theme-original.gresource` before being overwritten.

use crate::config::{
    GNOME_BACKUP_THEME_FILE, GNOME_SYSTEM_THEME_DIR, GNOME_SYSTEM_THEME_FILE,
};
use crate::error::{Result, WallGdmError};
use std::{fs, path::Path};

/// Install a compiled `.gresource` file as the active GDM theme.
///
/// This function:
/// 1. Verifies the process is running as root.
/// 2. Backs up the existing system theme if no backup exists.
/// 3. Replaces the system GNOME Shell theme with the provided resource.
///
/// # Errors
/// Returns `WallGdmError::Install` if:
/// - The process is not running as root
/// - The provided resource does not exist
/// - Any filesystem operation fails
pub fn run(compiled_gresource: &Path) -> Result<()> {
    ensure_root()?;

    let system_dir = Path::new(GNOME_SYSTEM_THEME_DIR);
    let system_file = system_dir.join(GNOME_SYSTEM_THEME_FILE);
    let backup_file = system_dir.join(GNOME_BACKUP_THEME_FILE);

    if !compiled_gresource.exists() {
        return Err(WallGdmError::Install);
    }

    // Backup once
    if !backup_file.exists() {
        fs::copy(&system_file, &backup_file)
            .map_err(|_| WallGdmError::Install)?;
        log::debug!("Gnome Shell theme backup created at {:?}", backup_file);
    }

    // Overwrite system theme
    fs::copy(compiled_gresource, &system_file)
        .map_err(|_| WallGdmError::Install)?;

    log::info!(
        "Installed GDM theme from {:?} to {:?}",
        compiled_gresource,
        system_file
    );

    Ok(())
}

fn ensure_root() -> Result<()> {
    if unsafe { libc::geteuid() } != 0 {
        return Err(WallGdmError::Install);
    }
    Ok(())
}
