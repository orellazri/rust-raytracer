use std::ops;

use crate::float::floats_equal;

#[derive(Debug, Copy, Clone)]
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

    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }

    pub fn normalized(&self) -> Tuple {
        *self / self.magnitude()
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

    fn add(self, rhs: Tuple) -> Tuple {
        Tuple::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
            self.w + rhs.w,
        )
    }
}

impl ops::Sub for Tuple {
    type Output = Tuple;

    fn sub(self, rhs: Tuple) -> Tuple {
        Tuple::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z,
            self.w - rhs.w,
        )
    }
}

impl ops::Neg for Tuple {
    type Output = Tuple;

    fn neg(self) -> Tuple {
        Tuple::new(-self.x, -self.y, -self.z, -self.w)
    }
}

impl ops::Mul<f32> for Tuple {
    type Output = Tuple;

    fn mul(self, rhs: f32) -> Tuple {
        Tuple::new(self.x * rhs, self.y * rhs, self.z * rhs, self.w * rhs)
    }
}

impl ops::Div<f32> for Tuple {
    type Output = Tuple;

    fn div(self, rhs: f32) -> Tuple {
        Tuple::new(self.x / rhs, self.y / rhs, self.z / rhs, self.w / rhs)
    }
}

// Dot product
impl ops::Mul<Tuple> for Tuple {
    type Output = f32;

    fn mul(self, rhs: Tuple) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
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

    #[test]
    fn multiply_tuple_by_scalar() {
        let tuple = Tuple::new(1.0, -2.0, 3.0, -4.0);
        let result = Tuple::new(3.5, -7.0, 10.5, -14.0);
        assert_eq!(tuple * 3.5, result);
    }

    #[test]
    fn multiply_tuple_by_fraction() {
        let tuple = Tuple::new(1.0, -2.0, 3.0, -4.0);
        let result = Tuple::new(0.5, -1.0, 1.5, -2.0);
        assert_eq!(tuple * 0.5, result);
    }

    #[test]
    fn divide_tuple_by_scalar() {
        let tuple = Tuple::new(1.0, -2.0, 3.0, -4.0);
        let result = Tuple::new(0.5, -1.0, 1.5, -2.0);
        assert_eq!(tuple / 2.0, result);
    }

    #[test]
    fn magnitue_of_vector_1_0_0() {
        let tuple = Tuple::vector(1.0, 0.0, 0.0);
        let result = 1.0;
        assert_eq!(tuple.magnitude(), result);
    }

    #[test]
    fn magnitue_of_vector_0_1_0() {
        let tuple = Tuple::vector(0.0, 1.0, 0.0);
        let result = 1.0;
        assert_eq!(tuple.magnitude(), result);
    }

    #[test]
    fn magnitue_of_vector_0_0_12() {
        let tuple = Tuple::vector(0.0, 0.0, 1.0);
        let result = 1.0;
        assert_eq!(tuple.magnitude(), result);
    }

    #[test]
    fn magnitue_of_vector_1_2_3() {
        let tuple = Tuple::vector(1.0, 2.0, 3.0);
        let result = (14.0 as f32).sqrt();
        assert_eq!(tuple.magnitude(), result);
    }

    #[test]
    fn magnitue_of_vector_negative_1_2_3() {
        let tuple = Tuple::vector(-1.0, -2.0, -3.0);
        let result = 3.74165738677; // sqrt of 14
        assert_eq!(tuple.magnitude(), result);
    }

    #[test]
    fn normalize_vector_4_0_0() {
        let tuple = Tuple::vector(4.0, 0.0, 0.0);
        let result = Tuple::vector(1.0, 0.0, 0.0);
        assert_eq!(tuple.normalized(), result);
    }

    #[test]
    fn normalize_vector_1_2_3() {
        let tuple = Tuple::vector(1.0, 2.0, 3.0);
        let sqrt_14 = 3.74165738677;
        let result = Tuple::vector(1.0 / sqrt_14, 2.0 / sqrt_14, 3.0 / sqrt_14);
        assert_eq!(tuple.normalized(), result);
    }

    #[test]
    fn magnitude_of_normalized_vector() {
        let tuple = Tuple::vector(1.0, 2.0, 3.0);
        let equal = floats_equal(tuple.normalized().magnitude(), 1.0);
        assert_eq!(equal, true);
    }

    #[test]
    fn dot_product_of_two_vectors() {
        let tuple1 = Tuple::vector(1.0, 2.0, 3.0);
        let tuple2 = Tuple::vector(2.0, 3.0, 4.0);
        let result = floats_equal(tuple1 * tuple2, 20.0);
        assert_eq!(result, true);
    }
}
