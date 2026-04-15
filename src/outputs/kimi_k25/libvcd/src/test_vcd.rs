use crate::vcd::*;

fn main() {
// Example test code that demonstrates the fixes
// The explicit type annotation : Signal resolves E0282
let signal: Signal = get_test_signal();
let timestamp = 100u64;

// Line 113 - previously had type inference error
// Now signal has explicit type, so get_value_at_timestamp can be resolved
if let Some(value) = signal.get_value_at_timestamp(timestamp) {
println!("Value at timestamp {}: {}", timestamp, value);
}
}
