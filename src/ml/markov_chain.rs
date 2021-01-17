//! # Resources
//!
//! - [Normalized Nerd's video series](https://www.youtube.com/playlist?list=PLM8wYQRetTxBkdvBtz-gw8b9lcVkdXQKV)
//!   - [Markov Chains Clearly Explained! Part - 1](https://www.youtube.com/watch?v=i3AkTO9HLXo)
//!   - [Markov Chains: Recurrence, Irreducibility, Classes | Part - 2](https://www.youtube.com/watch?v=VNHeFp6zXKU)
//!   - [Markov Chains: n-step Transition Matrix | Part - 3](https://www.youtube.com/watch?v=Zo3ieESzr4E)
//!   - [Markov Chains: Generating Sherlock Holmes Stories | Part - 4](https://www.youtube.com/watch?v=E4WcBWuQQws)
//!   - [Hidden Markov Model Clearly Explained! Part - 5](https://www.youtube.com/watch?v=RWkHJnFj5rY)

// This file is WIP.

use crate::math::linalg::{Matrix, Solution};
use nalgebra::{Dynamic, VecStorage};
use num::complex::Complex;
type DMatrixf64 = nalgebra::Matrix<f64, Dynamic, Dynamic, VecStorage<f64, Dynamic, Dynamic>>;

pub struct MarkovChain {
    stochastic_matrix: Matrix,
}

impl MarkovChain {
    pub fn new(stochastic_matrix: Matrix) -> Self {
        Self::from_right_stochastic_matrix(stochastic_matrix)
    }
    /// `A[i][j]` represents the probability of transition from `j` to `i`.
    /// Every column sums up to 1.
    pub fn from_left_stochastic_matrix(stochastic_matrix: Matrix) -> Self {
        Self { stochastic_matrix }
    }
    /// `A[i][j]` represents the probability of transition from `i` to `j`
    /// Every row sums up to 1.
    pub fn from_right_stochastic_matrix(stochastic_matrix: Matrix) -> Self {
        Self {
            stochastic_matrix: stochastic_matrix.transpose(),
        }
    }
    pub fn solve(mut self) -> Option<Vec<f64>> {
        if eigen(&self.stochastic_matrix)
            .iter()
            .any(|&eigenvalue| eigenvalue.im == 0.0 && (eigenvalue.re - 1.0).abs() < 1e-6)
        {
            for i in 0..self.stochastic_matrix.nrows() {
                self.stochastic_matrix[i][i] -= 1.;
                self.stochastic_matrix.row_mut(i).push(0.);
            }
            println!("{:?}", self.stochastic_matrix);
            if let Solution::Infinite(_, v) = self.stochastic_matrix.solve().first() {
                let v = &v[0];
                let scale: f64 = v.iter().sum();
                let ans: Vec<f64> = v.iter().map(|&x| x / scale).collect();
                return Some(ans);
            }
        }
        None
    }
}

fn eigen(m: &Matrix) -> Vec<Complex<f64>> {
    let m = DMatrixf64::from_vec(m.nrows(), m.nrows(), m.iter().collect());
    m.complex_eigenvalues().column(0).iter().cloned().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::matrix;
    #[test]
    fn t() {
        let m = matrix! [
            0.2, 0.6, 0.2;
            0.3, 0.0, 0.7;
            0.5, 0.0, 0.5;
        ];

        let res = MarkovChain::from_right_stochastic_matrix(m)
            .solve()
            .unwrap();
        assert_eq!(
            res,
            vec![0.35211267605633806, 0.2112676056338028, 0.4366197183098592]
        )
    }
}
