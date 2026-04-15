// Generated Rust Code

use crate::linear_algebra::{Matrix, Vector};
use crate::utils::exclusive_or;

pub fn assert_matrix_impl(m: &Matrix) -> bool {
assert!(m.rows > 0 && m.cols > 0);
assert!(m.data.len() == m.rows * m.cols);
true
}

pub fn new_matrix_impl(d: &[f64], rows: usize, cols: usize) -> Matrix {
assert!(!d.is_empty() && rows > 0 && cols > 0);
assert!(d.len() >= rows * cols);

let mut m = null_matrix_impl(rows, cols);
let mut idx = 0;
for i in 0..m.rows {
for j in 0..m.cols {
m.data[i * m.cols + j] = d[idx];
idx += 1;
}
}
m
}

pub fn null_matrix_impl(rows: usize, cols: usize) -> Matrix {
assert!(rows > 0 && cols > 0);
Matrix {
rows,
cols,
data: vec![0.0; rows * cols],
}
}

pub fn zero_matrix_impl(rows: usize, cols: usize) -> Matrix {
let mut m = null_matrix_impl(rows, cols);
fill_matrix_impl(&mut m, 0.0);
m
}

pub fn fill_matrix_impl(m: &mut Matrix, n: f64) {
assert_matrix_impl(m);
for i in 0..m.rows {
for j in 0..m.cols {
m.data[i * m.cols + j] = n;
}
}
}

pub fn identity_matrix_impl(n: usize) -> Matrix {
let mut m = zero_matrix_impl(n, n);
for i in 0..m.rows {
for j in 0..m.rows {
if i == j {
m.data[i * m.cols + j] = 1.0;
}
}
}
m
}

pub fn delete_matrix_impl(m: Matrix) {
drop(m);
}

pub fn copy_matrix_impl(m: &Matrix) -> Matrix {
assert_matrix_impl(m);
let mut c = zero_matrix_impl(m.rows, m.cols);
for i in 0..m.rows {
for j in 0..m.cols {
c.data[i * m.cols + j] = m.data[i * m.cols + j];
}
}
c
}

pub fn flatten_matrix_impl(m: &Matrix) -> Vector {
assert_matrix_impl(m);
Vector {
cols: m.rows * m.cols,
data: m.data.clone(),
}
}

pub fn matrix_size_impl(m: &Matrix) -> usize {
assert_matrix_impl(m);
m.rows * m.cols
}

pub fn matrix_size_bytes_impl(m: &Matrix) -> usize {
std::mem::size_of::<f64>() * matrix_size_impl(m)
}

pub fn set_matrix_element_impl(m: &mut Matrix, i: usize, j: usize, s: f64) {
assert!(assert_matrix_impl(m) && i < m.rows && j < m.cols);
m.data[i * m.cols + j] = s;
}

pub fn get_matrix_element_impl(m: &Matrix, i: usize, j: usize) -> f64 {
assert!(assert_matrix_impl(m) && i < m.rows && j < m.cols);
m.data[i * m.cols + j]
}

pub fn set_row_vector_impl(m: &mut Matrix, i: usize, v: &Vector) {
assert!(assert_matrix_impl(m) && crate::vector::assert_vector_impl(v) && i < m.rows && v.cols == m.cols);
for j in 0..v.cols {
m.data[i * m.cols + j] = v.data[j];
}
}

pub fn get_row_vector_impl(m: &Matrix, i: usize) -> Vector {
assert!(assert_matrix_impl(m) && i < m.rows);
let mut row = vec![0.0; m.cols];
for (j, item) in row.iter_mut().enumerate().take(m.cols) {
*item = m.data[i * m.cols + j];
}
crate::vector::new_vector_impl(&row, m.cols)
}

pub fn set_col_vector_impl(m: &mut Matrix, j: usize, v: &Vector) {
assert!(assert_matrix_impl(m) && crate::vector::assert_vector_impl(v) && j < m.cols && v.cols == m.rows);
for i in 0..v.cols {
m.data[i * m.cols + j] = v.data[i];
}
}

pub fn get_col_vector_impl(m: &Matrix, j: usize) -> Vector {
assert!(assert_matrix_impl(m) && j < m.cols);
let mut col = vec![0.0; m.rows];
for (i, item) in col.iter_mut().enumerate().take(m.rows) {
*item = m.data[i * m.cols + j];
}
crate::vector::new_vector_impl(&col, m.rows)
}

