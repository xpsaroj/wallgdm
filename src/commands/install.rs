use crate::error::{Result, WallGdmError};
use std::{fs, path::Path};

const SYSTEM_THEME_DIR: &str = "/usr/share/gnome-shell";
const SYSTEM_THEME_FILE: &str = "gnome-shell-theme.gresource";
const BACKUP_THEME_FILE: &str = "gnome-shell-theme-original.gresource";

pub fn run(compiled_gresource: &Path) -> Result<()> {
    ensure_root()?;

    let system_dir = Path::new(SYSTEM_THEME_DIR);
    let system_file = system_dir.join(SYSTEM_THEME_FILE);
    let backup_file = system_dir.join(BACKUP_THEME_FILE);

    if !compiled_gresource.exists() {
        return Err(WallGdmError::Install);
    }

    // Backup once
    if !backup_file.exists() {
        fs::copy(&system_file, &backup_file)
            .map_err(|_| WallGdmError::Install)?;
    }

    // Overwrite system theme
    fs::copy(compiled_gresource, &system_file)
        .map_err(|_| WallGdmError::Install)?;

    Ok(())
}

fn ensure_root() -> Result<()> {
    if unsafe { libc::geteuid() } != 0 {
        return Err(WallGdmError::Install);
    }
    Ok(())
}
