//! Theme modification utilities for WallGDM.
//!
//! Handles extracting the GNOME Shell theme, updating CSS, generating
//! gresource XML, compiling the theme, and installing it to the login screen.
//!
//! **Note:** Installing the theme involves copying files to system directories,
//! which requires elevated permissions. The `extract_and_modify_theme` function
//! will invoke `sudo` to perform this step.

mod compile;
mod css;
mod extract;
mod xml;

use crate::{config::SetDirs, error::ThemeError};
use std::{fs, path::Path, process::Command};

/// Extract the GNOME Shell theme, update it with a new wallpaper, and install it.
///
/// # Parameters
/// - `working_dirs`: directories used for temporary files and theme extraction.
/// - `theme_image_path`: path to the wallpaper image to set in the theme.
///
/// # Returns
/// `Ok(())` if the theme is successfully modified and installed, otherwise a `ThemeError`.
///
/// # Note
/// Installing the theme requires copying files to system directories, which
/// requires `sudo` privileges. The function automatically invokes `sudo`
/// to perform the installation step.

pub fn extract_and_modify_theme(
    working_dirs: &SetDirs,
    theme_image_path: &Path,
) -> Result<(), ThemeError> {
    log::info!(
        "Extracting and modifying theme with image: {:?}",
        theme_image_path
    );
    // Extract GNOME Shell theme
    extract::extract_gnome_shell_theme(&working_dirs.theme_workdir)?;

    // Copy the new wallpaper image into the theme directory
    let theme_dir = working_dirs.theme_workdir.join("theme");
    let dest_image_path = theme_dir.join("background.png");

    log::info!(
        "Copying wallpaper from {:?} to {:?}",
        theme_image_path,
        dest_image_path
    );
    fs::copy(&theme_image_path, &dest_image_path)
        .map_err(|_| ThemeError::Filesystem)?;

    log::info!("Updating theme CSS");
    // Update the theme CSS to use the new wallpaper
    css::update_theme_css(&working_dirs.theme_workdir)?;

    log::info!("Generating gresource XML");
    // Generate gresource XML for the modified theme
    let xml_path = xml::generate_gresource_xml(&working_dirs.theme_workdir)?;

    log::info!("Compiling GNOME Shell theme");
    // Compile the theme resources
    let compiled_resource = compile::compile_theme(&xml_path)?;

    log::info!("Installing compiled theme resource");
    // Install the compiled theme resource using sudo
    Command::new("sudo")
        .arg(
            std::env::current_exe()
                .map_err(|_| ThemeError::ThemeInstallationFailed)?,
        )
        .arg("install")
        .arg(compiled_resource)
        .status()
        .map_err(|_| ThemeError::ThemeInstallationFailed)?;

    log::info!("Theme successfully modified and installed");

    println!(
        "GDM wallpaper applied successfully.\nPlease restart GDM (log out and log back in) or reboot your system to see the changes."
    );

    Ok(())
}
