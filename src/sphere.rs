use crate::intersection::Intersection;
use crate::material::Material;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::tuple::Tuple;

#[derive(Debug, PartialEq)]
pub struct Sphere {
    pub transform: Matrix,
    pub material: Material,
}

impl Sphere {
    pub fn new() -> Self {
        Sphere {
            transform: Matrix::identity(),
            material: Material::new(),
        }
    }

    pub fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let ray = ray.transform(&self.transform.inverse());
        let sphere_to_ray = ray.origin - Tuple::point(0.0, 0.0, 0.0);
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * ray.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;
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

    pub fn normal_at(&self, world_point: &Tuple) -> Tuple {
        let object_point = self.transform.inverse() * world_point;
        let object_normal = object_point - Tuple::point(0.0, 0.0, 0.0);
        let mut world_normal = self.transform.inverse().transpose() * object_normal;
        world_normal.w = 0.0;
        world_normal.normalized()
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::{FRAC_1_SQRT_2, PI, SQRT_2};

    use super::*;

    use crate::{
        transformation::{rotation_z, scaling, translation},
        utils::floats_equal,
        F,
    };

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

    #[test]
    fn sphere_default_transformation() {
        let s = Sphere::new();

        assert_eq!(s.transform, Matrix::identity());
    }

    #[test]
    fn change_sphere_transformation() {
        let mut s = Sphere::new();
        let t = translation(2.0, 3.0, 4.0);
        s.transform = t;

        assert_eq!(s.transform, translation(2.0, 3.0, 4.0));
    }

    #[test]
    fn intersect_scaled_sphere_with_ray() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let mut s = Sphere::new();
        s.transform = scaling(2.0, 2.0, 2.0);
        let xs = s.intersect(&r);

        assert_eq!(xs.len(), 2);
        assert!(floats_equal(xs[0].t, 3.0));
        assert!(floats_equal(xs[1].t, 7.0));
    }

    #[test]
    fn intersect_translated_sphere_with_ray() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let mut s = Sphere::new();
        s.transform = translation(5.0, 0.0, 0.0);
        let xs = s.intersect(&r);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn normal_on_sphere_at_point_on_x_axis() {
        let s = Sphere::new();
        let n = s.normal_at(&Tuple::point(1.0, 0.0, 0.0));

        assert_eq!(n, Tuple::vector(1.0, 0.0, 0.0));
    }

    #[test]
    fn normal_on_sphere_at_point_on_y_axis() {
        let s = Sphere::new();
        let n = s.normal_at(&Tuple::point(0.0, 1.0, 0.0));

        assert_eq!(n, Tuple::vector(0.0, 1.0, 0.0));
    }

    #[test]
    fn normal_on_sphere_at_point_on_z_axis() {
        let s = Sphere::new();
        let n = s.normal_at(&Tuple::point(0.0, 0.0, 1.0));

        assert_eq!(n, Tuple::vector(0.0, 0.0, 1.0));
    }

    #[test]
    fn normal_on_sphere_at_nonaxial_point() {
        let s = Sphere::new();
        let n = s.normal_at(&Tuple::point(F::sqrt(3.0) / 3.0, F::sqrt(3.0) / 3.0, F::sqrt(3.0) / 3.0));

        assert_eq!(n, Tuple::vector(F::sqrt(3.0) / 3.0, F::sqrt(3.0) / 3.0, F::sqrt(3.0) / 3.0));
    }

    #[test]
    fn normal_is_a_normalized_vector() {
        let s = Sphere::new();
        let n = s.normal_at(&Tuple::point(F::sqrt(3.0) / 3.0, F::sqrt(3.0) / 3.0, F::sqrt(3.0) / 3.0));

        assert_eq!(n, n.normalized());
    }

    #[test]
    fn normal_on_translated_sphere() {
        let mut s = Sphere::new();
        s.transform = translation(0.0, 1.0, 0.0);
        let n = s.normal_at(&Tuple::point(0.0, 1.70711, -FRAC_1_SQRT_2));

        assert_eq!(n, Tuple::vector(0.0, FRAC_1_SQRT_2, -FRAC_1_SQRT_2));
    }

    #[test]
    fn normal_on_transformed_sphere() {
        let mut s = Sphere::new();
        s.transform = scaling(1.0, 0.5, 1.0) * rotation_z(PI / 5.0);
        let n = s.normal_at(&Tuple::point(0.0, SQRT_2 / 2.0, -SQRT_2 / 2.0));

        assert_eq!(n, Tuple::vector(0.0, 0.97014, -0.24254));
    }

    #[test]
    fn sphere_has_default_material() {
        let s = Sphere::new();

        assert_eq!(s.material, Material::new());
    }

    #[test]
    fn assign_sphere_material() {
        let mut s = Sphere::new();
        let mut m = Material::new();
        m.ambient = 1.0;
        s.material = m;

        assert_eq!(s.material, m);
    }
}
