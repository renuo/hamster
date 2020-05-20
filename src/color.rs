use ux::u4;
use image::{Rgb};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct AmigaRgb (pub [u4; 3]); // 12bit RGB

// Would the following be worth a try? We could then use a downcast_channel adaption:
// pub type AmigaRgb = Rgb<u4>;

impl From<[u8; 3]> for AmigaRgb {
    fn from(rgb: [u8; 3]) -> Self {
        AmigaRgb([u4::new(rgb[0]), u4::new(rgb[1]), u4::new(rgb[2])])
    }
}
impl From<AmigaRgb> for Rgb<u8> {
    fn from(amiga_rgb: AmigaRgb) -> Self {
        let [r, g, b] = amiga_rgb.0;
        Rgb([
            // the plus is for spreading otherwise we'd miss 15 colors from RGB (max 240)
            (u8::from(r) << 4) + u8::from(r),
            (u8::from(g) << 4) + u8::from(g),
            (u8::from(b) << 4) + u8::from(b),
        ])
    }
}
impl From<Rgb<u8>> for AmigaRgb {
    fn from(rgb: Rgb<u8>) -> Self {
        let [r, g, b] = rgb.0;
        AmigaRgb([u4::new(r >> 4), u4::new(g >> 4), u4::new(b >> 4)])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_array() {
        let amiga_rgb = AmigaRgb([u4::new(1), u4::new(8), u4::new(15)]);
        let [r, g, b] = amiga_rgb.0;
        assert_eq!(r, u4::new(1));
        assert_eq!(g, u4::new(8));
        assert_eq!(b, u4::new(15));
    }

    #[test]
    fn test_from_rgb() {
        let amiga_rgb = AmigaRgb::from(Rgb([255, 20, 0]));
        let [r, g, b] = amiga_rgb.0;
        assert_eq!(r, u4::new(15));
        assert_eq!(g, u4::new(1));
        assert_eq!(b, u4::new(0));
    }

    #[test]
    fn test_into_rgb() {
        let rgb: Rgb<u8> = AmigaRgb([u4::new(0), u4::new(1), u4::new(15)]).into();
        let [r, g, b] = rgb.0;
        assert_eq!(r, 0);
        assert_eq!(g, 17);
        assert_eq!(b, 255);
    }
}
