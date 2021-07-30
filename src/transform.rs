use crate::matrix::Matrix;

use crate::F;

pub fn translate(x: F, y: F, z: F) -> Matrix {
    #[rustfmt::skip]
    let v = vec![
        1.0, 0.0, 0.0, x,
        0.0, 1.0, 0.0, y,
        0.0, 0.0, 1.0, z,
        0.0, 0.0, 0.0, 1.0,
    ];

    Matrix::new(4, &v)
}

pub fn scale(x: F, y: F, z: F) -> Matrix {
    #[rustfmt::skip]
    let v = vec![
        x,   0.0, 0.0, 0.0,
        0.0,  y,  0.0, 0.0,
        0.0, 0.0,   z, 0.0,
        0.0, 0.0, 0.0, 1.0,
    ];

    Matrix::new(4, &v)
}

pub fn rotate_x(r: F) -> Matrix {
    #[rustfmt::skip]
    let v = vec![
        1.0, 0.0, 0.0, 0.0,
        0.0, r.cos(), -r.sin(), 0.0,
        0.0, r.sin(), r.cos(), 0.0,
        0.0, 0.0, 0.0, 1.0
    ];

    Matrix::new(4, &v)
}

pub fn rotate_y(r: F) -> Matrix {
    #[rustfmt::skip]
    let v = vec![
        r.cos(), 0.0, r.sin(), 0.0,
        0.0, 1.0, 0.0, 0.0,
        -r.sin(), 0.0, r.cos(), 0.0,
        0.0, 0.0, 0.0, 1.0
    ];

    Matrix::new(4, &v)
}

pub fn rotate_z(r: F) -> Matrix {
    #[rustfmt::skip]
    let v = vec![
        r.cos(), -r.sin(), 0.0, 0.0,
        r.sin(), r.cos(), 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0
    ];

    Matrix::new(4, &v)
}

