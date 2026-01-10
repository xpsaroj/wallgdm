use crate::{
    error::ImageError,
    image::{blur, resize},
    monitor::MonitorLayout,
};
use image::{
    imageops, {DynamicImage, RgbaImage},
};

pub fn compose_wallpaper(
    img_path: &str,
    layout: &MonitorLayout,
    blur_amount: f32,
) -> Result<DynamicImage, ImageError> {
    let img = image::open(img_path).map_err(|_| ImageError::ImageLoadFailed)?;

    let mut canvas = RgbaImage::from_pixel(
        layout.total_width,
        layout.total_height,
        image::Rgba([0, 0, 0, 255]),
    );

    for monitor in &layout.monitors {
        let resized_img =
            resize::resize_to_monitor(&img, monitor.width, monitor.height);

        let processed_img = if blur_amount > 0.0 {
            blur::blur_image(&resized_img, blur_amount)
        } else {
            resized_img
        };

        imageops::replace(
            &mut canvas,
            &processed_img,
            monitor.x as i64,
            monitor.y as i64,
        );
    }

    Ok(DynamicImage::ImageRgba8(canvas))
}
