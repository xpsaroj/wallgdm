//! Theme extraction utilities for WallGDM.
//!
//! Provides functions to extract GNOME Shell theme resources from the system
//! gresource. These resources are later modified for wallpaper changes.
//!
//! **Note:** Requires `gresource` to be installed on the system; extraction
//! will fail if it is not available.

use std::{fs, path::Path, process::Command};

use crate::config::GNOME_SHELL_THEME_RESOURCE;
use crate::error::ThemeError;

/// Extract the GNOME Shell theme resources into a working directory.
///
/// # Parameters
/// - `workdir`: path to the temporary directory where theme resources will be extracted.
///
/// # Returns
/// `Ok(())` if extraction succeeds, otherwise a `ThemeError`.
pub fn extract_gnome_shell_theme(workdir: &Path) -> Result<(), ThemeError> {
    log::debug!("Extracting GNOME Shell theme to {:?}", workdir);
    let resource_list = list_resources()?;

    for resource in resource_list {
        extract_single_resource(&resource, workdir)?;
    }

    Ok(())
}

/// List all resources in the GNOME Shell theme gresource.
fn list_resources() -> Result<Vec<String>, ThemeError> {
    log::debug!("Listing resources in {:?}", GNOME_SHELL_THEME_RESOURCE);

    let output = Command::new("gresource")
        .arg("list")
        .arg(GNOME_SHELL_THEME_RESOURCE)
        .output()
        .map_err(|_| ThemeError::ThemeExtractionFailed)?;

    if !output.status.success() {
        return Err(ThemeError::ThemeExtractionFailed);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let resources: Vec<String> =
        stdout.lines().map(|s| s.to_string()).collect();

    log::debug!("Found {} resources", resources.len());

    Ok(resources)
}

/// Extract a single resource file to the output path.
fn extract_single_resource(
    resource: &str,
    output_path: &Path,
) -> Result<(), ThemeError> {
    let relative_path = resource
        .strip_prefix("/org/gnome/shell/")
        .ok_or(ThemeError::ThemeExtractionFailed)?;

    let final_output_path = output_path.join(relative_path);

    if let Some(parent) = final_output_path.parent() {
        fs::create_dir_all(parent).map_err(|_| ThemeError::Filesystem)?;
    }

    log::debug!("Extracting resource {:?} to {:?}", resource, final_output_path);

    let output = Command::new("gresource")
        .arg("extract")
        .arg(GNOME_SHELL_THEME_RESOURCE)
        .arg(resource)
        .output()
        .map_err(|_| ThemeError::CommandFailed("gresource"))?;

    if !output.status.success() {
        return Err(ThemeError::CommandFailed("gresource"));
    }

    fs::write(&final_output_path, &output.stdout)
        .map_err(|_| ThemeError::Filesystem)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_extract_gnome_shell_theme() {
        let tmp_dir = tempdir().expect("failed to create temp dir");

        let result = extract_gnome_shell_theme(tmp_dir.path());
        assert!(result.is_ok());

        // Check if some expected files are extracted
        let expected_files = vec![
            "theme/gnome-shell-dark.css",
            "theme/gnome-shell-light.css",
            "theme/gnome-shell-start.svg",
            "theme/gnome-shell-high-contrast.css",
        ];

        for file in expected_files {
            let file_path = tmp_dir.path().join(file);
            assert!(
                file_path.exists(),
                "expected extracted file {:?} to exist",
                file_path
            );
        }
    }
}
