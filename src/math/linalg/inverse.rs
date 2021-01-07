//! # Resources
//!
//! - [Linear Algebra 11q: Algorithm for Calculating the Inverse Matrix](https://www.youtube.com/watch?v=ZW5FBIyGzJk)
//! - [Linear Algebra 11r: First Explanation for the Inversion Algorithm](https://www.youtube.com/watch?v=CgkuBFPMkNA)
//! - [l](https://www.youtube.com/watch?v=sdO5UoSyrzM)

use super::{LinearSystemSolver, Matrix, Solution};

impl Matrix {
    fn try_inverse<S: LinearSystemSolver>(mut self) -> Option<Matrix> {
        assert!(self.is_square_matrix());
        let dim = self.nrows();
        let mut sol = Self::identity(dim);
        if S::solve_multiple(&mut self, &mut sol)
            .iter()
            .all(|s| matches!(s, Solution::Unique(_)))
        {
            Some(sol)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::gaussian_elimination::GaussJordanElimination;
    use super::*;
    #[test]
    fn matrix_inverse() {
        let m = Matrix::new(vec![vec![1., 2., 3.], vec![4., 5., 6.], vec![7., 8., 10.]]);
        let i = m.clone().try_inverse::<GaussJordanElimination>().unwrap();
        assert_eq!(m.clone() * i.clone(), Matrix::identity(3));
        assert_eq!(i * m, Matrix::identity(3));
    }
}
