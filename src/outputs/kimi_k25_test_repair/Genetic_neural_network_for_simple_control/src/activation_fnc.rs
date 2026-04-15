
pub fn sigmoid(x: f64) -> f64 {
1.0 / (1.0 + (-x).exp())
}
pub fn relu(x: f64) -> f64 {
if x > 0.0 { x } else { 0.0 }
}
pub fn tanh(x: f64) -> f64 {
x.tanh()
}
pub fn linear(x: f64) -> f64 {
x
}
pub fn sigmoid_derivative(x: f64) -> f64 {
let s = sigmoid(x);
s * (1.0 - s)
}
pub fn relu_derivative(x: f64) -> f64 {
if x > 0.0 { 1.0 } else { 0.0 }
}
