//! Channel module

use crate::error::AcesError;

/// Channel structure for ACES
pub struct Channel {
pub id: u32,
}

impl Channel {
/// Initialize a channel
pub fn init(a: i64, b: i64, c: i64) -> Result<Self, AcesError> {
// Stub implementation
Ok(Channel { id: 0 })
}
}

/// Channel parameters
pub struct Parameters {
pub gain: f64,
pub offset: f64,
pub dim: usize,
pub N: usize,
}

impl Default for Parameters {
fn default() -> Self {
Parameters {
gain: 0.0,
offset: 0.0,
dim: 0,
N: 0,
}
}
}
