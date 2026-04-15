use std::io::{self, Write};

pub fn write_output(content: &str) -> io::Result<()> {
let stdout = io::stdout();
let mut handle = stdout.lock();
handle.write_all(content.as_bytes())?;
handle.flush()
}
