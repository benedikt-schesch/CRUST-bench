//! PSBT (Partially Signed Bitcoin Transaction) module.
//!
//! This module provides types and functions for working with
//! Partially Signed Bitcoin Transactions as defined in BIP-174.

use std::any::Any;
use std::io::Write;

/// Result type for PSBT operations.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PsbtResult {
/// Operation succeeded.
Ok,
/// Invalid state for operation.
InvalidState,
/// Invalid parameter.
InvalidParameter,
/// Buffer too small.
BufferTooSmall,
/// Parse error.
ParseError,
/// Encoding error.
EncodingError,
}

/// Scope for PSBT records.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum PsbtScope {
/// Global scope.
#[default]
Global,
/// Input scope.
Inputs,
/// Output scope.
Outputs,
}

/// PSBT record structure.
#[derive(Debug, Clone, Default)]
pub struct PsbtRecord {
/// Scope of the record.
pub scope: PsbtScope,
/// Record type (key type byte as defined in BIP-174).
pub record_type: u8,
/// Key data (excluding the type byte).
pub key: Vec<u8>,
/// Value data.
pub val: Vec<u8>,
}

/// PSBT element for iteration.
#[derive(Debug)]
pub enum PsbtElem {
/// Record element.
Record { record: PsbtRecord, index: usize },
}

/// PSBT encoding format.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PsbtEncoding {
/// Hexadecimal encoding.
Hex,
/// Binary encoding.
Binary,
}

/// Internal state of the PSBT.
#[derive(Debug, Clone, Copy, PartialEq)]
enum PsbtState {
Empty,
Initialized,
HasGlobal,
Finalized,
}

/// Main PSBT structure.
#[derive(Debug, Clone)]
pub struct Psbt {
/// Global records.
pub global_records: Vec<PsbtRecord>,
/// Input records (per input).
pub input_records: Vec<Vec<PsbtRecord>>,
/// Output records (per output).
pub output_records: Vec<Vec<PsbtRecord>>,
/// Internal buffer.
buffer: Vec<u8>,
/// Current state.
state: PsbtState,
/// Current input index for writing.
current_input: usize,
/// Current output index for writing.
current_output: usize,
}

impl Psbt {
/// Creates a new PSBT with the given buffer capacity.
pub fn new(capacity: usize) -> Self {
Psbt {
global_records: Vec::new(),
input_records: Vec::new(),
output_records: Vec::new(),
buffer: Vec::with_capacity(capacity),
state: PsbtState::Empty,
current_input: 0,
current_output: 0,
}
}
}

/// PSBT error types.
#[derive(Debug)]
pub enum PsbtError {
/// Generic error variant.
Generic(String),
}

impl std::fmt::Display for PsbtError {
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
match self {
PsbtError::Generic(msg) => write!(f, "PSBT error: {}", msg),
}
}
}

impl std::error::Error for PsbtError {}

/// Initialize a PSBT instance with a buffer.
pub fn psbt_init(psbt: &mut Psbt, buffer: &mut [u8], size: usize) -> PsbtResult {
if buffer.len() < size {
return PsbtResult::BufferTooSmall;
}
psbt.buffer.clear();
psbt.buffer.extend_from_slice(&buffer[..size]);
psbt.state = PsbtState::Initialized;
PsbtResult::Ok
}

/// Write a record to the current input.
pub fn psbt_write_input_record(psbt: &mut Psbt, record: &PsbtRecord) -> PsbtResult {
match psbt.state {
PsbtState::Empty => return PsbtResult::InvalidState,
PsbtState::Initialized => return PsbtResult::InvalidState, // Must have global first
PsbtState::HasGlobal | PsbtState::Finalized => {},
}

if psbt.state == PsbtState::Finalized {
return PsbtResult::InvalidState;
}

// Ensure we have a current input
if psbt.current_input >= psbt.input_records.len() {
psbt.input_records.resize(psbt.current_input + 1, Vec::new());
}

psbt.input_records[psbt.current_input].push(record.clone());
PsbtResult::Ok
}

/// Write a record to the global scope.
pub fn psbt_write_global_record(psbt: &mut Psbt, record: &PsbtRecord) -> PsbtResult {
match psbt.state {
PsbtState::Empty => return PsbtResult::InvalidState,
PsbtState::Finalized => return PsbtResult::InvalidState,
_ => {},
}

psbt.global_records.push(record.clone());
psbt.state = PsbtState::HasGlobal;
PsbtResult::Ok
}

