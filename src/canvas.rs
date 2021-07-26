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
        self.pixels[y * self.width + x] = color;
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

        for x in 0..canvas.width {
            for y in 0..canvas.height {
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
}
