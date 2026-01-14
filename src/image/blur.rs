//! Image blurring utilities for WallGDM.
//!
//! Provides functions to apply Gaussian blur to images, used when
//! composing wallpapers for GDM login screens.

use image::DynamicImage;

/// Apply a Gaussian blur to an image.
///
/// # Parameters
/// - `img`: the source image to blur.
/// - `blur_amount`: blur radius; if <= 0.0, no blur is applied.
///
/// # Returns
/// A new `DynamicImage` with the blur applied (or a clone if `blur_amount` ≤ 0).
pub fn blur_image(img: &DynamicImage, blur_amount: f32) -> DynamicImage {
    if blur_amount <= 0.0 {
        return img.clone();
    }

    log::debug!("Applying Gaussian blur with radius {}", blur_amount);
    img.blur(blur_amount)
}
