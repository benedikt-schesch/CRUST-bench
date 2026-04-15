
use crate::pid_controller::PID;
use crate::signal_designer::Signal;
pub struct SystemConfig {
pub pid_params: (f64, f64, f64),
pub signal_type: Signal,
}
pub fn selectSystem() -> SystemConfig {
SystemConfig {
pid_params: (1.0, 0.1, 0.01),
signal_type: Signal::Step(1.0),
}
}
pub fn build_pid(config: &SystemConfig) -> PID {
let (kp, ki, kd) = config.pid_params;
PID::new(kp, ki, kd)
}
