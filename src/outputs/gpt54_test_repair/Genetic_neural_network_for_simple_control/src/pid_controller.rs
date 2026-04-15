use crate::systems_builder;
use std::fs::File;
use std::io::Write;
use crate::signal_designer::Signal;
#[derive(Default)]
pub struct PID {
pub Kp: f32,
pub Ki: f32,
pub Kd: f32,
pub tauI: f32,
pub tauD: f32,
pub prevError: f32,
pub proportiError: f32,
pub integralError: f32,
pub differenError: f32,
pub signal: Option<Box<Signal>>,
pub func_system: Option<fn(&mut [f32]) -> f32>,
pub output: Option<Box<Signal>>,
pub dataSystem: Option<Box<[f32]>>,
pub sizeDataSystem: i32,
pub limMaxInt: f32,
pub limMinInt: f32,
pub limMax: f32,
pub limMin: f32,
pub fit: f32,
pub steadyRiseCheck: i32,
pub maxCounter: i32,
}
pub fn createNewPidController(pid: &mut PID) {
pid.Kp = 0.0;
pid.Ki = 0.0;
pid.Kd = 0.0;
pid.tauD = 1.0;
pid.tauI = 1.0;
let mut signal = Signal::default();
crate::signal_designer::cliSignalSelector(&mut signal);
pid.signal = Some(Box::new(signal));
let size = systems_builder::selectSystem(&mut pid.func_system);
pid.dataSystem = Some(vec![0.0; size as usize].into_boxed_slice());
pid.sizeDataSystem = size;
let sig = pid.signal.as_ref().unwrap();
pid.output = Some(Box::new(Signal {
length: sig.length,
dt: sig.dt,
signal: vec![0.0; sig.signal.len()],
}));
pid.prevError = 0.0;
pid.integralError = 0.0;
}
pub fn deletePid(pid: &mut PID) {
if let Some(mut s) = pid.signal.take() {
crate::signal_designer::deleteSignal(&mut s);
}
if let Some(mut o) = pid.output.take() {
crate::signal_designer::deleteSignal(&mut o);
}
pid.dataSystem = None;
}
pub fn resetOutputMemoryPid(pid: &mut PID) {
if let Some(output) = pid.output.as_mut() {
for v in output.signal.iter_mut() {
*v = 0.0;
}
}
if let Some(data) = pid.dataSystem.as_mut() {
for v in data.iter_mut() {
*v = 0.0;
}
if data.len() > 1 {
data[1] = pid.signal.as_ref().unwrap().dt;
}
}
pid.proportiError = 0.0;
pid.integralError = 0.0;
pid.differenError = 0.0;
pid.prevError = 0.0;
pid.steadyRiseCheck = 1;
pid.maxCounter = 0;
}
pub fn makeSimulationOfSignal(pid: &mut PID, csv_file: &mut File, csv: i32) {
resetOutputMemoryPid(pid);
pid.fit = 0.0;
if csv == 1 {
let _ = writeln!(
csv_file,
"{},{},{},{},{}",
pid.proportiError,
pid.integralError,
pid.differenError,
pid.output.as_ref().unwrap().signal[0],
pid.signal.as_ref().unwrap().signal[0]
);
}
let len = pid.signal.as_ref().unwrap().length as usize;
let dt = pid.signal.as_ref().unwrap().dt;
let func = pid.func_system.unwrap();
for i in 2..len {
let error = pid.signal.as_ref().unwrap().signal[i] - pid.output.as_ref().unwrap().signal[i - 1];
pid.proportiError = pid.Kp * error;
pid.integralError =
pid.integralError + 0.5f32 * pid.Ki * (error - pid.prevError) * dt;
pid.differenError = -(2.0f32 * pid.Kd
* (pid.output.as_ref().unwrap().signal[i - 1] - pid.output.as_ref().unwrap().signal[i - 2])
+ (2.0f32 * pid.tauD - dt) * pid.differenError)
/ (2.0f32 * pid.tauD + dt);
if pid.integralError > pid.limMaxInt {
pid.integralError = pid.limMaxInt;
} else if pid.integralError < pid.limMinInt {
pid.integralError = pid.limMinInt;
}
if let Some(data) = pid.dataSystem.as_mut() {
data[0] = pid.proportiError + pid.integralError + pid.differenError;
let mut out = func(data);
if out > pid.limMax {
out = pid.limMax;
pid.maxCounter += 1;
} else if out < pid.limMin {
out = pid.limMin;
pid.maxCounter += 1;
}
if let Some(output) = pid.output.as_mut() {
output.signal[i] = out;
}
}
if csv == 1 {
let _ = writeln!(
csv_file,
"{},{},{},{},{}",
pid.proportiError,
pid.integralError,
pid.differenError,
pid.output.as_ref().unwrap().signal[i],
pid.signal.as_ref().unwrap().signal[i]
);
}
pid.prevError = error;
let diff = pid.signal.as_ref().unwrap().signal[i] - pid.output.as_ref().unwrap().signal[i];
pid.fit += if diff > 0.0 { diff } else { diff * -1.0 };
if pid.output.as_ref().unwrap().signal[i - 1] > pid.output.as_ref().unwrap().signal[i] {
pid.steadyRiseCheck = 0;
}
}
if pid.steadyRiseCheck == 1 {
pid.fit = f32::MAX;
} else if pid.maxCounter > pid.signal.as_ref().unwrap().length * 1 / 100 {
pid.fit = f32::MAX;
}
}