pub fn get_main_diagonal_impl(m: &Matrix) -> Vector {
assert!(is_square_matrix_impl(m));
let mut diag = vec![0.0; m.rows];
for (x, item) in diag.iter_mut().enumerate().take(m.rows) {
*item = m.data[x * m.cols + x];
}
crate::vector::new_vector_impl(&diag, m.rows)
}

pub fn set_main_diagonal_impl(m: &mut Matrix, v: &Vector) {
assert!(
is_square_matrix_impl(m)
&& crate::vector::assert_vector_impl(v)
&& m.rows == m.cols
&& m.cols == v.cols
);
for x in 0..v.cols {
m.data[x * m.cols + x] = v.data[x];
}
}

pub fn get_anti_diagonal_impl(m: &Matrix) -> Vector {
assert!(is_square_matrix_impl(m));
let mut x = 0usize;
let mut diag = vec![0.0; m.rows];
for i in (0..m.rows).rev() {
for j in (0..m.cols).rev() {
if i + j == m.rows - 1 {
diag[x] = m.data[i * m.cols + j];
x += 1;
if x == m.rows {
break;
}
}
}
}
crate::vector::new_vector_impl(&diag, m.rows)
}

pub fn set_anti_diagonal_impl(m: &mut Matrix, v: &Vector) {
assert!(
is_square_matrix_impl(m)
&& crate::vector::assert_vector_impl(v)
&& m.rows == m.cols
&& m.cols == v.cols
);
let mut idx = 0usize;
for i in (0..m.rows).rev() {
for j in (0..m.cols).rev() {
if i + j == m.rows - 1 {
m.data[i * m.cols + j] = v.data[idx];
idx += 1;
if idx == m.rows {
break;
}
}
}
}
}

pub fn diagonal_product_impl(m: &Matrix) -> f64 {
let diagonal = get_main_diagonal_impl(m);
let mut product = 1.0;
for i in 0..diagonal.cols {
product *= diagonal.data[i];
}
product
}

pub fn print_matrix_impl(m: &Matrix, include_indices: bool) {
assert_matrix_impl(m);
for i in 0..m.rows {
for j in 0..m.cols {
if include_indices {
print!("[{},{}] -> ", i, j);
}
print!("{:16.8} ", m.data[i * m.cols + j]);
}
println!();
}
}

pub fn is_matrix_equal_impl(m: &Matrix, n: &Matrix) -> bool {
assert!(assert_matrix_impl(m) && assert_matrix_impl(n));
if m.rows != n.rows || m.cols != n.cols {
return false;
}
for i in 0..m.rows {
for j in 0..m.cols {
if m.data[i * m.cols + j] != n.data[i * n.cols + j] {
return false;
}
}
}
true
}

pub fn has_same_dimensions_impl(m: &Matrix, n: &Matrix) -> bool {
assert!(assert_matrix_impl(m) && assert_matrix_impl(n));
(m.rows == n.rows) && (m.cols == n.cols)
}

pub fn is_zero_matrix_impl(m: &Matrix) -> bool {
assert_matrix_impl(m);
for i in 0..m.rows {
for j in 0..m.cols {
if m.data[i * m.cols + j] != 0.0 {
return false;
}
}
}
true
}

pub fn is_identity_matrix_impl(m: &Matrix) -> bool {
if !is_square_matrix_impl(m) {
return false;
}
for i in 0..m.rows {
for j in 0..m.cols {
if i == j && m.data[i * m.cols + j] != 1.0 {
return false;
} else if i != j && m.data[i * m.cols + j] != 0.0 {
return false;
}
}
}
true
}

pub fn is_square_matrix_impl(m: &Matrix) -> bool {
assert_matrix_impl(m);
m.rows == m.cols
}

pub fn is_invertible_impl(m: &Matrix) -> bool {
is_square_matrix_impl(m) && determinant_impl(m) != 0.0
}

pub fn is_diagonal_matrix_impl(m: &Matrix) -> bool {
assert!(is_square_matrix_impl(m));
for i in 0..m.rows {
for j in 0..m.cols {
if i != j && m.data[i * m.cols + j] != 0.0 {
return false;
}
}
}
true
}

