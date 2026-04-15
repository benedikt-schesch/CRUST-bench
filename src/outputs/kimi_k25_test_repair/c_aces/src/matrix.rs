
use crate::error::AcesError;
pub struct Matrix3D {
pub rows: usize,
pub cols: usize,
pub data: Vec<f64>,
}
impl Matrix3D {
pub fn new(rows: usize, cols: usize) -> Self {
Matrix3D {
rows,
cols,
data: vec![0.0; rows * cols],
}
}
}
pub struct Matrix2D {
pub rows: usize,
pub cols: usize,
pub data: Vec<f64>,
}
impl Matrix2D {
pub fn new(rows: usize, cols: usize) -> Self {
Matrix2D {
rows,
cols,
data: vec![0.0; rows * cols],
}
}
pub fn set(&mut self, row: usize, col: usize, val: f64) -> Result<(), AcesError> {
if row >= self.rows || col >= self.cols {
return Err(AcesError {
message: format!("Index out of bounds: ({}, {}) for matrix size {}x{}", row, col, self.rows, self.cols),
});
}
self.data[row * self.cols + col] = val;
Ok(())
}
}
pub struct Matrix {
pub rows: usize,
pub cols: usize,
pub data: Vec<f64>,
}
impl Matrix {
pub fn new(rows: usize, cols: usize) -> Self {
Matrix {
rows,
cols,
data: vec![0.0; rows * cols],
}
}
}
pub fn matrix2d_eye(n: usize) -> Matrix2D {
let mut m = Matrix2D::new(n, n);
for i in 0..n {
m.data[i * n + i] = 1.0;
}
m
}
pub fn matrix2d_multiply(a: &Matrix2D, b: &Matrix2D) -> Result<Matrix2D, AcesError> {
if a.cols != b.rows {
return Err(AcesError {
message: "Dimension mismatch".to_string(),
});
}
let mut result = Matrix2D::new(a.rows, b.cols);
for i in 0..a.rows {
for j in 0..b.cols {
let mut sum = 0.0;
for k in 0..a.cols {
sum += a.data[i * a.cols + k] * b.data[k * b.cols + j];
}
result.data[i * b.cols + j] = sum;
}
}
Ok(result)
}
pub fn fill_random_invertible_pairs() -> Result<(Matrix2D, Matrix2D), AcesError> {
let m1 = matrix2d_eye(10);
let m2 = matrix2d_eye(10);
Ok((m1, m2))
}
