use crate::base64::base62_encode;
use crate::base64::base64_decode;
use crate::base64::base64_encode;
use crate::compactsize::compactsize_length;
use crate::compactsize::compactsize_peek_length;
use crate::compactsize::compactsize_read;
use crate::compactsize::compactsize_write;
use crate::tx::*;
use std::fmt;

// Common constant from common.h
pub const MAX_SERIALIZE_SIZE: u32 = 0x02000000;

// --- Enum definitions ---
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PsbtResult {
Ok,
CompactReadError,
ReadError,
WriteError,
InvalidState,
NotImplemented,
OobWrite,
}
impl fmt::Display for PsbtResult {
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
write!(f, "{:?}", self)
}
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PsbtGlobalType {
UnsignedTx = 0,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PsbtEncoding {
Hex,
Base64,
Base62,
Protobuf,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PsbtInputType {
NonWitnessUtxo = 0,
WitnessUtxo = 1,
PartialSig = 2,
SighashType = 3,
RedeemScript = 4,
WitnessScript = 5,
Bip32Derivation = 6,
FinalScriptSig = 7,
FinalScriptWitness = 8,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PsbtOutputType {
RedeemScript = 0,
WitnessScript = 1,
Bip32Derivation = 2,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum PsbtScope {
Global,
Inputs,
Outputs,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PsbtState {
Init = 2,
Global,
Inputs,
InputsNew,
Outputs,
OutputsNew,
Finalized,
}

#[derive(Debug, PartialEq, Eq)]
pub enum PsbtElemType {
Record,
TxElem,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PsbtTxElemType {
TxIn,
TxOut,
Tx,
WitnessItem,
}

// --- Struct definitions ---
/// Translates the C struct psbt.
/// (Here we use a Vec<u8> to hold the PSBT data and a write position index.)
pub struct Psbt {
pub state: PsbtState,
pub data: Vec<u8>,
pub write_pos: usize,
pub data_capacity: usize,
pub records: Vec<PsbtRecord>,
}
impl Psbt {
pub fn new(capacity: usize) -> Self {
Self {
state: PsbtState::Init,
data: Vec::with_capacity(capacity),
write_pos: 0,
data_capacity: capacity,
records: Vec::new(),
}
}
}

/// Translates the C struct psbt_record.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PsbtRecord {
pub record_type: u8,
pub key: Vec<u8>,
pub val: Vec<u8>,
pub scope: PsbtScope,
}

/// Translates the C union (record/txelem) in psbt_elem into an enum.
pub enum PsbtElem {
Record { index: i32, record: PsbtRecord },
TxElem { index: i32, txelem: PsbtTxElem },
}

/// The C typedef for a handler function.
pub type PsbtElemHandler = fn(elem: &mut PsbtElem, user_data: &mut dyn std::any::Any);

// External constants
pub const PSBT_MAGIC: [u8; 4] = [0x70, 0x73, 0x62, 0x74];
pub static PSBT_ERRMSG: &str = "psbt error";

fn push_byte(psbt: &mut Psbt, b: u8) -> PsbtResult {
if psbt.write_pos + 1 > psbt.data_capacity {
return PsbtResult::OobWrite;
}
if psbt.write_pos < psbt.data.len() {
psbt.data[psbt.write_pos] = b;
} else {
psbt.data.push(b);
}
psbt.write_pos += 1;
PsbtResult::Ok
}

fn push_bytes(psbt: &mut Psbt, bytes: &[u8]) -> PsbtResult {
if psbt.write_pos + bytes.len() > psbt.data_capacity {
return PsbtResult::OobWrite;
}
if psbt.write_pos == psbt.data.len() {
psbt.data.extend_from_slice(bytes);
} else {
let needed = psbt.write_pos + bytes.len();
if needed > psbt.data.len() {
psbt.data.resize(needed, 0);
}
psbt.data[psbt.write_pos..psbt.write_pos + bytes.len()].copy_from_slice(bytes);
}
psbt.write_pos += bytes.len();
PsbtResult::Ok
}

fn psbt_write_header(tx: &mut Psbt) -> PsbtResult {
let r = push_bytes(tx, &PSBT_MAGIC);
if r != PsbtResult::Ok {
return r;
}
let r = push_byte(tx, 0xff);
if r != PsbtResult::Ok {
return r;
}
tx.state = PsbtState::Global;
PsbtResult::Ok
}

fn psbt_close_records(tx: &mut Psbt) -> PsbtResult {
push_byte(tx, 0)
}

fn psbt_write_record(tx: &mut Psbt, rec: &PsbtRecord) -> PsbtResult {
let key_size_with_type = rec.key.len() as u64 + 1;
let size = compactsize_length(key_size_with_type) as usize;
let mut buf = [0u8; 9];
compactsize_write(&mut buf[..size], key_size_with_type);
let r = push_bytes(tx, &buf[..size]);
if r != PsbtResult::Ok {
return r;
}

let r = push_byte(tx, rec.record_type);
if r != PsbtResult::Ok {
return r;
}

let r = push_bytes(tx, &rec.key);
if r != PsbtResult::Ok {
return r;
}

let vsize = compactsize_length(rec.val.len() as u64) as usize;
let mut vbuf = [0u8; 9];
compactsize_write(&mut vbuf[..vsize], rec.val.len() as u64);
let r = push_bytes(tx, &vbuf[..vsize]);
if r != PsbtResult::Ok {
return r;
}

let r = push_bytes(tx, &rec.val);
if r != PsbtResult::Ok {
return r;
}

PsbtResult::Ok
}

/// Return the number of bytes stored in the PSBT.
pub fn psbt_size(tx: &Psbt) -> usize {
tx.data.len()
}

/// Read and parse a PSBT.
pub fn psbt_read(
src: &[u8],
_src_size: usize,
psbt: &mut Psbt,
elem_handler: Option<PsbtElemHandler>,
user_data: &mut dyn std::any::Any,
) -> PsbtResult {
if psbt.state != PsbtState::Init {
return PsbtResult::InvalidState;
}
if src.len() > psbt.data_capacity {
return PsbtResult::OobWrite;
}

psbt.data.clear();
psbt.data.extend_from_slice(src);
psbt.write_pos = 0;
psbt.state = PsbtState::Init;
psbt.records.clear();

if psbt.data.len() < 5 {
return PsbtResult::ReadError;
}
if psbt.data[0..4] != PSBT_MAGIC {
return PsbtResult::ReadError;
}
if psbt.data[4] != 0xff {
return PsbtResult::ReadError;
}
psbt.write_pos = 5;
psbt.state = PsbtState::Global;

let mut kvs = 0i32;
let mut input_sets = 0usize;
let mut output_sets = 0usize;
let mut unsigned_tx_seen = false;

if psbt.write_pos > psbt.data.len() {
return PsbtResult::ReadError;
}

while psbt.write_pos < psbt.data.len() {
match psbt.state {
PsbtState::Global | PsbtState::Inputs | PsbtState::Outputs => {
if psbt.write_pos >= psbt.data.len() {
break;
}
if psbt.data[psbt.write_pos] == 0 {
match psbt.state {
PsbtState::Global => {
psbt.state = PsbtState::InputsNew;
psbt.write_pos += 1;
kvs = 0;
}
PsbtState::Inputs => {
input_sets += 1;
psbt.write_pos += 1;
if unsigned_tx_seen && input_sets >= 1 {
psbt.state = PsbtState::OutputsNew;
kvs = 0;
} else {
psbt.state = PsbtState::InputsNew;
}
}
PsbtState::Outputs => {
output_sets += 1;
let _ = output_sets;
psbt.write_pos += 1;
psbt.state = PsbtState::Finalized;
break;
}
_ => return PsbtResult::InvalidState,
}
} else {
let start = psbt.write_pos;
let size_len = compactsize_peek_length(psbt.data[start]) as usize;
if start + size_len > psbt.data.len() {
return PsbtResult::ReadError;
}
let (size_u64, res) = compactsize_read(&psbt.data[start..]);
if res != PsbtResult::Ok {
return res;
}
let size = size_u64 as usize;
if size == 0 {
return PsbtResult::ReadError;
}
psbt.write_pos += size_len;
if psbt.write_pos + size > psbt.data.len() {
return PsbtResult::ReadError;
}

let record_type = psbt.data[psbt.write_pos];
let key = if size > 1 {
psbt.data[psbt.write_pos + 1..psbt.write_pos + size].to_vec()
} else {
Vec::new()
};
psbt.write_pos += size;

if psbt.write_pos >= psbt.data.len() {
return PsbtResult::ReadError;
}
let vsize_len = compactsize_peek_length(psbt.data[psbt.write_pos]) as usize;
if psbt.write_pos + vsize_len > psbt.data.len() {
return PsbtResult::ReadError;
}
let (val_size_u64, res2) = compactsize_read(&psbt.data[psbt.write_pos..]);
if res2 != PsbtResult::Ok {
return res2;
}
let val_size = val_size_u64 as usize;
psbt.write_pos += vsize_len;
if psbt.write_pos + val_size > psbt.data.len() {
return PsbtResult::ReadError;
}
let val = psbt.data[psbt.write_pos..psbt.write_pos + val_size].to_vec();
psbt.write_pos += val_size;

let scope = match psbt.state {
PsbtState::Global => PsbtScope::Global,
PsbtState::Inputs => PsbtScope::Inputs,
PsbtState::Outputs => PsbtScope::Outputs,
_ => return PsbtResult::InvalidState,
};

let rec = PsbtRecord {
record_type,
key,
val: val.clone(),
scope: scope.clone(),
};
psbt.records.push(rec.clone());

if let Some(handler) = elem_handler {
let mut elem = PsbtElem::Record {
index: kvs,
record: rec.clone(),
};
handler(&mut elem, user_data);
}

if matches!(scope, PsbtScope::Global) && record_type == 0 {
unsigned_tx_seen = true;
let _ = psbt_btc_tx_parse(&val, val.len(), user_data, None);
}

kvs += 1;
}
}
PsbtState::InputsNew => {
psbt.state = PsbtState::Inputs;
}
PsbtState::OutputsNew => {
psbt.state = PsbtState::Outputs;
}
PsbtState::Finalized => break,
PsbtState::Init => return PsbtResult::InvalidState,
}
}

if psbt.state != PsbtState::Finalized {
return PsbtResult::InvalidState;
}

PsbtResult::Ok
}

fn hexdigit(hex: u8) -> u8 {
if hex <= b'9' {
hex - b'0'
} else {
hex.to_ascii_uppercase() - b'A' + 10
}
}

/// Decode a hex or base64 string into dest.
pub fn psbt_decode(
src: &str,
_src_size: usize,
dest: &mut [u8],
dest_size: usize,
psbt_len: &mut usize,
) -> PsbtResult {
let bytes = src.as_bytes();
let b64_magic = b"cHNid";
if bytes.len() < b64_magic.len() {
return PsbtResult::ReadError;
}

if &bytes[..b64_magic.len()] == b64_magic {
let slice_len = dest_size.min(dest.len());
match base64_decode(bytes, &mut dest[..slice_len]) {
Some(n) => {
*psbt_len = n;
PsbtResult::Ok
}
None => PsbtResult::ReadError,
}
} else {
if bytes.len() % 2 != 0 {
return PsbtResult::ReadError;
}
if dest_size < bytes.len() / 2 || dest.len() < bytes.len() / 2 {
return PsbtResult::ReadError;
}
let mut j = 0usize;
let mut i = 0usize;
while i < bytes.len() {
let c1 = bytes[i];
let c2 = bytes[i + 1];
if !(c1 as char).is_ascii_hexdigit() || !(c2 as char).is_ascii_hexdigit() {
return PsbtResult::ReadError;
}
dest[j] = (hexdigit(c1) << 4) | hexdigit(c2);
j += 1;
i += 2;
}
*psbt_len = bytes.len() / 2;
PsbtResult::Ok
}
}

fn hexchar(val: u8) -> u8 {
if val < 10 {
b'0' + val
} else {
b'a' + (val - 10)
}
}

/// Encode the PSBT data into a destination buffer using the requested encoding.
pub fn psbt_encode(
psbt: &Psbt,
encoding: PsbtEncoding,
dest: &mut [u8],
dest_size: usize,
out_len: &mut usize,
) -> PsbtResult {
if psbt.state != PsbtState::Finalized {
return PsbtResult::WriteError;
}
psbt_encode_raw(&psbt.data, psbt_size(psbt), encoding, dest, dest_size, out_len)
}

/// Encode raw PSBT data into dest using the requested encoding.
pub fn psbt_encode_raw(
psbt_data: &[u8],
_psbt_len: usize,
encoding: PsbtEncoding,
dest: &mut [u8],
dest_size: usize,
out_len: &mut usize,
) -> PsbtResult {
match encoding {
PsbtEncoding::Hex => {
let needed = psbt_data.len() * 2 + 1;
if dest_size < needed || dest.len() < needed {
return PsbtResult::OobWrite;
}
let mut j = 0usize;
for &b in psbt_data {
dest[j] = hexchar(b >> 4);
dest[j + 1] = hexchar(b & 0x0f);
j += 2;
}
dest[j] = 0;
*out_len = needed;
PsbtResult::Ok
}
PsbtEncoding::Base64 => {
let slice_len = dest_size.min(dest.len());
match base64_encode(psbt_data, &mut dest[..slice_len]) {
Some(n) => {
*out_len = n;
PsbtResult::Ok
}
None => PsbtResult::WriteError,
}
}
PsbtEncoding::Base62 => {
let slice_len = dest_size.min(dest.len());
match base62_encode(psbt_data, &mut dest[..slice_len]) {
Some(n) => {
*out_len = n;
PsbtResult::Ok
}
None => PsbtResult::WriteError,
}
}
PsbtEncoding::Protobuf => PsbtResult::NotImplemented,
}
}

/// Return the last error message.
pub fn psbt_geterr() -> &'static str {
PSBT_ERRMSG
}

/// Convert a PSBT state to a human–readable string.
pub fn psbt_state_tostr(state: PsbtState) -> &'static str {
match state {
PsbtState::Init => "INIT",
PsbtState::Global => "GLOBAL",
PsbtState::Inputs => "INPUTS",
PsbtState::InputsNew => "INPUTS_NEW",
PsbtState::Outputs => "OUTPUTS",
PsbtState::OutputsNew => "OUTPUTS_NEW",
PsbtState::Finalized => "FINALIZED",
}
}

/// Return a string for a record type and scope.
pub fn psbt_type_tostr(record_type: u8, scope: PsbtScope) -> &'static str {
match scope {
PsbtScope::Global => match record_type {
0 => psbt_global_type_tostr(PsbtGlobalType::UnsignedTx),
_ => "UNKNOWN_GLOBAL_TYPE",
},
PsbtScope::Inputs => match record_type {
0 => psbt_input_type_tostr(PsbtInputType::NonWitnessUtxo),
1 => psbt_input_type_tostr(PsbtInputType::WitnessUtxo),
2 => psbt_input_type_tostr(PsbtInputType::PartialSig),
3 => psbt_input_type_tostr(PsbtInputType::SighashType),
4 => psbt_input_type_tostr(PsbtInputType::RedeemScript),
5 => psbt_input_type_tostr(PsbtInputType::WitnessScript),
6 => psbt_input_type_tostr(PsbtInputType::Bip32Derivation),
7 => psbt_input_type_tostr(PsbtInputType::FinalScriptSig),
8 => psbt_input_type_tostr(PsbtInputType::FinalScriptWitness),
_ => "UNKNOWN_INPUT_TYPE",
},
PsbtScope::Outputs => match record_type {
0 => psbt_output_type_tostr(PsbtOutputType::RedeemScript),
1 => psbt_output_type_tostr(PsbtOutputType::WitnessScript),
2 => psbt_output_type_tostr(PsbtOutputType::Bip32Derivation),
_ => "UNKNOWN_OUTPUT_TYPE",
},
}
}

/// Return a string for a psbt_txelem type.
pub fn psbt_txelem_type_tostr(txelem_type: PsbtTxElemType) -> &'static str {
match txelem_type {
PsbtTxElemType::Tx => "TX",
PsbtTxElemType::TxIn => "TXIN",
PsbtTxElemType::TxOut => "TXOUT",
PsbtTxElemType::WitnessItem => "WITNESS_ITEM",
}
}

pub fn psbt_global_type_tostr(gt: PsbtGlobalType) -> &'static str {
match gt {
PsbtGlobalType::UnsignedTx => "GLOBAL_UNSIGNED_TX",
}
}

