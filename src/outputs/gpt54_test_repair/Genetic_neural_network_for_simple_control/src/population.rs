
use crate::general_math::createRandomFloat;
#[derive(Default)]
pub struct InputPop {
pub rows: usize,
pub cols: usize,
pub s: [Vec<f32>; 2],
}
#[derive(Default)]
pub struct Pop {
pub rows: usize,
pub cols: usize,
pub pop: Vec<Vec<f32>>,
pub s: [Vec<f32>; 2],
}
fn clearInputPop(input: &mut InputPop) {
input.s[0].clear();
input.s[1].clear();
input.rows = 0;
input.cols = 0;
}
pub fn createInputPop(input_pop: &mut InputPop, max: &[f32], min: &[f32], size: &[i32]) {
input_pop.cols = size[1] as usize;
input_pop.rows = size[0] as usize;
input_pop.s[0] = max[..input_pop.cols].to_vec();
input_pop.s[1] = min[..input_pop.cols].to_vec();
}
fn alocatePopulation(input: &mut InputPop, population: &mut Pop) {
population.cols = input.cols;
population.rows = input.rows;
population.pop = vec![vec![0.0; population.cols]; population.rows];
population.s[0] = input.s[0].clone();
population.s[1] = input.s[1].clone();
clearInputPop(input);
}
pub fn clearPopulation(population: &mut Pop) {
population.pop.clear();
population.s[0].clear();
population.s[1].clear();
population.cols = 0;
population.rows = 0;
}
pub fn createStructure(input: &mut InputPop, population: &mut Pop) {
alocatePopulation(input, population);
generateRandomPopulation(population);
}
pub fn generateRandomPopulation(population: &mut Pop) {
for y in 0..population.rows {
for i in 0..population.cols {
let new_value = createRandomFloat(population.s[1][i], population.s[0][i]);
population.pop[y][i] = new_value;
}
}
}
pub fn placePartOfPop(pop: &mut Pop, source: &Pop, indexes: &[usize; 2]) {
let mut global_index = 0usize;
for i in indexes[0]..indexes[1] {
pop.pop[i] = source.pop[global_index].clone();
global_index += 1;
}
}
