//! Error handling module
use std::fmt;

/// ACES error type
#[derive(Debug)]
pub struct AcesError {
pub message: String,
}

impl fmt::Display for AcesError {
fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
write!(f, "{}", self.message)
}
}

impl std::error::Error for AcesError {}
