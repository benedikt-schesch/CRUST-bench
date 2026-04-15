use std::cell::RefCell;

// Simple LCG to mimic C's rand() behavior from stdlib
fn rand() -> i32 {
thread_local! {
static SEED: RefCell<u32> = RefCell::new(1);
}

SEED.with(|seed| {
let mut s = seed.borrow_mut();
// glibc rand() implementation: next = next * 1103515245 + 12345; return (next / 65536) % 32768;
*s = s.wrapping_mul(1103515245).wrapping_add(12345);
((*s / 65536) % 32768) as i32
})
}

pub fn generate_matrix(x: usize, y: usize, random: bool) -> Vec<Vec<f32>> {
let mut matrix = Vec::with_capacity(x);

if !random {
for i in 0..x {
let mut row = Vec::with_capacity(y);
for j in 0..y {
row.push((j + i * x) as f32);
}
matrix.push(row);
}
} else {
for _ in 0..x {
let mut row = Vec::with_capacity(y);
for _ in 0..y {
row.push(rand() as f32);
}
matrix.push(row);
}
}

matrix
}

pub fn free_matrix(matrix: &mut Vec<Vec<f32>>) -> i32 {
matrix.clear();
matrix.shrink_to_fit();
0
}

pub fn multiply(m1: &[Vec<f32>], m2: &[Vec<f32>], result: &mut [Vec<f32>], method: i32) -> i32 {
if m1.is_empty() || m2.is_empty() {
println!("The number of columns in the first matrix must be equal to the number of rows in the second matrix.");
return -1;
}

let x1 = m1.len();
let y1 = m1[0].len();
let x2 = m2.len();

// Check if y1 == x2
if y1 != x2 {
println!("The number of columns in the first matrix must be equal to the number of rows in the second matrix.");
return -1;
}

match method {
1 => {
algorithm1(m1, m2, result);
0
},
2 => {
// algorithm2 assumes square matrices of size x1
let y2 = m2[0].len();
if x1 != y1 || x1 != x2 || x1 != y2 {
println!("Choose the correct method!");
return -1;
}
algorithm2(m1, m2, result);
0
},
3 => {
algorithm3(m1, m2, result);
0
},
_ => {
println!("Choose the correct method!");
-1
}
}
}

pub fn algorithm1(m1: &[Vec<f32>], m2: &[Vec<f32>], result: &mut [Vec<f32>]) {
if m1.is_empty() || m2.is_empty() {
return;
}

let x1 = m1.len();
let y1 = m1[0].len();
let y2 = m2[0].len();

for i in 0..x1 {
for j in 0..y2 {
result[i][j] = 0.0;
for k in 0..y1 {
result[i][j] += m1[i][k] * m2[k][j];
}
}
}
}

pub fn algorithm3(m1: &[Vec<f32>], m2: &[Vec<f32>], result: &mut [Vec<f32>]) {
if m1.is_empty() || m2.is_empty() {
return;
}

let x1 = m1.len();
let y1 = m1[0].len();
let y2 = m2[0].len();

// Initialize result to 0
for i in 0..x1 {
for j in 0..y2 {
result[i][j] = 0.0;
}
}

// Multiply with loop order i-k-j for better cache locality
for i in 0..x1 {
for k in 0..y1 {
let m1_ik = m1[i][k];
for j in 0..y2 {
result[i][j] += m1_ik * m2[k][j];
}
}
}
}

pub fn algorithm2(m1: &[Vec<f32>], m2: &[Vec<f32>], result: &mut [Vec<f32>]) {
if m1.is_empty() {
return;
}

let size = m1.len();

// Create transposed version of m2 (bb in C code)
let mut b = vec![vec![0.0; size]; size];
for i in 0..size {
for j in 0..size {
b[i][j] = m2[j][i];
}
}

// Initialize result to 0
for i in 0..size {
for j in 0..size {
result[i][j] = 0.0;
}
}

// Perform multiplication: c[i][k] = sum_j a[i][j] * b[k][j]
// This is the scalar equivalent of the SIMD implementation
for i in 0..size {
for j in 0..size {
let a_val = m1[i][j];
for k in 0..size {
result[i][k] += a_val * b[k][j];
}
}
}
}
