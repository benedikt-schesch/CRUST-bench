use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

#[derive(Debug, Clone, Default)]
pub struct TwoBitMaskedIdx {
pub size: Vec<u32>,
pub n_block_count: Vec<u32>,
pub n_block_start: Vec<Vec<u32>>,
pub n_block_sizes: Vec<Vec<u32>>,
pub mask_block_count: Vec<u32>,
pub mask_block_start: Vec<Vec<u32>>,
pub mask_block_sizes: Vec<Vec<u32>>,
pub offset: Vec<u64>,
}

#[derive(Debug, Clone, Default)]
pub struct TwoBitHeader {
pub magic: u32,
pub version: u32,
pub n_chroms: u32,
}

#[derive(Debug, Clone, Default)]
pub struct TwoBitCL {
pub chrom: Vec<String>,
pub offset: Vec<u32>,
}

#[derive(Debug)]
pub struct TwoBit {
pub fp: File,
pub sz: u64,
pub offset: u64,
pub data: Vec<u8>,
pub hdr: TwoBitHeader,
pub cl: TwoBitCL,
pub idx: TwoBitMaskedIdx,
}

impl TwoBit {
pub fn twobit_open(fname: &str, store_masked: bool) -> Self {
let mut fp = File::open(fname).expect("failed to open 2bit file");
let sz = fp.metadata().map(|m| m.len()).unwrap_or(0);

let mut data = Vec::new();
fp.read_to_end(&mut data)
.expect("failed to read 2bit file into memory");

let fp2 = File::open(fname).expect("failed to reopen 2bit file");

let mut tb = TwoBit {
fp: fp2,
sz,
offset: 0,
data,
hdr: TwoBitHeader::default(),
cl: TwoBitCL::default(),
idx: TwoBitMaskedIdx::default(),
};

tb.twobitHdrRead();
assert!(tb.hdr.magic != 0, "invalid 2bit header");

tb.twobitChromListRead();
assert!(!tb.cl.chrom.is_empty(), "failed to read chromosome list");

tb.twoBitIndexRead(if store_masked { 1 } else { 0 });
assert!(!tb.idx.size.is_empty(), "failed to read 2bit index");

tb
}

pub fn twobit_close(&mut self) {
self.twobitChromListDestroy();
self.twoBitIndexDestroy();
self.twobitHdrDestroy();
self.data.clear();
self.offset = 0;
self.sz = 0;
}

pub fn twobit_chrom_len(&self, chrom: &str) -> u32 {
for (i, c) in self.cl.chrom.iter().enumerate() {
if c == chrom {
return self.idx.size.get(i).copied().unwrap_or(0);
}
}
0
}

pub fn twobit_sequence(&self, chrom: &str, start: u32, end: u32) -> String {
let mut tid_opt = None;
for (i, c) in self.cl.chrom.iter().enumerate() {
if c == chrom {
tid_opt = Some(i as u32);
break;
}
}
let tid = match tid_opt {
Some(v) => v,
None => return String::new(),
};

let mut real_end = end;
if start == 0 && end == 0 {
real_end = self.idx.size[tid as usize];
}

if real_end > self.idx.size[tid as usize] || start >= real_end {
return String::new();
}

let mut tb = self.clone_for_mutation();
let seq = tb.constructSequence(tid, start, real_end);
seq.into_iter().collect()
}

pub fn twobit_bases(&self, chrom: &str, start: u32, end: u32, fraction: i32) -> Vec<u8> {
let mut tid_opt = None;
for (i, c) in self.cl.chrom.iter().enumerate() {
if c == chrom {
tid_opt = Some(i as u32);
break;
}
}
let tid = match tid_opt {
Some(v) => v,
None => return Vec::new(),
};

let mut real_end = end;
if start == 0 && end == 0 {
real_end = self.idx.size[tid as usize];
}

if real_end > self.idx.size[tid as usize] || start >= real_end {
return Vec::new();
}

let mut tb = self.clone_for_mutation();
let counts = tb.twoBitBasesWorkerInternal(tid, start, real_end, fraction);
if fraction != 0 {
let mut out = Vec::with_capacity(32);
for v in counts {
out.extend_from_slice(&v.to_le_bytes());
}
out
} else {
let mut out = Vec::with_capacity(16);
for v in counts {
out.extend_from_slice(&(v as u32).to_le_bytes());
}
out
}
}

pub fn twobitTell(&mut self) -> u64 {
self.offset
}

pub fn twobitRead(&mut self, _data: &Vec<u8>, sz: usize, nmemb: usize) -> usize {
let total = sz.saturating_mul(nmemb);
if self.offset + total as u64 > self.data.len() as u64 {
0
} else {
self.offset += total as u64;
nmemb
}
}

pub fn twobitSeek(&mut self, offset: u64) {
if offset < self.sz {
self.offset = offset;
let _ = self.fp.seek(SeekFrom::Start(offset));
}
}

pub fn NMask(&mut self, seq: &mut [char], tid: u32, start: u32, end: u32) {
let tidx = tid as usize;
for i in 0..self.idx.n_block_count[tidx] as usize {
let block_start = self.idx.n_block_start[tidx][i];
let mut block_end = block_start + self.idx.n_block_sizes[tidx][i];
if block_end <= start {
continue;
}
if block_start >= end {
break;
}

let (mut pos, width);
if block_start < start {
block_end = block_end.min(end);
pos = 0usize;
width = (block_end - start) as usize;
} else {
block_end = block_end.min(end);
pos = (block_start - start) as usize;
width = (block_end - block_start) as usize;
}
let stop = width + pos;
while pos < stop && pos < seq.len() {
seq[pos] = 'N';
pos += 1;
}
}
}

pub fn softMask(&mut self, seq: &mut [char], tid: u32, start: u32, end: u32) {
let tidx = tid as usize;
if self.idx.mask_block_start.is_empty() {
return;
}
for i in 0..self.idx.mask_block_count[tidx] as usize {
let block_start = self.idx.mask_block_start[tidx][i];
let mut block_end = block_start + self.idx.mask_block_sizes[tidx][i];
if block_end <= start {
continue;
}
if block_start >= end {
break;
}

let (mut pos, width);
if block_start < start {
block_end = block_end.min(end);
pos = 0usize;
width = (block_end - start) as usize;
} else {
block_end = block_end.min(end);
pos = (block_start - start) as usize;
width = (block_end - block_start) as usize;
}
let stop = width + pos;
while pos < stop && pos < seq.len() {
if seq[pos] != 'N' {
seq[pos] = seq[pos].to_ascii_lowercase();
}
pos += 1;
}
}
}

pub fn constructSequence(&mut self, tid: u32, start: u32, end: u32) -> Vec<char> {
let sz = end - start + 1;
let block_start = start / 4;
let offset = (start % 4) as i32;
let block_end = end / 4 + if end % 4 != 0 { 1 } else { 0 };

let bytes_len = block_end.saturating_sub(block_start) as usize;
let mut bytes = vec![0u8; bytes_len];
let data_offset = self.idx.offset[tid as usize] + block_start as u64;

if data_offset as usize + bytes_len > self.data.len() {
return Vec::new();
}

self.twobitSeek(data_offset);
bytes.copy_from_slice(&self.data[data_offset as usize..data_offset as usize + bytes_len]);

let mut seq = vec!['\0'; sz as usize];
bytes2bases(&mut seq, &mut bytes, sz - 1, offset);

if !seq.is_empty() {
seq[(sz - 1) as usize] = '\0';
}

self.NMask(&mut seq[..(sz - 1) as usize], tid, start, end);
self.softMask(&mut seq[..(sz - 1) as usize], tid, start, end);
seq[..(sz - 1) as usize].to_vec()
}

pub fn getMask(&mut self, tid: u32, start: u32, end: u32) -> (u32, u32, u32) {
let tidx = tid as usize;
let count = self.idx.n_block_count[tidx] as usize;
for i in 0..count {
let mask_start = self.idx.n_block_start[tidx][i];
let mask_end = mask_start + self.idx.n_block_sizes[tidx][i];
if mask_end < start {
continue;
}
if mask_start >= end {
break;
}
return (i as u32, mask_start, mask_end);
}
(u32::MAX, u32::MAX, u32::MAX)
}

pub fn twoBitBasesWorker(&mut self, _tid: u32, _start: u32, _end: u32, _fraction: i32) {}

pub fn twoBitIndexRead(&mut self, storeMasked: i32) {
let n = self.hdr.n_chroms as usize;
let mut idx = TwoBitMaskedIdx {
size: vec![0; n],
n_block_count: vec![0; n],
n_block_start: vec![Vec::new(); n],
n_block_sizes: vec![Vec::new(); n],
mask_block_count: vec![0; n],
mask_block_start: if storeMasked != 0 {
vec![Vec::new(); n]
} else {
Vec::new()
},
mask_block_sizes: if storeMasked != 0 {
vec![Vec::new(); n]
} else {
Vec::new()
},
offset: vec![0; n],
};

for i in 0..n {
let base = self.cl.offset[i] as usize;
if base >= self.data.len() {
self.idx = TwoBitMaskedIdx::default();
return;
}

let mut p = base;

let size = read_u32_le(&self.data, &mut p);
let n_block_count = read_u32_le(&self.data, &mut p);
idx.size[i] = size;
idx.n_block_count[i] = n_block_count;

let mut starts = Vec::with_capacity(n_block_count as usize);
for _ in 0..n_block_count {
starts.push(read_u32_le(&self.data, &mut p));
}
idx.n_block_start[i] = starts;

let mut sizes = Vec::with_capacity(n_block_count as usize);
for _ in 0..n_block_count {
sizes.push(read_u32_le(&self.data, &mut p));
}
idx.n_block_sizes[i] = sizes;

let mbc = read_u32_le(&self.data, &mut p);
idx.mask_block_count[i] = mbc;

if storeMasked != 0 {
let mut mstarts = Vec::with_capacity(mbc as usize);
for _ in 0..mbc {
mstarts.push(read_u32_le(&self.data, &mut p));
}
idx.mask_block_start[i] = mstarts;

let mut msizes = Vec::with_capacity(mbc as usize);
for _ in 0..mbc {
msizes.push(read_u32_le(&self.data, &mut p));
}
idx.mask_block_sizes[i] = msizes;
} else {
p = p.saturating_add((mbc as usize) * 8);
}

let _reserved = read_u32_le(&self.data, &mut p);
idx.offset[i] = p as u64;
}

self.idx = idx;
}

pub fn twoBitIndexDestroy(&mut self) {
self.idx = TwoBitMaskedIdx::default();
}

pub fn twobitChromListRead(&mut self) {
let n = self.hdr.n_chroms as usize;
let mut cl = TwoBitCL {
chrom: Vec::with_capacity(n),
offset: Vec::with_capacity(n),
};

let mut p = 16usize;
for _ in 0..n {
if p >= self.data.len() {
self.cl = TwoBitCL::default();
return;
}
let byte = self.data[p] as usize;
p += 1;
if p + byte > self.data.len() {
self.cl = TwoBitCL::default();
return;
}

let name = String::from_utf8_lossy(&self.data[p..p + byte]).to_string();
p += byte;
let off = read_u32_le(&self.data, &mut p);

cl.chrom.push(name);
cl.offset.push(off);
}

self.offset = p as u64;
self.cl = cl;
}

pub fn twobitChromListDestroy(&mut self) {
self.cl = TwoBitCL::default();
}

pub fn twobitHdrRead(&mut self) {
if self.data.len() < 16 {
self.hdr = TwoBitHeader::default();
return;
}

let mut p = 0usize;
let magic = read_u32_le(&self.data, &mut p);
let version = read_u32_le(&self.data, &mut p);
let n_chroms = read_u32_le(&self.data, &mut p);
let _reserved = read_u32_le(&self.data, &mut p);

if magic != 0x1A412743 || version != 0 || n_chroms == 0 {
self.hdr = TwoBitHeader::default();
return;
}

self.hdr = TwoBitHeader {
magic,
version,
n_chroms,
};
self.offset = p as u64;
}

pub fn twobitHdrDestroy(&mut self) {
self.hdr = TwoBitHeader::default();
}

fn clone_for_mutation(&self) -> Self {
TwoBit {
fp: self
.fp
.try_clone()
.expect("failed to clone file handle for 2bit"),
sz: self.sz,
offset: self.offset,
data: self.data.clone(),
hdr: self.hdr.clone(),
cl: self.cl.clone(),
idx: self.idx.clone(),
}
}

fn twoBitBasesWorkerInternal(
&mut self,
tid: u32,
start: u32,
end: u32,
fraction: i32,
) -> [f64; 4] {
let seq = self.constructSequence(tid, start, end);
let mut tmp = [0u32; 4];

for ch in seq {
match ch.to_ascii_uppercase() {
'T' => tmp[0] += 1,
'C' => tmp[1] += 1,
'A' => tmp[2] += 1,
'G' => tmp[3] += 1,
_ => {}
}
}

if fraction != 0 {
let seq_len = (end - start) as f64;
if seq_len == 0.0 {
[0.0, 0.0, 0.0, 0.0]
} else {
[
tmp[2] as f64 / seq_len,
tmp[1] as f64 / seq_len,
tmp[0] as f64 / seq_len,
tmp[3] as f64 / seq_len,
]
}
} else {
[tmp[2] as f64, tmp[1] as f64, tmp[0] as f64, tmp[3] as f64]
}
}
}

