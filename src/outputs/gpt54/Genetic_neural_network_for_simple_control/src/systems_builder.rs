// systems_builder.rs
use crate::pid_controller;
use std::io::{self, Write};

pub fn linear(data: &mut [f32]) -> f32 {
data[0]
}

pub fn complexYDot(data: &mut [f32]) -> f32 {
data[2] = data[2] + data[0] * data[1];
data[2]
}

pub fn complexYDddot(data: &mut [f32]) -> f32 {
data[5] = data[0] - 2.0 * data[2] - 3.0 * data[3] - 4.0 * data[4];
data[2] = data[2] + data[5] * data[1];
data[3] = data[3] + data[2] * data[1];
data[4] = data[4] + data[3] * data[1];
data[4]
}

pub fn selectSystem(func_ptr: &mut Option<fn(&mut [f32]) -> f32>) -> i32 {
print!("Please select the system:\n1 - linear\n2 - complexYDddot\n3 - complexYDot\nSelect: ");
let _ = io::stdout().flush();

let mut input = String::new();
let user_choice = if io::stdin().read_line(&mut input).is_ok() {
input.trim().parse::<i32>().unwrap_or(0)
} else {
0
};

if user_choice == 1 {
*func_ptr = Some(linear);
2
} else if user_choice == 2 {
*func_ptr = Some(complexYDddot);
6
} else if user_choice == 3 {
*func_ptr = Some(complexYDot);
4
} else {
std::process::exit(0);
}
}
