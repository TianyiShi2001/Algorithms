pub mod eigen;

use num_traits::Num;
use std::{
    collections::binary_heap::Iter,
    ops::{Index, IndexMut, Mul},
};

#[derive(Debug, PartialEq)]
pub struct Matrix<T: Num + Copy> {
    inner: Vec<T>,
    dim: [usize; 2],
}

impl<T: Num + Copy> Matrix<T> {
    pub fn new(dim: [usize; 2], m: Vec<T>) -> Self {
        assert_eq!(m.len(), dim[0] * dim[1]);
        Self { inner: m, dim }
    }
    pub fn identity(dim: usize) -> Self {
        let mut res = Self::new([dim, dim], vec![T::zero(); dim * dim]);
        for i in 0..dim {
            res[[i, i]] = T::one();
        }
        res
    }
    pub fn from_nested(dim: [usize; 2], m: Vec<Vec<T>>) -> Self {
        assert_eq!(m.len(), dim[0]);
        assert_eq!(m[0].len(), dim[1]);
        Self {
            inner: m.into_iter().flat_map(|row| row.into_iter()).collect(),
            dim,
        }
    }
    /// Compute the determinant of the matrix.
    pub fn determinant(&self) -> T {
        assert_eq!(self.dim[0], self.dim[1]);
        let dim = self.dim[0];
        if dim == 1 {
            self.inner[0]
        } else {
            let mut res = T::zero();
            let mut sign = T::one();
            for i in 0..dim {
                res = res
                        // first row, column `i`
                        + self.inner[i] * sign *
                            // matrix formed by numbers that are not in column `i`
                            Matrix::new([dim - 1, dim - 1],
                            (dim..dim * dim)
                                .filter(|&j| j % dim != i)
                                .map(|j| self.inner[j])
                                .collect::<Vec<_>>(), ).determinant();
                // invert the sign
                sign = T::zero() - sign;
            }
            res
        }
    }
    pub fn row(&self, i: usize) -> &[T] {
        let start = i * self.dim[1];
        let end = start + self.dim[1];
        &self.inner[start..end]
    }
    pub fn rows(&self) -> impl Iterator<Item = &[T]> {
        self.inner.chunks_exact(self.dim[1])
    }
    pub fn column(&self, j: usize) -> impl Iterator<Item = T> + '_ {
        (0..self.dim[0]).map(move |i| self[[i, j]])
    }
    pub fn columns(&self) -> impl Iterator<Item = impl Iterator<Item = T> + '_> + '_ {
        (0..self.dim[1]).map(move |j| (0..self.dim[0]).map(move |i| self[[i, j]]))
    }
}

impl<T: Num + Copy> Index<[usize; 2]> for Matrix<T> {
    type Output = T;
    fn index(&self, index: [usize; 2]) -> &Self::Output {
        &self.inner[index[0] * self.dim[1] + index[1]]
    }
}
impl<T: Num + Copy> IndexMut<[usize; 2]> for Matrix<T> {
    fn index_mut(&mut self, index: [usize; 2]) -> &mut Self::Output {
        &mut self.inner[index[0] * self.dim[1] + index[1]]
    }
}
impl<T: Num + Copy> Mul<T> for Matrix<T> {
    type Output = Matrix<T>;
    fn mul(self, rhs: T) -> Self::Output {
        Self {
            inner: self.inner.into_iter().map(|x| rhs * x).collect(),
            dim: self.dim,
        }
    }
}

impl<T: Num + Copy + std::iter::Sum> Mul<Matrix<T>> for Matrix<T> {
    type Output = Matrix<T>;
    fn mul(self, rhs: Matrix<T>) -> Self::Output {
        assert_eq!(self.dim[1], rhs.dim[0]);
        let m = self.dim[0];
        let n = rhs.dim[1];
        let mut res = vec![T::zero(); m * n];
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
    fn square_matrix() {
        let m = Matrix::from_nested(
            [3, 3],
            vec![
                vec![b'a', b'b', b'c'],
                vec![b'd', b'e', b'f'],
                vec![b'g', b'h', b'i'],
            ],
        );
        assert_eq!(m[[1, 1]], b'e');
        assert_eq!(m[[1, 2]], b'f');
    }

    #[test]
    fn determinant() {
        let m2x2 = Matrix::from_nested([2, 2], vec![vec![1, 2], vec![3, 4]]);
        assert_eq!(m2x2.determinant(), -2);
        let m3x3 = Matrix::from_nested([3, 3], vec![vec![6, 1, 1], vec![4, -2, 5], vec![2, 8, 7]]);
        assert_eq!(m3x3.determinant(), -306);
    }

    #[test]
    fn mul() {
        let m3x3 = Matrix::from_nested([3, 3], vec![vec![6, 1, 1], vec![4, -2, 5], vec![2, 8, 7]]);
        assert_eq!(
            m3x3 * 3,
            Matrix::from_nested(
                [3, 3],
                vec![vec![18, 3, 3], vec![12, -6, 15], vec![6, 24, 21]]
            )
        );
        let m2x3 = Matrix::from_nested([2, 3], vec![vec![6, 1, 1], vec![4, -2, 5]]);
        let m3x2 = Matrix::from_nested([3, 2], vec![vec![3, 2], vec![0, -1], vec![-3, 4]]);
        assert_eq!(m2x3 * m3x2, Matrix::new([2, 2], vec![15, 15, -3, 30]))
    }
}
