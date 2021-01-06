//! # Resources
//!
//! - [**MathTheBeautiful's Video Series**](https://www.youtube.com/watch?v=l6B0w9juOcY&list=PLlXfTHzgMRUKXD88IdzS14F4NxAZudSmv&index=81)
//!     - [Linear Algebra 9a: Introduction to Gaussian Elimination](https://www.youtube.com/watch?v=l6B0w9juOcY)
//!     - [Linear Algebra 9b: The Operations of Gaussian Elimination](https://www.youtube.com/watch?v=Qq03PW55NQA)
//!     - [Linear Algebra 9c-: Review - Column Space Versus Null Space](https://www.youtube.com/watch?v=KT3NOmOOcmE)
//!     - [Linear Algebra 9c: Why Gaussian Elimination Works](https://www.youtube.com/watch?v=OApLVXbiWb4)
//!     - [Linear Algebra 9c1: Gaussian Elimination Cannot Be Used to Determine the Column Space](https://www.youtube.com/watch?v=ZP9SJGHFQ6k)
//!     - [Linear Algebra 9d: First Gaussian Elimination Example](https://www.youtube.com/watch?v=QX8A9np9VRQ)
//!     - [Linear Algebra 9e: Gaussian Elimination and Systems Without Solutions](https://www.youtube.com/watch?v=DlRcSZd0SIQ)
//!     - [Linear Algebra 9f: Row Switching in Gaussian Elimination](https://www.youtube.com/watch?v=E-y8XFuCssI)

use super::Matrix;

impl Matrix {
    pub fn gaussian_elimination(mut self, mut rhs: Vec<f64>) -> Option<Vec<f64>> {
        assert!(self.is_square_matrix());
        let dim = self.dim[0];
        assert_eq!(dim, rhs.len());
        let mut indices = (0..dim).collect::<Vec<_>>();
        for i_ in 0..dim {
            let i = if let Some(idx) = (i_..dim)
                .filter(|&idx| self[[indices[idx], i_]] != 0.)
                .nth(0)
            {
                indices.swap(i_, idx);
                indices[i_]
            } else {
                return None;
            };

            let pivot = self[[i, i_]];
            for coef in self.row_mut(i).into_iter().skip(i_) {
                *coef /= pivot;
            }
            rhs[i] /= pivot;
            if i_ < dim {
                for &curr_i in &indices[i_ + 1..] {
                    let factor = self[[curr_i, i_]];
                    for j in i_..dim {
                        self[[curr_i, j]] -= factor * self[[i, j]];
                    }
                    rhs[curr_i] -= factor * rhs[i];
                }
            }
        }

        for j_ in (1..dim).rev() {
            let j = indices[j_];
            for &i in indices[0..j_].iter() {
                let factor = self[[i, j_]];
                rhs[i] -= factor * rhs[j];
            }
        }
        Some(indices.into_iter().map(|i| rhs[i]).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[rustfmt::skip]
    fn gaussian_elimination() {
        let m = Matrix::new([3, 3],
            vec![ 1., 2., 3.,
                    2., 4., 7.,
                    3., 7., 11.]
        );
        let rhs = vec![1., 2., 2.];
        let res = m.gaussian_elimination(rhs).unwrap();
        assert_eq!(&res, &[3., -1., 0.]);
    }
}
