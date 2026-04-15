//! Module for working with 2bit file format.

use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use std::path::Path;

/// Statistics for base counts returned by twobit_bases.
#[derive(Debug, Default)]
pub struct BaseStats {
pub a: u64,
pub c: u64,
pub g: u64,
pub t: u64,
pub n: u64,
}

/// Header structure for TwoBit format.
pub struct TwoBitHeader {
pub n_chroms: u32,
version: u32,
reserved: u32,
}

/// Chromosome/Contig Length structure containing chromosome names.
pub struct TwoBitCL {
pub chrom: Vec<String>,
}

/// Masked index structure for TwoBit format containing sizes and offsets.
pub struct TwoBitMaskedIdx {
pub size: Vec<u32>,
pub offset: Vec<u32>,
}

/// Main structure for handling TwoBit files.
pub struct TwoBit {
file: File,
pub hdr: TwoBitHeader,
pub cl: TwoBitCL,
pub idx: TwoBitMaskedIdx,
}

impl TwoBit {
/// Open a 2bit file.
///
/// # Arguments
/// * `filename` - Path to the 2bit file
/// * `mask` - Whether to load mask information
///
/// # Returns
/// * `Ok(TwoBit)` on success
/// * `Err(io::Error)` if the file cannot be opened or is invalid
pub fn twobit_open<P: AsRef<Path>>(filename: P, _mask: bool) -> io::Result<Self> {
let mut file = File::open(filename)?;

// Read header (16 bytes)
let mut header_buf = [0u8; 16];
file.read_exact(&mut header_buf)?;

// Parse header (big-endian format)
let magic = u32::from_be_bytes([header_buf[0], header_buf[1], header_buf[2], header_buf[3]]);
let version = u32::from_be_bytes([header_buf[4], header_buf[5], header_buf[6], header_buf[7]]);
let n_chroms = u32::from_be_bytes([header_buf[8], header_buf[9], header_buf[10], header_buf[11]]);
let reserved = u32::from_be_bytes([header_buf[12], header_buf[13], header_buf[14], header_buf[15]]);

// Verify magic number (0x1A412743 for 2bit format)
if magic != 0x1A412743 {
return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid 2bit magic number"));
}

// Read index for each chromosome
let mut chroms = Vec::with_capacity(n_chroms as usize);
let mut sizes = Vec::with_capacity(n_chroms as usize);
let mut offsets = Vec::with_capacity(n_chroms as usize);

for _ in 0..n_chroms {
// Read name length (1 byte)
let mut name_len_buf = [0u8; 1];
file.read_exact(&mut name_len_buf)?;
let name_len = name_len_buf[0] as usize;

// Read name
let mut name_buf = vec![0u8; name_len];
file.read_exact(&mut name_buf)?;
let name = String::from_utf8(name_buf)
.map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
chroms.push(name);

// Read offset (4 bytes, big-endian)
let mut offset_buf = [0u8; 4];
file.read_exact(&mut offset_buf)?;
let offset = u32::from_be_bytes(offset_buf);
offsets.push(offset);

// Read DNA size from the offset location
let current_pos = file.stream_position()?;
file.seek(SeekFrom::Start(offset as u64))?;

let mut size_buf = [0u8; 4];
file.read_exact(&mut size_buf)?;
let size = u32::from_be_bytes(size_buf);
sizes.push(size);

// Return to index position
file.seek(SeekFrom::Start(current_pos))?;
}

let hdr = TwoBitHeader { n_chroms, version, reserved };
let cl = TwoBitCL { chrom: chroms };
let idx = TwoBitMaskedIdx { size: sizes, offset: offsets };

Ok(TwoBit { file, hdr, cl, idx })
}

/// Extract a DNA sequence from a specific chromosome and range.
///
/// # Arguments
/// * `chrom` - Chromosome name
/// * `start` - Start position (0-based)
/// * `end` - End position (exclusive)
///
/// # Returns
/// The DNA sequence as a String, or an error if the chromosome is not found or range is invalid
pub fn twobit_sequence(&mut self, chrom: &str, start: u32, end: u32) -> io::Result<String> {
// Find chromosome index
let chrom_idx = self.cl.chrom.iter()
.position(|c| c == chrom)
.ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, format!("Chromosome {} not found", chrom)))?;

