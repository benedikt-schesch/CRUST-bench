
use crate::matrix::{Matrix2D, matrix2d_multiply, matrix2d_eye, fill_random_invertible_pairs};
fn main() {
let dim = 10;
let mut m1 = Matrix2D::new(dim, dim);
let mut m2 = Matrix2D::new(dim, dim);
for i in 0..dim {
for j in 0..dim {
let val = (i * dim + j) as f64;
m1.set(i, j, val).unwrap();
}
}
m2 = matrix2d_eye(dim);
let _result = matrix2d_multiply(&m1, &m2).unwrap();
let mut m1 = Matrix2D::new(dim, dim);
let mut m2 = Matrix2D::new(dim, dim);
let (m1_new, m2_new) = fill_random_invertible_pairs().unwrap();
m1 = m1_new;
m2 = m2_new;
let _result = matrix2d_multiply(&m1, &m2).unwrap();
let mut exp_result = Matrix2D::new(dim, dim);
exp_result = matrix2d_eye(dim);
}
