use crate::signal_designer::{
Signal, cliSignalSelector, deleteSignal
};

fn main() {
// Use default() which is now implemented
let signal = cliSignalSelector();
let val = signal.value(1.0);
println!("Signal value at t=1.0: {}", val);

let sine = Signal::Sine(1.0, 0.5);
println!("Sine value: {}", sine.value(2.0));

// Use Signal::default() or specific variant
let default_signal = Signal::default();
println!("Default signal value: {}", default_signal.value(1.0));

deleteSignal(signal);
}
