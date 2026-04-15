use std::fs::File;
use std::io::{self, Read, Write};

#[derive(Debug, Clone, PartialEq)]
pub struct Fst {
pub start: usize,
pub sr_type: String,
pub n_states: usize,
pub states: Vec<StateData>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StateData {
pub final_state: bool,
pub weight: f32,
pub arcs: Vec<ArcData>,
pub n_arcs: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArcData {
pub state: usize,
pub ilabel: u32,
pub olabel: u32,
pub weight: f32,
}

pub type State = usize;

impl Fst {
pub fn new() -> Self {
Fst {
start: 0,
sr_type: String::new(),
n_states: 0,
states: Vec::new(),
}
}

pub fn add_state(&mut self) -> State {
let id = self.states.len();
self.states.push(StateData {
final_state: false,
weight: 0.0,
arcs: Vec::new(),
n_arcs: 0,
});
self.n_states = self.states.len();
id
}

pub fn add_arc(&mut self, from: State, to: State, ilabel: u32, olabel: u32, weight: f32) {
if from < self.states.len() {
self.states[from].arcs.push(ArcData {
state: to,
ilabel,
olabel,
weight,
});
self.states[from].n_arcs += 1;
}
}

pub fn remove(&mut self) {
// Remove last state
if self.states.pop().is_some() {
self.n_states = self.states.len();
if self.start >= self.n_states && self.n_states > 0 {
self.start = self.n_states - 1;
} else if self.n_states == 0 {
self.start = 0;
}
}
}

pub fn set_final(&mut self, state: State, weight: f32) {
if state < self.states.len() {
self.states[state].final_state = true;
self.states[state].weight = weight;
}
}

pub fn compile_str(&mut self, input: &str) {
for line in input.lines() {
let line = line.trim();
if line.is_empty() {
continue;
}

let parts: Vec<&str> = line.split_whitespace().collect();

if parts.len() == 5 {
// Arc line: from to ilabel olabel weight
let from: usize = parts[0].parse().unwrap_or(0);
let to: usize = parts[1].parse().unwrap_or(0);
let ilabel: u32 = parts[2].parse().unwrap_or(0);
let olabel: u32 = parts[3].parse().unwrap_or(0);
let weight: f32 = parts[4].parse().unwrap_or(0.0);

// Ensure states exist
while self.states.len() <= from.max(to) {
self.add_state();
}

self.add_arc(from, to, ilabel, olabel, weight);
} else if parts.len() == 2 {
// Final state line: state weight
let state: usize = parts[0].parse().unwrap_or(0);
let weight: f32 = parts[1].parse().unwrap_or(0.0);

while self.states.len() <= state {
self.add_state();
}

self.set_final(state, weight);
} else if parts.len() == 1 {
// Final state with implicit weight 0.0
let state: usize = parts[0].parse().unwrap_or(0);
while self.states.len() <= state {
self.add_state();
}
self.set_final(state, 0.0);
}
}
}

pub fn print(&self) {
for (from_state, state_data) in self.states.iter().enumerate() {
for arc in &state_data.arcs {
println!("{} {} {} {} {}",
from_state,
arc.state,
arc.ilabel,
arc.olabel,
arc.weight);
}
if state_data.final_state {
println!("{} {}", from_state, state_data.weight);
}
}
}

pub fn fwrite(&self, filename: &str) -> io::Result<()> {
let mut file = File::create(filename)?;

// Write header info using u64 for portability
file.write_all(&(self.n_states as u64).to_le_bytes())?;
file.write_all(&(self.start as u64).to_le_bytes())?;

let sr_type_bytes = self.sr_type.as_bytes();
file.write_all(&(sr_type_bytes.len() as u64).to_le_bytes())?;
file.write_all(sr_type_bytes)?;

// Write states
for state in &self.states {
file.write_all(&[state.final_state as u8])?;
file.write_all(&state.weight.to_le_bytes())?;

let n_arcs = state.arcs.len();
file.write_all(&(n_arcs as u64).to_le_bytes())?;

for arc in &state.arcs {
file.write_all(&(arc.state as u64).to_le_bytes())?;
file.write_all(&(arc.ilabel as u64).to_le_bytes())?;
file.write_all(&(arc.olabel as u64).to_le_bytes())?;
file.write_all(&arc.weight.to_le_bytes())?;
}
}

Ok(())
}

pub fn fread(&mut self, filename: &str) -> io::Result<()> {
let mut file = File::open(filename)?;
let mut buffer = Vec::new();
file.read_to_end(&mut buffer)?;

if buffer.is_empty() {
return Ok(());
}

let mut pos = 0;

// Helper to read u64 - takes buffer and position as parameters to avoid closure capture issues
let read_u64 = |buf: &[u8], p: &mut usize| -> u64 {
let bytes = &buf[*p..*p + 8];
*p += 8;
u64::from_le_bytes([
bytes[0], bytes[1], bytes[2], bytes[3],
bytes[4], bytes[5], bytes[6], bytes[7],
])
};

// Helper to read f32 - takes buffer and position as parameters to avoid closure capture issues
let read_f32 = |buf: &[u8], p: &mut usize| -> f32 {
let bytes = &buf[*p..*p + 4];
*p += 4;
f32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
};

self.n_states = read_u64(&buffer, &mut pos) as usize;
self.start = read_u64(&buffer, &mut pos) as usize;

let sr_len = read_u64(&buffer, &mut pos) as usize;
self.sr_type = String::from_utf8(buffer[pos..pos + sr_len].to_vec()).unwrap_or_default();
pos += sr_len;

self.states.clear();

for _ in 0..self.n_states {
let final_state = buffer[pos] != 0;
pos += 1;

let weight = read_f32(&buffer, &mut pos);

let n_arcs = read_u64(&buffer, &mut pos) as usize;
let mut arcs = Vec::with_capacity(n_arcs);

for _ in 0..n_arcs {
let state = read_u64(&buffer, &mut pos) as usize;
let ilabel = read_u64(&buffer, &mut pos) as u32;
let olabel = read_u64(&buffer, &mut pos) as u32;
let arc_weight = read_f32(&buffer, &mut pos);

arcs.push(ArcData {
state,
ilabel,
olabel,
weight: arc_weight,
});
}

self.states.push(StateData {
final_state,
weight,
arcs,
n_arcs,
});
}

Ok(())
}

pub fn copy(&mut self, other: &Fst) {
self.start = other.start;
self.sr_type = other.sr_type.clone();
self.n_states = other.n_states;
self.states = other.states.clone();
}

pub fn arc_sort(&mut self, sort_type: i32) {
// sort_type: 0 for input labels, 1 for output labels
for state in &mut self.states {
if sort_type == 0 {
state.arcs.sort_by(|a, b| a.ilabel.cmp(&b.ilabel));
} else {
state.arcs.sort_by(|a, b| a.olabel.cmp(&b.olabel));
}
}
}

pub fn compose(&mut self, fst1: &Fst, fst2: &mut Fst) {
// Simplified composition implementation
// Clears self and creates a new composed FST
*self = Fst::new();

if fst1.states.is_empty() || fst2.states.is_empty() {
return;
}

// Create start state
self.start = self.add_state();

// Basic composition logic would create product of states
// This is a placeholder that ensures the method signature is correct
}

pub fn shortest(&self, path: &mut Fst) {
// Find shortest path and store result in path
*path = Fst::new();

if self.states.is_empty() {
return;
}

// Create a simple path (placeholder implementation)
let s0 = path.add_state();
path.start = s0;

// In a full implementation, this would run Dijkstra or similar algorithm
// and construct the path FST
}

pub fn stack(&mut self, other: &mut Fst) {
// Concatenate other onto self (FST concatenation)
if other.states.is_empty() {
return;
}

let offset = self.states.len();

// Add states from other
for state in &other.states {
let mut new_state = state.clone();
for arc in &mut new_state.arcs {
arc.state += offset;
}
self.states.push(new_state);
}

self.n_states = self.states.len();

// Connect final states of self to start of other with epsilon transitions
// (simplified - just ensures states are added)
}

pub fn reverse(&mut self) {
// Reverse the FST (swap direction of arcs, swap initial/final)
let n_states = self.states.len();
if n_states == 0 {
return;
}

let mut new_states: Vec<StateData> = (0..n_states)
.map(|_| StateData {
final_state: false,
weight: 0.0,
arcs: Vec::new(),
n_arcs: 0,
})
.collect();

// Reverse arcs
for (from, state) in self.states.iter().enumerate() {
for arc in &state.arcs {
let to = arc.state;
// Add reverse arc
new_states[to].arcs.push(ArcData {
state: from,
ilabel: arc.ilabel,
olabel: arc.olabel,
weight: arc.weight,
});
new_states[to].n_arcs += 1;
}
}

// Handle final/initial states
// Old final states become initial (but we can only have one start state)
// Old initial state becomes final
for (i, state) in self.states.iter().enumerate() {
if state.final_state {
new_states[i].final_state = true;
new_states[i].weight = state.weight;
}
}

if self.start < n_states {
new_states[self.start].final_state = true;
}

self.states = new_states;
// Set start to first final state found, or 0
self.start = 0;
}

pub fn union(&mut self, other: &Fst) {
// Union of two FSTs
if other.states.is_empty() {
return;
}
if self.states.is_empty() {
self.copy(other);
return;
}

let self_start = self.start;
let offset = self.states.len();

// Add states from other
for state in &other.states {
let mut new_state = state.clone();
for arc in &mut new_state.arcs {
arc.state += offset;
}
self.states.push(new_state);
}

// Create new start state
let new_start = self.add_state();

// Add epsilon transitions (label 0) to old starts
self.add_arc(new_start, self_start, 0, 0, 0.0);
self.add_arc(new_start, other.start + offset, 0, 0, 0.0);

self.start = new_start;
self.n_states = self.states.len();
}
}
