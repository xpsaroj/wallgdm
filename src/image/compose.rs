//! Image composition for WallGDM.
//!
//! This module handles creating a wallpaper for the GDM login screen by:
//! - Loading source images.
//! - Resizing images to fit each monitor.
//! - Applying blur effects if specified.
//! - Composing the final wallpaper from multiple monitor images.

use crate::{
    error::ImageError,
    image::{blur, resize},
    monitor::MonitorLayout,
};
use image::{
    imageops, {DynamicImage, RgbaImage},
};
use std::path::Path;

/// Compose a wallpaper for the given monitor layout.
///
/// Loads the source image, resizes it for each monitor, applies blur if specified,
/// and combines the results into a single wallpaper image.
///
/// # Parameters
/// - `img_path`: Path to the source image.
/// - `layout`: Monitor layout specifying dimensions and positions for each monitor.
/// - `blur_amount`: Blur radius to apply to each monitor’s image (0 for no blur).
///
/// # Returns
/// A `DynamicImage` representing the composed wallpaper, or an `ImageError` if the source
/// image is invalid, cannot be loaded, or processed.
pub fn compose_wallpaper(
    img_path: &str,
    layout: &MonitorLayout,
    blur_amount: f32,
) -> Result<DynamicImage, ImageError> {
    let path = Path::new(img_path);
    if !path.exists() {
        return Err(ImageError::InvalidImagePath {
            path: path.to_path_buf(),
        });
    }

    // Load the source image
    let img =
        image::open(img_path).map_err(|e| ImageError::ImageLoadFailed {
            path: img_path.into(),
            source: e,
        })?;

    // Create an empty canvas the size of the total monitor layout
    let mut canvas = RgbaImage::from_pixel(
        layout.total_width,
        layout.total_height,
        image::Rgba([0, 0, 0, 255]),
    );

    for monitor in &layout.monitors {
        log::debug!(
            "Processing monitor '{}' at ({}, {}) with size {}x{}",
            monitor.name,
            monitor.x,
            monitor.y,
            monitor.width,
            monitor.height
        );

        // Resize image to monitor size
        let resized_img =
            resize::resize_to_monitor(&img, monitor.width, monitor.height);

        // Apply blur if specified
        let processed_img = if blur_amount > 0.0 {
            blur::blur_image(&resized_img, blur_amount)
        } else {
            resized_img
        };

        // Place the processed image onto the canvas at the monitor's position
        imageops::replace(
            &mut canvas,
            &processed_img,
            monitor.x as i64,
            monitor.y as i64,
        );
    }

    Ok(DynamicImage::ImageRgba8(canvas))
}
