
use std::io::{self, Write};
pub fn tangenth(x: f32) -> f32 {
(5.0 * x).tanh()
}
pub fn sigmoid(x: f32) -> f32 {
1.0 / (1.0 + (-x).exp())
}
pub fn selectTangActivationFunction(func_ptr: &mut Option<fn(f32) -> f32>) {
*func_ptr = Some(tangenth);
}
pub fn selectSigmActivationFunction(func_ptr: &mut Option<fn(f32) -> f32>) {
*func_ptr = Some(sigmoid);
}
pub fn selectActivationFunction(func_ptr: &mut Option<fn(f32) -> f32>) {
print!("Please select the AF:\n1 - tanh\n2 - sigmoid\nSelect: ");
let _ = io::stdout().flush();
let mut input = String::new();
let user_choice = if io::stdin().read_line(&mut input).is_ok() {
input.trim().parse::<i32>().unwrap_or(0)
} else {
0
};
if user_choice == 1 {
*func_ptr = Some(tangenth);
} else if user_choice == 2 {
*func_ptr = Some(sigmoid);
}
}
