
use crate::neural_network::{NeuralNetwork, NNInput, fillMatrixesNN};
use crate::pid_controller::PID;
pub trait SystemModel {
fn step(&mut self, input: f64, dt: f64) -> f64;
fn get_output(&self) -> f64;
}
pub struct FirstOrderSystem {
pub gain: f64,
pub time_constant: f64,
pub output: f64,
}
impl SystemModel for FirstOrderSystem {
fn step(&mut self, input: f64, dt: f64) -> f64 {
let derivative = (self.gain * input - self.output) / self.time_constant;
self.output += derivative * dt;
self.output
}
fn get_output(&self) -> f64 {
self.output
}
}
pub struct SystemNN {
pub neural_network: NeuralNetwork,
pub input: NNInput,
}
impl Default for SystemNN {
fn default() -> Self {
SystemNN {
neural_network: NeuralNetwork {
layers: vec![],
activations: vec![],
},
input: NNInput::default(),
}
}
}
pub fn createFirstOrderSystem(gain: f64, time_constant: f64) -> FirstOrderSystem {
FirstOrderSystem {
gain,
time_constant,
output: 0.0,
}
}
pub fn simulate_system(
system: &mut dyn SystemModel,
controller: &mut PID,
setpoint: f64,
duration: f64,
dt: f64,
) -> Vec<f64> {
let steps = (duration / dt) as usize;
let mut outputs = Vec::with_capacity(steps);
controller.set_setpoint(setpoint);
for _ in 0..steps {
let output = system.get_output();
let control = controller.update(output, dt);
system.step(control, dt);
outputs.push(output);
}
outputs
}
pub fn createNNSystem(system: &mut SystemNN, input: &mut NNInput) {
system.input = NNInput {
data: input.data.clone(),
};
}
pub fn createDeNormalization(_system: &mut SystemNN) {
}
pub fn makeSimulationOfSignalNN(_system: &mut SystemNN, _output: &mut Vec<f64>, _flag: bool) {
}
pub fn clearNNSystem(_system: &mut SystemNN) {
}
pub fn pidFitFunction(_pop: &mut crate::population::Pop, _fit: &mut Vec<f64>, _pid: &mut PID) {
}
pub fn nnFitFunction(_pop: &mut crate::population::Pop, _fit: &mut Vec<f64>, _system: &mut SystemNN) {
}
