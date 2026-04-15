
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
pub fn get(&self, row: usize, col: usize) -> f64 {
self.data[row * self.cols + col]
}
pub fn set(&mut self, row: usize, col: usize, val: f64) {
self.data[row * self.cols + col] = val;
}
}
impl Default for Matrix {
fn default() -> Self {
Self::new(0, 0)
}
}
impl Clone for Matrix {
fn clone(&self) -> Self {
Self {
rows: self.rows,
cols: self.cols,
data: self.data.clone(),
}
}
}
pub fn matrixMul(a: &Matrix, b: &Matrix) -> Matrix {
assert_eq!(a.cols, b.rows);
let mut result = Matrix::new(a.rows, b.cols);
for i in 0..a.rows {
for j in 0..b.cols {
let mut sum = 0.0;
for k in 0..a.cols {
sum += a.get(i, k) * b.get(k, j);
}
result.set(i, j, sum);
}
}
result
}
pub fn matrixSubstAdd(a: &mut Matrix, b: &Matrix, subtract: bool) {
assert_eq!(a.rows, b.rows);
assert_eq!(a.cols, b.cols);
for i in 0..a.data.len() {
if subtract {
a.data[i] -= b.data[i];
} else {
a.data[i] += b.data[i];
}
}
}
pub fn matrixAdd(a: &mut Matrix, b: &Matrix) {
matrixSubstAdd(a, b, false);
}
pub fn matrixSub(a: &mut Matrix, b: &Matrix) {
matrixSubstAdd(a, b, true);
}
pub fn matrixAllValuesFormula<F>(mat: &mut Matrix, _func: F)
where
F: Fn(f64) -> f64
{
for val in &mut mat.data {
*val = _func(*val);
}
}
