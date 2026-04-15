use crate::neural_network::*;
use crate::population::*;
use crate::matrix_math::*;
use crate::activation_fnc::*;
fn check_matrixes_nn(_neural_network: &NN, _population: &[f32]) -> bool {
true
}
fn create_simple_neural_network(_neural_network: &mut NN, _check: i32) {
}
fn main() {
let nn = createNeuralNetwork(&[2, 3, 1]);
let inputs = vec![0.5, 0.3];
let output = nn.forward(&inputs);
for val in output.iter() {
let val_f64: f64 = *val;
let rounded = (val_f64 * 10_000.0_f64).round() / 10_000.0_f64;
println!("Rounded output: {}", rounded);
}
deleteNeuralNetwork(nn);
let mut neural_network = NN::default();
neural_network.layers = createStructure(&[2, 3, 1]);
let weights: Vec<f64> = vec![0.1; 100];
fillMatrixesNN(&mut neural_network, &weights);
let mut input = [0.5, 0.3];
let mut output = [0.0; 1];
oneCalculation(&mut neural_network, &mut input, &mut output);
println!("One calculation output: {}", output[0]);
deNormalizationProcess(&mut neural_network, &mut [0.0; 10], 0);
clearNeuralNetwork(&mut neural_network);
let mut pop = Pop::default();
let mut input_pop = InputPop::default();
input_pop.data = createInputPop(10, 5);
let structure = createStructure(&[5, 3, 1]);
pop.pop = createPopulation(10, &structure);
clearPopulation(&mut pop);
let mut mat = Matrix::default();
mat.rows = 2;
mat.cols = 2;
mat.data = vec![0.0; 4];
}