pub fn shear(xy: F, xz: F, yx: F, yz: F, zx: F, zy: F) -> Matrix {
    #[rustfmt::skip]
    let v = vec![
        1.0, xy, xz, 0.0,
        yx, 1.0, yz, 0.0,
        zx, zy, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0
    ];

    Matrix::new(4, &v)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tuple::Tuple;
    use std::f64::consts::{PI, SQRT_2};

    #[test]
    fn multiply_by_translation_matrix() {
        let transform = translate(5.0, -3.0, 2.0);
        let point = Tuple::point(-3.0, 4.0, 5.0);

        assert_eq!(transform * point, Tuple::point(2.0, 1.0, 7.0));
    }

    #[test]
    fn multiply_by_inverse_oftranslation_matrix() {
        let transform = translate(5.0, -3.0, 2.0);
        let inverse = transform.inverse();
        let point = Tuple::point(-3.0, 4.0, 5.0);

        assert_eq!(inverse * point, Tuple::point(-8.0, 7.0, 3.0));
    }

    #[test]
    fn translation_does_not_affect_vectors() {
        let transform = translate(5.0, -3.0, 2.0);
        let vector = Tuple::vector(-3.0, 4.0, 5.0);

        assert_eq!(transform * vector, vector);
    }

    #[test]
    fn scale_point() {
        let transform = scale(2.0, 3.0, 4.0);
        let point = Tuple::point(-4.0, 6.0, 8.0);

        assert_eq!(transform * point, Tuple::point(-8.0, 18.0, 32.0));
    }

    #[test]
    fn scale_vector() {
        let transform = scale(2.0, 3.0, 4.0);
        let vector = Tuple::vector(-4.0, 6.0, 8.0);

        assert_eq!(transform * vector, Tuple::vector(-8.0, 18.0, 32.0));
    }

    #[test]
    fn reflect_point() {
        let transform = scale(-1.0, 1.0, 1.0);
        let point = Tuple::point(2.0, 3.0, 4.0);

        assert_eq!(transform * point, Tuple::point(-2.0, 3.0, 4.0));
    }

    #[test]
    fn rotating_point_around_x_axis() {
        let point = Tuple::point(0.0, 1.0, 0.0);
        let half_quarter = rotate_x(PI / 4.0);
        let full_quarter = rotate_x(PI / 2.0);

        assert_eq!(half_quarter * point, Tuple::point(0.0, SQRT_2 / 2.0, SQRT_2 / 2.0));
        assert_eq!(full_quarter * point, Tuple::point(0.0, 0.0, 1.0));
    }

    #[test]
    fn inverse_of_x_rotation_rotates_in_opposite_direction() {
        let point = Tuple::point(0.0, 1.0, 0.0);
        let half_quarter = rotate_x(PI / 4.0);
        let inverse = half_quarter.inverse();

        assert_eq!(inverse * point, Tuple::point(0.0, SQRT_2 / 2.0, -SQRT_2 / 2.0));
    }

    #[test]
    fn rotating_point_around_y_axis() {
        let point = Tuple::point(0.0, 0.0, 1.0);
        let half_quarter = rotate_y(PI / 4.0);
        let full_quarter = rotate_y(PI / 2.0);

        assert_eq!(half_quarter * point, Tuple::point(SQRT_2 / 2.0, 0.0, SQRT_2 / 2.0));
        assert_eq!(full_quarter * point, Tuple::point(1.0, 0.0, 0.0));
    }

    #[test]
    fn rotating_point_around_z_axis() {
        let point = Tuple::point(0.0, 1.0, 0.0);
        let half_quarter = rotate_z(PI / 4.0);
        let full_quarter = rotate_z(PI / 2.0);

        assert_eq!(half_quarter * point, Tuple::point(-SQRT_2 / 2.0, SQRT_2 / 2.0, 0.0));
        assert_eq!(full_quarter * point, Tuple::point(-1.0, 0.0, 0.0));
    }

    #[test]
    fn shearing_moves_x_in_proporiton_to_y() {
        let transform = shear(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let point = Tuple::point(2.0, 3.0, 4.0);

        assert_eq!(transform * point, Tuple::point(5.0, 3.0, 4.0));
    }

    #[test]
    fn shearing_moves_x_in_proportion_to_z() {
        let transform = shear(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let point = Tuple::point(2.0, 3.0, 4.0);

        assert_eq!(transform * point, Tuple::point(6.0, 3.0, 4.0));
    }

    #[test]
    fn shearing_moves_y_in_proportion_to_x() {
        let transform = shear(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let point = Tuple::point(2.0, 3.0, 4.0);

        assert_eq!(transform * point, Tuple::point(2.0, 5.0, 4.0));
    }

    #[test]
    fn shearing_moves_y_in_proportion_to_z() {
        let transform = shear(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let point = Tuple::point(2.0, 3.0, 4.0);

        assert_eq!(transform * point, Tuple::point(2.0, 7.0, 4.0));
    }

    #[test]
    fn shearing_moves_z_in_proportion_to_x() {
        let transform = shear(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let point = Tuple::point(2.0, 3.0, 4.0);

        assert_eq!(transform * point, Tuple::point(2.0, 3.0, 6.0));
    }

    #[test]
    fn shearing_moves_z_in_proportion_to_y() {
        let transform = shear(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let point = Tuple::point(2.0, 3.0, 4.0);

        assert_eq!(transform * point, Tuple::point(2.0, 3.0, 7.0));
    }

    #[test]
    fn individual_transformations_are_applied_in_sequence() {
        let p = Tuple::point(1.0, 0.0, 1.0);
        let a = rotate_x(PI / 2.0);
        let b = scale(5.0, 5.0, 5.0);
        let c = translate(10.0, 5.0, 7.0);

        let p2 = a * p;
        assert_eq!(p2, Tuple::point(1.0, -1.0, 0.0));

        let p3 = b * p2;
        assert_eq!(p3, Tuple::point(5.0, -5.0, 0.0));

        let p4 = c * p3;
        assert_eq!(p4, Tuple::point(15.0, 0.0, 7.0));
    }

    #[test]
    fn chained_transformations_must_be_applied_in_reverse_order() {
        let p = Tuple::point(1.0, 0.0, 1.0);
        let a = rotate_x(PI / 2.0);
        let b = scale(5.0, 5.0, 5.0);
        let c = translate(10.0, 5.0, 7.0);

        assert_eq!((c * b * a) * p, Tuple::point(15.0, 0.0, 7.0));
    }
}
