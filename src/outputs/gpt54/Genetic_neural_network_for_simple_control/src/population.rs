// population.rs
use crate::general_math::createRandomFloat;

#[derive(Default)]
/// Structure representing the input to create a population.
pub struct InputPop {
pub rows: usize,
pub cols: usize,
pub s: [Vec<f32>; 2],
}

#[derive(Default)]
/// Structure representing the population.
pub struct Pop {
pub rows: usize,
pub cols: usize,
pub pop: Vec<Vec<f32>>,
pub s: [Vec<f32>; 2],
}

/// Clear/deallocate InputPop (in Rust usually handled by `Drop`, but shown for parity).
fn clearInputPop(input: &mut InputPop) {
input.s[0].clear();
input.s[1].clear();
input.rows = 0;
input.cols = 0;
}

/// Creates an InputPop structure using max/min bounds and a size array.
pub fn createInputPop(input_pop: &mut InputPop, max: &[f32], min: &[f32], size: &[i32]) {
input_pop.cols = size[1] as usize;
input_pop.rows = size[0] as usize;
input_pop.s[0] = max[..input_pop.cols].to_vec();
input_pop.s[1] = min[..input_pop.cols].to_vec();
}

/// Allocates a population from an InputPop definition.
fn alocatePopulation(input: &mut InputPop, population: &mut Pop) {
population.cols = input.cols;
population.rows = input.rows;
population.pop = vec![vec![0.0; population.cols]; population.rows];
population.s[0] = input.s[0].clone();
population.s[1] = input.s[1].clone();

clearInputPop(input);
}

/// Deallocates the memory held by a Pop structure.
pub fn clearPopulation(population: &mut Pop) {
population.pop.clear();
population.s[0].clear();
population.s[1].clear();
population.cols = 0;
population.rows = 0;
}

/// Creates a random population from the given input constraints.
pub fn createStructure(input: &mut InputPop, population: &mut Pop) {
alocatePopulation(input, population);
generateRandomPopulation(population);
}

/// Fills the population with random values within the constraints in `s`.
pub fn generateRandomPopulation(population: &mut Pop) {
for y in 0..population.rows {
for i in 0..population.cols {
let new_value = createRandomFloat(population.s[1][i], population.s[0][i]);
population.pop[y][i] = new_value;
}
}
}

/// Places a part of the source population into the target population using the index range.
pub fn placePartOfPop(pop: &mut Pop, source: &Pop, indexes: &[usize; 2]) {
let mut global_index = 0usize;
for i in indexes[0]..indexes[1] {
pop.pop[i] = source.pop[global_index].clone();
global_index += 1;
}
}
