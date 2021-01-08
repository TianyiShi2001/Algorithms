use super::Matrix;
use std::ops::{Mul, MulAssign};

pub trait Vector {
    fn dot(&self, rhs: &Self) -> f64;
}

impl Vector for Vec<f64> {
    fn dot(&self, rhs: &Self) -> f64 {
        self.iter().zip(rhs.iter()).map(|(&a, &b)| a * b).sum()
    }
}

impl Matrix {
    /// Multiply a matrix by a column vector produces anoother column vector
    pub fn mul_col_vec(&self, rhs: &[f64]) -> Vec<f64> {
        assert_eq!(self.ncols(), rhs.len());
        self.rows()
            .map(|row| rhs.iter().zip(row).map(|(&a, &b)| a * b).sum())
            .collect()
    }

    /// Multiply a row vector by a matrix produces another row vector.
    pub fn mul_by_row_vec(&self, lhs: &[f64]) -> Vec<f64> {
        assert_eq!(lhs.len(), self.nrows());
        self.columns()
            .map(|column| lhs.iter().zip(column).map(|(&a, b)| a * b).sum())
            .collect()
    }
}

impl Mul<Matrix> for Vec<f64> {
    type Output = Vec<f64>;
    fn mul(self, rhs: Matrix) -> Self::Output {
        rhs.mul_by_row_vec(&self)
    }
}

impl MulAssign<Matrix> for Vec<f64> {
    fn mul_assign(&mut self, rhs: Matrix) {
        *self = rhs.mul_by_row_vec(&self)
    }
}

impl Mul<Vec<f64>> for Matrix {
    type Output = Vec<f64>;
    fn mul(self, rhs: Vec<f64>) -> Self::Output {
        assert_eq!(self.ncols(), rhs.len());
        self.mul_col_vec(&rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::matrix;
    #[test]
    #[allow(non_snake_case)]
    fn vector_arithmetics() {
        let A = matrix! {
            1, 2, 3;
            4, 5, 6;
        };
        let u = vec![1., 2.];
        assert_eq!(u * A.clone(), vec![9., 12., 15.]);

        let v = vec![1., 2., 3.];
        assert_eq!(A * v, vec![14., 32.]);
    }
}
