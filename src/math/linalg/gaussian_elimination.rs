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
    pub fn swap_row(&mut self, i0: usize, i1: usize) {
        self.0.swap(i0, i1)
    }
    pub fn gauss_jordan_elimination(mut self, mut rhs: Vec<f64>) -> Solution {
        assert!(self.is_square_matrix());
        let dim = self.nrows();
        assert_eq!(dim, rhs.len());
        // from top to bottom (from left to right)
        for i in 0..dim {
            // if `matrix[i][i]` (which will become a pivot) is zero,
            // swap row `i` with a row where `matrix[i][i]` is not zero.
            if let Some(idx) = (i..dim).filter(|&idx| self[[idx, i]] != 0.).next() {
                if idx != i {
                    self.swap_row(idx, i);
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
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[rustfmt::skip]
    fn simple() {
        let m = Matrix::new(vec![
            vec![1., 2., 3. ],
            vec![2., 4., 7. ],
            vec![3., 7., 11.]
        ]);
        let rhs = vec![1., 2., 2.];
        let res = m.gauss_jordan_elimination(rhs).unwrap();
        assert_eq!(&res, &[3., -1., 0.]);
    }
    #[test]
    #[rustfmt::skip]
    fn no_solution() {
        let m = Matrix::new(vec![
            vec![1., 2., 3.],
            vec![4., 5., 6.],
            vec![7., 8., 9.]
        ]);
        let rhs = vec![3., 9., 6.];
        let res = m.gauss_jordan_elimination(rhs);
        assert_eq!(res, Solution::None);
    }
    #[test]
    fn infinite_solutions() {
        let m = Matrix::new(vec![vec![1., 2., 3.], vec![4., 5., 6.], vec![7., 8., 9.]]);
        let rhs = vec![3., 9., 15.];
        let res = m.clone().gauss_jordan_elimination(rhs.clone());
        assert_eq!(
            &res,
            &Solution::Infinite((vec![1.0, 1.0, 0.0], vec![vec![-1.0, 2.0, -1.0]]))
        );

        let m = Matrix::new(vec![
            vec![1., 2., 3., 4., 5.],
            vec![3., 7., 10., 13., 16.],
            vec![0., 0., 0., 0., 0.],
            vec![0., 0., 0., 0., 0.],
            vec![0., 0., 0., 0., 0.],
        ]);
        let rhs = vec![-4., -16., 0., 0., 0.];
        let res = m.clone().gauss_jordan_elimination(rhs.clone());
        assert_eq!(
            &res,
            &Solution::Infinite((
                vec![4.0, -4.0, 0.0, 0.0, 0.0],
                vec![
                    vec![1.0, 1.0, -1.0, 0.0, 0.0],
                    vec![2.0, 1.0, 0.0, -1.0, 0.0],
                    vec![3.0, 1.0, 0.0, 0.0, -1.0]
                ]
            ))
        );
    }
}
