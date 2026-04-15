use std::io::{self, Read, Write};
pub fn read_line() -> io::Result<String> {
let mut buffer = String::new();
io::stdin().read_line(&mut buffer)?;
Ok(buffer)
}
pub fn write_output<W: Write>(writer: &mut W, output: &str) -> io::Result<()> {
writer.write_all(output.as_bytes())?;
Ok(())
}
