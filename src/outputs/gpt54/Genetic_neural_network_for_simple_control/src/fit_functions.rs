use crate::fit_functions;
use crate::population;
use crate::pid_controller;
use crate::signal_designer;
use crate::model_system;
use crate::neural_network;
use std::fs::File;

pub fn pidFitFunction(population: &mut population::Pop, fit: &mut Vec<f32>, pid: &mut pid_controller::PID) {
let csv = 0;
let path = std::env::temp_dir().join("pid_fit_function_trash.csv");
let mut trash = File::create(path).unwrap();

if fit.len() < population.rows {
fit.resize(population.rows, 0.0);
}

for i in 0..population.rows {
if let Some(data) = pid.dataSystem.as_mut() {
for v in data.iter_mut() {
*v = 0.0;
}
}

pid.Kp = population.pop[i][0];
pid.Ki = population.pop[i][1];
pid.Kd = population.pop[i][2];
pid.tauD = population.pop[i][3];

pid_controller::makeSimulationOfSignal(pid, &mut trash, csv);
fit[i] = pid.fit;
}
}

pub fn nnFitFunction(population: &mut population::Pop, fit: &mut Vec<f32>, nn: &mut model_system::SystemNN){
let path = std::env::temp_dir().join("nn_fit_function_trash.csv");
let mut trash = std::io::BufWriter::new(std::fs::File::create(path).unwrap());

if fit.len() < population.rows {
fit.resize(population.rows, 0.0);
}

for i in 0..population.rows {
for v in nn.data_system.iter_mut() {
*v = 0.0;
}

neural_network::fillMatrixesNN(&mut nn.neural_network, &population.pop[i]);
model_system::makeSimulationOfSignalNN(nn, &mut trash, false);
fit[i] = nn.fit;
}
}
