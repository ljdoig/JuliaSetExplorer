use image::Rgb;

fn rgb_to_u32(rgb: Rgb<u8>) -> u32 {
    let Rgb([r, g, b]) = rgb;
    (r as u32) << 16 | (g as u32) << 8 | b as u32
}

pub struct ColorPalette {
    colors: Vec<(f64, Rgb<u8>)>,
}

impl ColorPalette {
    pub fn new(colors: Vec<(f64, Rgb<u8>)>) -> Option<ColorPalette> {
        if colors.is_empty() {
            return None;
        }

        let mut sorted_colors = colors;
        sorted_colors.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        let first = sorted_colors.first().unwrap().0;
        let last = sorted_colors.last().unwrap().0;
        if first != 0.0 || last != 1.0 {
            return None;
        }

        Some(ColorPalette {
            colors: sorted_colors,
        })
    }

    pub fn default() -> ColorPalette {
        Self::new(vec![
            (0.0, Rgb([0, 18, 25])),
            (0.1, Rgb([20, 33, 61])),
            (0.25, Rgb([252, 163, 17])),
            (0.5, Rgb([229, 229, 229])),
            (1.0, Rgb([255, 255, 255])),
        ])
        .unwrap()
    }

    pub fn value(&self, value: f64) -> u32 {
        if value > 1.0 {
            return rgb_to_u32(self.colors.last().unwrap().1);
        } else if value < 0.0 {
            return rgb_to_u32(self.colors.first().unwrap().1);
        }
        match self
            .colors
            .binary_search_by(|&(v, _)| v.partial_cmp(&value).unwrap())
        {
            Ok(i) => rgb_to_u32(self.colors[i].1),
            Err(i) => {
                let (v1, c1) = self.colors[i - 1];
                let (v2, c2) = self.colors[i];

                let t = (value - v1) / (v2 - v1);

                let r = c1[0] + (t * (c2[0] as f64 - c1[0] as f64)) as u8;
                let g = c1[1] + (t * (c2[1] as f64 - c1[1] as f64)) as u8;
                let b = c1[2] + (t * (c2[2] as f64 - c1[2] as f64)) as u8;

                rgb_to_u32(Rgb([r, g, b]))
            }
        }
    }
}
