//! Activation functions for neural networks

/// Sigmoid activation function
pub fn sigmoid(x: f64) -> f64 {
1.0 / (1.0 + (-x).exp())
}

/// ReLU activation function
pub fn relu(x: f64) -> f64 {
if x > 0.0 { x } else { 0.0 }
}

/// Tanh activation function
pub fn tanh(x: f64) -> f64 {
x.tanh()
}

/// Linear activation function
pub fn linear(x: f64) -> f64 {
x
}

/// Derivative of sigmoid
pub fn sigmoid_derivative(x: f64) -> f64 {
let s = sigmoid(x);
s * (1.0 - s)
}

/// Derivative of ReLU
pub fn relu_derivative(x: f64) -> f64 {
if x > 0.0 { 1.0 } else { 0.0 }
}
