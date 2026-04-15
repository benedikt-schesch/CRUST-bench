// Generated Rust Code
use crate::neural_network::{NN, NNInput};
use crate::signal_designer::Signal;
use std::io::Write;

#[derive(Default)]
/// Represents a neural network-based system with simulation data and signal processing.
pub struct SystemNN {
pub input_sys: Option<fn(&mut [f32])>,
pub input_data: Vec<f32>,
pub input_data_size: Vec<i32>,
pub input_types: Vec<usize>,
pub signal: Option<Box<Signal>>,
pub output: Option<Box<Signal>>,
pub data_system: Vec<f32>,
pub size_data_system: usize,
pub fit: f32,
pub max_counter: i32,
pub steady_rise_check: i32,
pub neural_network: Box<NN>,
pub func_system: Option<fn(&mut [f32]) -> f32>,
pub max_sys: f32,
pub min_sys: f32,
}

/// Creates and initializes a SystemNN and its neural network.
pub fn createNNSystem(system_nn: &mut SystemNN, input: &mut NNInput) {
let mut selected_input_sys: Option<fn(&mut [f32])> = None;
crate::input_toolbox::selectInputNNFunction(&mut selected_input_sys, system_nn);
system_nn.input_sys = selected_input_sys;

input.neuronsSize[0] = (system_nn.input_data_size[2] - 1) as usize;

input.normalizationMatrix = [
vec![0.0; input.neuronsSize[0]],
vec![0.0; input.neuronsSize[0]],
];
input.denormalizationMatrix = [vec![0.0; 1], vec![0.0; 1]];

let mut nn = NN::default();
crate::neural_network::createNeuralNetwork(std::mem::take(input), &mut nn);
system_nn.neural_network = Box::new(nn);

let mut signal = Signal::default();
crate::signal_designer::cliSignalSelector(&mut signal);
system_nn.signal = Some(Box::new(signal));

let size = crate::systems_builder::selectSystem(&mut system_nn.func_system) as usize;
system_nn.data_system = vec![0.0; size];
system_nn.size_data_system = size;

let sig = system_nn.signal.as_ref().unwrap();
system_nn.output = Some(Box::new(Signal {
length: sig.length,
dt: sig.dt,
signal: vec![0.0; sig.signal.len()],
}));
}

/// Frees memory associated with a SystemNN.
pub fn clearNNSystem(system_nn: &mut SystemNN) {
let nn = std::mem::take(&mut system_nn.neural_network);
crate::neural_network::clearNeuralNetwork(*nn);

if let Some(mut s) = system_nn.signal.take() {
crate::signal_designer::deleteSignal(&mut s);
}
if let Some(mut o) = system_nn.output.take() {
crate::signal_designer::deleteSignal(&mut o);
}

system_nn.data_system.clear();
system_nn.input_data.clear();
system_nn.input_data_size.clear();
}

/// Resets system simulation state for a clean run.
pub fn cleanNNSystem(system_nn: &mut SystemNN) {
if let Some(output) = system_nn.output.as_mut() {
for v in output.signal.iter_mut() {
*v = 0.0;
}
}

for v in system_nn.data_system.iter_mut() {
*v = 0.0;
}
if system_nn.data_system.len() > 1 {
let dt = system_nn.signal.as_ref().unwrap().dt;
system_nn.data_system[1] = dt;
}

for v in system_nn.input_data.iter_mut() {
*v = 0.0;
}
if !system_nn.input_data.is_empty() {
system_nn.input_data[0] = system_nn.signal.as_ref().unwrap().dt;
}

system_nn.steady_rise_check = 0;
system_nn.max_counter = 0;
system_nn.fit = 0.0;
}

/// Analyzes signal to find its maximum and minimum values.
pub fn getMaxMinSignalValues(signal: &Signal, max: &mut f32, min: &mut f32) {
*max = 0.0;
*min = 0.0;
for &v in signal.signal.iter() {
if v > *max {
*max = v;
} else if v < *min {
*min = v;
}
}
}

