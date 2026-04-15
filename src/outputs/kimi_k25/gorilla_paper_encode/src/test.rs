use crate::gorilla::{FloatEncoder, FloatDecoder, BitWriter, BitReader};

fn main() {
// Initialize encoder - removed non-existent 'first' field
let mut encode = FloatEncoder {
w: BitWriter::new(),
val: 0,
leading: 0,
trailing: 0,
finished: false,
};

// Initialize decoder - removed non-existent 'first' field
let data: &[u8] = &[];
let mut decode = FloatDecoder {
val: 0,
leading: 0,
trailing: 0,
br: BitReader::new(data),
b: [0; 1024],
finished: false,
err: 0,
};

// Test encoding
let mut buffer = [0u8; 1024];
let mut length: usize = 0;  // Fixed: changed from u32 to usize
encode.float_encode_flush(&mut buffer, &mut length);

// Test decoding
let mut de_arr = [0.0f64; 100];
let mut de_len: usize = 0;  // Fixed: changed from u32 to usize
decode.float_decode_block(&buffer, &mut de_arr, &mut de_len);
}
