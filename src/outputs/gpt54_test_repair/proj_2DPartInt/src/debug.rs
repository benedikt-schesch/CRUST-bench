
use std::fs::{self, File};
use std::io::Write;
pub fn write_debug_information(
step: u64,
particle_index: usize,
contacts_size: usize,
debug_folder: &str,
) {
let _ = fs::create_dir_all(debug_folder);
let path = format!("{}/debug_{}_{}.txt", debug_folder, step, particle_index);
if let Ok(mut file) = File::create(path) {
write_header(step, particle_index, &mut file);
write_values(particle_index, contacts_size, &mut file);
}
}
pub fn write_header(step: u64, particle_index: usize, file: &mut std::fs::File) {
let _ = writeln!(file, "step={}", step);
let _ = writeln!(file, "particle_index={}", particle_index);
}
pub fn write_values(
particle_index: usize,
contacts_size: usize,
file: &mut std::fs::File,
) {
let _ = writeln!(file, "particle_index={}", particle_index);
let _ = writeln!(file, "contacts_size={}", contacts_size);
}
