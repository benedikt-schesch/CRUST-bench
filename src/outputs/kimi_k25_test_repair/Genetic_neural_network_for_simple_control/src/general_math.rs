
pub fn clamp(val: f64, min: f64, max: f64) -> f64 {
val.max(min).min(max)
}
pub fn map(val: f64, in_min: f64, in_max: f64, out_min: f64, out_max: f64) -> f64 {
(val - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
}
pub fn std_dev(values: &[f64]) -> f64 {
let mean = values.iter().sum::<f64>() / values.len() as f64;
let variance = values.iter()
.map(|v| (v - mean).powi(2))
.sum::<f64>() / values.len() as f64;
variance.sqrt()
}
