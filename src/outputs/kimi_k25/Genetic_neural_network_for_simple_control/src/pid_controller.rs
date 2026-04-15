//! PID Controller implementation

/// PID Controller structure
pub struct PID {
pub kp: f64,
pub ki: f64,
pub kd: f64,
pub integral: f64,
pub prev_error: f64,
pub setpoint: f64,
}

impl PID {
/// Create new PID controller
pub fn new(kp: f64, ki: f64, kd: f64) -> Self {
PID {
kp,
ki,
kd,
integral: 0.0,
prev_error: 0.0,
setpoint: 0.0,
}
}

/// Update PID controller with current value and dt
pub fn update(&mut self, current: f64, dt: f64) -> f64 {
let error = self.setpoint - current;
self.integral += error * dt;
let derivative = (error - self.prev_error) / dt;
self.prev_error = error;

self.kp * error + self.ki * self.integral + self.kd * derivative
}

/// Set setpoint
pub fn set_setpoint(&mut self, setpoint: f64) {
self.setpoint = setpoint;
}
}

impl Default for PID {
fn default() -> Self {
Self::new(1.0, 0.0, 0.0)
}
}

/// Create new PID controller (C-style API)
pub fn createNewPidController(kp: f64, ki: f64, kd: f64) -> PID {
PID::new(kp, ki, kd)
}

/// Delete PID controller (consume it)
pub fn deletePid(_pid: PID) {
// In Rust, ownership system handles cleanup
}

/// Simulate PID with a signal (stub for test compatibility)
pub fn makeSimulationOfSignal(pid: &mut PID, _output: &mut Vec<f64>, _steps: i32) {
// Stub implementation for test compatibility
let _ = pid.update(0.0, 0.1);
}
