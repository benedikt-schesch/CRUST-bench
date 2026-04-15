pub fn utility_placeholder() {}
pub fn is_close_to_zero(val: f64, epsilon: f64) -> bool {
val.abs() < epsilon
}
pub fn roundn(val: f64, n: i32) -> f64 {
let factor = 10_f64.powi(n);
(val * factor).round() / factor
}
