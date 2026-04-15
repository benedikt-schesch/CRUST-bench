use crate::matrix_math::{Matrix, matrixSubstAdd, matrixMul, matrixAllValuesFormula};
use crate::activation_fnc::*;
fn compare_matrices(a: &Matrix, b: &Matrix) -> bool {
if a.rows != b.rows || a.cols != b.cols {
return false;
}
for i in 0..a.data.len() {
if (a.data[i] - b.data[i]).abs() > f64::EPSILON {
return false;
}
}
true
}
fn main() {
let mut m1 = Matrix::new(2, 2);
m1.set(0, 0, 1.0);
m1.set(1, 1, 1.0);
let m2 = Matrix::new(2, 2);
let m3 = matrixMul(&m1, &m2);
println!("Matrix multiplication completed");
let mut a = Matrix::new(2, 2);
a.set(0, 0, 1.0);
a.set(0, 1, 2.0);
a.set(1, 0, 3.0);
a.set(1, 1, 4.0);
let b = Matrix::new(2, 2);
matrixSubstAdd(&mut a, &b, false);
matrixSubstAdd(&mut a, &b, true);
matrixAllValuesFormula(&mut a, sigmoid);
let _func_ptr_tanh = tanh;
let _func_ptr_sigm = sigmoid;
let _result = vec![
vec![sigmoid(1.0), sigmoid(1.0)],
vec![sigmoid(2.0), sigmoid(2.0)],
];
let original = Matrix::new(2, 2);
let _copied = original.clone();
}
