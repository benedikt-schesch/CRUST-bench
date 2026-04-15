use crate::vcd::*;
fn main() {
let signal: Signal = get_test_signal();
let timestamp = 100u64;
if let Some(value) = signal.get_value_at_timestamp(timestamp) {
println!("Value at timestamp {}: {}", timestamp, value);
}
}
