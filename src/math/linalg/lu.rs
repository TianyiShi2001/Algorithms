use super::Matrix;

impl Matrix {
    #[allow(non_snake_case)]
    pub fn lu(mut self) -> [Matrix; 2] {
        assert!(self.is_square_matrix());
        let dim = self.nrows();
        let mut L = Self::identity(dim);
        // from top to bottom (from left to right)
        for i in 0..dim {
            // if `matrix[i][i]` (which will become a pivot) is zero,
            // swap row `i` with a row where `matrix[i][i]` is not zero.
            if let Some(idx) = (i..dim).filter(|&idx| self[[idx, i]] != 0.).next() {
                if idx != i {
                    self.swap_row(idx, i);
                    // rhs.swap(idx, i);
                    L *= Self::row_swapping_matrix_inverse(dim, idx, i);
                }
            } else {
                continue;
            };

            let pivot = self[[i, i]];
            // rhs[i] /= pivot;
            if i < dim {
                // subtract `m[curr_i][i] / pivot` * `m[i][j]` from `m[curr_i][j]` for each row below row `i`
                // to make `m[curr_i][i]` zero
                let params = (i + 1..dim)
                    .map(|curr_i| (curr_i, -self[[curr_i, i]] / pivot))
                    .collect::<Vec<_>>();
                for &(curr_i, factor) in &params {
                    for j in i..dim {
                        self[[curr_i, j]] += factor * self[[i, j]];
                    }
                    // rhs[curr_i] -= factor * rhs[i];
                }
                L *= Self::row_addition_matrix_inverse(dim, i, &params);
            }
        }
        [L, self]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::lazy_static;
    lazy_static! {
        static ref M: Matrix = Matrix(vec![
            vec![1., 2., 3., 4., 5.],
            vec![6., 7., 8., 9., 0.],
            vec![5., 8., 3., 5., 8.],
            vec![9., 3., 2., 5., 9.],
            vec![4., 7., 1., 3., 9.],
        ]);
    }

    #[test]
    #[allow(non_snake_case)]
    fn lu_decomposition() {
        let [L, U] = M.clone().lu();
        assert_eq!(L * U, *M);
    }
}
