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

use super::Solution;
use super::{LinearSystemSolver, Matrix};

pub struct GaussJordanElimination;

impl GaussJordanElimination {
    #[allow(dead_code)]
    fn solve_single(mut coefficients: Matrix, mut rhs: Vec<f64>) -> Solution {
        assert!(coefficients.is_square_matrix());
        let dim = coefficients.nrows();
        assert_eq!(dim, rhs.len());
        // from top to bottom (from left to right)
        for i in 0..dim {
            // if `matrix[i][i]` (which will become a pivot) is zero,
            // swap row `i` with a row where `matrix[i][i]` is not zero.
            if let Some(idx) = (i..dim).filter(|&idx| coefficients[[idx, i]] != 0.).next() {
                if idx != i {
                    coefficients.swap_row(idx, i);
                    rhs.swap(idx, i);
                }
            } else {
                continue;
            };

            let pivot = coefficients[[i, i]];
            // scale the row by 1/pivot, so that the pivot becomes 1
            for coef in coefficients.row_mut(i).iter_mut().skip(i) {
                *coef /= pivot;
            }
            rhs[i] /= pivot;
            if i < dim {
                // subtract `row[i]` * `matrix[i][j]` from `row[j]` for each row below row `i`
                // to make `row[i]` zero
                for curr_i in i + 1..dim {
                    let factor = coefficients[[curr_i, i]];
                    for j in i..dim {
                        coefficients[[curr_i, j]] -= factor * coefficients[[i, j]];
                    }
                    rhs[curr_i] -= factor * rhs[i];
                }
            }
        }

        // from right to left
        let mut null_space_cols = Vec::new();
        for i in (1..dim).rev() {
            if coefficients[[i, i]] == 0.0 {
                if rhs[i] != 0. {
                    return Solution::None;
                } else {
                    null_space_cols.push(i);
                    continue;
                }
            }

            for curr_i in 0..i {
                let factor = coefficients[[curr_i, i]];
                for j in i..dim {
                    coefficients[[curr_i, j]] -= factor * coefficients[[i, j]];
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
                    let mut ns_el = coefficients.column(j_).collect::<Vec<_>>();
                    ns_el[j_] = -1.;
                    ns_el
                })
                .collect();
            Solution::Infinite((rhs, null_space))
        }
    }
}

