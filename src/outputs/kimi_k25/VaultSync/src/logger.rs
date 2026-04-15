//! Logger module for VaultSync

use std::fmt;

/// Initialize the logging system
pub fn init() -> Result<(), String> {
Ok(())
}

/// Log a message at the info level
pub fn info(message: &str) {
println!("[INFO] {}", message);
}

/// Log a message at the error level
pub fn error(message: &str) {
eprintln!("[ERROR] {}", message);
}
