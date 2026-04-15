use crate::pid_controller::{
createNewPidController, deletePid, PID
};
fn main() {
let mut pid = createNewPidController(1.0, 0.5, 0.1);
pid.set_setpoint(10.0);
let output = pid.update(5.0, 0.1);
println!("PID output: {}", output);
deletePid(pid);
let mut pid2 = PID::default();
pid2.set_setpoint(5.0);
let output2 = pid2.update(2.0, 0.1);
println!("PID default output: {}", output2);
}
