use std::ops;

use crate::tuple::Tuple;
use crate::utils::floats_equal;
use crate::F;

#[derive(Debug)]
pub struct Matrix {
    pub dim: usize,
    pub elems: Vec<F>,
}

impl Matrix {
    pub fn new(dim: usize, elems: &[F]) -> Self {
        Matrix { dim, elems: elems.to_vec() }
    }

    pub fn identity() -> Self {
        Matrix {
            dim: 4,
            elems: vec![1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0],
        }
    }

    pub fn at(&self, row: usize, col: usize) -> F {
        self.elems[row * self.dim + col]
    }

    pub fn transpose(&self) -> Self {
        let mut elems: Vec<F> = Vec::with_capacity(self.dim * self.dim);

        for row in 0..self.dim {
            for col in 0..self.dim {
                elems.push(self.at(col, row));
            }
        }

        Matrix::new(self.dim, &elems)
    }

    pub fn det(&self) -> F {
        if self.dim == 2 {
            self.at(0, 0) * self.at(1, 1) - self.at(0, 1) * self.at(1, 0)
        } else {
            (0..self.dim).map(|col| self.at(0, col) * self.cofactor(0, col)).sum()
        }
    }

    pub fn submatrix(&self, row: usize, col: usize) -> Self {
        let mut elems: Vec<F> = Vec::with_capacity((self.dim - 1) * (self.dim - 1));

        for xrow in 0..self.dim {
            for xcol in 0..self.dim {
                if xrow != row && xcol != col {
                    elems.push(self.at(xrow, xcol));
                }
            }
        }

        Matrix::new(self.dim - 1, &elems)
    }

    pub fn minor(&self, row: usize, col: usize) -> F {
        self.submatrix(row, col).det()
    }

    pub fn cofactor(&self, row: usize, col: usize) -> F {
        if (row + col) % 2 == 0 {
            return self.minor(row, col);
        }

        -self.minor(row, col)
    }

    pub fn invertible(&self) -> bool {
        !floats_equal(self.det(), 0.0)
    }

    pub fn inverse(&self) -> Self {
        assert!(self.invertible());

        let mut v: Vec<F> = Vec::with_capacity(self.dim * self.dim);
        let det = self.det();

        for row in 0..self.dim {
            for col in 0..self.dim {
                v.push(self.cofactor(col, row) / det);
            }
        }

        Matrix::new(self.dim, &v)
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        assert!(self.dim == other.dim);

        for y in 0..self.dim {
            for x in 0..self.dim {
                if !floats_equal(self.at(x, y), other.at(x, y)) {
                    return false;
                }
            }
        }

        true
    }
}

impl ops::Mul for Matrix {
    type Output = Matrix;

    fn mul(self, other: Matrix) -> Matrix {
        assert!(self.dim == other.dim);

        let mut v: Vec<F> = vec![0.0; self.dim * self.dim];

        for row in 0..self.dim {
            for col in 0..self.dim {
                for x in 0..self.dim {
                    v[row * self.dim + col] += self.at(row, x) * other.at(x, col);
                }
            }
        }

        Matrix::new(self.dim, &v)
    }
}

impl ops::Mul<Tuple> for Matrix {
    type Output = Tuple;

    fn mul(self, other: Tuple) -> Tuple {
        Tuple::new(
            self.at(0, 0) * other.x + self.at(0, 1) * other.y + self.at(0, 2) * other.z + self.at(0, 3) * other.w,
            self.at(1, 0) * other.x + self.at(1, 1) * other.y + self.at(1, 2) * other.z + self.at(1, 3) * other.w,
            self.at(2, 0) * other.x + self.at(2, 1) * other.y + self.at(2, 2) * other.z + self.at(2, 3) * other.w,
            self.at(3, 0) * other.x + self.at(3, 1) * other.y + self.at(3, 2) * other.z + self.at(3, 3) * other.w,
        )
    }
}

impl ops::Mul<Tuple> for &Matrix {
    type Output = Tuple;

