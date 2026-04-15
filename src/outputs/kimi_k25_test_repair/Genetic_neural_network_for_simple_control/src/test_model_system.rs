use crate::model_system::{
createFirstOrderSystem, simulate_system, SystemModel, SystemNN,
createNNSystem, createDeNormalization, makeSimulationOfSignalNN, clearNNSystem,
pidFitFunction, nnFitFunction
};
use crate::neural_network::{
createNeuralNetwork, deleteNeuralNetwork, NNInput, fillMatrixesNN
};
use crate::population::{
createInputPop, createStructure, createPopulation, Pop, InputPop, clearPopulation
};
use crate::pid_controller::{
createNewPidController, deletePid
};
fn main() {
let mut system = createFirstOrderSystem(1.0, 0.5);
let mut pid = createNewPidController(1.0, 0.1, 0.01);
let outputs = simulate_system(&mut system, &mut pid, 1.0, 10.0, 0.01);
println!("Simulation completed with {} steps", outputs.len());
deletePid(pid);
let mut system_nn = SystemNN::default();
let mut nn_input = NNInput::default();
nn_input.data = vec![0.5, 0.3];
createNNSystem(&mut system_nn, &mut nn_input);
createDeNormalization(&mut system_nn);
let mut output_vec: Vec<f64> = vec![];
makeSimulationOfSignalNN(&mut system_nn, &mut output_vec, true);
clearNNSystem(&mut system_nn);
let input_size = 10;
let pop_size = 50;
let inputs = createInputPop(pop_size, input_size);
let structure = createStructure(&[input_size, 5, 1]);
let pop_vec = createPopulation(pop_size, &structure);
let mut pop = Pop::default();
pop.pop = pop_vec;
let mut fit: Vec<f64> = vec![0.0; pop_size];
let mut pid2 = createNewPidController(1.0, 0.1, 0.01);
pidFitFunction(&mut pop, &mut fit, &mut pid2);
nnFitFunction(&mut pop, &mut fit, &mut system_nn);
clearPopulation(&mut pop);
}
