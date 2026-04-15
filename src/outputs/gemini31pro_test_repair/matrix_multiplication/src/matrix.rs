pub fn generate_matrix(x: usize, y: usize, random: bool) -> Vec<Vec<f32>> {
let mut matrix = Vec::with_capacity(x);
let mut seed = 12345u32;
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
seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
let rand_val = (seed / 65536) % 32768;
row.push(rand_val as f32);
}
matrix.push(row);
}
}
matrix
}
pub fn free_matrix(matrix: &mut Vec<Vec<f32>>) -> i32 {
matrix.clear();
0
}
pub fn multiply(m1: &[Vec<f32>], m2: &[Vec<f32>], result: &mut [Vec<f32>], method: i32) -> i32 {
let x1 = m1.len();
let y1 = if x1 > 0 { m1[0].len() } else { 0 };
let x2 = m2.len();
let _y2 = if x2 > 0 { m2[0].len() } else { 0 };
if y1 != x2 {
println!("The number of columns in the first matrix must be equal to the number of rows in the second matrix. ");
return -1;
}
match method {
1 => {
algorithm1(m1, m2, result);
}
2 => {
algorithm2(m1, m2, result);
}
3 => {
algorithm3(m1, m2, result);
}
_ => {
println!("Choose the correct method!");
return -1;
}
}
0
}
pub fn algorithm1(m1: &[Vec<f32>], m2: &[Vec<f32>], result: &mut [Vec<f32>]) {
let x1 = m1.len();
let y1 = if x1 > 0 { m1[0].len() } else { 0 };
let y2 = if m2.len() > 0 { m2[0].len() } else { 0 };
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
let x1 = m1.len();
let y1 = if x1 > 0 { m1[0].len() } else { 0 };
let y2 = if m2.len() > 0 { m2[0].len() } else { 0 };
for i in 0..x1 {
for j in 0..y2 {
result[i][j] = 0.0;
}
}
for i in 0..x1 {
for k in 0..y1 {
for j in 0..y2 {
result[i][j] += m1[i][k] * m2[k][j];
}
}
}
}
pub fn algorithm2(m1: &[Vec<f32>], m2: &[Vec<f32>], result: &mut [Vec<f32>]) {
let size = m1.len();
let mut b = vec![vec![0.0; size]; size];
for i in 0..size {
for j in 0..size {
b[i][j] = m2[j][i];
result[i][j] = 0.0;
}
}
for i in 0..size {
let mut j = 0;
while j < size {
let j_end = (j + 8).min(size);
for k in 0..size {
let mut sum = 0.0;
for jj in j..j_end {
sum += m1[i][jj] * b[k][jj];
}
result[i][k] += sum;
}
j += 8;
}
}
}
