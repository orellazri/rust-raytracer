use crate::utils::floats_equal;
use crate::F;
use std::ops;

#[derive(Debug)]
pub struct Matrix {
    pub dim: usize,
    pub elems: Vec<F>,
}

impl Matrix {
    pub fn new(dim: usize, elems: Vec<F>) -> Self {
        Matrix { dim, elems }
    }

    pub fn at(&self, row: usize, col: usize) -> F {
        self.elems[row * self.dim + col]
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        if self.dim != other.dim {
            return false;
        }

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construct_2_2_matrix() {
        let vec: Vec<F> = vec![-3.0, 5.0, 1.0, -2.0];
        let matrix = Matrix::new(2, vec);
        assert!(floats_equal(matrix.at(0, 0), -3.0));
        assert!(floats_equal(matrix.at(0, 1), 5.0));
        assert!(floats_equal(matrix.at(1, 0), 1.0));
        assert!(floats_equal(matrix.at(1, 1), -2.0));
    }

    #[test]
    fn construct_3_3_matrix() {
        let vec: Vec<F> = vec![-3.0, 5.0, 0.0, 1.0, -2.0, -7.0, 0.0, 1.0, 1.0];
        let matrix = Matrix::new(3, vec);
        assert!(floats_equal(matrix.at(0, 0), -3.0));
        assert!(floats_equal(matrix.at(1, 1), -2.0));
        assert!(floats_equal(matrix.at(2, 2), 1.0));
    }

    #[test]
    fn construct_4_4_matrix() {
        let vec: Vec<F> = vec![
            1.0, 2.0, 3.0, 4.0, 5.5, 6.5, 7.5, 8.5, 9.0, 10.0, 11.0, 12.0, 13.5, 14.5, 15.5, 16.5,
            69.2,
        ];
        let matrix = Matrix::new(4, vec);
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
        let matrix1 = Matrix::new(
            4,
            vec![
                1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
            ],
        );
        let matrix2 = Matrix::new(
            4,
            vec![
                1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
            ],
        );
        assert!(matrix1 == matrix2);
    }

    #[test]
    fn matrix_equality_with_different_matrices() {
        let matrix1 = Matrix::new(
            4,
            vec![
                1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
            ],
        );
        let matrix2 = Matrix::new(
            4,
            vec![
                2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0,
            ],
        );
        assert!(matrix1 != matrix2);
    }
}
