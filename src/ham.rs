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
    pub width: u32,
    pub height: u32,
    pub data: Vec<P>,
    pub color_map: ColorMap,
}

#[derive(Debug, Copy, Clone)]
pub struct DistanceComputation {
    pub operation: u2,
    pub payload: u4,
    pub color: AmigaRgb,
    pub distance: f64,
}

impl HamImage<Ham6Pixel> {
    pub fn from_rgb(image: RgbImage) -> HamImage<Ham6Pixel> {
        let mut data: Vec<Ham6Pixel> = Vec::with_capacity((image.width() * image.height()) as usize);
        let color_map = ColorMap::default();

        let mut previous_color = AmigaRgb::from([0, 0, 0]);

        for (_x, _y, pixel) in image.enumerate_pixels() {
            let target_color = AmigaRgb::from(pixel.clone());
            let color_index = color_map.index_of_similar(target_color);

            let index_color = color_map[color_index];
            let red_modified = AmigaRgb([target_color.r(), previous_color.g(), previous_color.b()]);
            let green_modified = AmigaRgb([previous_color.r(), target_color.g(), previous_color.b()]);
            let blue_modified = AmigaRgb([previous_color.r(), previous_color.g(), target_color.b()]);

            let mut computations = [
                DistanceComputation {
                    operation: u2::new(0),
                    payload: color_index,
                    color: index_color,
                    distance: index_color.euclidean_dist2(&target_color),
                },
                DistanceComputation {
                    operation: u2::new(1),
                    payload: red_modified.r(),
                    color: red_modified,
                    distance: red_modified.euclidean_dist2(&target_color),
                },
                DistanceComputation {
                    operation: u2::new(2),
                    payload: green_modified.g(),
                    color: green_modified,
                    distance: green_modified.euclidean_dist2(&target_color),
                },
                DistanceComputation {
                    operation: u2::new(3),
                    payload: blue_modified.b(),
                    color: blue_modified,
                    distance: blue_modified.euclidean_dist2(&target_color),
                },
            ];

            computations.sort_unstable_by(|a, b|
                a.distance.partial_cmp(&b.distance).unwrap()
            );

            let best_comp = computations[0];
            previous_color = best_comp.color;
            data.push(Ham6Pixel { color_index: best_comp.payload, operation: best_comp.operation });
        }

        HamImage {
            width: image.width(),
            height: image.height(),
            data,
            color_map,
        }
    }

    pub fn to_rgb(&self) -> RgbImage {
        let mut output_image: RgbImage = RgbImage::new(self.width, self.height);
        let mut previous_amiga_pixel = AmigaRgb::from([0, 0, 0]);

        for (x, y, pixel) in output_image.enumerate_pixels_mut() {
            let Ham6Pixel { color_index, operation } = self.data[(y * self.width + x) as usize];
            let [previous_r, previous_g, previous_b] = previous_amiga_pixel.0;

            let amiga_pixel = match u8::from(operation) {
                0 => { // look up index color
                    self.color_map[color_index]
                },
                1 => { // modify red
                    AmigaRgb([color_index, previous_g, previous_b])
                },
                2 => { // modify green
                    AmigaRgb([previous_r, color_index, previous_b])
                },
                3 => { // modify blue
                    AmigaRgb([previous_r, previous_g, color_index])
                },
                _ => {
                    panic!("'operation' should be 'u4', but apparently is not")
                }
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