pub fn psbt_output_type_tostr(ot: PsbtOutputType) -> &'static str {
match ot {
PsbtOutputType::RedeemScript => "OUT_REDEEM_SCRIPT",
PsbtOutputType::WitnessScript => "OUT_WITNESS_SCRIPT",
PsbtOutputType::Bip32Derivation => "OUT_BIP32_DERIVATION",
}
}

pub fn psbt_input_type_tostr(it: PsbtInputType) -> &'static str {
match it {
PsbtInputType::NonWitnessUtxo => "IN_NON_WITNESS_UTXO",
PsbtInputType::WitnessUtxo => "IN_WITNESS_UTXO",
PsbtInputType::PartialSig => "IN_PARTIAL_SIG",
PsbtInputType::SighashType => "IN_SIGHASH_TYPE",
PsbtInputType::RedeemScript => "IN_REDEEM_SCRIPT",
PsbtInputType::WitnessScript => "IN_WITNESS_SCRIPT",
PsbtInputType::Bip32Derivation => "IN_BIP32_DERIVATION",
PsbtInputType::FinalScriptSig => "IN_FINAL_SCRIPTSIG",
PsbtInputType::FinalScriptWitness => "IN_FINAL_SCRIPTWITNESS",
}
}

/// Write a global record into the PSBT.
pub fn psbt_write_global_record(psbt: &mut Psbt, rec: &PsbtRecord) -> PsbtResult {
if psbt.state == PsbtState::Init {
let r = psbt_write_header(psbt);
if r != PsbtResult::Ok {
return r;
}
psbt.state = PsbtState::Global;
} else if psbt.state != PsbtState::Global {
return PsbtResult::InvalidState;
}
psbt.records.push(rec.clone());
psbt_write_record(psbt, rec)
}

