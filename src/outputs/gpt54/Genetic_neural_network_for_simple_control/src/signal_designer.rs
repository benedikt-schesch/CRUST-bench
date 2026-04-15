// signal_designer.rs
use std::io::{self, Write};

#[derive(Default)]
pub struct Signal {
pub signal: Vec<f32>,
pub dt: f32,
pub length: i32,
}

pub fn deleteSignal(signal: &mut Signal) {
signal.signal.clear();
signal.dt = 0.0;
signal.length = 0;
}

pub fn cliSignalSelector(signal: &mut Signal) {
print!("Please select the Signal:\n1 - step\n2 - customa\nSelect: ");
let _ = io::stdout().flush();

let mut input = String::new();
let user_choice = if io::stdin().read_line(&mut input).is_ok() {
input.trim().parse::<i32>().unwrap_or(0)
} else {
0
};

if user_choice == 1 {
selectStepSignal(signal);
} else if user_choice == 2 {
selectCustomASignal(signal);
}
}

fn selectStepSignal(signal: &mut Signal) {
signal.length = 1000;
signal.dt = 0.01;
signal.signal = vec![0.0; signal.length as usize];

let mut global_index = 0usize;
let mut i = 0.0f32;
while i < 0.3 {
signal.signal[global_index] = 0.0;
global_index += 1;
i += signal.dt;
}

let mut i2 = 0.3 + signal.dt;
while i2 < 10.0 {
if global_index >= signal.signal.len() {
break;
}
signal.signal[global_index] = 1.0;
global_index += 1;
i2 += signal.dt;
}

signal.length = global_index as i32;
signal.signal.truncate(global_index);
}

fn selectCustomASignal(signal: &mut Signal) {
signal.length = 1002;
signal.dt = 0.01;
signal.signal = vec![0.0; signal.length as usize];

let mut global_index = 0usize;

let mut i = 0.0f32;
while i < 1.0 {
signal.signal[global_index] = 0.0;
global_index += 1;
i += signal.dt;
}

let mut i = 1.0f32;
while i < 3.0 {
signal.signal[global_index] = 30.0;
global_index += 1;
i += signal.dt;
}

let mut i = 3.0f32;
while i < 5.0 {
signal.signal[global_index] = 15.0;
global_index += 1;
i += signal.dt;
}

let mut i = 5.0f32;
while i < 8.0 {
signal.signal[global_index] = 5.0;
global_index += 1;
i += signal.dt;
}

let mut i = 8.0f32;
while i < 9.0 {
signal.signal[global_index] = 25.0;
global_index += 1;
i += signal.dt;
}

let mut i = 9.0f32;
while i < 10.0 {
signal.signal[global_index] = 0.0;
global_index += 1;
i += signal.dt;
}

signal.length = global_index as i32;
signal.signal.truncate(global_index);
}
