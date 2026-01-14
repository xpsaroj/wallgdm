//! Image processing for WallGDM.
//!
//! Provides functions to compose wallpapers for multiple monitors, apply blur effects,
//! and save the resulting image to the appropriate directory.

mod blur;
mod compose;
mod resize;

use crate::config::SetDirs;
use crate::{
    error::ImageError, image::compose::compose_wallpaper,
    monitor::MonitorLayout,
};
use std::path::PathBuf;

/// Compose a wallpaper for the given monitor layout and save it.
///
/// # Parameters
/// - `working_dirs`: Directories for temporary files and output.
/// - `img_path`: Path to the source image.
/// - `layout`: Monitor layout (positions and dimensions of all monitors).
/// - `blur_amount`: Blur radius to apply to the wallpaper.
///
/// # Returns
/// Path to the saved wallpaper image, or an `ImageError` on failure.
pub fn compose_and_save_wallpaper(
    working_dirs: &SetDirs,
    img_path: &str,
    layout: &MonitorLayout,
    blur_amount: f32,
) -> Result<PathBuf, ImageError> {
    log::info!(
        "Composing wallpaper from '{}' with blur: {}",
        img_path,
        blur_amount
    );

    // Compose the wallpaper across monitors
    let wallpaper = compose_wallpaper(img_path, &layout, blur_amount)?;

    let image_file_path = &working_dirs
        .wallpaper_output_dir
        .join("composed_wallpaper.png");

    // Save the composed wallpaper
    wallpaper.save(image_file_path).map_err(|e| {
        ImageError::ImageSaveFailed {
            path: image_file_path.into(),
            source: e,
        }
    })?;

    log::info!(
        "Wallpaper composed and saved to '{}'",
        image_file_path.display()
    );

    Ok(image_file_path.to_path_buf())
}