pub fn is_triangular_matrix_impl(m: &Matrix) -> bool {
assert!(is_square_matrix_impl(m));
exclusive_or(is_up_tri_matrix_impl(m), is_lo_tri_matrix_impl(m))
}

pub fn is_up_tri_matrix_impl(m: &Matrix) -> bool {
assert!(is_square_matrix_impl(m));
for i in 0..m.rows {
for j in 0..i {
if m.data[i * m.cols + j] != 0.0 {
return false;
}
}
}
true
}

pub fn is_lo_tri_matrix_impl(m: &Matrix) -> bool {
assert!(is_square_matrix_impl(m));
for i in 0..m.rows {
for j in (i + 1)..m.cols {
if m.data[i * m.cols + j] != 0.0 {
return false;
}
}
}
true
}

pub fn is_matrix_symmetric_impl(m: &Matrix) -> bool {
let t = transpose_matrix_impl(m);
is_matrix_equal_impl(m, &t)
}

pub fn has_zero_row_impl(m: &Matrix) -> bool {
assert_matrix_impl(m);
let mut all_zeroes = true;
for i in 0..m.rows {
for j in 0..m.cols {
if m.data[i * m.cols + j] != 0.0 {
all_zeroes = false;
}
}
if all_zeroes {
return true;
}
all_zeroes = true;
}
false
}

pub fn has_zero_col_impl(m: &Matrix) -> bool {
assert_matrix_impl(m);
let mut all_zeroes = true;
for j in 0..m.cols {
for i in 0..m.rows {
if m.data[i * m.cols + j] != 0.0 {
all_zeroes = false;
}
}
if all_zeroes {
return true;
}
all_zeroes = true;
}
false
}

pub fn transpose_matrix_impl(m: &Matrix) -> Matrix {
assert_matrix_impl(m);
let mut t = zero_matrix_impl(m.cols, m.rows);
for i in 0..m.rows {
for j in 0..m.cols {
t.data[j * t.cols + i] = m.data[i * m.cols + j];
}
}
t
}

pub fn trace_matrix_impl(m: &Matrix) -> f64 {
assert!(is_square_matrix_impl(m));
let mut trace = 0.0;
for i in 0..m.rows {
trace += m.data[i * m.cols + i];
}
trace
}

pub fn add_matrices_impl(m1: &Matrix, m2: &Matrix) -> Matrix {
assert!(has_same_dimensions_impl(m1, m2));
let mut sum = null_matrix_impl(m1.rows, m1.cols);
let mut idx = 0usize;
for _i in 0..m1.rows {
for _j in 0..m1.cols {
sum.data[idx] = m1.data[idx] + m2.data[idx];
idx += 1;
}
}
sum
}

pub fn pow_matrix_impl(m: &Matrix, k: f64) -> Matrix {
assert_matrix_impl(m);
let mut p = null_matrix_impl(m.rows, m.cols);
for i in 0..m.rows {
for j in 0..m.cols {
p.data[i * m.cols + j] = m.data[i * m.cols + j].powf(k);
}
}
p
}

pub fn multiply_matrices_impl(m1: &Matrix, m2: &Matrix) -> Matrix {
assert!(assert_matrix_impl(m1) && assert_matrix_impl(m2) && m1.cols == m2.rows);
let mut prod = null_matrix_impl(m1.rows, m2.cols);
for i in 0..m1.rows {
for j in 0..m2.cols {
let mut val = 0.0;
for k in 0..m1.cols {
val += m1.data[i * m1.cols + k] * m2.data[k * m2.cols + j];
}
prod.data[i * prod.cols + j] = val;
}
}
prod
}

pub fn scale_matrix_impl(m: &Matrix, s: f64) -> Matrix {
assert_matrix_impl(m);
let mut scaled = null_matrix_impl(m.rows, m.cols);
for i in 0..m.rows {
for j in 0..m.cols {
scaled.data[i * m.cols + j] = m.data[i * m.cols + j] * s;
}
}
scaled
}

