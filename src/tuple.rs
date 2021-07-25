use std::ops;

use crate::float::floats_equal;

#[derive(Debug)]
struct Tuple {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl Tuple {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Tuple { x, y, z, w }
    }

    pub fn point(x: f32, y: f32, z: f32) -> Self {
        Tuple { x, y, z, w: 1.0 }
    }

    pub fn vector(x: f32, y: f32, z: f32) -> Self {
        Tuple { x, y, z, w: 0.0 }
    }

    pub fn is_point(&self) -> bool {
        self.w == 1.0
    }

    pub fn is_vector(&self) -> bool {
        self.w == 0.0
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        floats_equal(self.x, other.x)
            && floats_equal(self.y, other.y)
            && floats_equal(self.z, other.z)
            && floats_equal(self.w, other.w)
    }
}

impl ops::Add for Tuple {
    type Output = Tuple;

    fn add(self, _rhs: Tuple) -> Tuple {
        Tuple::new(
            self.x + _rhs.x,
            self.y + _rhs.y,
            self.z + _rhs.z,
            self.w + _rhs.w,
        )
    }
}

impl ops::Sub for Tuple {
    type Output = Tuple;

    fn sub(self, _rhs: Tuple) -> Tuple {
        Tuple::new(
            self.x - _rhs.x,
            self.y - _rhs.y,
            self.z - _rhs.z,
            self.w - _rhs.w,
        )
    }
}

impl ops::Neg for Tuple {
    type Output = Tuple;

    fn neg(self) -> Tuple {
        Tuple::new(-self.x, -self.y, -self.z, -self.w)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tuple_with_w_1_is_point() {
        let tuple = Tuple::new(4.3, -4.2, 3.1, 1.0);
        assert_eq!(tuple.is_point(), true);
    }

    #[test]
    fn tuple_with_w_0_is_vector() {
        let tuple = Tuple::new(4.3, -4.2, 3.1, 0.0);
        assert_eq!(tuple.is_vector(), true);
    }

    #[test]
    fn point_creates_tuple_with_w_1() {
        let tuple = Tuple::point(4.0, -4.0, 3.0);
        assert_eq!(tuple.is_point(), true);
    }

    #[test]
    fn vector_creates_tuple_with_w_0() {
        let tuple = Tuple::vector(4.0, -4.0, 3.0);
        assert_eq!(tuple.is_vector(), true);
    }

    #[test]
    fn add_tuples() {
        let tuple1 = Tuple::new(3.0, -2.0, 5.0, 1.0);
        let tuple2 = Tuple::vector(-2.0, 3.0, 1.0);
        let result = Tuple::new(1.0, 1.0, 6.0, 1.0);
        assert_eq!(tuple1 + tuple2, result);
    }

    #[test]
    fn subtract_two_points() {
        let tuple1 = Tuple::point(3.0, 2.0, 1.0);
        let tuple2 = Tuple::point(5.0, 6.0, 7.0);
        let result = Tuple::vector(-2.0, -4.0, -6.0);
        assert_eq!(tuple1 - tuple2, result);
    }

    #[test]
    fn subtract_vector_from_point() {
        let tuple1 = Tuple::point(3.0, 2.0, 1.0);
        let tuple2 = Tuple::vector(5.0, 6.0, 7.0);
        let result = Tuple::point(-2.0, -4.0, -6.0);
        assert_eq!(tuple1 - tuple2, result);
    }

    #[test]
    fn subtract_two_vectors() {
        let tuple1 = Tuple::vector(3.0, 2.0, 1.0);
        let tuple2 = Tuple::vector(5.0, 6.0, 7.0);
        let result = Tuple::vector(-2.0, -4.0, -6.0);
        assert_eq!(tuple1 - tuple2, result);
    }

    #[test]
    fn subtract_vector_from_zero_vector() {
        let tuple1 = Tuple::vector(0.0, 0.0, 0.0);
        let tuple2 = Tuple::vector(1.0, -2.0, 3.0);
        let result = Tuple::vector(-1.0, 2.0, -3.0);
        assert_eq!(tuple1 - tuple2, result);
    }

    #[test]
    fn negate_tuple() {
        let tuple = Tuple::new(1.0, -2.0, 3.0, -4.0);
        let result = Tuple::new(-1.0, 2.0, -3.0, 4.0);
        assert_eq!(-tuple, result);
    }
}
