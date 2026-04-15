
pub fn mse(predicted: &[f64], actual: &[f64]) -> f64 {
predicted.iter()
.zip(actual.iter())
.map(|(p, a)| (p - a).powi(2))
.sum::<f64>() / predicted.len() as f64
}
pub fn rmse(predicted: &[f64], actual: &[f64]) -> f64 {
mse(predicted, actual).sqrt()
}
pub fn simple_fitness(error: f64) -> f64 {
if error == 0.0 {
f64::MAX
} else {
1.0 / error
}
}
