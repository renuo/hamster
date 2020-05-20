use hamster::ham::{HamImage};

fn main() {
    let img = image::open("data_in/2.jpg").unwrap();
    let h = HamImage::from_rgb(img.to_rgb());
    h.to_rgb().save("data_out/2.png").unwrap();
}
