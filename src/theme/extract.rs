use std::{fs, path::Path, process::Command};

use crate::config::GNOME_SHELL_THEME_RESOURCE;
use crate::error::ThemeError;

pub fn extract_gnome_shell_theme(
    workdir: &Path,
) -> Result<(), ThemeError> {
    let resource_list = list_resources()?;

    for resource in resource_list {
        extract_single_resource(&resource, workdir)?;
    }

    Ok(())
}

fn list_resources() -> Result<Vec<String>, ThemeError> {
    let output = Command::new("gresource")
        .arg("list")
        .arg(GNOME_SHELL_THEME_RESOURCE)
        .output()
        .map_err(|_| ThemeError::ThemeExtractionFailed)?;

    if !output.status.success() {
        return Err(ThemeError::ThemeExtractionFailed);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let resources =
        stdout.lines().map(|s| s.to_string()).collect();

    Ok(resources)
}

fn extract_single_resource(
    resource: &str,
    output_path: &Path,
) -> Result<(), ThemeError> {
    let relative_path = resource
        .strip_prefix("/org/gnome/shell/")
        .ok_or(ThemeError::ThemeExtractionFailed)?;

    let final_output_path = output_path.join(relative_path);

    if let Some(parent) = final_output_path.parent() {
        fs::create_dir_all(parent).map_err(|_| {
            ThemeError::ThemeExtractionFailed
        })?;
    }

    let output = Command::new("gresource")
        .arg("extract")
        .arg(GNOME_SHELL_THEME_RESOURCE)
        .arg(resource)
        .output()
        .map_err(|_| ThemeError::ThemeExtractionFailed)?;

    if !output.status.success() {
        return Err(ThemeError::ThemeExtractionFailed);
    }

    fs::write(&final_output_path, &output.stdout)
        .map_err(|_| ThemeError::ThemeExtractionFailed)?;

    Ok(())
}
