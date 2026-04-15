// Generated Rust Code
pub trait EncoderInterface {
fn encode_bit(&mut self, _bit: bool, _prob: u16) {}

fn encode_direct_bits(&mut self, _bits: u32, _num_bits: u32) {}
}
