//! # Resources
//!
//! - [Linear Algebra 11q: Algorithm for Calculating the Inverse Matrix](https://www.youtube.com/watch?v=ZW5FBIyGzJk)
//! - [Linear Algebra 11r: First Explanation for the Inversion Algorithm](https://www.youtube.com/watch?v=CgkuBFPMkNA)
//! - [l](https://www.youtube.com/watch?v=sdO5UoSyrzM)

use super::{LinearSystemSolver, Matrix};

impl Matrix {
    pub fn try_inverse<S: LinearSystemSolver>(mut self) -> Option<Matrix> {
        assert!(self.is_square_matrix());
        let dim = self.nrows();
        self.hstack(&Self::identity(dim));
        S::solve(self).solutions_matrix()
    }
    pub fn try_inverse_gauss_jordan_elimination(mut self) -> Option<Matrix> {
        assert!(self.is_square_matrix());
        let dim = self.nrows();
        // from top to bottom (from left to right)
        let mut l = Self::identity(dim);
        for i in 0..dim {
            // if `matrix[i][i]` (which will become a pivot) is zero,
            // swap row `i` with a row where `matrix[i][i]` is not zero.
            if let Some(idx) = (i..dim).find(|&idx| self[[idx, i]] != 0.) {
                if idx != i {
                    self.swap_row(idx, i);
                    l = Self::row_swapping_matrix(dim, idx, i) * l;
                }
            } else {
                // if there isn't any row `i` with `m[i][i] != 0`, the matrix is not invertible
                return None;
            };

            let pivot = self[[i, i]];
            // scale the row by 1/pivot, so that the pivot becomes 1
            for coef in self.row_mut(i).iter_mut().skip(i) {
                *coef /= pivot;
            }
            l = Matrix::row_multiplication_matrix(dim, &[(i, 1. / pivot)]) * l;
            if i < dim {
                // subtract `m[curr_i][i] / pivot` * `m[i][j]` from `m[curr_i][j]` for each row below row `i`
                // to make `m[curr_i][i]` zero
                let params = (i + 1..dim)
                    .map(|curr_i| (curr_i, -self[[curr_i, i]]))
                    .collect::<Vec<_>>();
                for &(curr_i, factor) in &params {
                    for j in i..dim {
                        self[[curr_i, j]] += factor * self[[i, j]];
                    }
                }
                l = Self::row_addition_matrix(dim, i, &params) * l;
            }
        }

        // from right to left
        for i in (1..dim).rev() {
            let params = (0..i)
                .map(|curr_i| (curr_i, -self[[curr_i, i]]))
                .collect::<Vec<_>>();
            for &(curr_i, factor) in &params {
                for j in i..dim {
                    self[[curr_i, j]] += factor * self[[i, j]];
                }
            }
            l = Self::row_addition_matrix(dim, i, &params) * l;
        }
        Some(l)
    }
}

#[cfg(test)]
mod tests {
    use super::super::gaussian_elimination::GaussJordanElimination;
    use super::*;
    use lazy_static::lazy_static;
    #[rustfmt::skip]
    lazy_static! {
        static ref M: Matrix = Matrix(vec![
            vec![1., 2., 3. ],
            vec![4., 5., 6. ],
            vec![7., 8., 10.]]);
    }
    #[test]
    fn matrix_inverse() {
        let i = M.clone().try_inverse_gauss_jordan_elimination().unwrap();
        assert_eq!(M.multiply_matrix(&i), Matrix::identity(3));
        assert_eq!(i.multiply_matrix(&M), Matrix::identity(3));

        let i = M.clone().try_inverse::<GaussJordanElimination>().unwrap();
        assert_eq!(M.multiply_matrix(&i), Matrix::identity(3));
        assert_eq!(i.multiply_matrix(&M), Matrix::identity(3));
    }
}
