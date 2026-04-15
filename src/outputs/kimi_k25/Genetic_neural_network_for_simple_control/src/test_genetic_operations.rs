use crate::genetic_operations::{selbest, selrand, selturn, crosov, mutx};
use crate::population::{createInputPop, createStructure, clearPopulation, Pop, InputPop};

fn main() {
// Create input population with correct signature (2 arguments)
let size = 10;
let input_size = 5;
let pop = createInputPop(size, input_size);

// Create structure with correct signature (1 argument)
let layers = vec![10, 5, 1];
let _structure = createStructure(&layers);

let fitness: Vec<f64> = (0..10).map(|i| i as f64).collect();

// Test selbest with correct signature (3 arguments)
let best = selbest(&pop, &fitness, 3);

// Test selrand with correct signature (2 arguments)
let random = selrand(&pop, 3);

// Test selturn with correct signature (3 arguments)
let tournament = selturn(&pop, &fitness, 3);

// Test crosov (crossover)
let parent1: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0];
let parent2: Vec<f64> = vec![5.0, 6.0, 7.0, 8.0];
let (_child1, _child2) = crosov(&parent1, &parent2, 0.5);

// Test mutx (mutate)
let mut individual: Vec<f64> = vec![1.0, 2.0, 3.0];
mutx(&mut individual, 0.1);

// Test Pop and clearPopulation
let mut population_struct = Pop::default();
clearPopulation(&mut population_struct);

println!("Selection operations completed");
println!("Best count: {}", best.len());
println!("Random count: {}", random.len());
println!("Tournament count: {}", tournament.len());
}
