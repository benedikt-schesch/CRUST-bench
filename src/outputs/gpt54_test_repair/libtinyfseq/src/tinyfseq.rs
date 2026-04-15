
#[derive(Debug, PartialEq)]
pub enum TFError {
TF_OK = 0,
TF_EINVALID_MAGIC,
TF_EINVALID_COMPRESSION_TYPE,
TF_EINVALID_BUFFER_SIZE,
TF_EINVALID_VAR_SIZE,
}
impl TFError {
pub fn to_string(&self) -> &'static str {
match self {
TFError::TF_OK => "TF_OK (ok)",
TFError::TF_EINVALID_MAGIC => {
"TF_EINVALID_MAGIC (invalid magic file signature)"
}
TFError::TF_EINVALID_COMPRESSION_TYPE => {
"TF_EINVALID_COMPRESSION_TYPE (unknown compression identifier)"
}
TFError::TF_EINVALID_BUFFER_SIZE => {
"TF_EINVALID_BUFFER_SIZE (undersized data decoding buffer argument)"
}
TFError::TF_EINVALID_VAR_SIZE => {
"TF_EINVALID_VAR_SIZE (invalid variable size in header)"
}
}
}
}
#[derive(Debug)]
pub enum TFCompressionType {
TF_COMPRESSION_NONE,
TF_COMPRESSION_ZSTD,
TF_COMPRESSION_ZLIB,
}
#[derive(Debug)]
pub struct TFHeader {
pub channel_data_offset: u16,
pub minor_version: u8,
pub major_version: u8,
pub variable_data_offset: u16,
pub channel_count: u32,
pub frame_count: u32,
pub frame_step_time_millis: u8,
pub compression_type: TFCompressionType,
pub compression_block_count: u8,
pub channel_range_count: u8,
pub sequence_uid: u64,
}
#[derive(Debug)]
pub struct TFCompressionBlock {
pub first_frame_id: u32,
pub size: u32,
}
#[derive(Debug)]
pub struct TFVarHeader {
pub size: u16,
pub id: [u8; 2],
}
#[derive(Debug)]
pub struct TFChannelRange {
pub first_channel_number: u32,
pub channel_count: u32,
}
fn tf_compression_type_valid(b: u8) -> bool {
matches!(b, 0 | 1 | 2)
}
fn read_u16_le(bytes: &[u8]) -> u16 {
u16::from_le_bytes([bytes[0], bytes[1]])
}
fn read_u32_le(bytes: &[u8]) -> u32 {
u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
}
fn read_u64_le(bytes: &[u8]) -> u64 {
u64::from_le_bytes([
bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
])
}
fn tf_uint24_read(bd: &[u8]) -> u32 {
(bd[0] as u32) | ((bd[1] as u32) << 8) | ((bd[2] as u32) << 16)
}
pub fn tf_var_header_read<'a>(
bd: &'a [u8],
var_header: &mut TFVarHeader,
vd: &mut [u8],
ep: Option<&mut &'a [u8]>,
) -> TFError {
const VAR_HEADER_SIZE: usize = 4;
if bd.len() <= VAR_HEADER_SIZE {
return TFError::TF_EINVALID_BUFFER_SIZE;
}
var_header.size = read_u16_le(&bd[0..2]);
if var_header.size as usize <= VAR_HEADER_SIZE {
return TFError::TF_EINVALID_VAR_SIZE;
}
var_header.id.copy_from_slice(&bd[2..4]);
let total_size = var_header.size as usize;
if !vd.is_empty() {
if bd.len() < total_size {
return TFError::TF_EINVALID_VAR_SIZE;
}
let value_size = total_size - VAR_HEADER_SIZE;
if vd.len() < value_size {
return TFError::TF_EINVALID_BUFFER_SIZE;
}
vd[..value_size].copy_from_slice(&bd[VAR_HEADER_SIZE..VAR_HEADER_SIZE + value_size]);
}
if let Some(ep_ref) = ep {
if bd.len() >= total_size {
*ep_ref = &bd[total_size..];
} else {
*ep_ref = &bd[bd.len()..];
}
}
TFError::TF_OK
}
pub fn tf_header_read<'a>(
bd: &'a [u8],
header: &mut TFHeader,
ep: Option<&mut &'a [u8]>,
) -> TFError {
const HEADER_SIZE: usize = 32;
if bd.len() < HEADER_SIZE {
return TFError::TF_EINVALID_BUFFER_SIZE;
}
if bd[0] != b'P' || bd[1] != b'S' || bd[2] != b'E' || bd[3] != b'Q' {
return TFError::TF_EINVALID_MAGIC;
}
header.channel_data_offset = read_u16_le(&bd[4..6]);
header.minor_version = bd[6];
header.major_version = bd[7];
header.variable_data_offset = read_u16_le(&bd[8..10]);
header.channel_count = read_u32_le(&bd[10..14]);
header.frame_count = read_u32_le(&bd[14..18]);
header.frame_step_time_millis = bd[18];
let compression_type = bd[20] & 0xF;
if !tf_compression_type_valid(compression_type) {
return TFError::TF_EINVALID_COMPRESSION_TYPE;
}
header.compression_type = match compression_type {
0 => TFCompressionType::TF_COMPRESSION_NONE,
1 => TFCompressionType::TF_COMPRESSION_ZSTD,
2 => TFCompressionType::TF_COMPRESSION_ZLIB,
_ => return TFError::TF_EINVALID_COMPRESSION_TYPE,
};
header.compression_block_count = bd[21];
header.channel_range_count = bd[22];
header.sequence_uid = read_u64_le(&bd[24..32]);
if let Some(ep_ref) = ep {
*ep_ref = &bd[HEADER_SIZE..];
}
TFError::TF_OK
}
pub fn tf_compression_block_read<'a>(
bd: &'a [u8],
block: &mut TFCompressionBlock,
ep: Option<&mut &'a [u8]>,
) -> TFError {
const COMPRESSION_BLOCK_SIZE: usize = 8;
if bd.len() < COMPRESSION_BLOCK_SIZE {
return TFError::TF_EINVALID_BUFFER_SIZE;
}
block.first_frame_id = read_u32_le(&bd[0..4]);
block.size = read_u32_le(&bd[4..8]);
if let Some(ep_ref) = ep {
*ep_ref = &bd[COMPRESSION_BLOCK_SIZE..];
}
TFError::TF_OK
}
pub fn tf_channel_range_read<'a>(
bd: &'a [u8],
channel_range: &mut TFChannelRange,
ep: Option<&mut &'a [u8]>,
) -> TFError {
const CHANNEL_RANGE_SIZE: usize = 6;
if bd.len() < CHANNEL_RANGE_SIZE {
return TFError::TF_EINVALID_BUFFER_SIZE;
}
channel_range.first_channel_number = tf_uint24_read(&bd[0..3]);
channel_range.channel_count = tf_uint24_read(&bd[3..6]);
if let Some(ep_ref) = ep {
*ep_ref = &bd[CHANNEL_RANGE_SIZE..];
}
TFError::TF_OK
}
