
#[derive(Default)]
pub struct Pop {
pub pop: Vec<Vec<Vec<Vec<f64>>>>,
}
#[derive(Default)]
pub struct InputPop {
pub data: Vec<Vec<f64>>,
}
pub fn createInputPop(size: usize, input_size: usize) -> Vec<Vec<f64>> {
use rand::Rng;
let mut rng = rand::thread_rng();
(0..size)
.map(|_| (0..input_size).map(|_| rng.r#gen::<f64>()).collect())
.collect()
}
pub fn createStructure(layers: &[usize]) -> Vec<Vec<Vec<f64>>> {
layers.windows(2)
.map(|w| {
let rows = w[1];
let cols = w[0] + 1; 
createRandomMatrix(rows, cols)
})
.collect()
}
fn createRandomMatrix(rows: usize, cols: usize) -> Vec<Vec<f64>> {
use rand::Rng;
let mut rng = rand::thread_rng();
(0..rows)
.map(|_| (0..cols).map(|_| rng.r#gen::<f64>() * 2.0 - 1.0).collect())
.collect()
}
pub fn createPopulation(pop_size: usize, structure: &[Vec<Vec<f64>>]) -> Vec<Vec<Vec<Vec<f64>>>> {
(0..pop_size)
.map(|_| structure.to_vec())
.collect()
}
pub fn clearPopulation(pop: &mut Pop) {
pop.pop.clear();
}
pub fn generateRandomPopulation(pop: &mut Pop) {
pop.pop.clear();
}