/// Executes one optimization round to find best U input for the system.
pub fn findUOneRound(system_nn: &mut SystemNN, for_values: &[f32; 3], data: &mut [f32; 12]) {
let mut min = f32::MAX;
let mut max_sig = 0.0;
let mut min_sig = 0.0;
getMaxMinSignalValues(system_nn.signal.as_ref().unwrap(), &mut max_sig, &mut min_sig);
let _ = min_sig;

let mut cur_value = for_values[0];
while cur_value < for_values[1] {
let mut max_y = 0.0;
let mut max_dy = 0.0;
let mut max_ddy = 0.0;
let mut max_e = 0.0;
let mut max_de = 0.0;
let mut max_dde = 0.0;

let mut count = 0;
let mut pre_y = 0.0;
let mut pre_dy = 0.0;
let mut iy = 0.0;
let mut pre_u = 0.0;
let mut pre_du = 0.0;
let mut iu = 0.0;
let mut pre_e = 0.0;
let mut pre_de = 0.0;
let mut ie = 0.0;
let mut penalty = 0.0;
let mut sum = 0.0;

for v in system_nn.data_system.iter_mut() {
*v = 0.0;
}
system_nn.data_system[1] = system_nn.signal.as_ref().unwrap().dt;

let dt = system_nn.output.as_ref().unwrap().dt;
let len = system_nn.signal.as_ref().unwrap().length as f32;
let func = system_nn.func_system.unwrap();

let mut j = 0.0;
while j < len {
if j < 1.0 {
system_nn.data_system[0] = 0.0;
} else {
system_nn.data_system[0] = cur_value;
}

let sys_output = func(&mut system_nn.data_system);
if sys_output > system_nn.max_sys || sys_output < system_nn.min_sys {
penalty = 50000.0;
}

let dy = (sys_output - pre_y) / dt;
let ddy = (dy - pre_dy) / dt;
let de = (max_sig - sys_output - pre_e) / dt;
let dde = (de - pre_de) / dt;

pre_y = sys_output;
pre_dy = dy;
pre_e = max_sig - sys_output;
pre_de = de;

if sys_output > max_y {
max_y = sys_output;
}
if pre_e > max_e {
max_e = pre_e;
}
if dy > max_dy {
max_dy = dy;
}
if ddy > max_ddy {
max_ddy = ddy;
}
if de > max_de {
max_de = de;
}
if dde > max_dde {
max_dde = dde;
}

iy += sys_output;
iu += cur_value;
ie += pre_e;

if j > len - 1.0 {
sum += sys_output;
count += 1;
}

let du = cur_value - pre_u;
let _ddu = du - pre_du;
pre_u = cur_value;
pre_du = du;

j += dt;
}

let mut analyzed_value = max_sig - (sum / count as f32) + penalty;
if analyzed_value < 0.0 {
analyzed_value *= -1.0;
}
if iy < 0.0 {
iy *= -1.0;
}
if iu < 0.0 {
iu *= -1.0;
}
if ie < 0.0 {
ie *= -1.0;
}

if analyzed_value < min {
min = analyzed_value;
data[0] = max_e;
data[1] = cur_value;
data[2] = max_y;
data[3] = max_de;
data[4] = max_dde;
data[5] = ie;
data[6] = cur_value / dt;
data[7] = data[7] / dt;
data[8] = iu;
data[9] = max_dy;
data[10] = max_ddy;
data[11] = iy;
}

cur_value += for_values[2];
}
}

/// Calls multiple search passes to fine-tune U for the system and signal.
pub fn findUForSystemAndSignal(system_nn: &mut SystemNN, data: &mut [f32; 12]) {
let for_value_one = [0.0, 10000.0, 1000.0];
findUOneRound(system_nn, &for_value_one, data);

let for_value_two = [data[1] - 100.0, data[1] + 100.0, 10.0];
findUOneRound(system_nn, &for_value_two, data);

let for_value_three = [data[1] - 10.0, data[1] + 10.0, 1.0];
findUOneRound(system_nn, &for_value_three, data);
}

