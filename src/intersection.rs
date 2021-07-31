use std::cmp::Ordering;

use crate::sphere::Sphere;
use crate::F;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Intersection<'a> {
    pub t: F,
    pub object: &'a Sphere,
}

impl<'a> Intersection<'a> {
    pub fn new(t: F, object: &'a Sphere) -> Intersection<'a> {
        Intersection { t, object }
    }
}

pub fn intersections<'a>(xs: &[Intersection<'a>]) -> Vec<Intersection<'a>> {
    let mut intersections = xs.to_owned();
    intersections.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap_or(Ordering::Equal));
    intersections.to_vec()
}

pub fn hit<'a>(xs: &'a [Intersection]) -> Option<&'a Intersection<'a>> {
    for x in xs {
        if x.t.is_sign_positive() {
            return Some(x);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::floats_equal;

    #[test]
    fn intersection_encapsulates_t_and_object() {
        let s = Sphere::new();
        let i = Intersection::new(3.5, &s);

        assert!(floats_equal(i.t, 3.5));
        assert_eq!(i.object, &s);
    }

    #[test]
    fn aggregate_intersections() {
        let s = Sphere::new();
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);
        let xs = intersections(&[i1, i2]);

        assert_eq!(xs.len(), 2);
        assert!(floats_equal(xs[0].t, 1.0));
        assert!(floats_equal(xs[1].t, 2.0));
    }

    #[test]
    fn hit_when_all_intersections_have_positive_t() {
        let s = Sphere::new();
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);
        let xs = intersections(&[i1, i2]);
        let i = hit(&xs);
        assert_eq!(i, Some(&i1));
    }

    #[test]
    fn hit_when_some_intersections_have_negative_t() {
        let s = Sphere::new();
        let i1 = Intersection::new(-1.0, &s);
        let i2 = Intersection::new(1.0, &s);
        let xs = intersections(&[i1, i2]);
        let i = hit(&xs);
        assert_eq!(i, Some(&i2));
    }

    #[test]
    fn hit_when_all_intersections_have_negative_t() {
        let s = Sphere::new();
        let i1 = Intersection::new(-2.0, &s);
        let i2 = Intersection::new(-1.0, &s);
        let xs = intersections(&[i1, i2]);
        let i = hit(&xs);
        assert_eq!(i, None);
    }

    #[test]
    fn hit_is_always_the_lowest_nonnegative_intersection() {
        let s = Sphere::new();
        let i1 = Intersection::new(5.0, &s);
        let i2 = Intersection::new(7.0, &s);
        let i3 = Intersection::new(-3.0, &s);
        let i4 = Intersection::new(2.0, &s);
        let xs = intersections(&[i1, i2, i3, i4]);
        let i = hit(&xs);
        assert_eq!(i, Some(&i4));
    }
}
