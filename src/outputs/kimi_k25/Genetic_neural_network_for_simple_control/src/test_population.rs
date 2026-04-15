use crate::population::{
createInputPop, createStructure, createPopulation, Pop, InputPop, clearPopulation
};

fn main() {
// Fixed: createInputPop takes 2 usize arguments: size and input_size
let size = 100;
let input_size = 10;
let inputs = createInputPop(size, input_size);

// Fixed: createStructure takes 1 argument: layers slice
let layers = vec![10, 5, 1];
let structure = createStructure(&layers);

// Fixed: createPopulation takes 2 arguments: pop_size and structure reference
let pop = createPopulation(50, &structure);

println!("Created population with {} individuals", pop.len());

// Test Pop struct
let mut pop_struct = Pop::default();
pop_struct.pop = pop;
clearPopulation(&mut pop_struct);

// Test InputPop
let mut input_pop = InputPop::default();
input_pop.data = inputs;
println!("Input pop size: {}", input_pop.data.len());
}
