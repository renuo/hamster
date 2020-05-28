use image::{RgbImage, Rgb};
use crate::color::AmigaRgb;

pub fn dither_linear_naive(mut img: RgbImage) -> RgbImage {
    let mut last_error_r: i16 = 0;
    let mut last_error_g: i16 = 0;
    let mut last_error_b: i16 = 0;

    for (_x, _y, pixel) in img.enumerate_pixels_mut() {
        let original_rgb = pixel.clone();
        let amiga_rgb = AmigaRgb::from(original_rgb);
        let target_rgb = Rgb::from(amiga_rgb);

        let corrected_rgb = Rgb([
            (original_rgb.0[0] as i16 + last_error_r).max(0).min(255) as u8,
            (original_rgb.0[1] as i16 + last_error_g).max(0).min(255) as u8,
            (original_rgb.0[2] as i16 + last_error_b).max(0).min(255) as u8,
        ]);
        last_error_r = original_rgb.0[0] as i16 - target_rgb.0[0] as i16;
        last_error_g = original_rgb.0[1] as i16 - target_rgb.0[1] as i16;
        last_error_b = original_rgb.0[2] as i16 - target_rgb.0[2] as i16;

        let corrected_amiga_rgb = AmigaRgb::from(corrected_rgb);
        let correted_target_rgb = Rgb::from(corrected_amiga_rgb);

        *pixel = correted_target_rgb;
    }

    img
}

/*
       X   7
   3   5   1

     (1/16)
 */
pub fn dither_floyd_steinberg(mut img: RgbImage) -> RgbImage {
    const FACTOR: f64 = 1.0/16.0;

    let mut current_error_r: i16;
    let mut current_error_g: i16;
    let mut current_error_b: i16;

    let (width, height) = img.dimensions();
    for x in 1..width-2 {
        for y in 1..height-2 {
            let original_rgb = img.get_pixel(x, y).clone();
            let amiga_rgb = AmigaRgb::from(original_rgb);
            let target_rgb = Rgb::from(amiga_rgb);

            current_error_r = original_rgb.0[0] as i16 - target_rgb.0[0] as i16;
            current_error_g = original_rgb.0[1] as i16 - target_rgb.0[1] as i16;
            current_error_b = original_rgb.0[2] as i16 - target_rgb.0[2] as i16;

            let corrected_rgb = Rgb([
                (original_rgb.0[0] as i16 + current_error_r).max(0).min(255) as u8,
                (original_rgb.0[1] as i16 + current_error_g).max(0).min(255) as u8,
                (original_rgb.0[2] as i16 + current_error_b).max(0).min(255) as u8,
            ]);

            img.put_pixel(x, y, corrected_rgb);
            img.put_pixel(x + 1, y, Rgb([
                (corrected_rgb.0[0] as f64 * FACTOR * 7.0).max(0.0).min(255.0) as u8,
                (corrected_rgb.0[1] as f64 * FACTOR * 7.0).max(0.0).min(255.0) as u8,
                (corrected_rgb.0[2] as f64 * FACTOR * 7.0).max(0.0).min(255.0) as u8,
            ]));
            img.put_pixel(x + 1, y + 1, Rgb([
                (corrected_rgb.0[0] as f64 * FACTOR * 1.0).max(0.0).min(255.0) as u8,
                (corrected_rgb.0[1] as f64 * FACTOR * 1.0).max(0.0).min(255.0) as u8,
                (corrected_rgb.0[2] as f64 * FACTOR * 1.0).max(0.0).min(255.0) as u8,
            ]));
            img.put_pixel(x, y + 1, Rgb([
                (corrected_rgb.0[0] as f64 * FACTOR * 5.0).max(0.0).min(255.0) as u8,
                (corrected_rgb.0[1] as f64 * FACTOR * 5.0).max(0.0).min(255.0) as u8,
                (corrected_rgb.0[2] as f64 * FACTOR * 5.0).max(0.0).min(255.0) as u8,
            ]));
            img.put_pixel(x - 1, y + 1, Rgb([
                (corrected_rgb.0[0] as f64 * FACTOR * 3.0).max(0.0).min(255.0) as u8,
                (corrected_rgb.0[1] as f64 * FACTOR * 3.0).max(0.0).min(255.0) as u8,
                (corrected_rgb.0[2] as f64 * FACTOR * 3.0).max(0.0).min(255.0) as u8,
            ]));
        }
    }

    img
}

/*
                 X   7   5
     3   5   7   5   3
     1   3   5   3   1

           (1/48)
 */
