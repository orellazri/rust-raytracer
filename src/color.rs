use std::ops;

use crate::utils::floats_equal;

#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Color { r, g, b }
    }

    pub fn black() -> Self {
        Color { r: 0.0, g: 0.0, b: 0.0 }
    }

    pub fn white() -> Self {
        Color { r: 1.0, g: 1.0, b: 1.0 }
    }

    pub fn red() -> Self {
        Color { r: 1.0, g: 0.0, b: 0.0 }
    }

    pub fn green() -> Self {
        Color { r: 0.0, g: 1.0, b: 0.0 }
    }

    pub fn blue() -> Self {
        Color { r: 0.0, g: 0.0, b: 1.0 }
    }

    pub fn yellow() -> Self {
        Color { r: 1.0, g: 1.0, b: 0.0 }
    }

    pub fn clamped(&self) -> Self {
        Color::new(self.r.min(1.0).max(0.0), self.g.min(1.0).max(0.0), self.b.min(1.0).max(0.0))
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        floats_equal(self.r, other.r) && floats_equal(self.g, other.g) && floats_equal(self.b, other.b)
    }
}

impl ops::Add for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        Color::new(self.r + other.r, self.g + other.g, self.b + other.b)
    }
}

impl ops::Sub for Color {
    type Output = Color;

    fn sub(self, other: Color) -> Color {
        Color::new(self.r - other.r, self.g - other.g, self.b - other.b)
    }
}

impl ops::Mul<f64> for Color {
    type Output = Color;

    fn mul(self, other: f64) -> Color {
        Color::new(self.r * other, self.g * other, self.b * other)
    }
}

impl ops::Mul for Color {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        Color::new(self.r * other.r, self.g * other.g, self.b * other.b)
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
