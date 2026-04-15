
pub fn normalize(val: f64, min: f64, max: f64) -> f64 {
(val - min) / (max - min)
}
pub fn denormalize(val: f64, min: f64, max: f64) -> f64 {
val * (max - min) + min
}
pub fn scale_vector(vec: &mut [f64], min: f64, max: f64) {
for val in vec.iter_mut() {
*val = normalize(*val, min, max);
}
}
