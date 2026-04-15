
use std::fmt;
pub fn init() -> Result<(), String> {
Ok(())
}
pub fn info(message: &str) {
println!("[INFO] {}", message);
}
pub fn error(message: &str) {
eprintln!("[ERROR] {}", message);
}
