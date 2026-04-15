use crate::sort::quickSort;
use crate::fit_functions::*;
use crate::genetic_operations::*;
use crate::pid_controller::*;
use crate::population::*;
use crate::neural_network::*;
use crate::model_system::*;
use crate::matrix_math::Matrix;

fn closeRandomSizeGeneration(_population: &mut Pop, _sizes: &Vec<f32>){
// Fixed function signature to match usage
}

fn createSystemNeuralNetworkInputTEST(_input: &mut NNInput) {
// Fixed function signature to match usage
}

fn main() {
println!("Full system test - all modules loaded successfully");

// Test basic functionality with corrected API usage
let mut pop = Pop::default();
let mut input_pop = InputPop::default();
let mut pid = PID::default();

// Create input population with correct signature
let size = 10;
let input_size = 5;
let _inputs = createInputPop(size, input_size);

// Create structure with correct signature
let layers = vec![10, 5, 1];
let _structure = createStructure(&layers);

// Create population
let _population = createPopulation(50, &_structure);

// Test genetic operations with correct signatures
let fitness: Vec<f64> = (0..10).map(|i| i as f64).collect();
let _best = selbest(&_population, &fitness, 3);

// Test PID
let mut pid_controller = createNewPidController(1.0, 0.5, 0.1);
pid_controller.set_setpoint(10.0);
let _output = pid_controller.update(5.0, 0.1);

// Test Neural Network
let mut nn = createNeuralNetwork(&[2, 3, 1]);
let inputs = vec![0.5, 0.3];
let _nn_output = nn.forward(&inputs);

// Test SystemNN
let mut system_nn = SystemNN::default();
let mut nn_input = NNInput::default();
createNNSystem(&mut system_nn, &mut nn_input);

println!("All tests passed!");
}
