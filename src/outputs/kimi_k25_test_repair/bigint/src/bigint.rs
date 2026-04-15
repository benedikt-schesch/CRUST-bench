#[derive(Debug, Clone)]
pub struct BigInt {
digits: Vec<u8>,
}
impl BigInt {
pub fn new() -> Self {
BigInt { digits: vec![0] }
}
pub fn from_i64(val: i64) -> Self {
BigInt { digits: vec![val as u8] }
}
}
