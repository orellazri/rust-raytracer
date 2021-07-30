use crate::sphere::Sphere;
use crate::F;

#[derive(Debug, PartialEq)]
pub struct Intersection<'a> {
    pub t: F,
    pub object: &'a Sphere,
}

impl<'a> Intersection<'a> {
    pub fn new(t: F, object: &'a Sphere) -> Intersection<'a> {
        Intersection { t, object }
    }
}

fn intersections<'a>(xs: &[Intersection<'a>]) -> Vec<Intersection<'a>> {
    let mut intersections: Vec<Intersection<'a>> = Vec::new();

    for inter in xs.iter() {
        intersections.push(Intersection::new(inter.t, inter.object));
    }

    intersections
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
}
