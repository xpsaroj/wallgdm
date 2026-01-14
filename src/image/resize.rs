//! Image resizing utilities for WallGDM.
//!
//! Provides functions to resize images to fit a monitor's dimensions,
//! used when composing wallpapers for multiple monitors.

use image::{DynamicImage, imageops::FilterType};

/// Resize an image to exactly fit the given monitor dimensions.
///
/// # Parameters
/// - `img`: the source image to resize.
/// - `width`: target width in pixels.
/// - `height`: target height in pixels.
///
/// # Returns
/// A new `DynamicImage` resized to the specified width and height.
///
/// # Notes
/// Uses the Lanczos3 filter for high-quality resizing.
pub fn resize_to_monitor(
    img: &DynamicImage,
    width: u32,
    height: u32,
) -> DynamicImage {
    log::debug!("Resizing image to {}x{}", width, height);
    img.resize_exact(width, height, FilterType::Lanczos3)
}
