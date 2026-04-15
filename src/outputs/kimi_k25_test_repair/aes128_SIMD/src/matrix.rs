
pub type State = [[u8; 4]; 4];
pub fn bytes_to_state(bytes: &[u8]) -> State {
let mut state = [[0u8; 4]; 4];
for i in 0..4 {
for j in 0..4 {
state[j][i] = bytes[i * 4 + j];
}
}
state
}
pub fn state_to_bytes(state: &State) -> [u8; 16] {
let mut bytes = [0u8; 16];
for i in 0..4 {
for j in 0..4 {
bytes[i * 4 + j] = state[j][i];
}
}
bytes
}
