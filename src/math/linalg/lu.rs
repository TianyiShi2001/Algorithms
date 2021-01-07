// use super::Matrix;

// impl Matrix {
//     pub fn lu(mut self, mut rhs: Vec<f64>) {
//         assert!(self.is_square_matrix());
//         let dim = self.nrows();
//         assert_eq!(dim, rhs.len());
//         // from top to bottom (from left to right)
//         for i in 0..dim {
//             // if `matrix[i][i]` (which will become a pivot) is zero,
//             // swap row `i` with a row where `matrix[i][i]` is not zero.
//             if let Some(idx) = (i..dim).filter(|&idx| self[[idx, i]] != 0.).next() {
//                 if idx != i {
//                     self.swap_row(idx, i);
//                     rhs.swap(idx, i);
//                 }
//             } else {
//                 continue;
//             };

//             let pivot = self[[i, i]];
//             // scale the row by 1/pivot, so that the pivot becomes 1
//             for coef in self.row_mut(i).iter_mut().skip(i) {
//                 *coef /= pivot;
//             }
//             rhs[i] /= pivot;
//             if i < dim {
//                 // subtract `row[i]` * `matrix[i][j]` from `row[j]` for each row below row `i`
//                 // to make `row[i]` zero
//                 for curr_i in i + 1..dim {
//                     let factor = self[[curr_i, i]];
//                     for j in i..dim {
//                         self[[curr_i, j]] -= factor * self[[i, j]];
//                     }
//                     rhs[curr_i] -= factor * rhs[i];
//                 }
//             }
//         }

//         // from right to left
//         let mut null_space_cols = Vec::new();
//         for i in (1..dim).rev() {
//             if self[[i, i]] == 0.0 {
//                 if rhs[i] != 0. {
//                     return Solution::None;
//                 } else {
//                     null_space_cols.push(i);
//                     continue;
//                 }
//             }

//             for curr_i in 0..i {
//                 let factor = self[[curr_i, i]];
//                 for j in i..dim {
//                     self[[curr_i, j]] -= factor * self[[i, j]];
//                 }
//                 rhs[curr_i] -= factor * rhs[i];
//             }
//         }
//         if null_space_cols.is_empty() {
//             Solution::Unique(rhs)
//         } else {
//             let null_space = null_space_cols
//                 .into_iter()
//                 .rev()
//                 .map(|j_| {
//                     let mut ns_el = self.column(j_).collect::<Vec<_>>();
//                     ns_el[j_] = -1.;
//                     ns_el
//                 })
//                 .collect();
//             Solution::Infinite((rhs, null_space))
//         }
//     }
// }
