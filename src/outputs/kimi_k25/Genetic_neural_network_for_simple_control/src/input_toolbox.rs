//! Input handling utilities

/// Normalize input to [0, 1]
pub fn normalize(val: f64, min: f64, max: f64) -> f64 {
(val - min) / (max - min)
}

/// Denormalize input from [0, 1]
pub fn denormalize(val: f64, min: f64, max: f64) -> f64 {
val * (max - min) + min
}

/// Scale input vector
pub fn scale_vector(vec: &mut [f64], min: f64, max: f64) {
for val in vec.iter_mut() {
*val = normalize(*val, min, max);
}
}
