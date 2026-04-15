use crate::vector::Vector;
use crate::utils::is_close_to_zero;
pub struct Matrix {
pub rows: usize,
pub cols: usize,
pub data: Vec<f64>,
}
impl Matrix {
pub fn new(rows: usize, cols: usize) -> Self {
Self {
rows,
cols,
data: vec![0.0; rows * cols],
}
}
}
pub fn new_matrix(data: &[f64], rows: usize, cols: usize) -> Matrix {
assert_eq!(data.len(), rows * cols, "Data length must match matrix dimensions");
Matrix {
rows,
cols,
data: data.to_vec(),
}
}
pub fn delete_matrix(_m: Matrix) {
}
pub fn get_matrix_element(m: &Matrix, row: usize, col: usize) -> f64 {
m.data[row * m.cols + col]
}
pub fn set_matrix_element(m: &mut Matrix, row: usize, col: usize, val: f64) {
m.data[row * m.cols + col] = val;
}
pub fn zero_matrix(rows: usize, cols: usize) -> Matrix {
Matrix::new(rows, cols)
}
pub fn copy_matrix(m: &Matrix) -> Matrix {
Matrix {
rows: m.rows,
cols: m.cols,
data: m.data.clone(),
}
}
pub fn matrix_size(m: &Matrix) -> usize {
m.rows * m.cols
}
pub fn matrix_size_bytes(m: &Matrix) -> usize {
m.data.len() * std::mem::size_of::<f64>()
}
pub fn flatten_matrix(m: &Matrix) -> Vector {
Vector::from_data(m.data.clone())
}
pub fn get_row_vector(m: &Matrix, row: usize) -> Vector {
let start = row * m.cols;
let end = start + m.cols;
Vector::from_data(m.data[start..end].to_vec())
}
pub fn set_row_vector(m: &mut Matrix, row: usize, v: &Vector) {
assert_eq!(v.len(), m.cols, "Vector length must match matrix columns");
let start = row * m.cols;
for (i, &val) in v.data.iter().enumerate() {
m.data[start + i] = val;
}
}
pub fn get_col_vector(m: &Matrix, col: usize) -> Vector {
let mut data = Vec::with_capacity(m.rows);
for row in 0..m.rows {
data.push(m.data[row * m.cols + col]);
}
Vector::from_data(data)
}
pub fn set_col_vector(m: &mut Matrix, col: usize, v: &Vector) {
let len = v.len().min(m.rows);
for row in 0..len {
m.data[row * m.cols + col] = v.data[row];
}
}
pub fn get_main_diagonal(m: &Matrix) -> Vector {
let min_dim = m.rows.min(m.cols);
let mut data = Vec::with_capacity(min_dim);
for i in 0..min_dim {
data.push(m.data[i * m.cols + i]);
}
Vector::from_data(data)
}
pub fn set_main_diagonal(m: &mut Matrix, v: &Vector) {
let min_dim = m.rows.min(m.cols);
assert_eq!(v.len(), min_dim, "Vector length must match min(rows, cols)");
for i in 0..min_dim {
m.data[i * m.cols + i] = v.data[i];
}
}
pub fn get_anti_diagonal(m: &Matrix) -> Vector {
let min_dim = m.rows.min(m.cols);
let mut data = Vec::with_capacity(min_dim);
for i in 0..min_dim {
data.push(m.data[(m.rows - 1 - i) * m.cols + i]);
}
Vector::from_data(data)
}
pub fn set_anti_diagonal(m: &mut Matrix, v: &Vector) {
let min_dim = m.rows.min(m.cols);
assert_eq!(v.len(), min_dim, "Vector length must match min(rows, cols)");
for i in 0..min_dim {
m.data[(m.rows - 1 - i) * m.cols + i] = v.data[i];
}
}
pub fn is_matrix_equal(m1: &Matrix, m2: &Matrix) -> bool {
if m1.rows != m2.rows || m1.cols != m2.cols {
return false;
}
m1.data.iter().zip(m2.data.iter()).all(|(a, b)| (a - b).abs() < 1e-10)
}
pub fn is_zero_matrix(m: &Matrix) -> bool {
m.data.iter().all(|&x| is_close_to_zero(x, 1e-10))
}
pub fn is_identity_matrix(m: &Matrix) -> bool {
if m.rows != m.cols {
return false;
}
for i in 0..m.rows {
for j in 0..m.cols {
let val = m.data[i * m.cols + j];
if i == j {
if (val - 1.0).abs() > 1e-10 {
return false;
}
} else {
if !is_close_to_zero(val, 1e-10) {
return false;
}
}
}
}
true
}
pub fn identity_matrix(n: usize) -> Matrix {
let mut m = Matrix::new(n, n);
for i in 0..n {
m.data[i * n + i] = 1.0;
}
m
}
pub fn is_square_matrix(m: &Matrix) -> bool {
m.rows == m.cols
}
pub fn is_diagonal_matrix(m: &Matrix) -> bool {
if m.rows != m.cols {
return false;
}
for i in 0..m.rows {
for j in 0..m.cols {
if i != j && !is_close_to_zero(m.data[i * m.cols + j], 1e-10) {
return false;
}
}
}
true
}
pub fn is_up_tri_matrix(m: &Matrix) -> bool {
if m.rows != m.cols {
return false;
}
for i in 1..m.rows {
for j in 0..i {
if !is_close_to_zero(m.data[i * m.cols + j], 1e-10) {
return false;
}
}
}
true
}
pub fn is_lo_tri_matrix(m: &Matrix) -> bool {
if m.rows != m.cols {
return false;
}
for i in 0..m.rows {
for j in (i + 1)..m.cols {
if !is_close_to_zero(m.data[i * m.cols + j], 1e-10) {
return false;
}
}
}
true
}
pub fn is_triangular_matrix(m: &Matrix) -> bool {
is_up_tri_matrix(m) || is_lo_tri_matrix(m)
}
pub fn is_matrix_symmetric(m: &Matrix) -> bool {
if m.rows != m.cols {
return false;
}
for i in 0..m.rows {
for j in (i + 1)..m.cols {
if (m.data[i * m.cols + j] - m.data[j * m.cols + i]).abs() > 1e-10 {
return false;
}
}
}
true
}
pub fn transpose_matrix(m: &Matrix) -> Matrix {
let mut result = Matrix::new(m.cols, m.rows);
for i in 0..m.rows {
for j in 0..m.cols {
result.data[j * m.rows + i] = m.data[i * m.cols + j];
}
}
result
}
pub fn trace_matrix(m: &Matrix) -> f64 {
assert!(is_square_matrix(m), "Matrix must be square");
let mut sum = 0.0;
for i in 0..m.rows {
sum += m.data[i * m.cols + i];
}
sum
}
pub fn add_matrices(m1: &Matrix, m2: &Matrix) -> Matrix {
assert_eq!(m1.rows, m2.rows, "Matrices must have same number of rows");
assert_eq!(m1.cols, m2.cols, "Matrices must have same number of columns");
let mut result = Matrix::new(m1.rows, m1.cols);
for i in 0..result.data.len() {
result.data[i] = m1.data[i] + m2.data[i];
}
result
}
pub fn multiply_matrices(m1: &Matrix, m2: &Matrix) -> Matrix {
assert_eq!(m1.cols, m2.rows, "First matrix columns must equal second matrix rows");
let mut result = Matrix::new(m1.rows, m2.cols);
for i in 0..m1.rows {
for j in 0..m2.cols {
let mut sum = 0.0;
for k in 0..m1.cols {
sum += m1.data[i * m1.cols + k] * m2.data[k * m2.cols + j];
}
result.data[i * result.cols + j] = sum;
}
}
result
}
pub fn scale_matrix(m: &Matrix, scalar: f64) -> Matrix {
let mut result = Matrix::new(m.rows, m.cols);
for i in 0..m.data.len() {
result.data[i] = m.data[i] * scalar;
}
result
}
pub fn sub_matrix(m: &Matrix, rows: usize, cols: usize) -> Matrix {
assert!(rows <= m.rows && cols <= m.cols, "Submatrix dimensions must be smaller than original");
let mut result = Matrix::new(rows, cols);
for i in 0..rows {
for j in 0..cols {
result.data[i * cols + j] = m.data[i * m.cols + j];
}
}
result
}
fn get_minor(m: &Matrix, row: usize, col: usize) -> Matrix {
let n = m.rows;
let mut minor = Matrix::new(n - 1, n - 1);
let mut minor_row = 0;
for i in 0..n {
if i == row {
continue;
}
let mut minor_col = 0;
for j in 0..n {
if j == col {
continue;
}
minor.data[minor_row * (n - 1) + minor_col] = m.data[i * n + j];
minor_col += 1;
}
minor_row += 1;
}
minor
}
pub fn determinant(m: &Matrix) -> f64 {
assert!(is_square_matrix(m), "Matrix must be square");
let n = m.rows;
if n == 1 {
return m.data[0];
}
if n == 2 {
return m.data[0] * m.data[3] - m.data[1] * m.data[2];
}
let mut det = 0.0;
for j in 0..n {
let minor = get_minor(m, 0, j);
let sign = if j % 2 == 0 { 1.0 } else { -1.0 };
det += sign * m.data[j] * determinant(&minor);
}
det
}
pub fn inverse_matrix(m: &Matrix) -> Matrix {
assert!(is_square_matrix(m), "Matrix must be square");
let det = determinant(m);
assert!(!is_close_to_zero(det, 1e-10), "Matrix is singular");
let n = m.rows;
if n == 1 {
let mut inv = Matrix::new(1, 1);
inv.data[0] = 1.0 / m.data[0];
return inv;
}
let mut adjugate = Matrix::new(n, n);
for i in 0..n {
for j in 0..n {
let minor = get_minor(m, i, j);
let sign = if (i + j) % 2 == 0 { 1.0 } else { -1.0 };
adjugate.data[j * n + i] = sign * determinant(&minor);
}
}
scale_matrix(&adjugate, 1.0 / det)
}
pub fn print_matrix(m: &Matrix, newline: bool) {
for i in 0..m.rows {
for j in 0..m.cols {
print!("{} ", m.data[i * m.cols + j]);
}
println!();
}
if newline {
println!();
}
}
