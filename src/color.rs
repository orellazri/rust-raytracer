use crate::F;
use std::ops;

use crate::utils::*;

#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub r: F,
    pub g: F,
    pub b: F,
}

impl Color {
    pub fn new(r: F, g: F, b: F) -> Color {
        Color { r, g, b }
    }

    pub fn black() -> Color {
        Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        }
    }

    pub fn white() -> Color {
        Color {
            r: 1.0,
            g: 1.0,
            b: 1.0,
        }
    }

    pub fn red() -> Color {
        Color {
            r: 1.0,
            g: 0.0,
            b: 0.0,
        }
    }

    pub fn green() -> Color {
        Color {
            r: 0.0,
            g: 1.0,
            b: 0.0,
        }
    }

    pub fn blue() -> Color {
        Color {
            r: 0.0,
            g: 0.0,
            b: 1.0,
        }
    }

    pub fn clamped(&self) -> Color {
        Color::new(
            self.r.min(1.0).max(0.0),
            self.g.min(1.0).max(0.0),
            self.b.min(1.0).max(0.0),
        )
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        floats_equal(self.r, other.r)
            && floats_equal(self.g, other.g)
            && floats_equal(self.b, other.b)
    }
}

impl ops::Add for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Color {
        Color::new(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b)
    }
}

impl ops::Sub for Color {
    type Output = Color;

    fn sub(self, rhs: Color) -> Color {
        Color::new(self.r - rhs.r, self.g - rhs.g, self.b - rhs.b)
    }
}

impl ops::Mul<F> for Color {
    type Output = Color;

    fn mul(self, rhs: F) -> Color {
        Color::new(self.r * rhs, self.g * rhs, self.b * rhs)
    }
}

impl ops::Mul for Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Color {
        Color::new(self.r * rhs.r, self.g * rhs.g, self.b * rhs.b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_colors() {
        let color1 = Color::new(0.9, 0.6, 0.75);
        let color2 = Color::new(0.7, 0.1, 0.25);
        let result = Color::new(1.6, 0.7, 1.0);
        assert_eq!(color1 + color2, result);
    }

    #[test]
    fn subtract_colors() {
        let color1 = Color::new(0.9, 0.6, 0.75);
        let color2 = Color::new(0.7, 0.1, 0.25);
        let result = Color::new(0.2, 0.5, 0.5);
        assert_eq!(color1 - color2, result);
    }

    #[test]
    fn multiply_color_by_scalar() {
        let color = Color::new(0.2, 0.3, 0.4);
        let result = Color::new(0.4, 0.6, 0.8);
        assert_eq!(color * 2.0, result);
    }

    #[test]
    fn multiply_colors() {
        let color1 = Color::new(1.0, 0.2, 0.4);
        let color2 = Color::new(0.9, 1.0, 0.1);
        let result = Color::new(0.9, 0.2, 0.04);
        assert_eq!(color1 * color2, result);
    }

    #[test]
    fn clamp_colors() {
        let color = Color::new(2.3, -6.7, 0.8);
        let result = color.clamped();
        assert_eq!(result, Color::new(1.0, 0.0, 0.8));
    }
}
