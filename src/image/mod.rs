mod blur;
mod compose;
mod resize;

use crate::{
    config::wallpaper_file_path, error::ImageError,
    image::compose::compose_wallpaper,
    monitor::MonitorLayout,
};

pub fn compose_and_save_wallpaper(
    img_path: &str,
    layout: &MonitorLayout,
    blur_amount: f32,
) -> Result<(), ImageError> {
    let wallpaper =
        compose_wallpaper(img_path, &layout, blur_amount)?;

    let image_file_path = wallpaper_file_path()
        .map_err(|_| ImageError::ImageSaveFailed)?;

    wallpaper
        .save(image_file_path)
        .map_err(|_| ImageError::ImageSaveFailed)
}
