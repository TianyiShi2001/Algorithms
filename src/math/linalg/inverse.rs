//! # Resources
//!
//! - [Linear Algebra 11q: Algorithm for Calculating the Inverse Matrix](https://www.youtube.com/watch?v=ZW5FBIyGzJk)
//! - [Linear Algebra 11r: First Explanation for the Inversion Algorithm](https://www.youtube.com/watch?v=CgkuBFPMkNA)
//! - [l](https://www.youtube.com/watch?v=sdO5UoSyrzM)

use super::{LinearSystemSolver, Matrix};

// impl Matrix {
//     fn try_inverse<S: LinearSystemSolver>(&self, mut m: Matrix) -> Option<Matrix> {
//         assert!(m.is_square_matrix());
//         let dim = m.nrows();
//         for i in 0..dim {
//             let mut rhs = vec![0.;dim];
//             rhs[i] = 1.;
//             if let S::solve(m.clone(), rhs)
//         }
//     }
// }
