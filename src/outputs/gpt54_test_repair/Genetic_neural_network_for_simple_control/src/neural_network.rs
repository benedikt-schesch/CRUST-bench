
use crate::population::Pop;
use crate::matrix_math::Matrix;
#[derive(Default)]
pub struct NN {
pub AW: Vec<Matrix>,
pub BW: Vec<Matrix>,
pub neuronsSize: Vec<usize>,
pub layerNumber: usize,
pub countOfValues: usize,
pub normalizationMatrix: [Vec<f32>; 2],
pub denormalizationMatrix: [Vec<f32>; 2],
pub func_ptr: Option<fn(f32) -> f32>,
pub layerType: Vec<i32>,
pub sdNeuronsTypes: Vec<Vec<u8>>,
pub SDMemory: Vec<Matrix>,
}
#[derive(Default)]
pub struct NNInput {
pub neuronsSize: Vec<usize>,
pub layerNumber: usize,
pub normalizationMatrix: [Vec<f32>; 2],
pub denormalizationMatrix: [Vec<f32>; 2],
pub layerType: Vec<i32>,
pub sdNumber: usize,
}
fn clearNNInput(_input: NNInput) {
}
pub fn createNeuralNetwork(input: NNInput, neural_network: &mut NN) {
neural_network.layerNumber = input.layerNumber;
neural_network.neuronsSize = input.neuronsSize.clone();
neural_network.layerType = input.layerType.clone();
neural_network.sdNeuronsTypes = Vec::with_capacity(input.sdNumber);
neural_network.SDMemory = Vec::with_capacity(input.sdNumber);
let mut global_index_sd = 0usize;
for i in 0..neural_network.layerNumber {
if neural_network.layerType[i] == 1 {
let size = neural_network.neuronsSize[i];
let mut sd_types = vec![0u8; size];
for j in 0..(size / 2) {
sd_types[j] = 0;
}
let mid_index_slice = size / 2 + (size - size / 2) / 2;
for j in (size / 2)..mid_index_slice {
sd_types[j] = 1;
}
for j in mid_index_slice..size {
sd_types[j] = 2;
}
neural_network.sdNeuronsTypes.push(sd_types);
let mut sd_matrix = Matrix::createMatrix(Matrix::default(), vec![size, 1]);
for j in 0..size {
sd_matrix.matrix[j][0] = 0.0;
}
neural_network.SDMemory.push(sd_matrix);
global_index_sd += 1;
}
}
neural_network.normalizationMatrix = [
vec![0.0; neural_network.neuronsSize[0]],
vec![0.0; neural_network.neuronsSize[0]],
];
neural_network.denormalizationMatrix = [
vec![0.0; neural_network.neuronsSize[neural_network.layerNumber - 1]],
vec![0.0; neural_network.neuronsSize[neural_network.layerNumber - 1]],
];
for i in 0..2 {
neural_network.normalizationMatrix[i] = input.normalizationMatrix[i].clone();
neural_network.denormalizationMatrix[i] = input.denormalizationMatrix[i].clone();
}
crate::activation_fnc::selectActivationFunction(&mut neural_network.func_ptr);
clearNNInput(input);
neural_network.AW = Vec::with_capacity(neural_network.layerNumber - 1);
neural_network.BW = Vec::with_capacity(neural_network.layerNumber - 1);
let mut layer_index = 0usize;
neural_network.countOfValues = 0;
for _i in 0..neural_network.layerNumber - 1 {
let sizes_aw = vec![
neural_network.neuronsSize[layer_index + 1],
neural_network.neuronsSize[layer_index],
];
let sizes_bw = vec![neural_network.neuronsSize[layer_index + 1], 1];
neural_network.countOfValues += neural_network.neuronsSize[layer_index + 1]
* neural_network.neuronsSize[layer_index]
+ neural_network.neuronsSize[layer_index + 1];
neural_network.AW.push(Matrix::createMatrix(Matrix::default(), sizes_aw));
neural_network.BW.push(Matrix::createMatrix(Matrix::default(), sizes_bw));
layer_index += 1;
}
neural_network.countOfValues -= neural_network.neuronsSize[layer_index];
let index_bw_last = neural_network.layerNumber - 2;
for i in 0..neural_network.BW[index_bw_last].sizes[0] {
for j in 0..neural_network.BW[index_bw_last].sizes[0] {
if j < neural_network.BW[index_bw_last].matrix[i].len() {
neural_network.BW[index_bw_last].matrix[i][j] = 0.0;
}
}
}
}
pub fn clearNeuralNetwork(_neural_network: NN) {
}
pub fn fillMatrixesNN(neural_network: &mut NN, population: &[f32]) {
let mut global_index = 0usize;
for i in 0..neural_network.layerNumber - 1 {
for x in 0..neural_network.AW[i].sizes[0] {
for y in 0..neural_network.AW[i].sizes[1] {
neural_network.AW[i].matrix[x][y] = population[global_index];
global_index += 1;
}
}
if i < neural_network.layerNumber - 2 {
for x in 0..neural_network.BW[i].sizes[0] {
for y in 0..neural_network.BW[i].sizes[1] {
neural_network.BW[i].matrix[x][y] = population[global_index];
global_index += 1;
}
}
}
}
}
pub fn clearSDMemory(neural_network: &mut NN) {
let mut global_index = 0usize;
for i in 0..neural_network.layerNumber {
if neural_network.layerType[i] == 1 {
for j in 0..neural_network.neuronsSize[i] {
neural_network.SDMemory[global_index].matrix[j][0] = 0.0;
}
global_index += 1;
}
}
}
pub fn deNormalizationProcess(neural_network: &mut NN, input: &mut Matrix, way: i32) {
for i in 0..input.sizes[0] {
let (r_min, r_max, t_min, t_max) = if way == 0 {
(
neural_network.normalizationMatrix[1][i],
neural_network.normalizationMatrix[0][i],
-1.0,
1.0,
)
} else {
(
-1.0,
1.0,
neural_network.denormalizationMatrix[1][i],
neural_network.denormalizationMatrix[0][i],
)
};
input.matrix[i][0] =
((input.matrix[i][0] - r_min) / (r_max - r_min)) * (t_max - t_min) + t_min;
if way == 0 {
if input.matrix[i][0] > 1.0 {
input.matrix[i][0] = 1.0;
} else if input.matrix[i][0] < -1.0 {
input.matrix[i][0] = -1.0;
}
} else if way == 1 {
if input.matrix[i][0] > neural_network.denormalizationMatrix[0][i] {
input.matrix[i][0] = neural_network.denormalizationMatrix[0][i];
} else if input.matrix[i][0] < neural_network.denormalizationMatrix[1][i] {
input.matrix[i][0] = neural_network.denormalizationMatrix[1][i];
}
}
}
}
fn makeSDLayerAction(neural_network: &mut NN, input: &mut Matrix, sd_index: usize, layer_index: usize) {
let mut dsd_memory: Vec<f32> = Vec::new();
for i in 0..neural_network.neuronsSize[layer_index] {
if neural_network.sdNeuronsTypes[sd_index][i] == 2 {
dsd_memory.push(-input.matrix[i][0]);
}
}
let output = crate::matrix_math::matrixSubstAdd(input, &neural_network.SDMemory[sd_index], 1);
let mut index_d = 0usize;
for i in 0..neural_network.neuronsSize[layer_index] {
if neural_network.sdNeuronsTypes[sd_index][i] == 2 {
neural_network.SDMemory[sd_index].matrix[i][0] = dsd_memory[index_d];
index_d += 1;
}
}
for i in 0..neural_network.neuronsSize[layer_index] {
if neural_network.sdNeuronsTypes[sd_index][i] == 1 {
neural_network.SDMemory[sd_index].matrix[i][0] = output.matrix[i][0];
}
}
*input = crate::matrix_math::fullCopyMatrix(&output);
}
pub fn oneCalculation(neural_network: &mut NN, input: &mut Matrix, output: &mut Matrix) {
let mut sd_index = 0usize;
deNormalizationProcess(neural_network, input, 0);
for i in 0..neural_network.layerNumber - 1 {
let temp_output = crate::matrix_math::matrixMultiply(&neural_network.AW[i], input);
*output = temp_output;
let temp_input = crate::matrix_math::matrixSubstAdd(output, &neural_network.BW[i], 0);
*input = temp_input;
let func = neural_network.func_ptr.unwrap();
crate::matrix_math::matrixAllValuesFormula(input, func);
if neural_network.layerType[i + 1] == 1 {
let layer_index = i + 1;
makeSDLayerAction(neural_network, input, sd_index, layer_index);
sd_index += 1;
}
}
*output = crate::matrix_math::fullCopyMatrix(input);
deNormalizationProcess(neural_network, output, 1);
}