pub enum Rhs<'a> {
    Single(&'a mut Vec<f64>),
    Multiple(&'a mut Matrix),
}

impl<'a> Rhs<'a> {
    fn len(&self) -> usize {
        match self {
            Rhs::Single(rhs) => rhs.len(),
            Rhs::Multiple(rhs) => rhs.nrows(),
        }
    }
    fn row_swap(&mut self, i: usize, j: usize) {
        match self {
            Rhs::Single(rhs) => rhs.swap(j, i),
            Rhs::Multiple(rhs) => rhs.swap_row(j, i),
        }
    }
    fn row_addition(&mut self, dim: usize, dst: usize, src: usize, factor: f64) {
        match self {
            Rhs::Single(rhs) => rhs[dst] += factor * rhs[src],
            Rhs::Multiple(rhs) => {
                for j in 0..dim {
                    rhs[[dst, j]] += factor * rhs[[src, j]];
                }
            }
        }
    }
    fn row_multiplication(&mut self, i: usize, factor: f64) {
        match self {
            Rhs::Single(rhs) => rhs[i] *= factor,
            Rhs::Multiple(rhs) => {
                for v in &mut rhs[i] {
                    *v *= factor;
                }
            }
        }
    }
}

pub enum _Solution {
    Single(Solution),
    Multiple(Vec<Solution>),
}

impl GaussJordanElimination {
    fn _solve(coefficients: &mut Matrix, mut rhs: Rhs<'_>) -> _Solution {
        assert!(coefficients.is_square_matrix());
        let dim = coefficients.nrows();
        assert_eq!(dim, rhs.len());
        // from top to bottom (from left to right)
        for i in 0..dim {
            // if `matrix[i][i]` (which will become a pivot) is zero,
            // swap row `i` with a row where `matrix[i][i]` is not zero.
            if let Some(idx) = (i..dim).filter(|&idx| coefficients[[idx, i]] != 0.).next() {
                if idx != i {
                    coefficients.swap_row(idx, i);
                    rhs.row_swap(idx, i);
                }
            } else {
                continue;
            };

            let pivot = coefficients[[i, i]];
            // scale the row by 1/pivot, so that the pivot becomes 1
            for coef in coefficients.row_mut(i).iter_mut().skip(i) {
                *coef /= pivot;
            }
            rhs.row_multiplication(i, 1. / pivot);

            if i < dim {
                // subtract `row[i]` * `matrix[i][j]` from `row[j]` for each row below row `i`
                // to make `row[i]` zero
                for curr_i in i + 1..dim {
                    let factor = coefficients[[curr_i, i]];
                    for j in i..dim {
                        coefficients[[curr_i, j]] -= factor * coefficients[[i, j]];
                    }
                    rhs.row_addition(dim, curr_i, i, -factor);
                }
            }
        }

        // from right to left
        let mut null_space_cols = Vec::new();
        for i in (1..dim).rev() {
            if coefficients[[i, i]] == 0.0 {
                null_space_cols.push(i);
            }

            for curr_i in 0..i {
                let factor = coefficients[[curr_i, i]];
                for j in i..dim {
                    coefficients[[curr_i, j]] -= factor * coefficients[[i, j]];
                }
                rhs.row_addition(dim, curr_i, i, -factor)
            }
        }
        match rhs {
            Rhs::Single(rhs) => _Solution::Single(Self::_make_solution(
                coefficients,
                rhs.clone(),
                &null_space_cols,
            )),
            Rhs::Multiple(rhs) => _Solution::Multiple(
                rhs.columns()
                    .map(|column| {
                        Self::_make_solution(
                            coefficients,
                            column.into_iter().collect(),
                            &null_space_cols,
                        )
                    })
                    .collect(),
            ),
        }
    }
    fn _make_solution(
        processed_coefficients: &Matrix,
        rhs: Vec<f64>,
        null_space_cols: &[usize],
    ) -> Solution {
        if null_space_cols.is_empty() {
            Solution::Unique(rhs)
        } else {
            for &i in null_space_cols {
                if rhs[i] != 0. {
                    return Solution::None;
                }
            }
            let null_space = null_space_cols
                .into_iter()
                .rev()
                .map(|&j_| {
                    let mut ns_el = processed_coefficients.column(j_).collect::<Vec<_>>();
                    ns_el[j_] = -1.;
                    ns_el
                })
                .collect();
            Solution::Infinite((rhs, null_space))
        }
    }
}

impl LinearSystemSolver for GaussJordanElimination {
    fn solve(coefficients: &mut Matrix, rhs: &mut Vec<f64>) -> Solution {
        match Self::_solve(coefficients, Rhs::Single(rhs)) {
            _Solution::Single(sol) => sol,
            _ => unreachable!(),
        }
    }
    fn solve_multiple(coefficients: &mut Matrix, rhs: &mut Matrix) -> Vec<Solution> {
        match Self::_solve(coefficients, Rhs::Multiple(rhs)) {
            _Solution::Multiple(sol) => sol,
            _ => unreachable!(),
        }
    }
}

impl Matrix {
    pub fn swap_row(&mut self, i0: usize, i1: usize) {
        self.0.swap(i0, i1)
    }
    pub fn solve_by_gauss_jordan_elimination(
        coefficients: &mut Matrix,
        rhs: &mut Vec<f64>,
    ) -> Solution {
        GaussJordanElimination::solve(coefficients, rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[rustfmt::skip]
    fn simple() {
        let mut m = Matrix::new(vec![
            vec![1., 2., 3. ],
            vec![2., 4., 7. ],
            vec![3., 7., 11.]
        ]);
        let mut rhs = vec![1., 2., 2.];
        let res = GaussJordanElimination::solve(&mut m, &mut rhs).unwrap();
        assert_eq!(&res, &[3., -1., 0.]);
    }
    #[test]
    #[rustfmt::skip]
    fn no_solution() {
        let mut m = Matrix::new(vec![
            vec![1., 2., 3.],
            vec![4., 5., 6.],
            vec![7., 8., 9.]
        ]);
        let mut rhs = vec![3., 9., 6.];
        let res = GaussJordanElimination::solve(&mut m, &mut rhs);
        assert_eq!(res, Solution::None);
    }
    #[test]
    fn infinite_solutions() {
        let mut m = Matrix::new(vec![vec![1., 2., 3.], vec![4., 5., 6.], vec![7., 8., 9.]]);
        let mut rhs = vec![3., 9., 15.];
        let res = GaussJordanElimination::solve(&mut m, &mut rhs);
        assert_eq!(
            &res,
            &Solution::Infinite((vec![1.0, 1.0, 0.0], vec![vec![-1.0, 2.0, -1.0]]))
        );

        let mut m = Matrix::new(vec![
            vec![1., 2., 3., 4., 5.],
            vec![3., 7., 10., 13., 16.],
            vec![0., 0., 0., 0., 0.],
            vec![0., 0., 0., 0., 0.],
            vec![0., 0., 0., 0., 0.],
        ]);
        let mut rhs = vec![-4., -16., 0., 0., 0.];
        let res = GaussJordanElimination::solve(&mut m, &mut rhs);
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
