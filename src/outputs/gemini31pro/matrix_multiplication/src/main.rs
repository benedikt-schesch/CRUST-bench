// Generated Rust Code
mod matrix;
mod utils;

use std::time::Instant;
use crate::matrix::{generate_matrix, free_matrix, multiply};

pub fn test_one(x1: usize, y1: usize, x2: usize, y2: usize, method: i32, _test_name: &str, time_taken: &mut u128) -> i32 {
let mut matrix1 = generate_matrix(x1, y1, true);
let mut matrix2 = generate_matrix(x2, y2, true);
let mut result = generate_matrix(x1, y2, true);

let start = Instant::now();
multiply(&matrix1, &matrix2, &mut result, method);
let duration = start.elapsed();

*time_taken += duration.as_nanos();

free_matrix(&mut matrix1);
free_matrix(&mut matrix2);
free_matrix(&mut result);

0
}

pub fn test_more(x1: usize, y1: usize, x2: usize, y2: usize, method: i32, test_name: &str, times: usize) -> i32 {
let mut time_taken = 0;
let start = Instant::now();

for _ in 0..times {
test_one(x1, y1, x2, y2, method, test_name, &mut time_taken);
}

let duration = start.elapsed();
let time = duration.as_millis();

println!("method: {} total consume: {} ms", method, time);
println!("method: {} total time_taken only multiply: {} ms\n================", method, time_taken / 1_000_000);

0
}

fn main() {
let mut test_name = "method: 1";
let x1 = 128;
let x2 = 128;
let y1 = 128;
let y2 = 128;
let times = 1000;

let mut method = 1;
test_more(x1, y1, x2, y2, method, test_name, times);

method = 2;
test_name = "method: 2";
test_more(x1, y1, x2, y2, method, test_name, times);

method = 3;
test_name = "method: 3";
test_more(x1, y1, x2, y2, method, test_name, times);
}
