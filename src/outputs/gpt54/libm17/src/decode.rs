use std::sync::{Mutex, OnceLock};

use crate::math::q_abs_diff;

// Synchronization symbols for LSF (Link Setup Frame)
pub const LSF_SYNC_SYMBOLS: [i8; 8] = [3, 3, 3, 3, -3, -3, 3, -3];
// Synchronization symbols for Stream
pub const STR_SYNC_SYMBOLS: [i8; 8] = [-3, -3, -3, -3, 3, 3, -3, 3];
// Synchronization symbols for Packet
pub const PKT_SYNC_SYMBOLS: [i8; 8] = [3, -3, 3, 3, -3, -3, -3, -3];
// Symbol levels for modulation
pub const SYMBOL_LEVELS: [f32; 4] = [-3.0, -1.0, 1.0, 3.0];
pub const NUM_STATES: usize = 1 << (5 - 1);

#[derive(Clone)]
struct ViterbiState {
prev_metrics: [u32; NUM_STATES],
curr_metrics: [u32; NUM_STATES],
prev_metrics_data: [u32; NUM_STATES],
curr_metrics_data: [u32; NUM_STATES],
history: [u16; 244],
}

impl Default for ViterbiState {
fn default() -> Self {
Self {
prev_metrics: [0; NUM_STATES],
curr_metrics: [0; NUM_STATES],
prev_metrics_data: [0; NUM_STATES],
curr_metrics_data: [0; NUM_STATES],
history: [0; 244],
}
}
}

fn state() -> &'static Mutex<ViterbiState> {
static STATE: OnceLock<Mutex<ViterbiState>> = OnceLock::new();
STATE.get_or_init(|| Mutex::new(ViterbiState::default()))
}

pub fn viterbi_decode(out: &mut [u8], input: &[u16], len: u16) -> u32 {
if len as usize > 244 * 2 {
eprintln!("Input size exceeds max history");
}
viterbi_reset();
let mut pos: usize = 0;
let mut i = 0usize;
while i + 1 < len as usize {
let s0 = input[i];
let s1 = input[i + 1];
viterbi_decode_bit(s0, s1, pos);
pos += 1;
i += 2;
}
viterbi_chainback(out, pos, len / 2)
}

pub fn viterbi_decode_punctured(
out: &mut [u8],
input: &[u16],
punct: &[u8],
in_len: u16,
p_len: u16,
) -> u32 {
if in_len as usize > 244 * 2 {
eprintln!("Input size exceeds max history");
}

let mut umsg = [0u16; 244 * 2];
let mut p: usize = 0;
let mut u: usize = 0;
let mut i: usize = 0;

while i < in_len as usize {
if punct[p] != 0 {
umsg[u] = input[i];
i += 1;
} else {
umsg[u] = 0x7FFF;
}
u += 1;
p += 1;
p %= p_len as usize;
}

viterbi_decode(out, &umsg[..u], u as u16) - ((u - in_len as usize) as u32) * 0x7FFF
}

pub fn viterbi_decode_bit(s0: u16, s1: u16, pos: usize) {
const COST_TABLE_0: [u16; 8] = [0, 0, 0, 0, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF];
const COST_TABLE_1: [u16; 8] = [0, 0xFFFF, 0xFFFF, 0, 0, 0xFFFF, 0xFFFF, 0];

let mut st = state().lock().expect("mutex poisoned");

for i in 0..(NUM_STATES / 2) {
let metric = q_abs_diff(COST_TABLE_0[i], s0) as u32 + q_abs_diff(COST_TABLE_1[i], s1) as u32;
let m0 = st.prev_metrics[i] + metric;
let m1 = st.prev_metrics[i + NUM_STATES / 2] + (0x1FFFEu32 - metric);
let m2 = st.prev_metrics[i] + (0x1FFFEu32 - metric);
let m3 = st.prev_metrics[i + NUM_STATES / 2] + metric;
let i0 = 2 * i;
let i1 = i0 + 1;

if m0 >= m1 {
st.history[pos] |= 1 << i0;
st.curr_metrics[i0] = m1;
} else {
st.history[pos] &= !(1 << i0);
st.curr_metrics[i0] = m0;
}

if m2 >= m3 {
st.history[pos] |= 1 << i1;
st.curr_metrics[i1] = m3;
} else {
st.history[pos] &= !(1 << i1);
st.curr_metrics[i1] = m2;
}
}

let tmp = st.curr_metrics;
st.curr_metrics = st.prev_metrics;
st.prev_metrics = tmp;
}

fn viterbi_chainback(out: &mut [u8], mut pos: usize, len: u16) -> u32 {
let st = state().lock().expect("mutex poisoned");

let mut state_val: u8 = 0;
let mut bit_pos = len as usize + 4;
let out_len = ((len as usize).saturating_sub(1)) / 8 + 1;
for b in out.iter_mut().take(out_len) {
*b = 0;
}

while pos > 0 {
bit_pos -= 1;
pos -= 1;
let bit = st.history[pos] & (1 << (state_val >> 4));
state_val >>= 1;
if bit != 0 {
state_val |= 0x80;
out[bit_pos / 8] |= 1 << (7 - (bit_pos % 8));
}
}

let mut cost = st.prev_metrics[0];
for &m in st.prev_metrics.iter() {
if m < cost {
cost = m;
}
}
cost
}

fn viterbi_reset() {
let mut st = state().lock().expect("mutex poisoned");
st.history = [0; 244];
st.curr_metrics = [0; NUM_STATES];
st.prev_metrics = [0; NUM_STATES];
st.curr_metrics_data = [0; NUM_STATES];
st.prev_metrics_data = [0; NUM_STATES];
}
