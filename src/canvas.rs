use crate::color::Color;

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Color>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
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

    pub fn to_ppm(&self) -> String {
        // Header
        let header = String::from(format!("P3\n{} {}\n255\n", self.width, self.height).as_str());
        let mut data = String::with_capacity(self.width * self.height * 3);

        // Pixel data
        for y in 0..self.height {
            for x in 0..self.width {
                let pixel = self.pixels[y * self.width + x];

                let mut topush = format!("{} ", (pixel.r * 255.0).round());
                data.push_str(topush.as_str());

                topush = format!("{} ", (pixel.g * 255.0).round());
                data.push_str(topush.as_str());

                topush = format!("{} ", (pixel.b * 255.0).round());
                data.push_str(topush.as_str());
            }

            // Remove last space in line
            data = String::from(&data[..data.len() - 1]);

            data.push('\n');
        }

        // Add newline after 70 characters where there is a space
        let mut line_char_counter = 1;
        for i in 1..data.len() {
            if data.as_bytes()[i] == b'\n' {
                line_char_counter = 0;
                continue;
            }

            line_char_counter += 1;

            if line_char_counter == 69 {
                // Look for next space
                if data.as_bytes()[i - 1] == b' ' {
                    data = format!("{}\n{}", &data[..i - 1], &data[i..]);
                } else if data.as_bytes()[i] == b' ' {
                    data = format!("{}\n{}", &data[..i], &data[i + 1..]);
                } else if data.as_bytes()[i + 1] == b' ' {
                    data = format!("{}\n{}", &data[..i + 1], &data[i + 2..]);
                } else if data.as_bytes()[i + 2] == b' ' {
                    data = format!("{}\n{}", &data[..i + 2], &data[i + 3..]);
                }

                line_char_counter = 0;
            }
        }

        format!("{}{}", header, data)
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
        assert_eq!(&ppm[..11], "P3\n5 3\n255\n");
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
        let expected_result = "P3\n5 3\n255\n255 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 128 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 0 0 0 0 0 0 0 255\n";
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

        let ppm = canvas.to_ppm();
        let expected_result = "P3\n10 2\n255\n255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204\n153 255 204 153 255 204 153 255 204 153 255 204 153\n255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204\n153 255 204 153 255 204 153 255 204 153 255 204 153\n";
        assert_eq!(ppm, expected_result);
    }

    #[test]
    fn ppm_ends_with_newline() {
        let canvas = Canvas::new(5, 3);
        let ppm = canvas.to_ppm();
        assert_eq!(ppm.as_bytes()[ppm.len() - 1], b'\n');
    }
}
