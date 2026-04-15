use crate::common::are_coprime;
use crate::error::{AcesError, Result};
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Parameters {
pub dim: u64,
pub N: u64,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Channel {
pub p: u64,
pub q: u64,
pub w: u64,
}
impl Channel {
pub fn new(p: u64, q: u64, w: u64) -> Self {
let mut channel = Self { p, q, w };
if !((p.saturating_mul(p)) < q && are_coprime(p, q)) {
channel.q = (p + 1).saturating_mul(p + 1);
}
channel
}
pub fn init(p: u64, q: u64, w: u64) -> Result<Self> {
if p == 0 {
return Err(AcesError::GenericError("p must be non-zero".to_string()));
}
Ok(Self::new(p, q, w))
}
}
