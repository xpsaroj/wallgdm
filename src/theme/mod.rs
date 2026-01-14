mod compile;
mod css;
mod extract;
mod xml;

use crate::{config::SetDirs, error::ThemeError};
use std::{fs, path::Path, process::Command};

pub fn extract_and_modify_theme(
    working_dirs: &SetDirs,
    theme_image_path: &Path,
) -> Result<(), ThemeError> {
    extract::extract_gnome_shell_theme(&working_dirs.theme_workdir)?;

    let theme_dir = working_dirs.theme_workdir.join("theme");
    let dest_image_path = theme_dir.join("background.png");

    fs::copy(&theme_image_path, &dest_image_path)
        .map_err(|_| ThemeError::Filesystem)?;

    css::update_theme_css(&working_dirs.theme_workdir)?;

    let xml_path = xml::generate_gresource_xml(&working_dirs.theme_workdir)?;

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
