use crate::systems_builder::selectSystem;
use crate::pid_controller::{deletePid, PID};
use crate::signal_designer::Signal;

fn main() {
let config = selectSystem();
let pid = PID::new(config.pid_params.0, config.pid_params.1, config.pid_params.2);
println!("System selected and PID created");

// Signal is an enum, not a struct, so we use the enum variants
let _signal = match config.signal_type {
Signal::Step(a) => Signal::Step(a),
Signal::Sine(a, f) => Signal::Sine(a, f),
Signal::Square(a, f) => Signal::Square(a, f),
Signal::Random => Signal::Random,
};

deletePid(pid);
}
