use crate::{color::Color, tuple::Tuple};

#[derive(Debug)]
pub struct PointLight {
    pub position: Tuple,
    pub intensity: Color,
}

impl PointLight {
    pub fn new(position: Tuple, intensity: Color) -> Self {
        PointLight { position, intensity }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_light_has_position_and_intensity() {
        let position = Tuple::point(0.0, 0.0, 0.0);
        let intensity = Color::new(1.0, 1.0, 1.0);
        let light = PointLight::new(position, intensity);

        assert_eq!(light.position, position);
        assert_eq!(light.intensity, intensity);
    }
}
