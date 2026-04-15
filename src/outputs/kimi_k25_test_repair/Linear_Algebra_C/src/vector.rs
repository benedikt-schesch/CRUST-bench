use crate::utils::is_close_to_zero;
pub struct Vector {
pub data: Vec<f64>,
}
impl Vector {
pub fn new(size: usize) -> Self {
Self {
data: vec![0.0; size],
}
}
pub fn from_data(data: Vec<f64>) -> Self {
Self { data }
}
pub fn len(&self) -> usize {
self.data.len()
}
pub fn is_empty(&self) -> bool {
self.data.is_empty()
}
}
pub fn new_vector(data: &[f64], size: usize) -> Vector {
assert_eq!(data.len(), size, "Data length must match vector size");
Vector::from_data(data.to_vec())
}
pub fn delete_vector(_v: Vector) {
}
pub fn get_vector_element(v: &Vector, index: usize) -> f64 {
v.data[index]
}
pub fn set_vector_element(v: &mut Vector, index: usize, val: f64) {
v.data[index] = val;
}
pub fn zero_vector(size: usize) -> Vector {
Vector::new(size)
}
pub fn copy_vector(v: &Vector) -> Vector {
Vector::from_data(v.data.clone())
}
pub fn vector_size_bytes(v: &Vector) -> usize {
v.data.len() * std::mem::size_of::<f64>()
}
pub fn is_vector_equal(v1: &Vector, v2: &Vector) -> bool {
if v1.len() != v2.len() {
return false;
}
v1.data.iter().zip(v2.data.iter()).all(|(a, b)| (a - b).abs() < 1e-10)
}
pub fn vector_magnitude(v: &Vector) -> f64 {
v.data.iter().map(|&x| x * x).sum::<f64>().sqrt()
}
pub fn is_unit_vector(v: &Vector) -> bool {
(vector_magnitude(v) - 1.0).abs() < 1e-10
}
pub fn dot_product(v1: &Vector, v2: &Vector) -> f64 {
assert_eq!(v1.len(), v2.len(), "Vectors must have same dimension");
v1.data.iter().zip(v2.data.iter()).map(|(a, b)| a * b).sum()
}
pub fn is_vector_orthogonal(v1: &Vector, v2: &Vector) -> bool {
is_close_to_zero(dot_product(v1, v2), 1e-10)
}
pub fn vector_distance(v1: &Vector, v2: &Vector) -> f64 {
assert_eq!(v1.len(), v2.len(), "Vectors must have same dimension");
v1.data.iter().zip(v2.data.iter())
.map(|(a, b)| (a - b) * (a - b))
.sum::<f64>()
.sqrt()
}
pub fn cross_product(v1: &Vector, v2: &Vector) -> Vector {
assert_eq!(v1.len(), 3, "Cross product only defined for 3D vectors");
assert_eq!(v2.len(), 3, "Cross product only defined for 3D vectors");
let mut result = Vector::new(3);
result.data[0] = v1.data[1] * v2.data[2] - v1.data[2] * v2.data[1];
result.data[1] = v1.data[0] * v2.data[2] - v1.data[2] * v2.data[0];
result.data[2] = v1.data[0] * v2.data[1] - v1.data[1] * v2.data[0];
result
}