/// Calculates a simple numerical derivative.
pub fn makeDerivation(x: f32, x_t: f32, dt: f32) -> f32 {
(x - x_t) / dt
}

/// Prepares and applies normalization matrices for the SystemNN based on system signal properties.
pub fn createDeNormalization(system_nn: &mut SystemNN) {
let mut normalization = [vec![0.0; 12], vec![0.0; 12]];
let mut data = [0.0f32; 12];

findUForSystemAndSignal(system_nn, &mut data);

for i in 0..12 {
normalization[0][i] = data[i];
normalization[1][i] = data[i] * -1.0;
}

normalization[0][1] *= 10.0;
normalization[1][1] *= 10.0;

let input_count = system_nn.input_data_size[2] as usize - 1;
for i in 0..input_count {
let t = system_nn.input_types[i];
system_nn.neural_network.normalizationMatrix[0][i] = normalization[0][t];
system_nn.neural_network.normalizationMatrix[1][i] = normalization[1][t];
}

system_nn.neural_network.denormalizationMatrix[0][0] = normalization[0][1];
system_nn.neural_network.denormalizationMatrix[1][0] = normalization[1][1];

system_nn.input_types.clear();
}

/// Simulates the system using its neural network and updates fit/error metrics.
pub fn makeSimulationOfSignalNN<W: Write>(system: &mut SystemNN, writer: &mut W, csv: bool) {
cleanNNSystem(system);
crate::neural_network::clearSDMemory(&mut system.neural_network);

let mut neural_output = 0.0f32;
let mut system_output = 0.0f32;
let mut max = 0.0f32;

let signal_len = system.signal.as_ref().unwrap().length as usize;

for i in 1..signal_len {
let prev_output = system.output.as_ref().unwrap().signal[i - 1];
let desired = system.signal.as_ref().unwrap().signal[i];
let error = desired - prev_output;

system.input_data[1] = error;
system.input_data[2] = neural_output;
system.input_data[3] = system_output;

if let Some(f) = system.input_sys {
f(&mut system.input_data);
}

let rows = system.neural_network.neuronsSize[0];
let mut input_matrix = crate::matrix_math::Matrix::createMatrix(
crate::matrix_math::Matrix::default(),
vec![rows, 1],
);
let mut output_matrix = crate::matrix_math::Matrix::default();

let mut matrix_index = 0usize;
for j in system.input_data_size[1] as usize..system.input_data_size[2] as usize {
input_matrix.matrix[matrix_index][0] = system.input_data[j];
matrix_index += 1;
}

crate::neural_network::oneCalculation(
&mut system.neural_network,
&mut input_matrix,
&mut output_matrix,
);

if output_matrix.matrix[0][0] > max {
max = output_matrix.matrix[0][0];
}

system.data_system[0] = output_matrix.matrix[0][0];
neural_output = system.data_system[0];

let func = system.func_system.unwrap();
let mut out_val = func(&mut system.data_system);
if out_val > system.max_sys {
out_val = system.max_sys;
system.max_counter += 1;
} else if out_val < system.min_sys {
out_val = system.min_sys;
system.max_counter += 1;
}

if let Some(output) = system.output.as_mut() {
output.signal[i] = out_val;
}

system_output = out_val;

if csv {
let _ = writeln!(
writer,
"{},{},{},{}",
system.output.as_ref().unwrap().signal[i],
system.signal.as_ref().unwrap().signal[i],
output_matrix.matrix[0][0],
system.output.as_ref().unwrap().signal[i]
);
}

let diff =
(system.signal.as_ref().unwrap().signal[i] - system.output.as_ref().unwrap().signal[i])
.abs();
system.fit += diff;

if system.output.as_ref().unwrap().signal[i - 1]
> system.output.as_ref().unwrap().signal[i]
{
system.steady_rise_check += 1;
}
}

if csv {
let _ = writeln!(writer, "{}", max);
}
}
