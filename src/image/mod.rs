mod blur;
mod compose;
mod resize;

use crate::{
    error::ImageError, image::compose::compose_wallpaper,
    monitor::MonitorLayout,
};
use image::DynamicImage;

pub fn compose_and_save_wallpaper(
    img_path: &str,
    layout: &MonitorLayout,
    blur_amount: f32,
) -> Result<String, ImageError> {
    let wallpaper =
        compose_wallpaper(img_path, &layout, blur_amount)?;

    let output_path = "images/composed_wallpaper.png";
    save_wallpaper(&wallpaper, output_path)?;

    Ok(output_path.to_string())
}

fn save_wallpaper(
    wallpaper: &DynamicImage,
    output_path: &str,
) -> Result<(), ImageError> {
    wallpaper
        .save(output_path)
        .map_err(|_| ImageError::ImageSaveFailed)
}
