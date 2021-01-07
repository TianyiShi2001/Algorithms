//! # Resources
//!
//! - [Wikipedia](https://www.wikiwand.com/en/Elementary_matrix)

use super::Matrix;

impl Matrix {
    pub fn row_swapping_matrix(dim: usize, i0: usize, i1: usize) -> Self {
        let mut res = Self::identity(dim);
        res[i0][i0] = 0.;
        res[i1][i1] = 0.;
        res[i0][i1] = 1.;
        res[i1][i0] = 1.;
        res
    }
    pub fn row_swapping_matrix_inverse(dim: usize, i0: usize, i1: usize) -> Self {
        Self::row_swapping_matrix(dim, i0, i1)
    }
    pub fn inverse_row_swapping_matrix(&self) -> &Self {
        &self
    }
    pub fn row_multiplication_matrix(dim: usize, row_and_factors: &[(usize, f64)]) -> Self {
        let mut res = Self::identity(dim);
        for &(i, factor) in row_and_factors {
            res[i][i] = factor;
        }
        res
    }
    pub fn row_multiplication_matrix_inverse(dim: usize, row_and_factors: &[(usize, f64)]) -> Self {
        let mut res = Self::identity(dim);
        for &(i, factor) in row_and_factors {
            res[i][i] = 1. / factor;
        }
        res
    }
    pub fn inverse_row_multiplication_matrix(&self) -> Self {
        let mut res = self.clone();
        let dim = self.nrows();
        for i in 0..dim {
            let x = &mut res[i][i];
            *x = 1. / *x
        }
        res
    }
    pub fn row_addition_matrix(dim: usize, from: usize, params: &[(usize, f64)]) -> Self {
        let mut res = Self::identity(dim);
        for &(to, by) in params {
            res[to][from] = by;
        }
        res
    }
    pub fn row_addition_matrix_inverse(dim: usize, from: usize, params: &[(usize, f64)]) -> Self {
        let mut res = Self::identity(dim);
        for &(to, by) in params {
            res[to][from] = -by;
        }
        res
    }
    pub fn inverse_row_addition_matrix(&self) -> Self {
        let mut res = self.clone();
        let dim = self.nrows();
        for i in 0..dim {
            for j in 0..dim {
                if i != j {
                    let x = &mut res[i][j];
                    if *x != 0. {
                        *x = -*x;
                    }
                }
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::lazy_static;
    lazy_static! {
        static ref M: Matrix = Matrix(vec![
            vec![1., 2., 3., 4., 5.],
            vec![6., 7., 8., 9., 0.],
            vec![5., 8., 3., 5., 8.],
            vec![9., 3., 2., 5., 9.],
            vec![4., 7., 1., 3., 5.],
        ]);
    }

    #[test]
    fn row_swapping() {
        let tr = Matrix::row_swapping_matrix(5, 2, 4);
        let transformed = tr.clone() * M.clone();
        assert_eq!(
            transformed,
            Matrix(vec![
                vec![1., 2., 3., 4., 5.],
                vec![6., 7., 8., 9., 0.],
                vec![4., 7., 1., 3., 5.],
                vec![9., 3., 2., 5., 9.],
                vec![5., 8., 3., 5., 8.],
            ])
        );
        let original = tr * transformed;
        assert_eq!(original, *M);
    }

    #[test]
    #[rustfmt::skip]
    fn row_multiplication() {
        let tr = Matrix::row_multiplication_matrix(5, &[(2, 4.), (4, 6.)]);
        let transformed = tr.clone() * M.clone();
        assert_eq!(
            transformed,
            Matrix(vec![
                vec![1., 2., 3., 4., 5.],
                vec![6., 7., 8., 9., 0.],
                vec![5., 8., 3., 5., 8.].into_iter().map(|x|4. * x).collect(),
                vec![9., 3., 2., 5., 9.],
                vec![4., 7., 1., 3., 5.].into_iter().map(|x|6. * x).collect(),
            ])
        );
        let tr_inv = tr.inverse_row_multiplication_matrix();
        let original = tr_inv * transformed;
        assert_eq!(original, *M);
    }
    #[test]
    fn row_addition() {
        let tr = Matrix::row_addition_matrix(5, 1, &[(2, 4.), (4, 6.)]);
        let transformed = tr.clone() * M.clone();
        assert_eq!(
            transformed,
            Matrix(vec![
                vec![1.0, 2.0, 3.0, 4.0, 5.0],
                vec![6.0, 7.0, 8.0, 9.0, 0.0],
                vec![29., 36., 35., 41., 8.0],
                vec![9.0, 3.0, 2.0, 5.0, 9.0],
                vec![40., 49., 49., 57., 5.0],
            ])
        );
        let tr_inv = tr.inverse_row_addition_matrix();
        let original = tr_inv * transformed;
        assert_eq!(original, *M);
    }
}