pub fn byte2base(byte: u8, offset: i32) -> char {
let rev = 3 - offset;
let mask = 3u8 << (2 * rev);
let foo = ((mask & byte) >> (2 * rev)) as usize;
let bases = ['T', 'C', 'A', 'G'];
bases[foo]
}

pub fn bytes2bases(seq: &mut [char], bytes: &mut [u8], sz: u32, offset: i32) {
let mut pos: u32 = 0;
let mut i: usize = 0;
let bases = ['T', 'C', 'A', 'G'];

if bytes.is_empty() || seq.is_empty() || sz == 0 {
return;
}

let mut off = offset;
let mut foo = bytes[0];

if off != 0 {
while off < 4 && pos < sz {
seq[pos as usize] = byte2base(foo, off);
pos += 1;
off += 1;
}
if pos >= sz {
return;
}
i += 1;
if i >= bytes.len() {
return;
}
foo = bytes[i];
}

let remainder = (sz - pos) % 4;
while pos < sz - remainder {
if i >= bytes.len() {
break;
}
foo = bytes[i];
i += 1;

seq[(pos + 3) as usize] = bases[(foo & 3) as usize];
foo >>= 2;
seq[(pos + 2) as usize] = bases[(foo & 3) as usize];
foo >>= 2;
seq[(pos + 1) as usize] = bases[(foo & 3) as usize];
foo >>= 2;
seq[pos as usize] = bases[(foo & 3) as usize];
pos += 4;
}

if remainder > 0 {
if i >= bytes.len() {
return;
}
foo = bytes[i];
for off2 in 0..remainder {
seq[pos as usize] = byte2base(foo, off2 as i32);
pos += 1;
}
}
}

pub fn getByteMaskFromOffset(offset: i32) {
let _ = offset;
}

fn read_u32_le(data: &[u8], p: &mut usize) -> u32 {
if *p + 4 > data.len() {
*p = data.len();
return 0;
}
let out = u32::from_le_bytes([data[*p], data[*p + 1], data[*p + 2], data[*p + 3]]);
*p += 4;
out
}
