//! Polynomial operations module

use crate::error::AcesError;

/// Polynomial array type
pub struct PolyArray {
pub polies: Vec<Polynomial>,
}

impl PolyArray {
pub fn new(polies: Vec<Polynomial>) -> Self {
PolyArray { polies }
}
}

/// Polynomial type
#[derive(Debug, Clone, PartialEq)]
pub struct Polynomial {
pub coeffs: Vec<i64>,
}

impl Polynomial {
pub fn new(coeffs: Vec<i64>) -> Self {
Polynomial { coeffs }
}

pub fn from_coeffs(coeffs: &[i64]) -> Self {
Polynomial {
coeffs: coeffs.to_vec(),
}
}

pub fn set_zero(&mut self) {
for coeff in &mut self.coeffs {
*coeff = 0;
}
}

pub fn coef_sum(&self) -> i64 {
self.coeffs.iter().sum()
}

pub fn degree(&self) -> usize {
for (i, &coeff) in self.coeffs.iter().enumerate().rev() {
if coeff != 0 {
return i;
}
}
0
}

pub fn add(&self, other: &Polynomial, modulus: i64) -> Result<Polynomial, AcesError> {
let len = self.coeffs.len().max(other.coeffs.len());
let mut result = vec![0i64; len];
for i in 0..len {
let a = self.coeffs.get(i).copied().unwrap_or(0);
let b = other.coeffs.get(i).copied().unwrap_or(0);
result[i] = (a + b) % modulus;
}
Ok(Polynomial::new(result))
}

pub fn mul(&self, other: &Polynomial, modulus: i64) -> Result<Polynomial, AcesError> {
if self.coeffs.is_empty() || other.coeffs.is_empty() {
return Ok(Polynomial::new(vec![]));
}
let mut result = vec![0i64; self.coeffs.len() + other.coeffs.len() - 1];
for (i, &a) in self.coeffs.iter().enumerate() {
for (j, &b) in other.coeffs.iter().enumerate() {
if i + j < result.len() {
result[i + j] = (result[i + j] + a * b) % modulus;
}
}
}
Ok(Polynomial::new(result))
}

pub fn lshift(&self, other: &Polynomial, modulus: i64) -> Result<Polynomial, AcesError> {
// Stub implementation - performs multiplication as placeholder
self.mul(other, modulus)
}
}
