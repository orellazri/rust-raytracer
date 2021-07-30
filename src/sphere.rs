use crate::intersection::Intersection;
use crate::ray::Ray;
use crate::tuple::Tuple;

#[derive(Debug, PartialEq)]
pub struct Sphere {}

impl Sphere {
    pub fn new() -> Self {
        Sphere {}
    }

    pub fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let sphere_to_ray = ray.origin - Tuple::point(0.0, 0.0, 0.0);
        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * ray.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;
        let discriminant = (b * b) - (4.0 * a * c);

        if discriminant < 0.0 {
            // No hits
            Vec::new()
        } else {
            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
            vec![Intersection::new(t1, self), Intersection::new(t2, self)]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::floats_equal;

    #[test]
    fn ray_intersects_sphere_at_two_points() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(&r);

        assert_eq!(xs.len(), 2);
        assert!(floats_equal(xs[0].t, 4.0));
        assert!(floats_equal(xs[1].t, 6.0));
    }

    #[test]
    fn ray_misses_sphere() {
        let r = Ray::new(Tuple::point(0.0, 2.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(&r);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn ray_originates_inside_sphere() {
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(&r);

        assert_eq!(xs.len(), 2);
        assert!(floats_equal(xs[0].t, -1.0));
        assert!(floats_equal(xs[1].t, 1.0));
    }

    #[test]
    fn sphere_behind_ray() {
        let r = Ray::new(Tuple::point(0.0, 0.0, 5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(&r);

        assert_eq!(xs.len(), 2);
        assert!(floats_equal(xs[0].t, -6.0));
        assert!(floats_equal(xs[1].t, -4.0));
    }

    #[test]
    fn intersect_sets_the_object_on_the_intersection() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(&r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].object, &s);
        assert_eq!(xs[1].object, &s);
    }
}
