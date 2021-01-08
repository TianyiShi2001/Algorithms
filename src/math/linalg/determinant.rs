use super::Matrix;

impl Matrix {
    /// Compute the determinant of the matrix.
    pub fn determinant(&self) -> f64 {
        assert!(self.is_square_matrix());
        let dim = self.nrows();
        if dim == 1 {
            self[0][0]
        } else {
            let mut res = 0.;
            let mut sign = 1.;
            for i in 0..dim {
                res +=
                    // first row, column `i`
                    self.0[0][i] * sign *
                        // matrix formed by numbers that are not in column `i`
                        Matrix(
                            self.rows().skip(1).map(|row|
                                row[..i].iter().chain(row[i..].iter().skip(1)).cloned().collect()
                            ).collect()
                        ).determinant();
                // invert the sign
                sign = -sign;
            }
            res
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::matrix;
    #[test]
    fn determinant() {
        let m2x2 = matrix![
            1, 2;
            3, 4;
        ];
        assert_eq!(m2x2.determinant(), -2.);
        let m3x3 = matrix![
            6, 1, 1;
            4,-2, 5;
            2, 8, 7;
        ];
        assert_eq!(m3x3.determinant(), -306.);
    }
}
