//! # Resources
//!
//! -[Eigenvector and Eigenvalue (mathisfun.com)](https://www.mathsisfun.com/algebra/eigenvalue.html)

//   // Given an n*n matrix, this method finds the determinant using Laplace/cofactor expansion.
//   // Time Complexity: ~O((n+2)!)
//   public static double determinant(double[][] matrix) {

//     final int n = matrix.length;

//     // Use closed form for 1x1 determinant
//     if (n == 1) return matrix[0][0];

//     // Use closed form for 2x2 determinant
//     if (n == 2) return matrix[0][0] * matrix[1][1] - matrix[0][1] * matrix[1][0];

//     // For 3x3 matrices and up use Laplace/cofactor expansion
//     return laplace(matrix);
//   }