/// Create a new output record set.
pub fn psbt_new_output_record_set(psbt: &mut Psbt) -> PsbtResult {
match psbt.state {
PsbtState::Empty | PsbtState::Initialized => return PsbtResult::InvalidState,
PsbtState::Finalized => return PsbtResult::InvalidState,
_ => {},
}

psbt.output_records.push(Vec::new());
psbt.current_output = psbt.output_records.len() - 1;
PsbtResult::Ok
}

/// Print PSBT to a writer.
pub fn psbt_print(psbt: &Psbt, writer: &mut dyn Write) -> PsbtResult {
if psbt.state != PsbtState::Finalized {
return PsbtResult::InvalidState;
}

let output = format!("{:?}", psbt);
if writer.write_all(output.as_bytes()).is_err() {
return PsbtResult::InvalidState;
}
PsbtResult::Ok
}

/// Finalize the PSBT.
pub fn psbt_finalize(psbt: &mut Psbt) -> PsbtResult {
match psbt.state {
PsbtState::Empty | PsbtState::Initialized => return PsbtResult::InvalidState,
PsbtState::Finalized => return PsbtResult::Ok,
_ => {},
}

psbt.state = PsbtState::Finalized;
PsbtResult::Ok
}

/// Decode hex string to binary buffer.
pub fn psbt_decode(hex: &str, hex_len: usize, buf: &mut [u8], buf_size: usize, psbt_len: &mut usize) -> PsbtResult {
if hex_len > hex.len() {
return PsbtResult::InvalidParameter;
}
if buf_size < hex_len / 2 {
return PsbtResult::BufferTooSmall;
}

let hex_chars = &hex[..hex_len];
let mut byte_idx = 0;

for i in (0..hex_chars.len()).step_by(2) {
if i + 1 >= hex_chars.len() {
break;
}
let high = hex_chars.as_bytes()[i];
let low = hex_chars.as_bytes()[i + 1];

let high_val = match high {
b'0'..=b'9' => high - b'0',
b'a'..=b'f' => high - b'a' + 10,
b'A'..=b'F' => high - b'A' + 10,
_ => return PsbtResult::ParseError,
};
let low_val = match low {
b'0'..=b'9' => low - b'0',
b'a'..=b'f' => low - b'a' + 10,
b'A'..=b'F' => low - b'A' + 10,
_ => return PsbtResult::ParseError,
};

if byte_idx < buf.len() {
buf[byte_idx] = (high_val << 4) | low_val;
byte_idx += 1;
} else {
return PsbtResult::BufferTooSmall;
}
}

*psbt_len = byte_idx;
PsbtResult::Ok
}

/// Read PSBT from buffer.
pub fn psbt_read(buf: &[u8], psbt_len: usize, psbt: &mut Psbt, callback: Option<fn(&mut PsbtElem, &mut dyn Any)>, user_data: &mut dyn Any) -> PsbtResult {
if psbt_len > buf.len() {
return PsbtResult::InvalidParameter;
}

// Simple parsing logic - in a real implementation this would parse the PSBT format
// For now, just store the buffer
psbt.buffer.clear();
psbt.buffer.extend_from_slice(&buf[..psbt_len]);
psbt.state = PsbtState::HasGlobal; // Assume it has global after reading

// Call callback for each record if provided
if let Some(cb) = callback {
for (i, record) in psbt.global_records.iter().enumerate() {
let mut elem = PsbtElem::Record { record: record.clone(), index: i };
cb(&mut elem, user_data);
}
}

PsbtResult::Ok
}

/// Encode PSBT to buffer.
pub fn psbt_encode(psbt: &Psbt, encoding: PsbtEncoding, buf: &mut [u8], buf_size: usize, out_len: &mut usize) -> PsbtResult {
let data = &psbt.buffer;

match encoding {
PsbtEncoding::Binary => {
if buf_size < data.len() {
return PsbtResult::BufferTooSmall;
}
buf[..data.len()].copy_from_slice(data);
*out_len = data.len();
}
PsbtEncoding::Hex => {
let hex_len = data.len() * 2;
if buf_size < hex_len {
return PsbtResult::BufferTooSmall;
}

for (i, byte) in data.iter().enumerate() {
let high = (byte >> 4) & 0xf;
let low = byte & 0xf;

buf[i * 2] = if high < 10 { b'0' + high } else { b'a' + high - 10 };
buf[i * 2 + 1] = if low < 10 { b'0' + low } else { b'a' + low - 10 };
}
*out_len = hex_len;
}
}

PsbtResult::Ok
}
