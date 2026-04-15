use crate::circular_buffer::{CircularBuffer, Error};

fn main() {
// Test basic functionality of the circular buffer
let mut buffer = CircularBuffer::new(2);

// Test write and read
assert!(buffer.write(1).is_ok());
assert!(buffer.write(2).is_ok());
assert_eq!(buffer.read(), Ok(1));

// Test full buffer error
assert!(buffer.write(3).is_ok());
assert_eq!(buffer.write(4), Err(Error::FullBuffer));

// Test overwrite
buffer.overwrite(4);
assert_eq!(buffer.read(), Ok(3));
assert_eq!(buffer.read(), Ok(4));

// Test empty buffer error
assert_eq!(buffer.read(), Err(Error::EmptyBuffer));

// Test clear
assert!(buffer.write(1).is_ok());
buffer.clear();
assert_eq!(buffer.read(), Err(Error::EmptyBuffer));

println!("All tests passed!");
}
