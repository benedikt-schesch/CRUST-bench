// input_toolbox.rs
use crate::model_system::SystemNN;
use std::io::{self, Write};

/// Input preparation for NN type one.
pub fn typeOne(data: &mut [f32]) {
data[4] = (data[1] - data[9]) / data[0];
data[5] += data[1];
data[9] = data[1];

data[8] = (data[2] - data[10]) / data[0];
data[10] = data[2];

data[6] = (data[3] - data[11]) / data[0];
data[7] = (data[6] - data[12]) / data[0];
data[11] = data[3];
data[12] = data[6];
}
/// Input preparation for NN type two.
pub fn typeTwo(_data: &mut [f32]) {
}
/// Constructs input arrays for type one networks.
fn makeInputDataSystem(system_nn: &mut SystemNN, size: usize, full: usize) {
system_nn.input_data = vec![0.0; full];
system_nn.input_data_size = vec![full as i32, 1, (size + 1) as i32];
system_nn.input_types = vec![0, 1, 2, 4, 5, 9, 10, 6];
}
/// Constructs input arrays for type two networks.
fn makeInputDataSystemTwo(system_nn: &mut SystemNN, size: usize, full: usize) {
system_nn.input_data = vec![0.0; full];
system_nn.input_data_size = vec![full as i32, 1, (size + 1) as i32];
system_nn.input_types = vec![0, 1, 2];
}
/// Lets the user select an input-preparation function and sets up the corresponding system config.
pub fn selectInputNNFunction(func_ptr: &mut Option<fn(&mut [f32])>, system_nn: &mut SystemNN) {
print!("Please select the AF:\n1 - typeOne\n2 - typeTwo (SD)\nSelect: ");
let _ = io::stdout().flush();

let mut input = String::new();
let user_choice = if io::stdin().read_line(&mut input).is_ok() {
input.trim().parse::<i32>().unwrap_or(0)
} else {
0
};

if user_choice == 1 {
*func_ptr = Some(typeOne);
makeInputDataSystem(system_nn, 8, 13);
} else if user_choice == 2 {
*func_ptr = Some(typeTwo);
makeInputDataSystemTwo(system_nn, 3, 4);
}
}
