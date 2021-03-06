use std::ops;

use crate::utils::floats_equal;

#[derive(Debug, Copy, Clone)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Tuple {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Tuple { x, y, z, w }
    }

    pub fn point(x: f64, y: f64, z: f64) -> Self {
        Tuple { x, y, z, w: 1.0 }
    }

    pub fn vector(x: f64, y: f64, z: f64) -> Self {
        Tuple { x, y, z, w: 0.0 }
    }

    pub fn is_point(&self) -> bool {
        floats_equal(self.w, 1.0)
    }

    pub fn is_vector(&self) -> bool {
        floats_equal(self.w, 0.0)
    }

    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }

    pub fn normalized(&self) -> Self {
        *self / self.magnitude()
    }

    pub fn dot(&self, other: &Tuple) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    pub fn cross(&self, other: Tuple) -> Self {
        Tuple::vector(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn reflect(&self, normal: &Tuple) -> Self {
        self - &(normal * 2.0 * self.dot(normal))
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        floats_equal(self.x, other.x) && floats_equal(self.y, other.y) && floats_equal(self.z, other.z) && floats_equal(self.w, other.w)
    }
}

impl ops::Add for Tuple {
    type Output = Tuple;

    fn add(self, other: Tuple) -> Tuple {
        Tuple::new(self.x + other.x, self.y + other.y, self.z + other.z, self.w + other.w)
    }
}

impl ops::Add for &Tuple {
    type Output = Tuple;

    fn add(self, other: &Tuple) -> Tuple {
        Tuple::new(self.x + other.x, self.y + other.y, self.z + other.z, self.w + other.w)
    }
}

impl ops::Sub for Tuple {
    type Output = Tuple;

    fn sub(self, other: Tuple) -> Tuple {
        Tuple::new(self.x - other.x, self.y - other.y, self.z - other.z, self.w - other.w)
    }
}

impl ops::Sub for &Tuple {
    type Output = Tuple;

    fn sub(self, other: &Tuple) -> Tuple {
        Tuple::new(self.x - other.x, self.y - other.y, self.z - other.z, self.w - other.w)
    }
}

impl ops::Neg for Tuple {
    type Output = Tuple;

    fn neg(self) -> Tuple {
        Tuple::new(-self.x, -self.y, -self.z, -self.w)
    }
}

impl ops::Mul<f64> for Tuple {
    type Output = Tuple;

    fn mul(self, other: f64) -> Tuple {
        Tuple::new(self.x * other, self.y * other, self.z * other, self.w * other)
    }
}

impl ops::Mul<f64> for &Tuple {
    type Output = Tuple;

    fn mul(self, other: f64) -> Tuple {
        Tuple::new(self.x * other, self.y * other, self.z * other, self.w * other)
    }
}

impl ops::Div<f64> for Tuple {
    type Output = Tuple;

    fn div(self, other: f64) -> Tuple {
        Tuple::new(self.x / other, self.y / other, self.z / other, self.w / other)
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::SQRT_2;

    use super::*;

    #[test]
    fn tuple_with_w_1_is_point() {
        let tuple = Tuple::new(4.3, -4.2, 3.1, 1.0);

        assert!(tuple.is_point());
    }

    #[test]
    fn tuple_with_w_0_is_vector() {
        let tuple = Tuple::new(4.3, -4.2, 3.1, 0.0);

        assert!(tuple.is_vector());
    }

    #[test]
    fn point_creates_tuple_with_w_1() {
        let tuple = Tuple::point(4.0, -4.0, 3.0);

        assert!(tuple.is_point());
    }

    #[test]
    fn vector_creates_tuple_with_w_0() {
        let tuple = Tuple::vector(4.0, -4.0, 3.0);

        assert!(tuple.is_vector());
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

        assert!(floats_equal(tuple.magnitude(), result))
    }

    #[test]
    fn magnitue_of_vector_0_1_0() {
        let tuple = Tuple::vector(0.0, 1.0, 0.0);
        let result = 1.0;

        assert!(floats_equal(tuple.magnitude(), result))
    }

    #[test]
    fn magnitue_of_vector_0_0_12() {
        let tuple = Tuple::vector(0.0, 0.0, 1.0);
        let result = 1.0;

        assert!(floats_equal(tuple.magnitude(), result))
    }

    #[test]
    fn magnitue_of_vector_1_2_3() {
        let tuple = Tuple::vector(1.0, 2.0, 3.0);
        let result = f64::sqrt(14.0);

        assert!(floats_equal(tuple.magnitude(), result))
    }

    #[test]
    fn magnitue_of_vector_negative_1_2_3() {
        let tuple = Tuple::vector(-1.0, -2.0, -3.0);
        let result = 3.74165738677; // sqrt of 14

        assert!(floats_equal(tuple.magnitude(), result))
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

        assert!(equal);
    }

    #[test]
    fn dot_product_of_two_vectors() {
        let tuple1 = Tuple::vector(1.0, 2.0, 3.0);
        let tuple2 = Tuple::vector(2.0, 3.0, 4.0);
        let result = floats_equal(tuple1.dot(&tuple2), 20.0);

        assert!(result);
    }

    #[test]
    fn cross_product_of_two_vectors() {
        let tuple1 = Tuple::vector(1.0, 2.0, 3.0);
        let tuple2 = Tuple::vector(2.0, 3.0, 4.0);
        let result1 = Tuple::vector(-1.0, 2.0, -1.0);
        let result2 = Tuple::vector(1.0, -2.0, 1.0);

        assert_eq!(tuple1.cross(tuple2), result1);
        assert_eq!(tuple2.cross(tuple1), result2);
    }

    #[test]
    fn tuple_x_y_z_w_getters() {
        let tuple = Tuple::new(-0.5, 0.4, 1.7, 1.0);

        assert!(floats_equal(tuple.x, -0.5));
        assert!(floats_equal(tuple.y, 0.4));
        assert!(floats_equal(tuple.z, 1.7));
        assert!(floats_equal(tuple.w, 1.0));
    }

    #[test]
    fn reflect_vector_approaching_at_45_deg() {
        let v = Tuple::vector(1.0, -1.0, 0.0);
        let n = Tuple::vector(0.0, 1.0, 0.0);
        let r = v.reflect(&n);
        assert_eq!(r, Tuple::vector(1.0, 1.0, 0.0));
    }

    #[test]
    fn reflect_vector_off_slanted_surface() {
        let v = Tuple::vector(0.0, -1.0, 0.0);
        let n = Tuple::vector(SQRT_2 / 2.0, SQRT_2 / 2.0, 0.0);
        let r = v.reflect(&n);
        assert_eq!(r, Tuple::vector(1.0, 0.0, 0.0));
    }
}
