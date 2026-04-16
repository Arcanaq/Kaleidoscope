use image::GenericImageView;
use xilem::masonry::peniko::{ImageAlphaType, ImageData};
use xilem::winit::window::Icon;
use xilem::{Blob, ImageBrush, ImageFormat};

pub fn get_icon_brush(id:&str) -> ImageBrush {
    let (img_raw, img_w, img_h) = {
        let img = image::open(format!("src/assets/images/{}", id))
            .expect("Failed to find window icon.")
            .into_rgba8();
        let (w, h) = img.dimensions();
        let raw = img.into_raw();
        (raw, w, h)
    };
    let img_data = ImageData {
        data: Blob::from(img_raw),
        width: img_w,
        height: img_h,
        format: ImageFormat::Rgba8,
        alpha_type: ImageAlphaType::Alpha,
    };
    ImageBrush::new(img_data)
}

pub fn get_icon() -> Icon {
    let (ico_raw, ico_w, ico_h) = {
        let img = image::open("src/assets/images/icon.png")
            .expect("Failed to find window icon.")
            .into_rgba8();
        let (w, h) = img.dimensions();
        let raw = img.into_raw();
        (raw, w, h)
    };
    Icon::from_rgba(ico_raw, ico_w, ico_h).expect("Failed to load window icon.")
}