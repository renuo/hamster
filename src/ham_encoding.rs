use ux::{u2, u4};
use crate::color::AmigaRgb;
use crate::color_map::ColorMap;

#[derive(Debug, Copy, Clone)]
pub struct Computation {
    pub operation: u2,
    pub payload: u4,
    pub color: AmigaRgb,
    pub distance: f64,
}

pub fn encode(color_map: &ColorMap, previous_color: AmigaRgb, target_color: AmigaRgb) -> Computation {
    let color_index = color_map.index_of_similar(target_color);

    let index_color = color_map[color_index];
    let red_modified = AmigaRgb([target_color.r(), previous_color.g(), previous_color.b()]);
    let green_modified = AmigaRgb([previous_color.r(), target_color.g(), previous_color.b()]);
    let blue_modified = AmigaRgb([previous_color.r(), previous_color.g(), target_color.b()]);

    let mut computations = [
        Computation {
            operation: u2::new(0),
            payload: color_index,
            color: index_color,
            distance: index_color.euclidean_dist2(&target_color),
        },
        Computation {
            operation: u2::new(1),
            payload: red_modified.r(),
            color: red_modified,
            distance: red_modified.euclidean_dist2(&target_color),
        },
        Computation {
            operation: u2::new(2),
            payload: green_modified.g(),
            color: green_modified,
            distance: green_modified.euclidean_dist2(&target_color),
        },
        Computation {
            operation: u2::new(3),
            payload: blue_modified.b(),
            color: blue_modified,
            distance: blue_modified.euclidean_dist2(&target_color),
        },
    ];

    computations.sort_unstable_by(|a, b|
        a.distance.partial_cmp(&b.distance).unwrap()
    );

    computations[0]
}
