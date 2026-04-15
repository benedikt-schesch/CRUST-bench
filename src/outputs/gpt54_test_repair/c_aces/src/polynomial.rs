use crate::error::{AcesError, Result};
pub type Coeff = i64;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Polynomial {
pub coeffs: Vec<Coeff>,
}
impl Polynomial {
pub fn new(coeffs: Vec<Coeff>) -> Self {
Self { coeffs }
}
pub fn degree(&self) -> usize {
if self.coeffs.is_empty() {
return 0;
}
for (i, c) in self.coeffs.iter().enumerate() {
if *c != 0 {
return self.coeffs.len() - i - 1;
}
}
0
}
pub fn set_zero(&mut self) {
for c in &mut self.coeffs {
*c = 0;
}
}
pub fn coef_sum(&self) -> Coeff {
if self.coeffs.is_empty() {
return 0;
}
self.coeffs.iter().copied().sum()
}
pub fn fit(&mut self, modulus: u64) -> Result<()> {
if self.coeffs.is_empty() {
return Err(AcesError::GenericError("empty polynomial".to_string()));
}
let degree = self.degree();
let idx = self.coeffs.len() - 1 - degree;
let mut new_coeffs = Vec::with_capacity(self.coeffs.len() - idx);
for i in idx..self.coeffs.len() {
new_coeffs.push(self.coeffs[i] % modulus as i64);
}
self.coeffs = new_coeffs;
Ok(())
}
pub fn add(&self, other: &Polynomial, modulus: u64) -> Result<Polynomial> {
if modulus == 0 {
return Err(AcesError::GenericError("modulus is zero".to_string()));
}
let size = self.coeffs.len().max(other.coeffs.len());
let diff1 = size - self.coeffs.len();
let diff2 = size - other.coeffs.len();
let mut coeffs = vec![0; size];
for i in 0..size {
let a = if i >= diff1 { self.coeffs[i - diff1] } else { 0 };
let b = if i >= diff2 { other.coeffs[i - diff2] } else { 0 };
coeffs[i] = (a + b) % modulus as i64;
}
Ok(Polynomial::new(coeffs))
}
pub fn sub(&self, other: &Polynomial, modulus: u64) -> Result<Polynomial> {
if modulus == 0 {
return Err(AcesError::GenericError("modulus is zero".to_string()));
}
let size = self.coeffs.len().max(other.coeffs.len());
let diff1 = size - self.coeffs.len();
let diff2 = size - other.coeffs.len();
let mut coeffs = vec![0; size];
for i in 0..size {
let a = if i >= diff1 { self.coeffs[i - diff1] } else { 0 };
let b = if i >= diff2 { other.coeffs[i - diff2] } else { 0 };
coeffs[i] = (a - b) % modulus as i64;
}
Ok(Polynomial::new(coeffs))
}
pub fn mul(&self, other: &Polynomial, modulus: u64) -> Result<Polynomial> {
let size = self.coeffs.len() + other.coeffs.len();
let mut result = Polynomial::new(vec![0; size]);
result.set_zero();
let deg1 = self.coeffs.len().saturating_sub(1);
let deg2 = other.coeffs.len().saturating_sub(1);
let res_deg = result.coeffs.len().saturating_sub(deg2 + deg1 + 1);
for i in 0..self.coeffs.len() {
for j in 0..other.coeffs.len() {
result.coeffs[res_deg + i + j] +=
(self.coeffs[i] * other.coeffs[j]) % modulus as i64;
}
}
result.fit(modulus)?;
Ok(result)
}
pub fn lshift(&self, other: &Polynomial, modulus: u64) -> Result<Polynomial> {
if other.coeffs.is_empty() || other.coeffs[0] != 1 {
return Err(AcesError::GenericError("invalid divisor".to_string()));
}
let degree1 = self.degree();
let degree2 = other.degree();
if degree1 < degree2 {
return Err(AcesError::GenericError("degree too small".to_string()));
}
let a_d = self.coeffs[0];
let mut result = self.clone();
for i in 0..self.coeffs.len() {
if i < other.coeffs.len() {
let res = (self.coeffs[i] - other.coeffs[i] * a_d) % modulus as i64;
result.coeffs[i] = if res < 0 { res + modulus as i64 } else { res };
} else {
result.coeffs[i] = self.coeffs[i];
}
}
result.fit(modulus)?;
Ok(result)
}
pub fn poly_mod(&mut self, divisor: &Polynomial, modulus: u64) -> Result<()> {
while let Ok(next) = self.lshift(divisor, modulus) {
*self = next;
}
Ok(())
}
pub fn sub_scaler(&self, scaler: u64, modulus: u64) -> Result<Polynomial> {
let mut coeffs = vec![0; self.coeffs.len()];
for (i, c) in self.coeffs.iter().enumerate() {
coeffs[i] = (c - scaler as i64) % modulus as i64;
}
Ok(Polynomial::new(coeffs))
}
pub fn add_scaler(&self, scaler: u64, modulus: u64) -> Result<Polynomial> {
let mut coeffs = vec![0; self.coeffs.len()];
for (i, c) in self.coeffs.iter().enumerate() {
coeffs[i] = (c + scaler as i64) % modulus as i64;
}
Ok(Polynomial::new(coeffs))
}
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolyArray {
pub polies: Vec<Polynomial>,
}
impl PolyArray {
pub fn new(polies: Vec<Polynomial>) -> Self {
Self { polies }
}
}
