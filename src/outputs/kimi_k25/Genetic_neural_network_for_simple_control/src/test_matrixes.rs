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
// Create matrices using new() instead of struct literal
let mut m1 = Matrix::new(2, 2);
m1.set(0, 0, 1.0);
m1.set(1, 1, 1.0);

let m2 = Matrix::new(2, 2);

let m3 = matrixMul(&m1, &m2);
println!("Matrix multiplication completed");

// Test matrixSubstAdd with correct signature (3 arguments, first is &mut)
let mut a = Matrix::new(2, 2);
a.set(0, 0, 1.0);
a.set(0, 1, 2.0);
a.set(1, 0, 3.0);
a.set(1, 1, 4.0);

let b = Matrix::new(2, 2);

// Test addition (subtract = false)
matrixSubstAdd(&mut a, &b, false);

// Test subtraction (subtract = true)
matrixSubstAdd(&mut a, &b, true);

// Test matrixAllValuesFormula
matrixAllValuesFormula(&mut a, sigmoid);

// Test tanh (tangenth -> tanh)
let _func_ptr_tanh = tanh;
let _func_ptr_sigm = sigmoid;

// Test with f64 instead of f32
let _result = vec![
vec![sigmoid(1.0), sigmoid(1.0)],
vec![sigmoid(2.0), sigmoid(2.0)],
];

// Test Matrix clone
let original = Matrix::new(2, 2);
let _copied = original.clone();
}
