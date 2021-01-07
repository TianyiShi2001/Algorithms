//! # Resources
//!
//! -[Determinant of a Matrix (mathsisfun.com)](https://www.mathsisfun.com/algebra/matrix-determinant.html)

pub mod determinant;
pub mod eigen;
pub mod elementary;
pub mod gaussian_elimination;
pub mod inverse;
pub mod lu;

use std::ops::{Index, IndexMut, Mul, MulAssign};

#[derive(Debug, Clone)]
pub struct Matrix(pub Vec<Vec<f64>>);

impl Matrix {
    pub fn new(m: Vec<Vec<f64>>) -> Self {
        Self(m)
    }
    #[allow(clippy::needless_range_loop)]
    pub fn identity(dim: usize) -> Self {
        let mut res = vec![vec![0.; dim]; dim];
        for i in 0..dim {
            res[i][i] = 1.;
        }
        Self(res)
    }
    pub fn zero(dim: [usize; 2]) -> Self {
        Self(vec![vec![0.; dim[1]]; dim[0]])
    }
    pub fn nrows(&self) -> usize {
        self.0.len()
    }
    pub fn ncols(&self) -> usize {
        self.0[0].len()
    }
    pub fn is_square_matrix(&self) -> bool {
        self.nrows() == self.ncols()
    }
    pub fn transpose(&self) -> Self {
        Self(self.columns().map(|col| col.collect()).collect())
    }

    pub fn row(&self, i: usize) -> &[f64] {
        &self.0[i]
    }
    pub fn row_mut(&mut self, i: usize) -> &mut [f64] {
        &mut self.0[i]
    }
    pub fn rows(&self) -> impl Iterator<Item = &Vec<f64>> {
        self.0.iter()
    }
    pub fn rows_mut(&mut self) -> impl Iterator<Item = &mut Vec<f64>> {
        self.0.iter_mut()
    }
    pub fn column(&self, j: usize) -> impl Iterator<Item = f64> + '_ {
        (0..self.nrows()).map(move |i| self[[i, j]])
    }
    pub fn columns(&self) -> impl Iterator<Item = impl Iterator<Item = f64> + '_> + '_ {
        (0..self.ncols()).map(move |j| (0..self.nrows()).map(move |i| self[[i, j]]))
    }
    pub fn main_diagonal(&self) -> impl Iterator<Item = f64> + '_ {
        assert!(self.is_square_matrix());
        let dim = self.nrows();
        (0..dim).map(move |i| self[i][i])
    }
    pub fn multiply_matrix(&self, rhs: &Self) -> Self {
        assert_eq!(self.ncols(), rhs.nrows());
        let (m, n) = (self.nrows(), rhs.ncols());
        let mut res = Self::zero([m, n]);
        for i in 0..m {
            let row = self.row(i);
            for j in 0..n {
                res[i][j] = row.iter().zip(rhs.column(j)).map(|(x, y)| *x * y).sum();
            }
        }
        res
    }
    fn iter(&self) -> impl Iterator<Item = f64> + '_ {
        self.rows().flat_map(move |row| row.iter().cloned())
    }
    // pub fn main_diagonal_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut f64> {
    //     assert!(self.is_square_matrix());
    //     let dim = self.nrows();
    //     (0..dim).map(|i| &mut self[i][i])
    // }
    pub fn hstack(&mut self, rhs: &Self) {
        for (l, r) in self.rows_mut().zip(rhs.rows()) {
            l.extend_from_slice(r);
        }
    }
    pub fn hsplit(self, j: usize) -> (Self, Self) {
        let mut left_rows = Vec::with_capacity(self.nrows());
        let mut right_rows = Vec::with_capacity(self.nrows());
        for mut row in self.0.into_iter() {
            let right = row.drain(j..).collect();
            left_rows.push(row);
            right_rows.push(right);
        }
        (Matrix(left_rows), Matrix(right_rows))
    }
}

