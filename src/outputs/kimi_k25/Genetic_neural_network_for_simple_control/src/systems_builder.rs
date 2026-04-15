//! System builder for creating control systems

use crate::pid_controller::PID;
use crate::signal_designer::Signal;

/// System configuration
pub struct SystemConfig {
pub pid_params: (f64, f64, f64),
pub signal_type: Signal,
}

/// Select system configuration
pub fn selectSystem() -> SystemConfig {
SystemConfig {
pid_params: (1.0, 0.1, 0.01),
signal_type: Signal::Step(1.0),
}
}

/// Build PID from config
pub fn build_pid(config: &SystemConfig) -> PID {
let (kp, ki, kd) = config.pid_params;
PID::new(kp, ki, kd)
}
