use hamster::ham::HamImage;
use hamster::color::AmigaRgb;
use hamster::dithering::*;
use image::{RgbImage, Rgb};

fn main() {
    let img = image::open("data_in/2.jpg").unwrap();
    // let h = HamImage::from_rgb(img.to_rgb()).to_rgb();
    let h = dither_floyd_steinberg(img.to_rgb());
    h.save("data_out/samuel_floyd.png").unwrap();
}

pub fn downsample(mut img: RgbImage) -> RgbImage {
    for (_x, _y, pixel) in img.enumerate_pixels_mut() {
        let original_rgb = pixel.clone();
        let amiga_rgb = AmigaRgb::from(original_rgb);
        let target_rgb = Rgb::from(amiga_rgb);

        *pixel = target_rgb;
    }

    img
}
