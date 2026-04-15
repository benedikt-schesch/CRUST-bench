// Generated Rust Code
use crate::data;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

pub fn ensure_output_folder(output_folder: &str) -> i32 {
match fs::create_dir_all(output_folder) {
Ok(_) => 0,
Err(_) => -1,
}
}

pub fn write_simulation_step(
num_particles: usize,
particles: &[data::Particle],
folder: &str,
step: u64,
) {
let _ = fs::create_dir_all(folder);
let path = format!("{}/step_{}.csv", folder, step);
if let Ok(mut file) = File::create(path) {
let _ = writeln!(file, "idx,x_coordinate,y_coordinate,radius");
for p in particles.iter().take(num_particles) {
let _ = writeln!(
file,
"{},{},{},{}",
p.idx, p.x_coordinate, p.y_coordinate, p.radius
);
}
}
}

pub fn write_grid(x_squares: i32, y_squares: i32, square_length: f64, folder: &str) {
let _ = fs::create_dir_all(folder);
let path = format!("{}/grid.csv", folder);
if let Ok(mut file) = File::create(path) {
let _ = writeln!(file, "row,col,square_length");
for row in 0..y_squares {
for col in 0..x_squares {
let _ = writeln!(file, "{},{},{}", row, col, square_length);
}
}
}
}

pub fn write_particles_from_grid(
x_squares: i32,
y_squares: i32,
folder: &str,
grid: &Vec<&mut Vec<&mut data::Particle>>,
step: i32,
) {
let _ = fs::create_dir_all(folder);
let path = format!("{}/grid_particles_{}.csv", folder, step);
if let Ok(mut file) = File::create(Path::new(&path)) {
let _ = writeln!(file, "square_idx,idx,x_coordinate,y_coordinate,radius");
let total = (x_squares * y_squares).max(0) as usize;
for square_idx in 0..total {
for p in grid[square_idx].iter() {
let _ = writeln!(
file,
"{},{},{},{},{}",
square_idx, p.idx, p.x_coordinate, p.y_coordinate, p.radius
);
}
}
}
}
