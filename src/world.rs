use crate::{
    color::Color,
    intersection::{hit, Computations, Intersection},
    light::PointLight,
    ray::Ray,
    sphere::Sphere,
    transformation,
    tuple::Tuple,
};

#[derive(Debug)]
pub struct World {
    pub light: Option<PointLight>,
    pub objects: Vec<Sphere>,
}

impl World {
    pub fn new() -> Self {
        World {
            light: None,
            objects: Vec::new(),
        }
    }

    pub fn default() -> Self {
        let mut s1 = Sphere::new();
        s1.material.color = Color::new(0.8, 1.0, 0.6);
        s1.material.diffuse = 0.7;
        s1.material.specular = 0.2;

        let mut s2 = Sphere::new();
        s2.transform = transformation::scaling(0.5, 0.5, 0.5);

        World {
            light: Some(PointLight::new(Tuple::point(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0))),
            objects: vec![s1, s2],
        }
    }

    pub fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let mut intersections = Vec::new();

        for object in &self.objects {
            for intersection in object.intersect(ray) {
                intersections.push(intersection);
            }
        }

        intersections.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());

        intersections
    }

    pub fn shade_hit(&self, comps: Computations) -> Color {
        comps
            .object
            .material
            .lighting(self.light.unwrap(), comps.point, comps.eyev, comps.normalv)
    }

    pub fn color_at(&self, ray: &Ray) -> Color {
        match hit(&self.intersect(ray)) {
            Some(xs) => self.shade_hit(xs.prepare_computations(ray)),
            None => Color::black(),
        }
    }
}

impl Default for World {
    fn default() -> Self {
        World::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::floats_equal;

    #[test]
    fn create_world() {
        let w = World::new();

        assert_eq!(w.objects.len(), 0);
        assert_eq!(w.light, None);
    }

    #[test]
    fn default_world() {
        let light = PointLight::new(Tuple::point(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

        let mut s1 = Sphere::new();
        s1.material.color = Color::new(0.8, 1.0, 0.6);
        s1.material.diffuse = 0.7;
        s1.material.specular = 0.2;

        let mut s2 = Sphere::new();
        s2.transform = transformation::scaling(0.5, 0.5, 0.5);

        let w = World::default();

        assert_eq!(w.light.unwrap(), light);
        assert!(w.objects.contains(&s1));
        assert!(w.objects.contains(&s2));
    }

    #[test]
    fn intersect_world_with_ray() {
        let w = World::default();
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let xs = w.intersect(&r);

        assert_eq!(xs.len(), 4);
        assert!(floats_equal(xs[0].t, 4.0));
        assert!(floats_equal(xs[1].t, 4.5));
        assert!(floats_equal(xs[2].t, 5.5));
        assert!(floats_equal(xs[3].t, 6.0));
    }

    #[test]
    fn shading_an_intersection() {
        let w = World::default();
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let shape = &w.objects[0];
        let i = Intersection::new(4.0, shape);
        let comps = i.prepare_computations(&r);
        let c = w.shade_hit(comps);

        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn shading_an_intersection_from_the_inside() {
        let mut w = World::default();
        w.light = Some(PointLight::new(Tuple::point(0.0, 0.25, 0.0), Color::new(1.0, 1.0, 1.0)));
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let shape = &w.objects[1];
        let i = Intersection::new(0.5, shape);
        let comps = i.prepare_computations(&r);
        let c = w.shade_hit(comps);

        assert_eq!(c, Color::new(0.90498, 0.90498, 0.90498));
    }

    #[test]
    fn color_when_ray_misses() {
        let w = World::default();
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 1.0, 0.0));
        let c = w.color_at(&r);

        assert_eq!(c, Color::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn color_when_ray_hits() {
        let w = World::default();
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let c = w.color_at(&r);

        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn color_when_an_intersection_behind_the_ray() {
        let mut w = World::default();
        let expected_result = w.objects[1].material.color;
        let mut outer = &mut w.objects[0];
        outer.material.ambient = 1.0;
        let mut inner = &mut w.objects[1];
        inner.material.ambient = 1.0;
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.75), Tuple::vector(0.0, 0.0, -1.0));
        let c = w.color_at(&r);

        assert_eq!(c, expected_result);
    }
}
