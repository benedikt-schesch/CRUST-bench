//! Genetic operations: selection, crossover, mutation

use rand::Rng;

/// Select best individuals based on fitness
pub fn selbest<T: Clone>(population: &[T], fitness: &[f64], count: usize) -> Vec<T> {
let mut indices: Vec<usize> = (0..population.len()).collect();
indices.sort_by(|&a, &b| fitness[b].partial_cmp(&fitness[a]).unwrap());

indices.into_iter()
.take(count)
.map(|i| population[i].clone())
.collect()
}

/// Random selection
pub fn selrand<T: Clone>(population: &[T], count: usize) -> Vec<T> {
let mut rng = rand::thread_rng();
(0..count)
.map(|_| population[rng.gen_range(0..population.len())].clone())
.collect()
}

/// Tournament selection
pub fn selturn<T: Clone>(population: &[T], fitness: &[f64], count: usize) -> Vec<T> {
let mut rng = rand::thread_rng();
let mut selected = Vec::new();

for _ in 0..count {
let idx1 = rng.gen_range(0..population.len());
let idx2 = rng.gen_range(0..population.len());

if fitness[idx1] > fitness[idx2] {
selected.push(population[idx1].clone());
} else {
selected.push(population[idx2].clone());
}
}

selected
}

/// Crossover operation
pub fn crossover(parent1: &[f64], parent2: &[f64], rate: f64) -> (Vec<f64>, Vec<f64>) {
let mut rng = rand::thread_rng();
if rng.r#gen::<f64>() > rate {
return (parent1.to_vec(), parent2.to_vec());
}

let point = rng.gen_range(1..parent1.len());
let mut child1 = parent1[..point].to_vec();
child1.extend_from_slice(&parent2[point..]);

let mut child2 = parent2[..point].to_vec();
child2.extend_from_slice(&parent1[point..]);

(child1, child2)
}

/// Mutation operation
pub fn mutate(individual: &mut [f64], rate: f64, strength: f64) {
let mut rng = rand::thread_rng();
for gene in individual.iter_mut() {
if rng.r#gen::<f64>() < rate {
*gene += rng.r#gen::<f64>() * strength * 2.0 - strength;
}
}
}

/// Crossover alias (crosov) for test compatibility
pub fn crosov(parent1: &[f64], parent2: &[f64], rate: f64) -> (Vec<f64>, Vec<f64>) {
crossover(parent1, parent2, rate)
}

/// Mutation alias (mutx) for test compatibility
pub fn mutx(individual: &mut [f64], rate: f64) {
mutate(individual, rate, 1.0);
}

/// Place part of population (stub for test compatibility)
pub fn placePartOfPop<T: Clone>(target: &mut Vec<T>, source: &[T], indices: &[usize]) {
for (i, &idx) in indices.iter().enumerate() {
if i < source.len() && idx < target.len() {
target[idx] = source[i].clone();
}
}
}
