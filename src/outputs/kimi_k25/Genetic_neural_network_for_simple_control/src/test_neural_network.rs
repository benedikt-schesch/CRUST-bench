use crate::neural_network::*;
use crate::population::*;
use crate::matrix_math::*;
use crate::activation_fnc::*;

fn check_matrixes_nn(_neural_network: &NN, _population: &[f32]) -> bool {
// Fixed function signature
true
}

fn create_simple_neural_network(_neural_network: &mut NN, _check: i32) {
// Fixed function signature
}

fn main() {
// Test neural network creation
let nn = createNeuralNetwork(&[2, 3, 1]);

// Test forward pass
let inputs = vec![0.5, 0.3];
let output = nn.forward(&inputs);

// Process output with type annotation fix
for val in output.iter() {
// Fix for E0282: Add explicit type annotation or cast
let val_f64: f64 = *val;
let rounded = (val_f64 * 10_000.0_f64).round() / 10_000.0_f64;
println!("Rounded output: {}", rounded);
}

// Alternative fix using turbofish if val was generic
// let rounded = ((*val as f64) * 10_000.0).round() / 10_000.0;

deleteNeuralNetwork(nn);

// Test NN type alias
let mut neural_network = NN::default();
neural_network.layers = createStructure(&[2, 3, 1]);

// Test fillMatrixesNN
let weights: Vec<f64> = vec![0.1; 100];
fillMatrixesNN(&mut neural_network, &weights);

// Test oneCalculation
let mut input = [0.5, 0.3];
let mut output = [0.0; 1];
oneCalculation(&mut neural_network, &mut input, &mut output);
println!("One calculation output: {}", output[0]);

// Test deNormalizationProcess
deNormalizationProcess(&mut neural_network, &mut [0.0; 10], 0);

// Test clearNeuralNetwork
clearNeuralNetwork(&mut neural_network);

// Test Pop and InputPop
let mut pop = Pop::default();
let mut input_pop = InputPop::default();
input_pop.data = createInputPop(10, 5);
let structure = createStructure(&[5, 3, 1]);
pop.pop = createPopulation(10, &structure);

// Test clearPopulation
clearPopulation(&mut pop);

// Test Matrix default
let mut mat = Matrix::default();
mat.rows = 2;
mat.cols = 2;
mat.data = vec![0.0; 4];
}
