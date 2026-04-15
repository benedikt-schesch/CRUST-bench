//! Matrix test binary
use crate::matrix::{Matrix2D, matrix2d_multiply, matrix2d_eye, fill_random_invertible_pairs};

fn main() {
let dim = 10;

// Fixed: Added second argument for columns
let mut m1 = Matrix2D::new(dim, dim);
let mut m2 = Matrix2D::new(dim, dim);

// Fill m1 with values
for i in 0..dim {
for j in 0..dim {
let val = (i * dim + j) as f64;
m1.set(i, j, val).unwrap();
}
}

// Fixed: matrix2d_eye returns Matrix2D directly, not Result
m2 = matrix2d_eye(dim);

// Fixed: Removed third argument (modulus) - function takes 2 arguments
let _result = matrix2d_multiply(&m1, &m2).unwrap();

// Test with random invertible pairs
let mut m1 = Matrix2D::new(dim, dim);
let mut m2 = Matrix2D::new(dim, dim);

// Fixed: fill_random_invertible_pairs takes 0 arguments and returns Result<(Matrix2D, Matrix2D)>
// Using destructuring to get the matrices
let (m1_new, m2_new) = fill_random_invertible_pairs().unwrap();
// If we need to use the existing m1, m2 variables, we can assign them:
m1 = m1_new;
m2 = m2_new;

// Fixed: Removed third argument (modulus)
let _result = matrix2d_multiply(&m1, &m2).unwrap();

// Expected result setup
let mut exp_result = Matrix2D::new(dim, dim);
// Fixed: matrix2d_eye returns Matrix2D directly
exp_result = matrix2d_eye(dim);
}
