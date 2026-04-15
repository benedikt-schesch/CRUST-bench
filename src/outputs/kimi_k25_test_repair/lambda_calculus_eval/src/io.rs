
use std::fs::{File, OpenOptions, remove_file};
use std::io;
use std::path::Path;
pub fn create_file(path: &Path) -> io::Result<File> {
File::create(path)
}
pub fn get_file(path: &Path, mode: &str) -> io::Result<File> {
match mode {
"r+" => OpenOptions::new().read(true).write(true).open(path),
"r" => OpenOptions::new().read(true).open(path),
"w" => OpenOptions::new().write(true).open(path),
"w+" => OpenOptions::new().read(true).write(true).create(true).open(path),
_ => File::open(path),
}
}
pub fn delete_file(path: &Path) -> io::Result<()> {
remove_file(path)
}
pub fn close_file(_file: File) -> io::Result<()> {
Ok(())
}