/// Write an input record into the PSBT.
pub fn psbt_write_input_record(psbt: &mut Psbt, rec: &PsbtRecord) -> PsbtResult {
if psbt.state == PsbtState::Global {
let r = psbt_close_records(psbt);
if r != PsbtResult::Ok {
return r;
}
psbt.state = PsbtState::Inputs;
} else if psbt.state != PsbtState::Inputs && psbt.state != PsbtState::InputsNew {
return PsbtResult::InvalidState;
}
psbt.records.push(rec.clone());
psbt_write_record(psbt, rec)
}

/// Write an output record into the PSBT.
pub fn psbt_write_output_record(psbt: &mut Psbt, rec: &PsbtRecord) -> PsbtResult {
if psbt.state == PsbtState::Inputs {
let r = psbt_close_records(psbt);
if r != PsbtResult::Ok {
return r;
}
psbt.state = PsbtState::Outputs;
} else if psbt.state != PsbtState::Outputs && psbt.state != PsbtState::OutputsNew {
return PsbtResult::InvalidState;
}
psbt.records.push(rec.clone());
psbt_write_record(psbt, rec)
}

/// Create a new input record set.
pub fn psbt_new_input_record_set(psbt: &mut Psbt) -> PsbtResult {
if psbt.state == PsbtState::Global
|| psbt.state == PsbtState::InputsNew
|| psbt.state == PsbtState::Inputs
{
let r = psbt_close_records(psbt);
if r != PsbtResult::Ok {
return r;
}
psbt.state = PsbtState::InputsNew;
PsbtResult::Ok
} else {
PsbtResult::InvalidState
}
}

