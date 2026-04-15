use crate::population::{
createInputPop, createStructure, createPopulation, Pop, InputPop, clearPopulation
};
fn main() {
let size = 100;
let input_size = 10;
let inputs = createInputPop(size, input_size);
let layers = vec![10, 5, 1];
let structure = createStructure(&layers);
let pop = createPopulation(50, &structure);
println!("Created population with {} individuals", pop.len());
let mut pop_struct = Pop::default();
pop_struct.pop = pop;
clearPopulation(&mut pop_struct);
let mut input_pop = InputPop::default();
input_pop.data = inputs;
println!("Input pop size: {}", input_pop.data.len());
}
