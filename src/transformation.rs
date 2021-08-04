use crate::{matrix::Matrix, tuple::Tuple};

pub fn translation(x: f64, y: f64, z: f64) -> Matrix {
    #[rustfmt::skip]
    let v = vec![
        1.0, 0.0, 0.0, x,
        0.0, 1.0, 0.0, y,
        0.0, 0.0, 1.0, z,
        0.0, 0.0, 0.0, 1.0,
    ];

    Matrix::new(4, &v)
}

pub fn scaling(x: f64, y: f64, z: f64) -> Matrix {
    #[rustfmt::skip]
    let v = vec![
        x,   0.0, 0.0, 0.0,
        0.0,  y,  0.0, 0.0,
        0.0, 0.0,   z, 0.0,
        0.0, 0.0, 0.0, 1.0,
    ];

    Matrix::new(4, &v)
}

pub fn rotation_x(r: f64) -> Matrix {
    #[rustfmt::skip]
    let v = vec![
        1.0, 0.0, 0.0, 0.0,
        0.0, r.cos(), -r.sin(), 0.0,
        0.0, r.sin(), r.cos(), 0.0,
        0.0, 0.0, 0.0, 1.0
    ];

    Matrix::new(4, &v)
}

pub fn rotation_y(r: f64) -> Matrix {
    #[rustfmt::skip]
    let v = vec![
        r.cos(), 0.0, r.sin(), 0.0,
        0.0, 1.0, 0.0, 0.0,
        -r.sin(), 0.0, r.cos(), 0.0,
        0.0, 0.0, 0.0, 1.0
    ];

    Matrix::new(4, &v)
}

pub fn rotation_z(r: f64) -> Matrix {
    #[rustfmt::skip]
    let v = vec![
        r.cos(), -r.sin(), 0.0, 0.0,
        r.sin(), r.cos(), 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0
    ];

    Matrix::new(4, &v)
}

