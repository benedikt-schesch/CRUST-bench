//! Signal generation and design

/// Signal types
pub enum Signal {
Step(f64),
Sine(f64, f64), // amplitude, frequency
Square(f64, f64),
Random,
}

impl Signal {
/// Get value at time t
pub fn value(&self, t: f64) -> f64 {
match self {
Signal::Step(amplitude) => {
if t > 0.0 { *amplitude } else { 0.0 }
},
Signal::Sine(amp, freq) => {
amp * (2.0 * std::f64::consts::PI * freq * t).sin()
},
Signal::Square(amp, freq) => {
let period = 1.0 / freq;
if (t % period) < (period / 2.0) { *amp } else { -*amp }
},
Signal::Random => {
use rand::Rng;
rand::thread_rng().r#gen::<f64>()
},
}
}
}

impl Default for Signal {
fn default() -> Self {
Signal::Step(1.0)
}
}

/// CLI signal selector
pub fn cliSignalSelector() -> Signal {
Signal::Step(1.0)
}

/// Delete signal (consume)
pub fn deleteSignal(_signal: Signal) {
// Ownership handles cleanup
}
