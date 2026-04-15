use crate::population;
#[derive(Debug, Clone, Default)]
pub struct Matrix {
pub matrix: Vec<Vec<f32>>,
pub sizes: Vec<usize>,
}
impl Matrix{
pub fn createMatrix(_matrix: Matrix, sizes: Vec<usize>) -> Matrix{
let rows = sizes[0];
let cols = sizes[1];
Matrix {
matrix: vec![vec![0.0; cols]; rows],
sizes: vec![rows, cols],
}
}
pub fn createMatrixFromPointer(input: &[f32], sizes: Vec<usize>) -> Matrix{
let rows = sizes[0];
let cols = sizes[1];
let mut matrix = vec![vec![0.0; cols]; rows];
let mut global_index = 0usize;
for i in 0..rows {
for j in 0..cols {
matrix[i][j] = input[global_index];
global_index += 1;
}
}
Matrix { matrix, sizes: vec![rows, cols] }
}
pub fn matrixDelete(&mut self){
self.matrix.clear();
self.sizes.clear();
}
pub fn matrixDeleteOnlyData(&mut self){
self.matrix.clear();
self.sizes.clear();
}
}
pub fn matrixMultiply(a: &Matrix, b: &Matrix) -> Matrix{
if a.sizes[1] != b.sizes[0] {
panic!("Error: Sizes of matrixes are incorect");
}
let rows = a.sizes[0];
let cols = b.sizes[1];
let mut output = Matrix::createMatrix(Matrix::default(), vec![rows, cols]);
for i in 0..b.sizes[1] {
for y in 0..a.sizes[0] {
output.matrix[y][i] = 0.0;
for x in 0..a.sizes[1] {
output.matrix[y][i] += a.matrix[y][x] * b.matrix[x][i];
}
}
}
output
}
pub fn fullCopyMatrix(a: &Matrix) -> Matrix{
a.clone()
}
pub fn matrixSubstAdd(a: &Matrix, b: &Matrix, operation: i32) -> Matrix{
let mut output = Matrix::createMatrix(Matrix::default(), vec![a.sizes[0], b.sizes[1]]);
for i in 0..output.sizes[0] {
for y in 0..output.sizes[1] {
if operation == 0 {
output.matrix[i][y] = a.matrix[i][y] - b.matrix[i][y];
} else {
output.matrix[i][y] = a.matrix[i][y] + b.matrix[i][y];
}
}
}
output
}
pub fn matrixAllValuesFormula(a: &mut Matrix, formula: fn(f32)->f32) -> Matrix{
for i in 0..a.sizes[0] {
for y in 0..a.sizes[1] {
a.matrix[i][y] = formula(a.matrix[i][y]);
}
}
a.clone()
}
