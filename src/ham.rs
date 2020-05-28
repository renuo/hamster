use std::cmp;
use ux::{u2, u4};
use image::{RgbImage, Rgb};
use crate::color::AmigaRgb;
use crate::color_map::ColorMap;

// Maybe we could reuse the Pixel trait from the image crate instead of defining this struct
// ourselves?
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ham6Pixel {
    pub color_index: u4,
    pub operation: u2,
}

pub struct HamImage<P> {
    pub width: usize,
    pub height: usize,
    pub data: Vec<P>,
    pub color_map: ColorMap,
}

impl HamImage<Ham6Pixel> {
    pub fn from_rgb(image: RgbImage) -> HamImage<Ham6Pixel> {
        let mut data: Vec<Ham6Pixel> = Vec::with_capacity((image.width() * image.height()) as usize);
        let color_map = ColorMap::default();

        fn difference([a_r, a_g, a_b]: [u8; 3], [b_r, b_g, b_b]: [u8; 3]) -> [u8; 3] {
            [
                cmp::max(a_r, b_r) - cmp::min(a_r, b_r),
                cmp::max(a_g, b_g) - cmp::min(a_g, b_g),
                cmp::max(a_b, b_b) - cmp::min(a_b, b_b),
            ]
        }

        for (x, y, pixel) in image.enumerate_pixels() {
            let ham_pixel = if x % image.width() == 0 {
                let color_index = color_map.index_of_similar(AmigaRgb::from(pixel.clone()));
                Ham6Pixel { operation: u2::new(0), color_index }
            } else {
                let [r, g, b] = pixel.0;
                let [diff_r, diff_g, diff_b] = difference(pixel.0, image.get_pixel(x - 1, y).0);

                let max = cmp::max(cmp::max(diff_r, diff_g), diff_b);
                let ham_pixel = if max >= 16 {
                    let color_index = color_map.index_of_similar(AmigaRgb::from(pixel.clone()));
                    Ham6Pixel { operation: u2::new(0), color_index }
                } else if max == diff_r {
                    Ham6Pixel { operation: u2::new(1), color_index: u4::new(r / 16) }
                } else if max == diff_g {
                    Ham6Pixel { operation: u2::new(2), color_index: u4::new(g / 16) }
                } else {
                    Ham6Pixel { operation: u2::new(3), color_index: u4::new(b / 16) }
                };
                ham_pixel
            };
            data.push(ham_pixel);
        }

        HamImage {
            width: image.width() as usize,
            height: image.height() as usize,
            data,
            color_map,
        }
    }

    pub fn to_rgb(&self) -> RgbImage {
        let mut output_image: RgbImage = RgbImage::new(self.width as u32, self.height as u32);
        let mut previous_amiga_pixel = AmigaRgb::from([0, 0, 0]);

        for (x, y, pixel) in output_image.enumerate_pixels_mut() {
            let Ham6Pixel { color_index, operation } = self.data[y as usize * self.width + x as usize];
            let [previous_r, previous_g, previous_b] = previous_amiga_pixel.0;

            let amiga_pixel = match u8::from(operation) {
                0 => { self.color_map[color_index] }, // mode
                1 => { AmigaRgb([color_index, previous_g, previous_b]) }, // r
                2 => { AmigaRgb([previous_r, color_index, previous_b]) }, // g
                3 => { AmigaRgb([previous_r, previous_g, color_index]) }, // b
                _ => { panic!("'operation' should be 'u4', but apparently is not") }
            };

            previous_amiga_pixel = amiga_pixel;
            *pixel = Rgb::from(amiga_pixel);
        }

        output_image
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_rgb() {
        let ham_img = HamImage::from_rgb(RgbImage::new(4, 4));
        assert_eq!(ham_img.width, 4);
        assert_eq!(ham_img.height, 4);
    }

    #[test]
    fn test_to_rgb() {
        let mut color_map = ColorMap::empty();
        color_map[0] = [0, 0, 0].into();
        color_map[1] = [1, 0, 0].into();
        color_map[2] = [8, 0, 0].into();
        color_map[3] = [15, 0, 0].into();

        let p00 = Ham6Pixel { color_index: u4::new(0), operation: u2::new(0) };
        let p01 = Ham6Pixel { color_index: u4::new(1), operation: u2::new(0) };
        let p10 = Ham6Pixel { color_index: u4::new(2), operation: u2::new(0) };
        let p11 = Ham6Pixel { color_index: u4::new(3), operation: u2::new(0) };

        let data = vec![p00, p01, p10, p11];
        let ham_img = HamImage { width: 2, height: 2, data, color_map };
        let img = ham_img.to_rgb();

        assert_eq!(img.get_pixel(0, 0), &Rgb([0, 0, 0]));
        assert_eq!(img.get_pixel(1, 0), &Rgb([17, 0, 0])); // 1 -> 17
        assert_eq!(img.get_pixel(0, 1), &Rgb([136, 0, 0])); // 8 -> 136 (middle of 128..143)
        assert_eq!(img.get_pixel(1, 1), &Rgb([255, 0, 0]));
    }
}
