//! Population management for genetic algorithm

/// Population structure
#[derive(Default)]
pub struct Pop {
pub pop: Vec<Vec<Vec<Vec<f64>>>>,
}

/// Input population structure
#[derive(Default)]
pub struct InputPop {
pub data: Vec<Vec<f64>>,
}

/// Create input population
pub fn createInputPop(size: usize, input_size: usize) -> Vec<Vec<f64>> {
use rand::Rng;
let mut rng = rand::thread_rng();
(0..size)
.map(|_| (0..input_size).map(|_| rng.r#gen::<f64>()).collect())
.collect()
}

/// Create structure for neural network
pub fn createStructure(layers: &[usize]) -> Vec<Vec<Vec<f64>>> {
layers.windows(2)
.map(|w| {
let rows = w[1];
let cols = w[0] + 1; // +1 for bias
createRandomMatrix(rows, cols)
})
.collect()
}

/// Create random matrix
fn createRandomMatrix(rows: usize, cols: usize) -> Vec<Vec<f64>> {
use rand::Rng;
let mut rng = rand::thread_rng();
(0..rows)
.map(|_| (0..cols).map(|_| rng.r#gen::<f64>() * 2.0 - 1.0).collect())
.collect()
}

/// Create population of structures
pub fn createPopulation(pop_size: usize, structure: &[Vec<Vec<f64>>]) -> Vec<Vec<Vec<Vec<f64>>>> {
(0..pop_size)
.map(|_| structure.to_vec())
.collect()
}

/// Clear population (stub for test compatibility)
pub fn clearPopulation(pop: &mut Pop) {
pop.pop.clear();
}

/// Generate random population (stub for test compatibility)
pub fn generateRandomPopulation(pop: &mut Pop) {
// Stub implementation
pop.pop.clear();
}