pub fn shearing(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Matrix {
    #[rustfmt::skip]
    let v = vec![
        1.0, xy, xz, 0.0,
        yx, 1.0, yz, 0.0,
        zx, zy, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0
    ];

    Matrix::new(4, &v)
}

pub fn view_transform(from: Tuple, to: Tuple, up: Tuple) -> Matrix {
    let forward = (to - from).normalized();
    let left = forward.cross(up.normalized());
    let true_up = left.cross(forward);

    #[rustfmt::skip]
    let v = vec![
        left.x,     left.y,     left.z,     0.0,
        true_up.x,  true_up.y,  true_up.z,  0.0,
        -forward.x, -forward.y, -forward.z, 0.0,
        0.0,        0.0,        0.0,        1.0
    ];
    let orientation = Matrix::new(4, &v);

    orientation * translation(-from.x, -from.y, -from.z)
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::f64::consts::{PI, SQRT_2};

    #[test]
    fn multiply_by_translation_matrix() {
        let transform = translation(5.0, -3.0, 2.0);
        let point = Tuple::point(-3.0, 4.0, 5.0);

        assert_eq!(transform * point, Tuple::point(2.0, 1.0, 7.0));
    }

    #[test]
    fn multiply_by_inverse_oftranslation_matrix() {
        let transform = translation(5.0, -3.0, 2.0);
        let inverse = transform.inverse();
        let point = Tuple::point(-3.0, 4.0, 5.0);

        assert_eq!(inverse * point, Tuple::point(-8.0, 7.0, 3.0));
    }

    #[test]
    fn translation_does_not_affect_vectors() {
        let transform = translation(5.0, -3.0, 2.0);
        let vector = Tuple::vector(-3.0, 4.0, 5.0);

        assert_eq!(transform * vector, vector);
    }

    #[test]
    fn scale_point() {
        let transform = scaling(2.0, 3.0, 4.0);
        let point = Tuple::point(-4.0, 6.0, 8.0);

        assert_eq!(transform * point, Tuple::point(-8.0, 18.0, 32.0));
    }

    #[test]
    fn scale_vector() {
        let transform = scaling(2.0, 3.0, 4.0);
        let vector = Tuple::vector(-4.0, 6.0, 8.0);

        assert_eq!(transform * vector, Tuple::vector(-8.0, 18.0, 32.0));
    }

    #[test]
    fn reflect_point() {
        let transform = scaling(-1.0, 1.0, 1.0);
        let point = Tuple::point(2.0, 3.0, 4.0);

        assert_eq!(transform * point, Tuple::point(-2.0, 3.0, 4.0));
    }

    #[test]
    fn rotating_point_around_x_axis() {
        let point = Tuple::point(0.0, 1.0, 0.0);
        let half_quarter = rotation_x(PI / 4.0);
        let full_quarter = rotation_x(PI / 2.0);

        assert_eq!(half_quarter * point, Tuple::point(0.0, SQRT_2 / 2.0, SQRT_2 / 2.0));
        assert_eq!(full_quarter * point, Tuple::point(0.0, 0.0, 1.0));
    }

    #[test]
    fn inverse_of_x_rotation_rotates_in_opposite_direction() {
        let point = Tuple::point(0.0, 1.0, 0.0);
        let half_quarter = rotation_x(PI / 4.0);
        let inverse = half_quarter.inverse();

        assert_eq!(inverse * point, Tuple::point(0.0, SQRT_2 / 2.0, -SQRT_2 / 2.0));
    }

    #[test]
    fn rotating_point_around_y_axis() {
        let point = Tuple::point(0.0, 0.0, 1.0);
        let half_quarter = rotation_y(PI / 4.0);
        let full_quarter = rotation_y(PI / 2.0);

        assert_eq!(half_quarter * point, Tuple::point(SQRT_2 / 2.0, 0.0, SQRT_2 / 2.0));
        assert_eq!(full_quarter * point, Tuple::point(1.0, 0.0, 0.0));
    }

    #[test]
    fn rotating_point_around_z_axis() {
        let point = Tuple::point(0.0, 1.0, 0.0);
        let half_quarter = rotation_z(PI / 4.0);
        let full_quarter = rotation_z(PI / 2.0);

        assert_eq!(half_quarter * point, Tuple::point(-SQRT_2 / 2.0, SQRT_2 / 2.0, 0.0));
        assert_eq!(full_quarter * point, Tuple::point(-1.0, 0.0, 0.0));
    }

    #[test]
    fn shearing_moves_x_in_proporiton_to_y() {
        let transform = shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let point = Tuple::point(2.0, 3.0, 4.0);

        assert_eq!(transform * point, Tuple::point(5.0, 3.0, 4.0));
    }

    #[test]
    fn shearing_moves_x_in_proportion_to_z() {
        let transform = shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let point = Tuple::point(2.0, 3.0, 4.0);

        assert_eq!(transform * point, Tuple::point(6.0, 3.0, 4.0));
    }

    #[test]
    fn shearing_moves_y_in_proportion_to_x() {
        let transform = shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let point = Tuple::point(2.0, 3.0, 4.0);

        assert_eq!(transform * point, Tuple::point(2.0, 5.0, 4.0));
    }

    #[test]
    fn shearing_moves_y_in_proportion_to_z() {
        let transform = shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let point = Tuple::point(2.0, 3.0, 4.0);

        assert_eq!(transform * point, Tuple::point(2.0, 7.0, 4.0));
    }

    #[test]
    fn shearing_moves_z_in_proportion_to_x() {
        let transform = shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let point = Tuple::point(2.0, 3.0, 4.0);

        assert_eq!(transform * point, Tuple::point(2.0, 3.0, 6.0));
    }

    #[test]
    fn shearing_moves_z_in_proportion_to_y() {
        let transform = shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let point = Tuple::point(2.0, 3.0, 4.0);

        assert_eq!(transform * point, Tuple::point(2.0, 3.0, 7.0));
    }

    #[test]
    fn individual_transformations_are_applied_in_sequence() {
        let p = Tuple::point(1.0, 0.0, 1.0);
        let a = rotation_x(PI / 2.0);
        let b = scaling(5.0, 5.0, 5.0);
        let c = translation(10.0, 5.0, 7.0);

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
        let a = rotation_x(PI / 2.0);
        let b = scaling(5.0, 5.0, 5.0);
        let c = translation(10.0, 5.0, 7.0);

        assert_eq!((c * b * a) * p, Tuple::point(15.0, 0.0, 7.0));
    }

    #[test]
    fn transformation_matrix_for_default_orientation() {
        let from = Tuple::point(0.0, 0.0, 0.0);
        let to = Tuple::point(0.0, 0.0, -1.0);
        let up = Tuple::vector(0.0, 1.0, 0.0);
        let t = view_transform(from, to, up);

        assert_eq!(t, Matrix::identity());
    }

    #[test]
    fn view_transformation_matrix_looking_in_positive_z_direction() {
        let from = Tuple::point(0.0, 0.0, 0.0);
        let to = Tuple::point(0.0, 0.0, 1.0);
        let up = Tuple::vector(0.0, 1.0, 0.0);
        let t = view_transform(from, to, up);

        assert_eq!(t, scaling(-1.0, 1.0, -1.0));
    }

    #[test]
    fn view_transformation_moves_the_world() {
        let from = Tuple::point(0.0, 0.0, 8.0);
        let to = Tuple::point(0.0, 0.0, 0.0);
        let up = Tuple::vector(0.0, 1.0, 0.0);
        let t = view_transform(from, to, up);

        assert_eq!(t, translation(0.0, 0.0, -8.0));
    }

    #[test]
    fn view_transformation_arbitrary() {
        let from = Tuple::point(1.0, 3.0, 2.0);
        let to = Tuple::point(4.0, -2.0, 8.0);
        let up = Tuple::vector(1.0, 1.0, 0.0);
        let t = view_transform(from, to, up);
        let expected_result = vec![
            -0.50709, 0.50709, 0.67612, -2.36643, 0.76772, 0.60609, 0.12122, -2.82843, -0.35857, 0.59761, -0.71714, 0.00000, 0.00000, 0.00000,
            0.00000, 1.00000,
        ];

        assert_eq!(t, Matrix::new(4, &expected_result));
    }
}
