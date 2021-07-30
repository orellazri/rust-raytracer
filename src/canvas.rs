use crate::color::Color;

use hashbrown::HashMap;

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Color>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Canvas {
            width,
            height,
            pixels: vec![Color::black(); width * height],
        }
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> Color {
        self.pixels[y * self.width + x]
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) {
        // Check bounds before writing (not checking less than 0 because due to type limits)
        if x > self.width || y > self.height {
            return;
        }

        self.pixels[y * self.width + x] = color.clamped();
    }

    pub fn to_ppm(&self) -> Vec<u8> {
        // Helper function to translate a vector of colors to a vector of bytes
        // representing the text that will be written to the ppm
        fn colors_to_ppm(colors: &[u8]) -> Vec<u8> {
            let mut pos = 0;
            let mut v: Vec<u8> = Vec::new();
            for i in colors {
                let n = format!("{}", i);
                if pos + n.len() >= 68 {
                    v.extend("\n".as_bytes());
                    pos = 0;
                }
                if pos != 0 {
                    v.extend(" ".as_bytes());
                    pos += 1;
                }
                v.extend(n.as_bytes());
                pos += n.len();
            }

            v
        }

        // Header
        let mut header = Vec::new();
        header.extend("P3\n".as_bytes());
        header.extend(format!("{} {}\n", self.width, self.height).as_bytes());
        header.extend(format!("{}\n", 255).as_bytes());

        // Data
        let mut data: Vec<u8> = Vec::with_capacity(self.width * self.height);
        let mut colors_map: HashMap<Vec<u8>, Vec<u8>> = HashMap::new();
        for y in 0..self.height {
            let mut row: Vec<u8> = Vec::new();
            for x in 0..self.width {
                let clamped = self.pixel_at(x, y).clamped();
                row.push((clamped.r * 255.0).round() as u8);
                row.push((clamped.g * 255.0).round() as u8);
                row.push((clamped.b * 255.0).round() as u8);
            }

            let rowclone = row.clone();
            if !colors_map.contains_key(&row) {
                colors_map.insert(row, colors_to_ppm(&rowclone));
            }

            data.extend(colors_map.get(&rowclone).unwrap());
            // data.extend(colors_to_ppm(&row));
            data.extend("\n".as_bytes());
        }

        header.into_iter().chain(data).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_canvas() {
        let canvas = Canvas::new(10, 20);

        assert_eq!(canvas.width, 10);
        assert_eq!(canvas.height, 20);

        for y in 0..canvas.height {
            for x in 0..canvas.width {
                assert_eq!(canvas.pixel_at(x, y), Color::black());
            }
        }
    }

    #[test]
    fn write_pixel() {
        let mut canvas = Canvas::new(10, 20);
        let red = Color::red();
        canvas.write_pixel(2, 3, red);

        assert_eq!(canvas.pixel_at(2, 3), red);
    }

    #[test]
    fn construct_ppm_header() {
        let canvas = Canvas::new(5, 3);
        let ppm = canvas.to_ppm();
        let actual_result = &ppm[..11];
        let expected_result = "P3\n5 3\n255\n".as_bytes();

        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn construct_ppm_pixel_data() {
        let mut canvas = Canvas::new(5, 3);
        let c1 = Color::new(1.5, 0.0, 0.0);
        let c2 = Color::new(0.0, 0.5, 0.0);
        let c3 = Color::new(-0.5, 0.0, 1.0);

        canvas.write_pixel(0, 0, c1);
        canvas.write_pixel(2, 1, c2);
        canvas.write_pixel(4, 2, c3);

        let ppm = canvas.to_ppm();
        let expected_result =
            "P3\n5 3\n255\n255 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 128 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 0 0 0 0 0 0 0 255\n".as_bytes();

        assert_eq!(ppm, expected_result);
    }

    #[test]
    fn split_long_lines_in_ppm() {
        let mut canvas = Canvas::new(10, 2);
        let color = Color::new(1.0, 0.8, 0.6);

        for y in 0..canvas.height {
            for x in 0..canvas.width {
                canvas.write_pixel(x, y, color);
            }
        }

        let actual_result = canvas.to_ppm();
        let expected_result = "P3\n10 2\n255\n255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204\n153 255 204 153 255 204 153 255 204 153 255 204 153\n255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204\n153 255 204 153 255 204 153 255 204 153 255 204 153\n".as_bytes();

        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn ppm_ends_with_newline() {
        let canvas = Canvas::new(5, 3);
        let actual_result = canvas.to_ppm();

        assert_eq!(actual_result[actual_result.len() - 1], b'\n');
    }
}