/// Create a new output record set.
pub fn psbt_new_output_record_set(psbt: &mut Psbt) -> PsbtResult {
if psbt.state == PsbtState::Inputs
|| psbt.state == PsbtState::InputsNew
|| psbt.state == PsbtState::OutputsNew
|| psbt.state == PsbtState::Outputs
{
let r = psbt_close_records(psbt);
if r != PsbtResult::Ok {
return r;
}
psbt.state = PsbtState::OutputsNew;
PsbtResult::Ok
} else {
PsbtResult::InvalidState
}
}

/// Initialize a PSBT using the given destination buffer.
pub fn psbt_init(psbt: &mut Psbt, _dest: &mut [u8], dest_size: usize) -> PsbtResult {
psbt.write_pos = 0;
psbt.data.clear();
psbt.data_capacity = dest_size;
psbt.state = PsbtState::Init;
psbt.records.clear();
PsbtResult::Ok
}

/// Print the PSBT (only succeeds after finalization).
pub fn psbt_print(psbt: &Psbt, stream: &mut dyn std::io::Write) -> PsbtResult {
if psbt.state != PsbtState::Finalized {
return PsbtResult::InvalidState;
}
for b in &psbt.data {
let _ = stream.write_all(&[hexchar(b >> 4), hexchar(b & 0x0f)]);
}
let _ = stream.write_all(b"\n");
PsbtResult::Ok
}

/// Finalize the PSBT.
pub fn psbt_finalize(psbt: &mut Psbt) -> PsbtResult {
if psbt.state != PsbtState::OutputsNew && psbt.state != PsbtState::Outputs {
return PsbtResult::InvalidState;
}
let r = psbt_close_records(psbt);
if r != PsbtResult::Ok {
return r;
}
psbt.state = PsbtState::Finalized;
PsbtResult::Ok
}
