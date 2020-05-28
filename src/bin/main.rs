use hamster::ham::{HamImage};
extern crate rand;

use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let img = image::open("data_in/1.png").unwrap();
    let h = HamImage::from_rgb(img.to_rgb());
    h.to_rgb().save(format!("data_out/{}.png", rng.gen::<u32>())).unwrap();
}
