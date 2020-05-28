use hamster::ham::{HamImage};

fn main() {
    let img = image::open("data_in/mario-face.jpg").unwrap();
    let h = HamImage::from_rgb(img.to_rgb());
    h.to_rgb().save("data_out/mario-face.png").unwrap();
}
