//! # Resources
//!
//! -[Determinant of a Matrix (mathsisfun.com)](https://www.mathsisfun.com/algebra/matrix-determinant.html)

pub mod matrix;
pub use matrix::*;
pub mod vector;
pub use vector::*;
pub mod linear_systems;
pub use linear_systems::*;

pub mod cholesky;
pub mod determinant;
pub mod eigen;
pub mod elementary;
pub mod gaussian_elimination;
pub mod inverse;
pub mod lu;

#[macro_export]
macro_rules! matrix {
    // Non-empty image of given channel type
    ($( $( $x: expr ),* ;)*) => {
        {
            let nested_vec = vec![ $( vec![ $($x as f64),* ] ),* ];
            Matrix::new(nested_vec)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mul_scalar() {
        let m3x3 = matrix! {
            6, 1, 1;
            4,-2, 5;
            2, 8, 7;
        };
        assert_eq!(
            m3x3 * 3.,
            matrix! {
                18,  3,  3;
                12, -6, 15;
                6,  24, 21;
            }
        );
    }
    #[test]
    fn mul_matrix() {
        let m2x3 = matrix! {
            6, 1, 1;
            4,-2, 5;
        };
        let m3x2 = matrix! {
             3, 2;
             0,-1;
            -3, 4;
        };
        assert_eq!(
            m2x3 * m3x2,
            matrix! {
                15, 15;
                -3, 30;
            }
        )
    }

    #[test]
    #[rustfmt::skip]
    fn transpose() {
        let m = matrix! {
            1, 2, 3;
            4, 5, 6;
        };
        let t = m.transpose();
        assert_eq!(t, matrix!{
            1, 4;
            2, 5;
            3, 6;
        });
    }

    #[test]
    fn random_lower_triangular() {
        let mut rng = rand::thread_rng();
        let m = Matrix::random_lower_triangular(5, &mut rng, -1e3..1e3);
        println!("{}", m);

        let mut rng = rand::thread_rng();
        let m = Matrix::random_symmetric(5, &mut rng, -1e3..1e3);
        println!("{}", m);
    }
}
