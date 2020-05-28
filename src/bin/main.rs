use hamster::ham::HamImage;
use hamster::color::AmigaRgb;
use hamster::dithering::*;

fn main() {
    let img = image::open("data_in/2.jpg").unwrap();
    // let h = HamImage::from_rgb(img.to_rgb()).to_rgb();
    let h = dither_jjn(img.to_rgb());
    h.save("data_out/2.png").unwrap();
}
