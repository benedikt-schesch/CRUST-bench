// activation_fnc.rs
use std::io::{self, Write};

/// Hyperbolic tangent activation function (scaled).
pub fn tangenth(x: f32) -> f32 {
(5.0 * x).tanh()
}
/// Sigmoid activation function.
pub fn sigmoid(x: f32) -> f32 {
1.0 / (1.0 + (-x).exp())
}
/// Sets the function pointer to the tanh activation function.
pub fn selectTangActivationFunction(func_ptr: &mut Option<fn(f32) -> f32>) {
*func_ptr = Some(tangenth);
}
/// Sets the function pointer to the sigmoid activation function.
pub fn selectSigmActivationFunction(func_ptr: &mut Option<fn(f32) -> f32>) {
*func_ptr = Some(sigmoid);
}
/// Asks the user to select an activation function via CLI and sets it accordingly.
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