    fn mul(self, other: Tuple) -> Tuple {
        Tuple::new(
            self.at(0, 0) * other.x + self.at(0, 1) * other.y + self.at(0, 2) * other.z + self.at(0, 3) * other.w,
            self.at(1, 0) * other.x + self.at(1, 1) * other.y + self.at(1, 2) * other.z + self.at(1, 3) * other.w,
            self.at(2, 0) * other.x + self.at(2, 1) * other.y + self.at(2, 2) * other.z + self.at(2, 3) * other.w,
            self.at(3, 0) * other.x + self.at(3, 1) * other.y + self.at(3, 2) * other.z + self.at(3, 3) * other.w,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construct_2_2_matrix() {
        let matrix = Matrix::new(2, &[-3.0, 5.0, 1.0, -2.0]);

        assert!(floats_equal(matrix.at(0, 0), -3.0));
        assert!(floats_equal(matrix.at(0, 1), 5.0));
        assert!(floats_equal(matrix.at(1, 0), 1.0));
        assert!(floats_equal(matrix.at(1, 1), -2.0));
    }

    #[test]
    fn construct_3_3_matrix() {
        let matrix = Matrix::new(3, &[-3.0, 5.0, 0.0, 1.0, -2.0, -7.0, 0.0, 1.0, 1.0]);

        assert!(floats_equal(matrix.at(0, 0), -3.0));
        assert!(floats_equal(matrix.at(1, 1), -2.0));
        assert!(floats_equal(matrix.at(2, 2), 1.0));
    }

    #[test]
    fn construct_4_4_matrix() {
        let matrix = Matrix::new(
            4,
            &[
                1.0, 2.0, 3.0, 4.0, 5.5, 6.5, 7.5, 8.5, 9.0, 10.0, 11.0, 12.0, 13.5, 14.5, 15.5, 16.5, 69.2,
            ],
        );

        assert!(floats_equal(matrix.at(0, 0), 1.0));
        assert!(floats_equal(matrix.at(0, 3), 4.0));
        assert!(floats_equal(matrix.at(1, 0), 5.5));
        assert!(floats_equal(matrix.at(1, 2), 7.5));
        assert!(floats_equal(matrix.at(2, 2), 11.0));
        assert!(floats_equal(matrix.at(3, 0), 13.5));
        assert!(floats_equal(matrix.at(3, 2), 15.5));
    }

    #[test]
    fn matrix_equality_with_identical_matrices() {
        let matrix1 = Matrix::new(4, &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0]);
        let matrix2 = Matrix::new(4, &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0]);

        assert!(matrix1 == matrix2);
    }

    #[test]
    fn matrix_equality_with_different_matrices() {
        let matrix1 = Matrix::new(4, &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0]);
        let matrix2 = Matrix::new(4, &[2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0]);

