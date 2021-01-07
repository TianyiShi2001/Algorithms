//! # Resources
//!
//! - [GeeksforGeeks](https://www.geeksforgeeks.org/cholesky-decomposition-matrix-decomposition/)

use super::Matrix;

impl Matrix {
    /// Run Cholesky decomposition and return the lower matrix.
    ///
    /// The input must be a Hermitian positive-definite matrix
    #[allow(non_snake_case)]
    pub fn cholesky(&self) -> Matrix {
        let n = self.nrows();
        let mut L = Matrix::zero([n, n]);
        for i in 0..n {
            for j in 0..=i {
                let mut sum = 0.;
                if j == i {
                    // summation for diagonals
                    for k in 0..j {
                        sum += (L[j][k]).powi(2);
                    }
                    L[j][j] = (self[j][j] - sum).sqrt();
                } else {
                    // Evaluating L(i, j) using L(j, j)
                    for k in 0..j {
                        sum += L[i][k] * L[j][k];
                    }
                    L[i][j] = (self[i][j] - sum) / L[j][j];
                }
            }
        }
        L
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cholesky() {
        let m = Matrix(vec![
            vec![4., 12., -16.],
            vec![12., 37., -43.],
            vec![-16., -43., 98.],
        ]);
        let mut c = m.cholesky();
        c *= c.transpose();
        assert_eq!(c, m);

        // TODO: random tests
    }
}