pub fn dither_jjn(mut img: RgbImage) -> RgbImage {
    const FACTOR: f64 = 1.0/48.0;

    let mut current_error_r: i16;
    let mut current_error_g: i16;
    let mut current_error_b: i16;

    let (width, height) = img.dimensions();
    for x in 3..(width - 1 - 2) {
        for y in 0..(height - 1 - 2) {
            let original_rgb = img.get_pixel(x, y).clone();
            let amiga_rgb = AmigaRgb::from(original_rgb);
            let target_rgb = Rgb::from(amiga_rgb);

            current_error_r = original_rgb.0[0] as i16 - target_rgb.0[0] as i16;
            current_error_g = original_rgb.0[1] as i16 - target_rgb.0[1] as i16;
            current_error_b = original_rgb.0[2] as i16 - target_rgb.0[2] as i16;

            let corrected_rgb = Rgb([
                (original_rgb.0[0] as i16 + current_error_r).max(0).min(255) as u8,
                (original_rgb.0[1] as i16 + current_error_g).max(0).min(255) as u8,
                (original_rgb.0[2] as i16 + current_error_b).max(0).min(255) as u8,
            ]);

            img.put_pixel(x, y, corrected_rgb);

            img.put_pixel(x + 1, y, Rgb([
                (corrected_rgb.0[0] as f64 * FACTOR * 7.0).max(0.0).min(255.0) as u8,
                (corrected_rgb.0[1] as f64 * FACTOR * 7.0).max(0.0).min(255.0) as u8,
                (corrected_rgb.0[2] as f64 * FACTOR * 7.0).max(0.0).min(255.0) as u8,
            ]));
            img.put_pixel(x + 2, y, Rgb([
                (corrected_rgb.0[0] as f64 * FACTOR * 5.0).max(0.0).min(255.0) as u8,
                (corrected_rgb.0[1] as f64 * FACTOR * 5.0).max(0.0).min(255.0) as u8,
                (corrected_rgb.0[2] as f64 * FACTOR * 5.0).max(0.0).min(255.0) as u8,
            ]));

            img.put_pixel(x - 3, y + 1, Rgb([
                (corrected_rgb.0[0] as f64 * FACTOR * 3.0).max(0.0).min(255.0) as u8,
                (corrected_rgb.0[1] as f64 * FACTOR * 3.0).max(0.0).min(255.0) as u8,
                (corrected_rgb.0[2] as f64 * FACTOR * 3.0).max(0.0).min(255.0) as u8,
            ]));
            img.put_pixel(x - 2, y + 1, Rgb([
                (corrected_rgb.0[0] as f64 * FACTOR * 5.0).max(0.0).min(255.0) as u8,
                (corrected_rgb.0[1] as f64 * FACTOR * 5.0).max(0.0).min(255.0) as u8,
                (corrected_rgb.0[2] as f64 * FACTOR * 5.0).max(0.0).min(255.0) as u8,
            ]));
            img.put_pixel(x - 1, y + 1, Rgb([
                (corrected_rgb.0[0] as f64 * FACTOR * 7.0).max(0.0).min(255.0) as u8,
                (corrected_rgb.0[1] as f64 * FACTOR * 7.0).max(0.0).min(255.0) as u8,
                (corrected_rgb.0[2] as f64 * FACTOR * 7.0).max(0.0).min(255.0) as u8,
            ]));
            img.put_pixel(x, y + 1, Rgb([
                (corrected_rgb.0[0] as f64 * FACTOR * 5.0).max(0.0).min(255.0) as u8,
                (corrected_rgb.0[1] as f64 * FACTOR * 5.0).max(0.0).min(255.0) as u8,
                (corrected_rgb.0[2] as f64 * FACTOR * 5.0).max(0.0).min(255.0) as u8,
            ]));
            img.put_pixel(x + 1, y + 1, Rgb([
                (corrected_rgb.0[0] as f64 * FACTOR * 3.0).max(0.0).min(255.0) as u8,
                (corrected_rgb.0[1] as f64 * FACTOR * 3.0).max(0.0).min(255.0) as u8,
                (corrected_rgb.0[2] as f64 * FACTOR * 3.0).max(0.0).min(255.0) as u8,
            ]));

            img.put_pixel(x - 3, y + 2, Rgb([
                (corrected_rgb.0[0] as f64 * FACTOR * 1.0).max(0.0).min(255.0) as u8,
                (corrected_rgb.0[1] as f64 * FACTOR * 1.0).max(0.0).min(255.0) as u8,
                (corrected_rgb.0[2] as f64 * FACTOR * 1.0).max(0.0).min(255.0) as u8,
            ]));
            img.put_pixel(x - 2, y + 2, Rgb([
                (corrected_rgb.0[0] as f64 * FACTOR * 3.0).max(0.0).min(255.0) as u8,
                (corrected_rgb.0[1] as f64 * FACTOR * 3.0).max(0.0).min(255.0) as u8,
                (corrected_rgb.0[2] as f64 * FACTOR * 3.0).max(0.0).min(255.0) as u8,
            ]));
            img.put_pixel(x - 1, y + 2, Rgb([
                (corrected_rgb.0[0] as f64 * FACTOR * 5.0).max(0.0).min(255.0) as u8,
                (corrected_rgb.0[1] as f64 * FACTOR * 5.0).max(0.0).min(255.0) as u8,
                (corrected_rgb.0[2] as f64 * FACTOR * 5.0).max(0.0).min(255.0) as u8,
            ]));
            img.put_pixel(x, y + 2, Rgb([
                (corrected_rgb.0[0] as f64 * FACTOR * 3.0).max(0.0).min(255.0) as u8,
                (corrected_rgb.0[1] as f64 * FACTOR * 3.0).max(0.0).min(255.0) as u8,
                (corrected_rgb.0[2] as f64 * FACTOR * 3.0).max(0.0).min(255.0) as u8,
            ]));
            img.put_pixel(x + 1, y + 2, Rgb([
                (corrected_rgb.0[0] as f64 * FACTOR * 1.0).max(0.0).min(255.0) as u8,
                (corrected_rgb.0[1] as f64 * FACTOR * 1.0).max(0.0).min(255.0) as u8,
                (corrected_rgb.0[2] as f64 * FACTOR * 1.0).max(0.0).min(255.0) as u8,
            ]));
        }
    }

    img
}