        assert!(matrix1 != matrix2);
    }

    #[test]
    fn multiply_matrices() {
        let matrix1 = Matrix::new(4, &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0]);
        let matrix2 = Matrix::new(4, &[-2.0, 1.0, 2.0, 3.0, 3.0, 2.0, 1.0, -1.0, 4.0, 3.0, 6.0, 5.0, 1.0, 2.0, 7.0, 8.0]);
        let expected_result = Matrix::new(
            4,
            &[
                20.0, 22.0, 50.0, 48.0, 44.0, 54.0, 114.0, 108.0, 40.0, 58.0, 110.0, 102.0, 16.0, 26.0, 46.0, 42.0,
            ],
        );

        assert_eq!(matrix1 * matrix2, expected_result);
    }

    #[test]
    fn multiply_matrix_by_tuple() {
        let matrix = Matrix::new(4, &[1.0, 2.0, 3.0, 4.0, 2.0, 4.0, 4.0, 2.0, 8.0, 6.0, 4.0, 1.0, 0.0, 0.0, 0.0, 1.0]);
        let tuple = Tuple::new(1.0, 2.0, 3.0, 1.0);
        let expected_result = Tuple::new(18.0, 24.0, 33.0, 1.0);

        assert_eq!(matrix * tuple, expected_result);
    }

    #[test]
    fn multiply_matrix_by_identity_matrix() {
        let matrix1 = Matrix::new(4, &[0.0, 1.0, 2.0, 4.0, 1.0, 2.0, 4.0, 8.0, 2.0, 4.0, 8.0, 16.0, 4.0, 8.0, 16.0, 32.0]);
        let matrix2 = Matrix::new(4, &[1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0]);
        let expected_result = Matrix::new(4, &[0.0, 1.0, 2.0, 4.0, 1.0, 2.0, 4.0, 8.0, 2.0, 4.0, 8.0, 16.0, 4.0, 8.0, 16.0, 32.0]);

        assert_eq!(matrix1 * matrix2, expected_result);
    }

    #[test]
    fn multiply_identity_matrix_by_tuple() {
        let matrix = Matrix::new(4, &[1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0]);
        let tuple = Tuple::new(1.0, 2.0, 3.0, 4.0);
        let expected_result = Tuple::new(1.0, 2.0, 3.0, 4.0);

        assert_eq!(matrix * tuple, expected_result);
    }

    #[test]
    fn transpose_matrix() {
        let matrix = Matrix::new(4, &[0.0, 9.0, 3.0, 0.0, 9.0, 8.0, 0.0, 8.0, 1.0, 8.0, 5.0, 3.0, 0.0, 0.0, 5.0, 8.0]);
        let expected_result = Matrix::new(4, &[0.0, 9.0, 1.0, 0.0, 9.0, 8.0, 8.0, 0.0, 3.0, 0.0, 5.0, 5.0, 0.0, 8.0, 3.0, 8.0]);
        assert_eq!(matrix.transpose(), expected_result);
    }

    #[test]
    fn transpose_identity_matrix() {
        let matrix = Matrix::new(4, &[1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0]);

        assert_eq!(matrix.transpose(), matrix);
    }

    #[test]
    fn determinant_of_2_2_matrix() {
        let matrix = Matrix::new(2, &[1.0, 5.0, -3.0, 2.0]);

        assert!(floats_equal(matrix.det(), 17.0));
    }

    #[test]
    fn submatrix_of_3_3_matrix() {
        let matrix = Matrix::new(3, &[1.0, 5.0, 0.0, -3.0, 2.0, 7.0, 0.0, 6.0, -3.0]);
        let expected_result = Matrix::new(2, &[-3.0, 2.0, 0.0, 6.0]);

        assert_eq!(matrix.submatrix(0, 2), expected_result);
    }

    #[test]
    fn submatrix_of_4_4_matrix() {
        let matrix = Matrix::new(4, &[-6.0, 1.0, 1.0, 6.0, -8.0, 5.0, 8.0, 6.0, -1.0, 0.0, 8.0, 2.0, -7.0, 1.0, -1.0, 1.0]);
        let expected_result = Matrix::new(3, &[-6.0, 1.0, 6.0, -8.0, 8.0, 6.0, -7.0, -1.0, 1.0]);

        assert_eq!(matrix.submatrix(2, 1), expected_result);
    }

    #[test]
    fn minor_of_3_3_matrix() {
        let matrix = Matrix::new(3, &[3.0, 5.0, 0.0, 2.0, -1.0, -7.0, 6.0, -1.0, 5.0]);
        let submatrix = matrix.submatrix(1, 0);

        assert!(floats_equal(submatrix.det(), 25.0));
        assert!(floats_equal(matrix.minor(1, 0), 25.0));
    }

    #[test]
    fn cofactor_of_3_3_matrix() {
        let matrix = Matrix::new(3, &[3.0, 5.0, 0.0, 2.0, -1.0, -7.0, 6.0, -1.0, 5.0]);
        assert!(floats_equal(matrix.minor(0, 0), -12.0));
        assert!(floats_equal(matrix.cofactor(0, 0), -12.0));
        assert!(floats_equal(matrix.minor(1, 0), 25.0));
        assert!(floats_equal(matrix.cofactor(1, 0), -25.0));
    }

    #[test]
    fn determinant_of_3_3_matrix() {
        let matrix = Matrix::new(3, &[1.0, 2.0, 6.0, -5.0, 8.0, -4.0, 2.0, 6.0, 4.0]);

        assert!(floats_equal(matrix.cofactor(0, 0), 56.0));
        assert!(floats_equal(matrix.cofactor(0, 1), 12.0));
        assert!(floats_equal(matrix.cofactor(0, 2), -46.0));
        assert!(floats_equal(matrix.det(), -196.0));
    }

    #[test]
    fn determinant_of_4_4_matrix() {
        let matrix = Matrix::new(4, &[-2.0, -8.0, 3.0, 5.0, -3.0, 1.0, 7.0, 3.0, 1.0, 2.0, -9.0, 6.0, -6.0, 7.0, 7.0, -9.0]);

        assert!(floats_equal(matrix.cofactor(0, 0), 690.0));
        assert!(floats_equal(matrix.cofactor(0, 1), 447.0));
        assert!(floats_equal(matrix.cofactor(0, 2), 210.0));
        assert!(floats_equal(matrix.cofactor(0, 3), 51.0));
        assert!(floats_equal(matrix.det(), -4071.0));
    }

    #[test]
    fn matrix_is_invertible() {
        let matrix = Matrix::new(4, &[6.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 6.0, 4.0, -9.0, 3.0, -7.0, 9.0, 1.0, 7.0, -6.0]);

        assert!(floats_equal(matrix.det(), -2120.0));
        assert!(matrix.invertible());
    }

    #[test]
    fn matrix_is_not_invertible() {
        let matrix = Matrix::new(4, &[-4.0, 2.0, -2.0, -3.0, 9.0, 6.0, 2.0, 6.0, 0.0, -5.0, 1.0, -5.0, 0.0, 0.0, 0.0, 0.0]);

        assert!(floats_equal(matrix.det(), 0.0));
        assert!(!matrix.invertible());
    }

    #[test]
    fn inverse_of_matrix() {
        let matrix = Matrix::new(4, &[-5.0, 2.0, 6.0, -8.0, 1.0, -5.0, 1.0, 8.0, 7.0, 7.0, -6.0, -7.0, 1.0, -3.0, 7.0, 4.0]);
        let inverse = matrix.inverse();

        assert!(floats_equal(matrix.det(), 532.0));
        assert!(floats_equal(matrix.cofactor(2, 3), -160.0));
        assert!(floats_equal(inverse.at(3, 2), -160.0 / 532.0));
        assert!(floats_equal(matrix.cofactor(3, 2), 105.0));
        assert!(floats_equal(inverse.at(2, 3), 105.0 / 532.0));

        let expected_result = Matrix::new(
            4,
            &[
                0.21805, 0.45113, 0.24060, -0.04511, -0.80827, -1.45677, -0.44361, 0.52068, -0.07895, -0.22368, -0.05263, 0.19737, -0.52256,
                -0.81391, -0.30075, 0.30639,
            ],
        );

        assert_eq!(inverse, expected_result);
    }

    #[test]
    fn inverse_of_matrix_2() {
        let matrix = Matrix::new(4, &[8.0, -5.0, 9.0, 2.0, 7.0, 5.0, 6.0, 1.0, -6.0, 0.0, 9.0, 6.0, -3.0, 0.0, -9.0, -4.0]);
        let expected_result = Matrix::new(
            4,
            &[
                -0.15385, -0.15385, -0.28205, -0.53846, -0.07692, 0.12308, 0.02564, 0.03077, 0.35897, 0.35897, 0.43590, 0.92308, -0.69231, -0.69231,
                -0.76923, -1.92308,
            ],
        );

        assert_eq!(matrix.inverse(), expected_result);
    }

    #[test]
    fn inverse_of_matrix_3() {
        let matrix = Matrix::new(4, &[9.0, 3.0, 0.0, 9.0, -5.0, -2.0, -6.0, -3.0, -4.0, 9.0, 6.0, 4.0, -7.0, 6.0, 6.0, 2.0]);
        let expected_result = Matrix::new(
            4,
            &[
                -0.04074, -0.07778, 0.14444, -0.22222, -0.07778, 0.03333, 0.36667, -0.33333, -0.02901, -0.14630, -0.10926, 0.12963, 0.17778, 0.06667,
                -0.26667, 0.33333,
            ],
        );

        assert_eq!(matrix.inverse(), expected_result);
    }

    #[test]
    fn multiply_product_by_inverse() {
        // TODO: Maybe implement ops::Mul with reference to avoid moving matrix
        let matrix1 = Matrix::new(4, &[3.0, -9.0, 7.0, 3.0, 3.0, -8.0, 2.0, -9.0, -4.0, 4.0, 4.0, 1.0, -6.0, 5.0, -1.0, 1.0]);
        let matrix1b = Matrix::new(4, &[3.0, -9.0, 7.0, 3.0, 3.0, -8.0, 2.0, -9.0, -4.0, 4.0, 4.0, 1.0, -6.0, 5.0, -1.0, 1.0]);
        let matrix2 = Matrix::new(4, &[8.0, 2.0, 2.0, 2.0, 3.0, -1.0, 7.0, 0.0, 7.0, 0.0, 5.0, 4.0, 6.0, -2.0, 0.0, 5.0]);
        let matrix2b = Matrix::new(4, &[8.0, 2.0, 2.0, 2.0, 3.0, -1.0, 7.0, 0.0, 7.0, 0.0, 5.0, 4.0, 6.0, -2.0, 0.0, 5.0]);
        let matrix3 = matrix1 * matrix2;

        assert_eq!(matrix3 * (matrix2b.inverse()), matrix1b);
    }
}
