//! Neural network implementation

use crate::activation_fnc;
use crate::matrix_math::{Matrix, matrixMul};

/// Neural Network structure
pub struct NeuralNetwork {
pub layers: Vec<Vec<Vec<f64>>>,
pub activations: Vec<fn(f64) -> f64>,
}

/// NN type alias for test compatibility
pub type NN = NeuralNetwork;

/// NNInput structure for test compatibility
#[derive(Default)]
pub struct NNInput {
pub data: Vec<f64>,
}

impl NeuralNetwork {
/// Forward pass
pub fn forward(&self, inputs: &[f64]) -> Vec<f64> {
let mut current = inputs.to_vec();

for (layer_idx, layer) in self.layers.iter().enumerate() {
let mut next = Vec::with_capacity(layer.len());
for neuron in layer.iter() {
let sum: f64 = neuron.iter().enumerate()
.map(|(i, w)| {
if i < current.len() {
current[i] * w
} else {
*w // bias
}
})
.sum();
let activated = if layer_idx < self.activations.len() {
(self.activations[layer_idx])(sum)
} else {
sum
};
next.push(activated);
}
current = next;
}

current
}
}

/// Create neural network
pub fn createNeuralNetwork(layers: &[usize]) -> NeuralNetwork {
let structure = crate::population::createStructure(layers);
NeuralNetwork {
layers: structure,
activations: vec![activation_fnc::sigmoid; layers.len() - 1],
}
}

/// Delete neural network
pub fn deleteNeuralNetwork(_nn: NeuralNetwork) {
// Ownership handles cleanup
}

/// Clear neural network (stub for test compatibility)
pub fn clearNeuralNetwork(_nn: &mut NeuralNetwork) {
// Stub implementation
}

/// Fill matrixes NN (stub for test compatibility)
pub fn fillMatrixesNN(nn: &mut NeuralNetwork, weights: &[f64]) {
let mut idx = 0;
for layer in &mut nn.layers {
for neuron in layer {
for weight in neuron {
if idx < weights.len() {
*weight = weights[idx];
idx += 1;
}
}
}
}
}

/// One calculation (stub for test compatibility)
pub fn oneCalculation(nn: &mut NeuralNetwork, input: &mut [f64], output: &mut [f64]) {
let result = nn.forward(input);
for (i, val) in result.iter().enumerate() {
if i < output.len() {
output[i] = *val;
}
}
}

/// De-normalization process (stub for test compatibility)
pub fn deNormalizationProcess(_nn: &mut NeuralNetwork, _data: &mut [f64], _layer: i32) {
// Stub implementation
}