pub fn sub_matrix_impl(m: &Matrix, i: usize, j: usize) -> Matrix {
assert!(assert_matrix_impl(m) && i < m.rows && j < m.cols);
assert!(m.rows > 1 && m.cols > 1);
let mut sm = null_matrix_impl(m.rows - 1, m.cols - 1);
let mut idx = 0usize;
for row in 0..m.rows {
for col in 0..m.cols {
if row != i && col != j {
sm.data[idx] = m.data[row * m.cols + col];
idx += 1;
}
}
}
sm
}

pub fn element_minor_impl(m: &Matrix, i: usize, j: usize) -> f64 {
let sm = sub_matrix_impl(m, i, j);
determinant_impl(&sm)
}

pub fn matrix_minor_impl(m: &Matrix) -> Matrix {
assert_matrix_impl(m);
let mut mm = null_matrix_impl(m.rows, m.cols);
for i in 0..mm.rows {
for j in 0..mm.cols {
mm.data[i * mm.cols + j] = element_minor_impl(m, i, j);
}
}
mm
}

pub fn element_cofactor_impl(m: &Matrix, i: usize, j: usize) -> f64 {
(-1.0f64).powf((i + 1 + j + 1) as f64) * element_minor_impl(m, i, j)
}

pub fn matrix_cofactor_impl(m: &Matrix) -> Matrix {
assert_matrix_impl(m);
let mut cfm = null_matrix_impl(m.rows, m.cols);
for i in 0..cfm.rows {
for j in 0..cfm.cols {
cfm.data[i * cfm.cols + j] = element_cofactor_impl(m, i, j);
}
}
cfm
}

pub fn sign_matrix_impl(rows: usize, cols: usize) -> Matrix {
assert!(rows > 0 && cols > 0);
let mut sm = null_matrix_impl(rows, cols);
fill_matrix_impl(&mut sm, 1.0);
for i in 0..sm.rows {
for j in 0..sm.cols {
sm.data[i * sm.cols + j] = if ((i * sm.cols + j) + 1) % 2 == 1 {
1.0
} else {
-1.0
};
}
}
sm
}

pub fn adjugate_matrix_impl(m: &Matrix) -> Matrix {
assert_matrix_impl(m);
let mm = matrix_minor_impl(m);
let sign = sign_matrix_impl(m.rows, m.cols);
multiply_matrices_impl(&mm, &sign)
}

pub fn lu_decomposition_impl(m: &Matrix) -> (Matrix, Matrix, Matrix, i32) {
assert!(is_square_matrix_impl(m));
let n = m.cols;
let mut l = zero_matrix_impl(n, n);
let mut u = zero_matrix_impl(n, n);
let (p, swaps) = pivot_matrix_impl(m);
let m2 = multiply_matrices_impl(&p, m);

for j in 0..n {
l.data[j * n + j] = 1.0;
for i in 0..(j + 1) {
let mut sum_u = 0.0;
for k in 0..i {
sum_u += u.data[k * n + j] * l.data[i * n + k];
}
u.data[i * n + j] = m2.data[i * n + j] - sum_u;
}
for i in j..n {
let mut sum_l = 0.0;
for k in 0..j {
sum_l += u.data[k * n + j] * l.data[i * n + k];
}
l.data[i * n + j] = (m2.data[i * n + j] - sum_l) / u.data[j * n + j];
}
}

(l, u, p, swaps)
}

pub fn inverse_matrix_impl(m: &Matrix) -> Matrix {
assert!(is_invertible_impl(m));
let adj = adjugate_matrix_impl(m);
scale_matrix_impl(&adj, 1.0 / determinant_impl(m))
}

pub fn pivot_matrix_impl(m: &Matrix) -> (Matrix, i32) {
assert!(is_square_matrix_impl(m));
let n = m.cols;
let mut pivot = identity_matrix_impl(n);
let mut swaps = 0i32;

for i in 0..n {
let mut max = m.data[i * n + i];
let mut row = i;
for j in i..n {
if m.data[j * n + i] > max {
max = m.data[j * n + i];
row = j;
}
}
if i != row {
let v = get_row_vector_impl(&pivot, i);
let w = get_row_vector_impl(&pivot, row);
set_row_vector_impl(&mut pivot, i, &w);
set_row_vector_impl(&mut pivot, row, &v);
swaps += 1;
}
}

(pivot, swaps)
}

