use crate::activation_fnc::*;

fn main() {
let x = 0.5;
println!("Sigmoid({}) = {}", x, sigmoid(x));
println!("ReLU({}) = {}", x, relu(x));
println!("Tanh({}) = {}", x, tanh(x));
println!("Linear({}) = {}", x, linear(x));
}
