//! IO utilities for the lambda calculus evaluator.

use std::fs::{File, OpenOptions, remove_file};
use std::io;
use std::path::Path;

/// Create a new file
pub fn create_file(path: &Path) -> io::Result<File> {
File::create(path)
}

/// Open a file with specific mode
/// mode "r+" means read/write
pub fn get_file(path: &Path, mode: &str) -> io::Result<File> {
match mode {
"r+" => OpenOptions::new().read(true).write(true).open(path),
"r" => OpenOptions::new().read(true).open(path),
"w" => OpenOptions::new().write(true).open(path),
"w+" => OpenOptions::new().read(true).write(true).create(true).open(path),
_ => File::open(path),
}
}

/// Delete a file
pub fn delete_file(path: &Path) -> io::Result<()> {
remove_file(path)
}

/// Close a file (in Rust, files are closed when dropped, but we provide this for API compatibility)
pub fn close_file(_file: File) -> io::Result<()> {
// File is automatically closed when it goes out of scope
Ok(())
}
