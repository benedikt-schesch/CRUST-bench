//! Matrix Operations Module for AES
//!
//! Handles the state matrix operations used in AES

/// Represents the AES state as a 4x4 matrix of bytes
pub type State = [[u8; 4]; 4];

/// Convert a byte array to state matrix (column-major order)
pub fn bytes_to_state(bytes: &[u8]) -> State {
let mut state = [[0u8; 4]; 4];
for i in 0..4 {
for j in 0..4 {
state[j][i] = bytes[i * 4 + j];
}
}
state
}

/// Convert state matrix back to byte array
pub fn state_to_bytes(state: &State) -> [u8; 16] {
let mut bytes = [0u8; 16];
for i in 0..4 {
for j in 0..4 {
bytes[i * 4 + j] = state[j][i];
}
}
bytes
}
