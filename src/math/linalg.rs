//! # Resources
//!
//! -[Determinant of a Matrix (mathsisfun.com)](https://www.mathsisfun.com/algebra/matrix-determinant.html)

pub mod eigen;
pub mod gaussian_elimination;

use std::ops::{Index, IndexMut, Mul};

#[derive(Debug, PartialEq)]
pub struct Matrix {
    inner: Vec<f64>,
    dim: [usize; 2],
}

impl Matrix {
    pub fn new(dim: [usize; 2], m: Vec<f64>) -> Self {
        assert_eq!(m.len(), dim[0] * dim[1]);
        Self { inner: m, dim }
    }
    pub fn identity(dim: usize) -> Self {
        let mut res = Self::new([dim, dim], vec![0.; dim * dim]);
        for i in 0..dim {
            res[[i, i]] = 1.;
        }
        res
    }
    pub fn from_nested(dim: [usize; 2], m: Vec<Vec<f64>>) -> Self {
        assert_eq!(m.len(), dim[0]);
        assert_eq!(m[0].len(), dim[1]);
        Self {
            inner: m.into_iter().flat_map(|row| row.into_iter()).collect(),
            dim,
        }
    }
    pub fn is_square_matrix(&self) -> bool {
        self.dim[0] == self.dim[1]
    }
    /// Compute the determinant of the matrix.
    pub fn determinant(&self) -> f64 {
        assert!(self.is_square_matrix());
        let dim = self.dim[0];
        if dim == 1 {
            self.inner[0]
        } else {
            let mut res = 0.;
            let mut sign = 1.;
            for i in 0..dim {
                res +=
                        // first row, column `i`
                         self.inner[i] * sign *
                            // matrix formed by numbers that are not in column `i`
                            Matrix::new([dim - 1, dim - 1],
                            (dim..dim * dim)
                                .filter(|&j| j % dim != i)
                                .map(|j| self.inner[j])
                                .collect::<Vec<_>>(), ).determinant();
                // invert the sign
                sign = -sign;
            }
            res
        }
    }
    pub fn row(&self, i: usize) -> &[f64] {
        let start = i * self.dim[1];
        let end = start + self.dim[1];
        &self.inner[start..end]
    }
    pub fn row_mut(&mut self, i: usize) -> &mut [f64] {
        let start = i * self.dim[1];
        let end = start + self.dim[1];
        &mut self.inner[start..end]
    }
    pub fn rows(&self) -> impl Iterator<Item = &[f64]> {
        self.inner.chunks_exact(self.dim[1])
    }
    pub fn column(&self, j: usize) -> impl Iterator<Item = f64> + '_ {
        (0..self.dim[0]).map(move |i| self[[i, j]])
    }
    pub fn columns(&self) -> impl Iterator<Item = impl Iterator<Item = f64> + '_> + '_ {
        (0..self.dim[1]).map(move |j| (0..self.dim[0]).map(move |i| self[[i, j]]))
    }
}

impl Index<[usize; 2]> for Matrix {
    type Output = f64;
    fn index(&self, index: [usize; 2]) -> &Self::Output {
        &self.inner[index[0] * self.dim[1] + index[1]]
    }
}
impl IndexMut<[usize; 2]> for Matrix {
    fn index_mut(&mut self, index: [usize; 2]) -> &mut Self::Output {
        &mut self.inner[index[0] * self.dim[1] + index[1]]
    }
}
impl Mul<f64> for Matrix {
    type Output = Matrix;
    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            inner: self.inner.into_iter().map(|x| rhs * x).collect(),
            dim: self.dim,
        }
    }
}

impl Mul<Matrix> for Matrix {
    type Output = Matrix;
    fn mul(self, rhs: Matrix) -> Self::Output {
        assert_eq!(self.dim[1], rhs.dim[0]);
        let m = self.dim[0];
        let n = rhs.dim[1];
        let mut res = vec![0.; m * n];
        let mut idx = 0;
        for i in 0..m {
            let row = self.row(i);
            for j in 0..n {
                res[idx] = row.iter().zip(rhs.column(j)).map(|(x, y)| *x * y).sum();
                idx += 1;
            }
        }
        Self {
            inner: res,
            dim: [m, n],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn determinant() {
        let m2x2 = Matrix::from_nested([2, 2], vec![vec![1., 2.], vec![3., 4.]]);
        assert_eq!(m2x2.determinant(), -2.);
        let m3x3 = Matrix::from_nested(
            [3, 3],
            vec![vec![6., 1., 1.], vec![4., -2., 5.], vec![2., 8., 7.]],
        );
        assert_eq!(m3x3.determinant(), -306.);
    }

    #[test]
    fn mul() {
        let m3x3 = Matrix::from_nested(
            [3, 3],
            vec![vec![6., 1., 1.], vec![4., -2., 5.], vec![2., 8., 7.]],
        );
        assert_eq!(
            m3x3 * 3.,
            Matrix::from_nested(
                [3, 3],
                vec![vec![18., 3., 3.], vec![12., -6., 15.], vec![6., 24., 21.]]
            )
        );
        let m2x3 = Matrix::from_nested([2, 3], vec![vec![6., 1., 1.], vec![4., -2., 5.]]);
        let m3x2 = Matrix::from_nested([3, 2], vec![vec![3., 2.], vec![0., -1.], vec![-3., 4.]]);
        assert_eq!(m2x3 * m3x2, Matrix::new([2, 2], vec![15., 15., -3., 30.]))
    }
}
