use crate::compactsize::compactsize_peek_length;
use crate::compactsize::compactsize_read;
use crate::psbt::PsbtResult;

/// Translates the C struct psbt_txin.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PsbtTxIn {
pub txid: Vec<u8>,
pub index: u32,
pub script: Vec<u8>,
pub sequence_number: u32,
}

/// Translates the C struct psbt_txout.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PsbtTxOut {
pub amount: u64,
pub script: Vec<u8>,
}

/// Translates the C struct psbt_witness_item.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PsbtWitnessItem {
pub input_index: i32,
pub item_index: i32,
pub item: Vec<u8>,
}

/// Translates the C struct psbt_tx.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PsbtTx {
pub version: u32,
pub lock_time: u32,
}

/// Translates the C union inside psbt_txelem.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PsbtTxElem {
TxIn(PsbtTxIn),
TxOut(PsbtTxOut),
Tx(PsbtTx),
WitnessItem(PsbtWitnessItem),
}

/// The handler type for psbt_txelem.
pub type PsbtTxElemHandler = fn(elem: &mut PsbtTxElem, user_data: &mut dyn std::any::Any);

fn parse_le32(cursor: &[u8]) -> Option<u32> {
if cursor.len() < 4 {
None
} else {
Some(u32::from_le_bytes([cursor[0], cursor[1], cursor[2], cursor[3]]))
}
}

fn parse_le64(cursor: &[u8]) -> Option<u64> {
if cursor.len() < 8 {
None
} else {
Some(u64::from_le_bytes([
cursor[0], cursor[1], cursor[2], cursor[3], cursor[4], cursor[5], cursor[6],
cursor[7],
]))
}
}

/// Parse a Bitcoin transaction.
pub fn psbt_btc_tx_parse(
tx: &[u8],
tx_size: usize,
user_data: &mut dyn std::any::Any,
handler: Option<PsbtTxElemHandler>,
) -> PsbtResult {
let data = if tx.len() >= tx_size { &tx[..tx_size] } else { tx };
let mut p = 0usize;

if data.len() < 4 {
return PsbtResult::ReadError;
}
let version = match parse_le32(&data[p..]) {
Some(v) => v,
None => return PsbtResult::ReadError,
};
p += 4;

if p >= data.len() {
return PsbtResult::ReadError;
}
let size_len = compactsize_peek_length(data[p]) as usize;
if p + size_len > data.len() {
return PsbtResult::ReadError;
}
let (count_u64, res) = compactsize_read(&data[p..]);
if res != PsbtResult::Ok {
return res;
}
p += size_len;
let inputs = count_u64 as usize;

for _ in 0..inputs {
if p + 32 > data.len() {
return PsbtResult::ReadError;
}
let txid = data[p..p + 32].to_vec();
p += 32;

if p + 4 > data.len() {
return PsbtResult::ReadError;
}
let index = match parse_le32(&data[p..]) {
Some(v) => v,
None => return PsbtResult::ReadError,
};
p += 4;

if p >= data.len() {
return PsbtResult::ReadError;
}
let slen_len = compactsize_peek_length(data[p]) as usize;
if p + slen_len > data.len() {
return PsbtResult::ReadError;
}
let (script_len_u64, res2) = compactsize_read(&data[p..]);
if res2 != PsbtResult::Ok {
return res2;
}
p += slen_len;
let script_len = script_len_u64 as usize;
if p + script_len > data.len() {
return PsbtResult::ReadError;
}
let script = if script_len == 0 {
Vec::new()
} else {
data[p..p + script_len].to_vec()
};
p += script_len;

if p + 4 > data.len() {
return PsbtResult::ReadError;
}
let sequence_number = match parse_le32(&data[p..]) {
Some(v) => v,
None => return PsbtResult::ReadError,
};
p += 4;

if let Some(h) = handler {
let mut elem = PsbtTxElem::TxIn(PsbtTxIn {
txid,
index,
script,
sequence_number,
});
h(&mut elem, user_data);
}
}

if p >= data.len() {
return PsbtResult::ReadError;
}
let out_count_len = compactsize_peek_length(data[p]) as usize;
if p + out_count_len > data.len() {
return PsbtResult::ReadError;
}
let (out_count_u64, res3) = compactsize_read(&data[p..]);
if res3 != PsbtResult::Ok {
return res3;
}
p += out_count_len;
let outputs = out_count_u64 as usize;

for _ in 0..outputs {
if p + 8 > data.len() {
return PsbtResult::ReadError;
}
let amount = match parse_le64(&data[p..]) {
Some(v) => v,
None => return PsbtResult::ReadError,
};
p += 8;

if p >= data.len() {
return PsbtResult::ReadError;
}
let slen_len = compactsize_peek_length(data[p]) as usize;
if p + slen_len > data.len() {
return PsbtResult::ReadError;
}
let (script_len_u64, res4) = compactsize_read(&data[p..]);
if res4 != PsbtResult::Ok {
return res4;
}
p += slen_len;
let script_len = script_len_u64 as usize;
if p + script_len > data.len() {
return PsbtResult::ReadError;
}
let script = data[p..p + script_len].to_vec();
p += script_len;

if let Some(h) = handler {
let mut elem = PsbtTxElem::TxOut(PsbtTxOut { amount, script });
h(&mut elem, user_data);
}
}

if p + 4 > data.len() {
return PsbtResult::ReadError;
}
let lock_time = match parse_le32(&data[p..]) {
Some(v) => v,
None => return PsbtResult::ReadError,
};
p += 4;

if p != data.len() {
return PsbtResult::ReadError;
}

if let Some(h) = handler {
let mut elem = PsbtTxElem::Tx(PsbtTx { version, lock_time });
h(&mut elem, user_data);
}

PsbtResult::Ok
}
