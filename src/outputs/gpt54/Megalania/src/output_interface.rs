// Generated Rust Code
use std::io;

pub trait OutputInterface {
fn write(&mut self, data: &[u8]) -> io::Result<usize>;
}
