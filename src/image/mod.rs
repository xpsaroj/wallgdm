mod blur;
mod compose;
mod resize;

use crate::config::SetDirs;
use crate::{
    error::ImageError, image::compose::compose_wallpaper,
    monitor::MonitorLayout,
};
use std::path::PathBuf;

pub fn compose_and_save_wallpaper(
    working_dirs: &SetDirs,
    img_path: &str,
    layout: &MonitorLayout,
    blur_amount: f32,
) -> Result<PathBuf, ImageError> {
    let wallpaper = compose_wallpaper(img_path, &layout, blur_amount)?;

    let image_file_path = &working_dirs
        .wallpaper_output_dir
        .join("composed_wallpaper.png");

    wallpaper.save(image_file_path).map_err(|e| {
        ImageError::ImageSaveFailed {
            path: image_file_path.into(),
            source: e,
        }
    })?;

    Ok(image_file_path.to_path_buf())
}