let offset = self.idx.offset[chrom_idx] as u64;
let dna_size = self.idx.size[chrom_idx];

// Validate range
if start > end {
return Err(io::Error::new(io::ErrorKind::InvalidInput, "Start position cannot be greater than end"));
}
if end > dna_size {
return Err(io::Error::new(io::ErrorKind::InvalidInput, "End position exceeds chromosome length"));
}

// Seek to chromosome data header
self.file.seek(SeekFrom::Start(offset))?;

// Skip DNA size (4 bytes) - already known
self.file.seek(SeekFrom::Current(4))?;

// Read nBlockCount (4 bytes)
let mut n_block_count_buf = [0u8; 4];
self.file.read_exact(&mut n_block_count_buf)?;
let n_block_count = u32::from_be_bytes(n_block_count_buf);

// Read mBlockCount (4 bytes)
let mut m_block_count_buf = [0u8; 4];
self.file.read_exact(&mut m_block_count_buf)?;
let m_block_count = u32::from_be_bytes(m_block_count_buf);

// Skip reserved (4 bytes)
self.file.seek(SeekFrom::Current(4))?;

// Skip nBlockStarts and nBlockSizes (4 bytes each per block)
self.file.seek(SeekFrom::Current((n_block_count as i64) * 8))?;

// Skip mBlockStarts and mBlockSizes (4 bytes each per block)
self.file.seek(SeekFrom::Current((m_block_count as i64) * 8))?;

// Now at packed DNA data
// Each byte contains 4 bases (2 bits each): 00=T, 01=C, 10=A, 11=G
// High bits are first base in byte

let seq_len = (end - start) as usize;
let mut seq = String::with_capacity(seq_len);

// Calculate byte positions in packed data
let start_byte = (start / 4) as u64;
let start_offset_in_byte = (start % 4) as usize;

// Calculate number of bytes to read
let end_byte = ((end + 3) / 4) as u64;
let bytes_to_read = (end_byte - start_byte) as usize;

// Seek to start position in packed data
self.file.seek(SeekFrom::Current(start_byte as i64))?;

let mut buffer = vec![0u8; bytes_to_read];
self.file.read_exact(&mut buffer)?;

// Decode sequence
for i in 0..seq_len {
let total_offset = start_offset_in_byte + i;
let byte_idx = total_offset / 4;
let bit_offset = (3 - (total_offset % 4)) * 2; // High to low order

if byte_idx >= buffer.len() {
break;
}

let byte = buffer[byte_idx];
let bits = (byte >> bit_offset) & 0x3;

let base = match bits {
0 => 'T',
1 => 'C',
2 => 'A',
3 => 'G',
_ => unreachable!(),
};
seq.push(base);
}

Ok(seq)
}

/// Get base statistics for a specific region.
///
/// # Arguments
/// * `chrom` - Chromosome name
/// * `start` - Start position
/// * `end` - End position
/// * `_scale` - Scale factor (unused in this implementation)
///
/// # Returns
/// BaseStats containing counts of each base
pub fn twobit_bases(&mut self, chrom: &str, start: u32, end: u32, _scale: u32) -> io::Result<BaseStats> {
let seq = self.twobit_sequence(chrom, start, end)?;

let mut stats = BaseStats::default();
for base in seq.chars() {
match base {
'A' | 'a' => stats.a += 1,
'C' | 'c' => stats.c += 1,
'G' | 'g' => stats.g += 1,
'T' | 't' => stats.t += 1,
'N' | 'n' => stats.n += 1,
_ => {}
}
}
Ok(stats)
}

/// Close the 2bit file.
///
/// Consumes the TwoBit object and closes the underlying file.
pub fn twobit_close(self) -> io::Result<()> {
// File is automatically closed when dropped
Ok(())
}
}
