use image::{DynamicImage, imageops::FilterType};

pub fn resize_to_monitor(
    img: &DynamicImage,
    width: u32,
    height: u32,
) -> DynamicImage {
    img.resize_exact(width, height, FilterType::Lanczos3)
}
