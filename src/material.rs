use crate::{color::Color, utils::floats_equal, F};

#[derive(Debug, Copy, Clone)]
pub struct Material {
    pub color: Color,
    pub ambient: F,
    pub diffuse: F,
    pub specular: F,
    pub shininess: F,
}

impl Material {
    pub fn new() -> Self {
        Material {
            color: Color::new(1.0, 1.0, 1.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }

    pub fn from_params(color: Color, ambient: F, diffuse: F, specular: F, shininess: F) -> Self {
        Material {
            color,
            ambient,
            diffuse,
            specular,
            shininess,
        }
    }
}

impl PartialEq for Material {
    fn eq(&self, other: &Self) -> bool {
        floats_equal(self.ambient, other.ambient)
            && floats_equal(self.diffuse, other.diffuse)
            && floats_equal(self.specular, other.specular)
            && floats_equal(self.shininess, other.shininess)
            && self.color == other.color
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::floats_equal;

    use super::*;

    #[test]
    fn default_material() {
        let m = Material::new();
        assert_eq!(m.color, Color::new(1.0, 1.0, 1.0));
        assert!(floats_equal(m.ambient, 0.1));
        assert!(floats_equal(m.diffuse, 0.9));
        assert!(floats_equal(m.specular, 0.9));
        assert!(floats_equal(m.shininess, 200.0));
    }

    #[test]
    fn materials_are_equal() {
        let mut m1 = Material::new();
        m1.shininess = 50.0;
        let mut m2 = Material::new();
        m2.shininess = 50.0;
        assert_eq!(m1, m2);
    }

    #[test]
    fn materials_are_not_equal() {
        let mut m1 = Material::new();
        m1.shininess = 50.0;
        let mut m2 = Material::new();
        m2.color = Color::new(0.8, 0.8, 0.8);
        assert_ne!(m1, m2);
    }
}
