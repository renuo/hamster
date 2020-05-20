use ux::u4;
use std::ops::{Index, IndexMut};
use crate::color::AmigaRgb;

pub struct ColorMap {
    colors: [AmigaRgb; 16]
}

impl ColorMap {
    pub fn default() -> ColorMap {
        ColorMap {
            colors: [
                [00, 00, 00].into(), // AmigaRgb::from([0, 0, 0])
                [08, 00, 00].into(),
                [00, 08, 00].into(),
                [08, 08, 00].into(),
                [00, 00, 08].into(),
                [08, 00, 08].into(),
                [00, 08, 08].into(),
                [12, 12, 12].into(),
                [08, 08, 08].into(),
                [15, 00, 00].into(),
                [00, 15, 00].into(),
                [15, 15, 00].into(),
                [00, 00, 15].into(),
                [15, 00, 15].into(),
                [00, 15, 15].into(),
                [15, 15, 15].into(),
            ]
        }
    }

    pub fn empty() -> ColorMap {
        ColorMap {
            colors: [
                [00, 00, 00].into(), // AmigaRgb::from([0, 0, 0])
                [00, 00, 00].into(),
                [00, 00, 00].into(),
                [00, 00, 00].into(),
                [00, 00, 00].into(),
                [00, 00, 00].into(),
                [00, 00, 00].into(),
                [00, 00, 00].into(),
                [00, 00, 00].into(),
                [00, 00, 00].into(),
                [00, 00, 00].into(),
                [00, 00, 00].into(),
                [00, 00, 00].into(),
                [00, 00, 00].into(),
                [00, 00, 00].into(),
                [00, 00, 00].into(),
            ]
        }
    }

    pub fn index_of_similar(&self, needle: AmigaRgb) -> u4 {
        let nr = u8::from(needle.0[0]) as i32;
        let ng = u8::from(needle.0[1]) as i32;
        let nb = u8::from(needle.0[2]) as i32;

        let mut index = u4::new(0);
        let mut min_dist2 = (self.colors.len() as i32).pow(2) * 3;

        for (i, color) in self.colors.iter().enumerate() {
            let r = u8::from(color.0[0]) as i32;
            let g = u8::from(color.0[1]) as i32;
            let b = u8::from(color.0[2]) as i32;

            let dist2 = (nr - r).pow(2) + (ng - g).pow(2) + (nb - b).pow(2);
            if dist2 < min_dist2 {
                min_dist2 = dist2;
                index = u4::new(i as u8);
            }
        }

        index
    }
}

impl Index<u4> for ColorMap {
    type Output = AmigaRgb;

    fn index(&self, i: u4) -> &Self::Output {
        &self.colors[u8::from(i) as usize]
    }
}
impl IndexMut<u4> for ColorMap {
    fn index_mut(&mut self, i: u4) -> &mut Self::Output {
        &mut self.colors[u8::from(i) as usize]
    }
}

impl Index<u8> for ColorMap {
    type Output = AmigaRgb;

    fn index(&self, i: u8) -> &Self::Output {
        &self.colors[i as usize]
    }
}
impl IndexMut<u8> for ColorMap {
    fn index_mut(&mut self, i: u8) -> &mut Self::Output {
        &mut self.colors[i as usize]
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index_u4() {
        let mut cm = ColorMap::empty();
        cm[u4::new(15)] = [8, 8, 8].into();
        assert_eq!(cm[u4::new(15)], [8, 8, 8].into());
    }

    #[test]
    fn test_index_u8() {
        let mut cm = ColorMap::empty();
        cm[15] = [8, 8, 8].into();
        assert_eq!(cm[15], [8, 8, 8].into());
    }

    #[test]
    fn test_index_of_similar_same() {
        let mut cm = ColorMap::empty();
        cm[5] = [7, 7, 7].into();
        assert_eq!(cm.index_of_similar([7, 7, 7].into()), u4::new(5));
    }

    #[test]
    fn test_index_of_similar_near() {
        let mut cm = ColorMap::empty();
        cm[15] = [14, 15, 14].into();
        assert_eq!(cm.index_of_similar([15, 14, 15].into()), u4::new(15));
    }

    #[test]
    fn test_index_of_similar_first_if_same_distance() {
        let mut cm = ColorMap::empty();
        cm[4] = [15, 15, 15].into();
        cm[8] = [13, 13, 13].into();
        assert_eq!(cm.index_of_similar([14, 14, 14].into()), u4::new(4));
    }
}
