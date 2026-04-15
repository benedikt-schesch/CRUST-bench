//! Utility module providing helper functions for linear algebra operations.

/// Utility function placeholder
pub fn utility_placeholder() {}

/// Helper function to check if a value is close to zero
pub fn is_close_to_zero(val: f64, epsilon: f64) -> bool {
val.abs() < epsilon
}

/// Rounds a floating point number to n decimal places
pub fn roundn(val: f64, n: i32) -> f64 {
let factor = 10_f64.powi(n);
(val * factor).round() / factor
}
