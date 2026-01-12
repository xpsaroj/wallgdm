mod compile;
mod css;
mod extract;
mod xml;

use crate::{
    config::{theme_workdir, wallpaper_file_path},
    error::ThemeError,
    monitor::MonitorLayout,
};
use std::{fs, process::Command};

pub fn extract_and_modify_theme(
    monitor_layout: &MonitorLayout,
) -> Result<(), ThemeError> {
    let workdir =
        theme_workdir().map_err(|_| ThemeError::ThemeExtractionFailed)?;

    extract::extract_gnome_shell_theme(&workdir)?;

    // copy image to the theme directory
    let theme_image_path =
        wallpaper_file_path().map_err(|_| ThemeError::ThemeExtractionFailed)?;
    let theme_dir = workdir.join("theme");
    let dest_image_path = theme_dir.join("background.png");

    fs::copy(&theme_image_path, &dest_image_path)
        .map_err(|_| ThemeError::ThemeExtractionFailed)?;

    css::update_theme_css(&workdir, monitor_layout)?;

    let xml_path = xml::generate_gresource_xml(&workdir)?;

    let compiled_resource = compile::compile_theme(&xml_path)?;

    Command::new("sudo")
        .arg(
            std::env::current_exe()
                .map_err(|_| ThemeError::ThemeInstallationFailed)?,
        )
        .arg("install")
        .arg(compiled_resource)
        .status()
        .map_err(|_| ThemeError::ThemeInstallationFailed)?;

    Ok(())
}
