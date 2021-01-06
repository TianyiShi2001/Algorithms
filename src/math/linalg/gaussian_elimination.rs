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

#[derive(Debug, PartialEq, Clone)]
pub enum Solution {
    Unique(Vec<f64>),
    Infinite((Vec<f64>, Vec<Vec<f64>>)),
    None,
}

impl Solution {
    pub fn unwrap(self) -> Vec<f64> {
        match self {
            Self::Unique(res) => res,
            Self::Infinite(_) => panic!("Infinite solutions!"),
            Self::None => panic!("No solutions!"),
        }
    }
}

impl Matrix {
    fn _swap_row(&mut self, i1: usize, i2: usize) {
        let n = self.dim[1];
        for j in 0..n {
            self.inner.swap(i1 * n + j, i2 * n + j);
        }
    }
    #[allow(dead_code)]
    pub fn gaussian_elimination_simple(mut self, mut rhs: Vec<f64>) -> Solution {
        assert!(self.is_square_matrix());
        let dim = self.dim[0];
        assert_eq!(dim, rhs.len());
        // from top to bottom (from left to right)
        for i in 0..dim {
            // if `matrix[i][i]` (which will become a pivot) is zero,
            // swap row `i` with a row where `matrix[i][i]` is not zero.
            if let Some(idx) = (i..dim).filter(|&idx| self[[idx, i]] != 0.).next() {
                if idx != i {
                    self._swap_row(idx, i);
                    rhs.swap(idx, i);
                }
            } else {
                continue;
            };

            let pivot = self[[i, i]];
            // scale the row by 1/pivot, so that the pivot becomes 1
            for coef in self.row_mut(i).iter_mut().skip(i) {
                *coef /= pivot;
            }
            rhs[i] /= pivot;
            if i < dim {
                // subtract `row[i]` * `matrix[i][j]` from `row[j]` for each row below row `i`
                // to make `row[i]` zero
                for curr_i in i + 1..dim {
                    let factor = self[[curr_i, i]];
                    for j in i..dim {
                        self[[curr_i, j]] -= factor * self[[i, j]];
                    }
                    rhs[curr_i] -= factor * rhs[i];
                }
            }
        }

        // from right to left
        let mut null_space_cols = Vec::new();
        for i in (1..dim).rev() {
            if self[[i, i]] == 0.0 {
                if rhs[i] != 0. {
                    return Solution::None;
                } else {
                    null_space_cols.push(i);
                    continue;
                }
            }

            for curr_i in 0..i {
                let factor = self[[curr_i, i]];
                for j in i..dim {
                    self[[curr_i, j]] -= factor * self[[i, j]];
                }
                rhs[curr_i] -= factor * rhs[i];
            }
        }
        if null_space_cols.is_empty() {
            Solution::Unique(rhs)
        } else {
            let null_space = null_space_cols
                .into_iter()
                .rev()
                .map(|j_| {
                    let mut ns_el = self.column(j_).collect::<Vec<_>>();
                    ns_el[j_] = -1.;
                    ns_el
                })
                .collect();
            Solution::Infinite((rhs, null_space))
        }
    }
    pub fn gaussian_elimination(mut self, mut rhs: Vec<f64>) -> Solution {
        assert!(self.is_square_matrix());
        let dim = self.dim[0];
        assert_eq!(dim, rhs.len());
        let mut indices = (0..dim).collect::<Vec<_>>();
        // from top to bottom (from left to right)
        for j_ in 0..dim {
            // if `matrix[i][i]` (which will become a pivot) is zero,
            // swap row `i` with a row where `matrix[i][i]` is not zero.
            let i = if let Some(idx) = (j_..dim)
                .filter(|&idx| self[[indices[idx], j_]] != 0.)
                .next()
            {
                indices.swap(j_, idx);
                indices[j_]
            } else {
                continue;
            };

            let pivot = self[[i, j_]];
            // scale the row by 1/pivot, so that the pivot becomes 1
            for coef in self.row_mut(i).iter_mut().skip(j_) {
                *coef /= pivot;
            }
            rhs[i] /= pivot;
            if j_ < dim {
                // subtract `row[j_]` * `matrix[i][j]` from `row[j]` for each row below row `i`
                // to make `row[j_]` zero
                for &curr_i in &indices[j_ + 1..] {
                    let factor = self[[curr_i, j_]];
                    for j in j_..dim {
                        self[[curr_i, j]] -= factor * self[[i, j]];
                    }
                    rhs[curr_i] -= factor * rhs[i];
                }
            }
        }

        // from right to left
        let mut null_space_cols = Vec::new();
        for j_ in (1..dim).rev() {
            let i_ = indices[j_];
            if self[[i_, j_]] == 0.0 {
                if rhs[i_] != 0. {
                    return Solution::None;
                } else {
                    null_space_cols.push(j_);
                    continue;
                }
            }

            for &i in indices[0..j_].iter() {
                let factor = self[[i, j_]];
                for j in j_..dim {
                    self[[i, j]] -= factor * self[[i_, j]];
                }
                rhs[i] -= factor * rhs[i_];
            }
        }
        if null_space_cols.is_empty() {
            Solution::Unique(indices.into_iter().map(|i| rhs[i]).collect())
        } else {
            let null_space = null_space_cols
                .into_iter()
                .rev()
                .map(|j_| {
                    let column: Vec<_> = self.column(j_).collect();
                    let mut ns_el = vec![0.; dim];
                    for (i, &real_i) in indices.iter().enumerate() {
                        ns_el[real_i] = column[i];
                    }
                    ns_el[j_] = -1.;
                    ns_el
                })
                .collect();
            Solution::Infinite((indices.into_iter().map(|i| rhs[i]).collect(), null_space))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[rustfmt::skip]
    fn simple() {
        let m = Matrix::new([3, 3],
            vec![ 1., 2., 3.,
                    2., 4., 7.,
                    3., 7., 11.]
        );
        let rhs = vec![1., 2., 2.];
        let res = m.gaussian_elimination(rhs).unwrap();
        assert_eq!(&res, &[3., -1., 0.]);
    }
    #[test]
    #[rustfmt::skip]
    fn no_solution() {
        let m = Matrix::new([3, 3],
            vec![ 1., 2., 3.,
                    4., 5., 6.,
                    7., 8., 9.]
        );
        let rhs = vec![3., 9., 6.];
        let res = m.gaussian_elimination(rhs);
        assert_eq!(res, Solution::None);
    }
    #[test]
    #[rustfmt::skip]
    fn infinite_solutions() {
        let m = Matrix::new([3, 3],
            vec![ 1., 2., 3.,
                    4., 5., 6.,
                    7., 8., 9.]
        );
        let rhs = vec![3., 9., 15.];
        let res = m.clone().gaussian_elimination(rhs.clone());
        assert_eq!(&res, &Solution::Infinite((vec![1.0, 1.0, 0.0], vec![vec![-1.0, 2.0, -1.0]])));
        assert_eq!(&res, &m.gaussian_elimination_simple(rhs));

        let m = Matrix::new([5, 5],
            vec![ 1., 2., 3., 4., 5.,
                    3., 7., 10., 13., 16.,
                    0., 0., 0., 0., 0.,
                    0., 0., 0., 0., 0.,
                    0., 0., 0., 0., 0.,]
        );
        let rhs = vec![-4., -16., 0., 0., 0.];
        let res = m.clone().gaussian_elimination(rhs.clone());
        assert_eq!(&res, &Solution::Infinite((vec![4.0, -4.0, 0.0, 0.0, 0.0],
            vec![vec![1.0, 1.0, -1.0, 0.0, 0.0],
                 vec![2.0, 1.0, 0.0, -1.0, 0.0],
                 vec![3.0, 1.0, 0.0, 0.0, -1.0]])));
        assert_eq!(&res, &m.gaussian_elimination_simple(rhs));
    }
}
