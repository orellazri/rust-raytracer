use crate::{color::Color, light::PointLight, tuple::Tuple, utils::floats_equal};

#[derive(Debug, Copy, Clone)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}

impl Material {
    pub fn new() -> Self {
        Material {
            color: Color::white(),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }

    pub fn from_params(color: Color, ambient: f64, diffuse: f64, specular: f64, shininess: f64) -> Self {
        Material {
            color,
            ambient,
            diffuse,
            specular,
            shininess,
        }
    }

    pub fn lighting(&self, light: PointLight, point: Tuple, eye: Tuple, normal: Tuple) -> Color {
        let effective_color = self.color * light.intensity;
        let lightv = (light.position - point).normalized();
        let ambient = effective_color * self.ambient;
        let light_dot_normal = lightv.dot(&normal);

        let diffuse;
        let specular;

        if light_dot_normal < 0.0 {
            diffuse = Color::black();
            specular = Color::black();
        } else {
            diffuse = effective_color * self.diffuse * light_dot_normal;
            let reflectv = -lightv.reflect(&normal);
            let reflect_dot_eye = reflectv.dot(&eye);

            if reflect_dot_eye <= 0.0 {
                specular = Color::black();
            } else {
                let factor = reflect_dot_eye.powf(self.shininess);
                specular = light.intensity * self.specular * factor;
            }
        }

        ambient + diffuse + specular
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
    use std::f64::consts::SQRT_2;

    use crate::utils::floats_equal;

    use super::*;

    #[test]
    fn default_material() {
        let m = Material::new();

        assert_eq!(m.color, Color::white());
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

    #[test]
    fn lighting_with_the_eye_between_the_light_and_the_surface() {
        let m = Material::new();
        let position = Tuple::point(0.0, 0.0, 0.0);
        let eye = Tuple::vector(0.0, 0.0, -1.0);
        let normal = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::point(0.0, 0.0, -10.0), Color::white());
        let result = m.lighting(light, position, eye, normal);

        assert_eq!(result, Color::new(1.9, 1.9, 1.9));
    }

    #[test]
    fn lighting_with_the_eye_between_light_surface_eye_offset_45() {
        let m = Material::new();
        let position = Tuple::point(0.0, 0.0, 0.0);
        let eye = Tuple::vector(0.0, SQRT_2 / 2.0, -SQRT_2 / 2.0);
        let normal = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::point(0.0, 0.0, -10.0), Color::white());
        let result = m.lighting(light, position, eye, normal);

        assert_eq!(result, Color::white());
    }

    #[test]
    fn lighting_with_eye_opposite_surface_light_offset_45() {
        let m = Material::new();
        let position = Tuple::point(0.0, 0.0, 0.0);
        let eye = Tuple::vector(0.0, 0.0, -1.0);
        let normal = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::point(0.0, 10.0, -10.0), Color::white());
        let result = m.lighting(light, position, eye, normal);

        assert_eq!(result, Color::new(0.7364, 0.7364, 0.7364));
    }

    #[test]
    fn lighting_with_eye_in_the_path_of_the_reflection_vector() {
        let m = Material::new();
        let position = Tuple::point(0.0, 0.0, 0.0);
        let eye = Tuple::vector(0.0, -SQRT_2 / 2.0, -SQRT_2 / 2.0);
        let normal = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::point(0.0, 10.0, -10.0), Color::white());
        let result = m.lighting(light, position, eye, normal);

        assert_eq!(result, Color::new(1.6364, 1.6364, 1.6364));
    }

    #[test]
    fn lighting_with_the_light_behind_the_surface() {
        let m = Material::new();
        let position = Tuple::point(0.0, 0.0, 0.0);
        let eye = Tuple::vector(0.0, 0.0, -1.0);
        let normal = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::point(0.0, 0.0, 10.0), Color::white());
        let result = m.lighting(light, position, eye, normal);

        assert_eq!(result, Color::new(0.1, 0.1, 0.1));
    }
}