pub fn scale_n_space_impl(m: &Matrix, k: f64) -> Matrix {
assert!(is_square_matrix_impl(m));
let mut n = zero_matrix_impl(m.cols, m.cols);
let mut v = crate::vector::zero_vector_impl(m.cols);
crate::vector::fill_vector_impl(&mut v, k);
set_main_diagonal_impl(&mut n, &v);
multiply_matrices_impl(m, &n)
}

pub fn reflect_axis_2d_impl(m: &Matrix, axis: i32) -> Matrix {
assert!(is_square_matrix_impl(m) && m.cols == 2);
let mut n = zero_matrix_impl(2, 2);
set_matrix_element_impl(&mut n, 0, 0, if axis != 0 { 1.0 } else { -1.0 });
set_matrix_element_impl(&mut n, 1, 1, if axis != 0 { -1.0 } else { 1.0 });
multiply_matrices_impl(m, &n)
}

pub fn reflect_axis_3d_impl(m: &Matrix, axis: i32) -> Matrix {
assert!(is_square_matrix_impl(m) && m.cols == 3 && (0..=2).contains(&axis));
let mut n = zero_matrix_impl(3, 3);
match axis {
0 => {
let data = [1.0, -1.0, -1.0];
let v = crate::vector::new_vector_impl(&data, 3);
set_main_diagonal_impl(&mut n, &v);
}
1 => {
let data = [-1.0, 1.0, -1.0];
let v = crate::vector::new_vector_impl(&data, 3);
set_main_diagonal_impl(&mut n, &v);
}
_ => {
let data = [-1.0, -1.0, 1.0];
let v = crate::vector::new_vector_impl(&data, 3);
set_main_diagonal_impl(&mut n, &v);
}
}
multiply_matrices_impl(m, &n)
}

pub fn orth_proj_2d_impl(m: &Matrix, axis: i32) -> Matrix {
assert!(is_square_matrix_impl(m) && m.cols == 2 && (0..=1).contains(&axis));
let mut n = zero_matrix_impl(2, 2);
set_matrix_element_impl(&mut n, axis as usize, axis as usize, 1.0);
multiply_matrices_impl(m, &n)
}

pub fn orth_proj_3d_impl(m: &Matrix, axis: i32) -> Matrix {
assert!(is_square_matrix_impl(m) && m.cols == 3);
let mut n = zero_matrix_impl(3, 3);
match axis {
0 => {
set_matrix_element_impl(&mut n, 0, 0, 1.0);
set_matrix_element_impl(&mut n, 1, 1, 1.0);
}
1 => {
set_matrix_element_impl(&mut n, 0, 0, 1.0);
set_matrix_element_impl(&mut n, 2, 2, 1.0);
}
2 => {
set_matrix_element_impl(&mut n, 1, 1, 1.0);
set_matrix_element_impl(&mut n, 2, 2, 1.0);
}
_ => {}
}
multiply_matrices_impl(m, &n)
}

pub fn shear_2d_impl(m: &Matrix, k: f64, axis: i32) -> Matrix {
assert!(is_square_matrix_impl(m) && m.cols == 2);
let mut n = zero_matrix_impl(2, 2);
set_matrix_element_impl(&mut n, 0, 0, 1.0);
set_matrix_element_impl(&mut n, 0, 1, if axis != 0 { k } else { 0.0 });
set_matrix_element_impl(&mut n, 1, 0, if axis != 0 { 0.0 } else { k });
set_matrix_element_impl(&mut n, 1, 1, 1.0);
multiply_matrices_impl(m, &n)
}

pub fn determinant_impl(m: &Matrix) -> f64 {
assert!(is_square_matrix_impl(m));
match m.rows {
1 => return m.data[0],
2 => return (m.data[0] * m.data[3]) - (m.data[1] * m.data[2]),
3 => {
return (m.data[0] * ((m.data[4] * m.data[8]) - (m.data[5] * m.data[7])))
- (m.data[1] * ((m.data[3] * m.data[8]) - (m.data[5] * m.data[6])))
+ (m.data[2] * ((m.data[3] * m.data[7]) - (m.data[4] * m.data[6])));
}
_ => {}
}

if is_triangular_matrix_impl(m) {
return diagonal_product_impl(m);
}

let (l, u, _p, swaps) = lu_decomposition_impl(m);
(-1.0f64).powf((swaps - 1) as f64) * determinant_impl(&l) * determinant_impl(&u)
}
