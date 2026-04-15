use crate::circular_buffer::{CircularBuffer, Error};
fn main() {
let mut buffer = CircularBuffer::new(2);
assert!(buffer.write(1).is_ok());
assert!(buffer.write(2).is_ok());
assert_eq!(buffer.read(), Ok(1));
assert!(buffer.write(3).is_ok());
assert_eq!(buffer.write(4), Err(Error::FullBuffer));
buffer.overwrite(4);
assert_eq!(buffer.read(), Ok(3));
assert_eq!(buffer.read(), Ok(4));
assert_eq!(buffer.read(), Err(Error::EmptyBuffer));
assert!(buffer.write(1).is_ok());
buffer.clear();
assert_eq!(buffer.read(), Err(Error::EmptyBuffer));
println!("All tests passed!");
}
