mod extract;
mod xml;

use crate::{
    config::{theme_workdir, wallpaper_file_path},
    error::ThemeError,
};
use std::fs;

pub fn extract_and_modify_theme() -> Result<(), ThemeError>
{
    let workdir = theme_workdir()
        .map_err(|_| ThemeError::ThemeExtractionFailed)?;

    extract::extract_gnome_shell_theme(&workdir)?;

    // copy image to the theme directory
    let theme_image_path = wallpaper_file_path()
        .map_err(|_| ThemeError::ThemeExtractionFailed)?;
    let theme_dir = workdir.join("theme");
    let dest_image_path = theme_dir.join("background.png");

    fs::copy(&theme_image_path, &dest_image_path)
        .map_err(|_| ThemeError::ThemeExtractionFailed)?;

    xml::generate_gresource_xml(&workdir)?;

    Ok(())
}
