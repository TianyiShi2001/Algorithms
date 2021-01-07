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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mul() {
        let m3x3 = Matrix::new(vec![vec![6., 1., 1.], vec![4., -2., 5.], vec![2., 8., 7.]]);
        assert_eq!(
            m3x3 * 3.,
            Matrix::new(vec![
                vec![18., 3., 3.],
                vec![12., -6., 15.],
                vec![6., 24., 21.]
            ])
        );
        let m2x3 = Matrix::new(vec![vec![6., 1., 1.], vec![4., -2., 5.]]);
        let m3x2 = Matrix::new(vec![vec![3., 2.], vec![0., -1.], vec![-3., 4.]]);
        assert_eq!(
            m2x3 * m3x2,
            Matrix::new(vec![vec![15., 15.], vec![-3., 30.]])
        )
    }

    #[test]
    #[rustfmt::skip]
    fn transpose() {
        let m = Matrix(vec![
            vec![1., 2., 3.],
            vec![4., 5., 6.],
        ]);
        let t = m.transpose();
        assert_eq!(t, Matrix(vec![
            vec![1., 4.],
            vec![2., 5.],
            vec![3., 6.],
        ]));
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