impl Index<[usize; 2]> for Matrix {
    type Output = f64;
    fn index(&self, index: [usize; 2]) -> &Self::Output {
        &self.0[index[0]][index[1]]
    }
}
impl IndexMut<[usize; 2]> for Matrix {
    fn index_mut(&mut self, index: [usize; 2]) -> &mut Self::Output {
        &mut self.0[index[0]][index[1]]
    }
}
impl Index<usize> for Matrix {
    type Output = [f64];
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
impl IndexMut<usize> for Matrix {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}
impl Mul<f64> for Matrix {
    type Output = Matrix;
    fn mul(self, rhs: f64) -> Self::Output {
        Self(
            self.0
                .iter()
                .map(|row| row.iter().map(|&x| rhs * x).collect())
                .collect(),
        )
    }
}

impl Mul<Matrix> for Matrix {
    type Output = Matrix;
    fn mul(self, rhs: Matrix) -> Self::Output {
        self.multiply_matrix(&rhs)
    }
}

impl MulAssign<Matrix> for Matrix {
    fn mul_assign(&mut self, rhs: Matrix) {
        *self = self.multiply_matrix(&rhs)
    }
}

use std::fmt;
impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.rows() {
            for &x in row {
                write!(f, "{:5.2} ", x)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Matrix) -> bool {
        self.iter()
            .zip(other.iter())
            .all(|(a, b)| (a - b).abs() < 0.00001)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Solution {
    Unique(Vec<f64>),
    Infinite(Vec<f64>, Vec<Vec<f64>>),
    None,
}

impl Solution {
    pub fn unwrap(self) -> Vec<f64> {
        match self {
            Self::Unique(res) => res,
            Self::Infinite(_, _) => panic!("Infinite solutions!"),
            Self::None => panic!("No solutions!"),
        }
    }
}

pub struct Solutions {
    pub augmented: Matrix,
    pub nrows: usize,
    pub ncols: usize,
    pub nullspace_cols: Vec<usize>,
}

impl Solutions {
    pub fn nullspace(&self) -> Vec<Vec<f64>> {
        self.nullspace_cols
            .iter()
            .rev()
            .map(|&j| {
                let mut ns_el = self.augmented.column(j).collect::<Vec<_>>();
                ns_el[j] = -1.;
                ns_el
            })
            .collect()
    }
    pub fn solutions_iter(&self) -> impl Iterator<Item = Solution> + '_ {
        println!("{}", self.augmented);
        println!("{:?}", self.nullspace_cols);
        println!("{:?}", (self.nrows, self.ncols));
        let nullspace = self.nullspace();
        (self.nrows..self.ncols).map(move |j| {
            let sol = self.augmented.column(j).collect();
            if self.nullspace_cols.is_empty() {
                Solution::Unique(sol)
            } else {
                for &j_ in &self.nullspace_cols {
                    if sol[j_] != 0. {
                        return Solution::None;
                    }
                }
                Solution::Infinite(sol, nullspace.clone())
            }
        })
    }
    pub fn solutions(&self) -> Vec<Solution> {
        self.solutions_iter().collect()
    }
    pub fn first(&self) -> Solution {
        self.solutions_iter().next().unwrap()
    }
    pub fn unwrap_first(&self) -> Vec<f64> {
        self.first().unwrap()
    }
    pub fn solutions_matrix(self) -> Option<Matrix> {
        if !self.nullspace_cols.is_empty() {
            None
        } else {
            Some(self.augmented.hsplit(self.nrows).1)
        }
    }
}

pub trait LinearSystemSolver {
    fn solve(augmented: Matrix) -> Solutions;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mul() {
        let m3x3 = Matrix::new(vec![vec![6., 1., 1.], vec![4., -2., 5.], vec![2., 8., 7.]]);
        assert_eq!(
            m3x3 * 3.,
            Matrix::new(vec![
                vec![18., 3., 3.],
                vec![12., -6., 15.],
                vec![6., 24., 21.]
            ])
        );
        let m2x3 = Matrix::new(vec![vec![6., 1., 1.], vec![4., -2., 5.]]);
        let m3x2 = Matrix::new(vec![vec![3., 2.], vec![0., -1.], vec![-3., 4.]]);
        assert_eq!(
            m2x3 * m3x2,
            Matrix::new(vec![vec![15., 15.], vec![-3., 30.]])
        )
    }

    #[test]
    #[rustfmt::skip]
    fn transpose() {
        let m = Matrix(vec![
            vec![1., 2., 3.],
            vec![4., 5., 6.],
        ]);
        let t = m.transpose();
        assert_eq!(t, Matrix(vec![
            vec![1., 4.],
            vec![2., 5.],
            vec![3., 6.],
        ]));
    }
}
