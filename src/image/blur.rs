use image::DynamicImage;

pub fn blur_image(img: &DynamicImage, blur_amount: f32) -> DynamicImage {
    if blur_amount <= 0.0 {
        return img.clone();
    }

    img.blur(blur_amount)
}
