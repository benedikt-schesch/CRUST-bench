
use crate::general_math::createRandomFloat;
use crate::population::{InputPop, Pop};
use crate::sort::quickSort;
use rand::Rng;
fn create_input_pop_custom(input_pop: &mut InputPop, rows: i32, population: &Pop) {
input_pop.cols = population.cols;
input_pop.rows = rows as usize;
input_pop.s[0] = population.s[0].clone();
input_pop.s[1] = population.s[1].clone();
}
pub fn selbest(
fit: &[f32],
population: &Pop,
new_population: &mut Pop,
selects: &[i32],
way: i32,
) {
let rows: usize = selects.iter().map(|v| *v as usize).sum();
let mut fit_vec = fit.to_vec();
let mut result: Vec<i32> = (0..fit.len() as i32).collect();
quickSort(&mut fit_vec, &mut result, fit.len() as i32);
let mut result_index: isize = if way == 0 {
fit.len() as isize - 1
} else if way == 1 {
0
} else {
panic!("Error: way should be 0 or 1");
};
let mut input_pop = InputPop::default();
create_input_pop_custom(&mut input_pop, rows as i32, population);
crate::population::createStructure(&mut input_pop, new_population);
let mut global_index = 0usize;
for num in selects.iter().copied() {
for _ in 0..num {
let src = result[result_index as usize] as usize;
new_population.pop[global_index] = population.pop[src].clone();
global_index += 1;
}
if way == 0 {
result_index -= 1;
} else {
result_index += 1;
}
}
}
pub fn selrand(population: &mut Pop, new_population: &mut Pop, rows: i32) {
let mut input_pop = InputPop::default();
create_input_pop_custom(&mut input_pop, rows, population);
crate::population::createStructure(&mut input_pop, new_population);
let mut rng = rand::thread_rng();
for i in 0..rows as usize {
let index = rng.gen_range(0..population.rows);
new_population.pop[i] = population.pop[index].clone();
}
}
pub fn selturn(population: &Pop, fit: &[f32], new_population: &mut Pop, rows: i32) {
let mut input_pop = InputPop::default();
create_input_pop_custom(&mut input_pop, rows, population);
crate::population::createStructure(&mut input_pop, new_population);
let mut rng = rand::thread_rng();
for i in 0..rows as usize {
let j = rng.gen_range(0..population.rows);
let k = rng.gen_range(0..population.rows);
if j == k || fit[j] <= fit[k] {
new_population.pop[i] = population.pop[j].clone();
} else {
new_population.pop[i] = population.pop[k].clone();
}
}
}
pub fn crosov(population: &mut Pop, selects: &mut Vec<i32>, selects_length: i32) {
let mut local_selects = selects.clone();
let mut local_len = selects_length;
if local_len % 2 != 0 {
local_selects.push(population.cols as i32);
local_len += 1;
}
for index in (0..population.rows.saturating_sub(1)).step_by(2) {
for i in (0..local_len as usize).step_by(2) {
let start = local_selects[i] as usize;
let end = local_selects[i + 1] as usize;
let saved = population.pop[index][start..end].to_vec();
for x in start..end {
population.pop[index][x] = population.pop[index + 1][x];
}
for (offset, x) in (start..end).enumerate() {
population.pop[index + 1][x] = saved[offset];
}
}
}
}
pub fn mutx(population: &mut Pop, chance: f32) {
let mut rng = rand::thread_rng();
for x in 0..population.rows {
for y in 0..population.cols {
let random_float: f32 = rng.r#gen();
if random_float < chance {
let value = createRandomFloat(population.s[1][y], population.s[0][y]);
population.pop[x][y] = value;
}
}
}
}
