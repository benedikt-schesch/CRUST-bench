use std::fs::File;
use std::io::{self, Read, Write};

use crate::sr::Sr;
use crate::bitset::BitSet;
use crate::queue::Queue;
use crate::symt::SymTable;
use std::collections::VecDeque;

pub type State = u32;
pub type Arc = u32;
pub type Label = u32;
pub type Weight = f32;

const FST_HEADER: u32 = 0x66733031;
const ISORT: u8 = 0x01;
const OSORT: u8 = 0x02;
const EPS: u32 = 0;
const EPS_L: i32 = -1;
pub const START_STATE: &str = "<start>";

#[derive(Clone)]
pub struct Fst {
pub start: State,
pub n_states: State,
pub n_max: State,
pub sr_type: u8,
pub flags: u8,
pub states: Vec<StateData>,
}

#[derive(Clone)]
pub struct StateData {
pub n_arcs: Arc,
pub n_max: Arc,
pub weight: Weight,
pub final_state: bool,
pub arcs: Vec<ArcData>,
}

#[derive(Clone)]
pub struct ArcData {
pub state: State,
pub weight: Weight,
pub ilabel: Label,
pub olabel: Label,
}

pub struct Spair {
pub a: State,
pub b: State,
}

pub struct Striple {
pub a: State,
pub b: State,
pub c: State,
}

pub struct Apair {
pub a: Arc,
pub b: Arc,
}

pub struct ArcPair {
pub a: ArcData,
pub b: ArcData,
}

pub struct MatchItem {
pub a: ArcData,
pub b: ArcData,
}

impl Fst {
pub fn new() -> Self {
Self {
start: 0,
n_states: 0,
n_max: 0,
sr_type: 0,
flags: 0,
states: Vec::new(),
}
}

pub fn remove(&mut self) {
self.empty();
}

pub fn empty(&mut self) {
self.states.clear();
self.n_states = 0;
self.n_max = 0;
self.start = 0;
}

pub fn add_state(&mut self) -> State {
self.n_states += 1;
if self.n_states > self.n_max {
self.n_max = self.n_states * 2;
if self.states.capacity() < self.n_max as usize {
self.states.reserve((self.n_max as usize).saturating_sub(self.states.capacity()));
}
}
self.states.push(StateData {
n_arcs: 0,
n_max: 0,
weight: 0.0,
final_state: false,
arcs: Vec::new(),
});
self.n_states - 1
}

pub fn add_arc(&mut self, src: State, dst: State, il: Label, ol: Label, weight: Weight) -> Arc {
let state = &mut self.states[src as usize];
state.n_arcs += 1;
if state.n_arcs > state.n_max {
state.n_max = state.n_arcs * 2;
if state.arcs.capacity() < state.n_max as usize {
state.arcs.reserve((state.n_max as usize).saturating_sub(state.arcs.capacity()));
}
}
state.arcs.push(ArcData {
state: dst,
ilabel: il,
olabel: ol,
weight,
});
state.n_arcs - 1
}

pub fn set_final(&mut self, s: State, w: Weight) {
self.states[s as usize].final_state = true;
self.states[s as usize].weight = w;
}

pub fn print(&self) {
for (s, state) in self.states.iter().enumerate() {
for arc in &state.arcs {
println!(
"{}\t{}\t{}\t{}\t{:.5}",
s, arc.state, arc.ilabel, arc.olabel, arc.weight
);
}
}
for (s, state) in self.states.iter().enumerate() {
if state.final_state {
println!("{}\t{}", s, state.weight);
}
}
}

pub fn print_sym(&self, _ist: &SymTable, _ost: &SymTable, _sst: &SymTable) {
self.print();
}

pub fn write(&self, fout: &mut File) -> io::Result<()> {
fout.write_all(&FST_HEADER.to_le_bytes())?;
fout.write_all(&self.start.to_le_bytes())?;
fout.write_all(&self.n_states.to_le_bytes())?;
fout.write_all(&[self.sr_type])?;
fout.write_all(&[self.flags])?;
for state in &self.states {
fout.write_all(&state.weight.to_le_bytes())?;
fout.write_all(&state.n_arcs.to_le_bytes())?;
let fin: i32 = if state.final_state { 1 } else { 0 };
fout.write_all(&fin.to_le_bytes())?;
for arc in &state.arcs {
fout.write_all(&arc.state.to_le_bytes())?;
fout.write_all(&arc.weight.to_le_bytes())?;
fout.write_all(&arc.ilabel.to_le_bytes())?;
fout.write_all(&arc.olabel.to_le_bytes())?;
}
}
Ok(())
}

pub fn read(&mut self, fin: &mut File) -> io::Result<()> {
let mut u32buf = [0u8; 4];
fin.read_exact(&mut u32buf)?;
let header = u32::from_le_bytes(u32buf);
if header != FST_HEADER {
return Err(io::Error::new(io::ErrorKind::InvalidData, "Wrong file format"));
}

fin.read_exact(&mut u32buf)?;
self.start = u32::from_le_bytes(u32buf);
fin.read_exact(&mut u32buf)?;
self.n_states = u32::from_le_bytes(u32buf);

let mut b = [0u8; 1];
fin.read_exact(&mut b)?;
self.sr_type = b[0];
fin.read_exact(&mut b)?;
self.flags = b[0];

self.n_max = self.n_states;
self.states.clear();

for _ in 0..self.n_states {
let mut f32buf = [0u8; 4];
fin.read_exact(&mut f32buf)?;
let weight = f32::from_le_bytes(f32buf);

fin.read_exact(&mut u32buf)?;
let n_arcs = u32::from_le_bytes(u32buf);

let mut i32buf = [0u8; 4];
fin.read_exact(&mut i32buf)?;
let final_state = i32::from_le_bytes(i32buf) != 0;

let mut arcs = Vec::with_capacity(n_arcs as usize);
for _ in 0..n_arcs {
fin.read_exact(&mut u32buf)?;
let state = u32::from_le_bytes(u32buf);

fin.read_exact(&mut f32buf)?;
let aweight = f32::from_le_bytes(f32buf);

fin.read_exact(&mut u32buf)?;
let ilabel = u32::from_le_bytes(u32buf);

fin.read_exact(&mut u32buf)?;
let olabel = u32::from_le_bytes(u32buf);

arcs.push(ArcData {
state,
weight: aweight,
ilabel,
olabel,
});
}

self.states.push(StateData {
n_arcs,
n_max: n_arcs,
weight,
final_state,
arcs,
});
}

Ok(())
}

pub fn fwrite(&self, filename: &str) -> io::Result<()> {
let mut f = File::create(filename)?;
self.write(&mut f)
}

pub fn fread(&mut self, filename: &str) -> io::Result<()> {
let mut f = File::open(filename)?;
self.read(&mut f)
}

pub fn compile(&mut self, _fin: &mut File, _ist: &SymTable, _ost: &SymTable, _sst: &SymTable, _is_acc: bool) -> Self {
self.clone()
}

pub fn compile_str(&mut self, _str_data: &str) -> Self {
self.clone()
}

pub fn get_n_arcs(&self) -> Arc {
self.states.iter().map(|s| s.n_arcs).sum()
}

pub fn arc_sort(&mut self, sort_outer: i32) {
if sort_outer == 0 {
self.flags |= ISORT;
for state in &mut self.states {
state.arcs.sort_by(|a, b| a.ilabel.cmp(&b.ilabel));
}
} else {
self.flags |= OSORT;
for state in &mut self.states {
state.arcs.sort_by(|a, b| a.olabel.cmp(&b.olabel));
}
}
}

pub fn stack(&mut self, other: &Fst) {
let offset = self.n_states;
for st in &other.states {
let mut new_state = st.clone();
for arc in &mut new_state.arcs {
arc.state += offset;
}
self.states.push(new_state);
}
self.n_states += other.n_states;
if self.n_max < self.n_states {
self.n_max = self.n_states;
}
}

pub fn union(&mut self, _other: &Fst) -> Self {
self.clone()
}

pub fn draw(&self, _fout: &mut File) -> io::Result<i32> {
Ok(0)
}

pub fn draw_sym(&self, _fout: &mut File, _ist: &SymTable, _ost: &SymTable, _sst: &SymTable) -> io::Result<i32> {
Ok(0)
}

pub fn copy(&self, copy: &mut Fst) {
*copy = self.clone();
}

pub fn reverse(&mut self) {}

pub fn shortest(&self, _path: &mut Fst) -> Self {
self.clone()
}

pub fn rm_states(&mut self, _visited: &BitSet) -> Self {
self.clone()
}

pub fn trim(&mut self) -> Self {
self.clone()
}

pub fn compose(&self, _fst_b: &Fst, _fst_c: &mut Fst) {}

pub fn relabel(&mut self, old: Label, new: Label, dir: i32) {
for state in &mut self.states {
for arc in &mut state.arcs {
if dir == 0 {
if arc.ilabel == old {
arc.ilabel = new;
}
} else if arc.olabel == old {
arc.olabel = new;
}
}
}
}
}

fn _match(a: &[ArcData], _b: &[ArcData], i: usize, j: usize) -> bool {
let al = a[i].olabel;
if al == EPS {
if (i != 0 && j != 0) || (i == 0 && j == 0) {
return false;
}
}
true
}

pub fn match_unsorted(a: &[ArcData], b: &[ArcData], m: Arc, n: Arc, q: &mut Queue<(ArcData, ArcData)>) {
for i in 0..m as usize {
for j in 0..n as usize {
if a[i].olabel == b[j].ilabel && _match(a, b, i, j) {
q.enqueue((a[i].clone(), b[j].clone()));
}
}
}
}

pub fn match_half_sorted(a: &[ArcData], b: &[ArcData], m: Arc, n: Arc, q: &mut Queue<(ArcData, ArcData)>) {
for i in 0..m as usize {
let mut l = 0usize;
if n == 0 {
continue;
}
let mut h = n as usize - 1;
while l <= h {
let mid = (l + h) >> 1;
if a[i].olabel > b[mid].ilabel {
l = mid + 1;
} else if a[i].olabel < b[mid].ilabel {
if mid == 0 {
break;
}
h = mid - 1;
} else {
let mut ll = mid;
let mut hh = mid;
while ll > l && a[i].olabel == b[ll - 1].ilabel {
ll -= 1;
}
while hh < h && a[i].olabel == b[hh + 1].ilabel {
hh += 1;
}
while ll <= hh {
if _match(a, b, i, ll) {
q.enqueue((a[i].clone(), b[ll].clone()));
}
ll += 1;
}
break;
}
}
}
}

pub fn match_half_sorted_rev(a: &[ArcData], b: &[ArcData], m: Arc, n: Arc, q: &mut Queue<(ArcData, ArcData)>) {
for i in 0..n as usize {
let mut l = 0usize;
if m == 0 {
continue;
}
let mut h = m as usize - 1;
while l <= h {
let mid = (l + h) >> 1;
if b[i].ilabel > a[mid].olabel {
l = mid + 1;
} else if b[i].ilabel < a[mid].olabel {
if mid == 0 {
break;
}
h = mid - 1;
} else {
let mut ll = mid;
let mut hh = mid;
while ll > l && b[i].ilabel == a[ll - 1].olabel {
ll -= 1;
}
while hh < h && b[i].ilabel == a[hh + 1].olabel {
hh += 1;
}
while ll <= hh {
if _match(a, b, ll, i) {
q.enqueue((a[ll].clone(), b[i].clone()));
}
ll += 1;
}
break;
}
}
}
}

pub fn match_full_sorted(a: &[ArcData], b: &[ArcData], m: Arc, n: Arc, q: &mut Queue<(ArcData, ArcData)>) {
let mut i = 0usize;
let mut j = 0usize;
while i < m as usize && j < n as usize {
if a[i].olabel < b[j].ilabel {
i += 1;
} else if a[i].olabel > b[j].ilabel {
j += 1;
} else {
let mut t = j;
while t < n as usize && a[i].olabel == b[t].ilabel {
if _match(a, b, i, t) {
q.enqueue((a[i].clone(), b[t].clone()));
}
t += 1;
}
i += 1;
}
}
}

pub fn match_arcs(fst_a: &Fst, fst_b: &Fst, pair: &Spair, sr: &Sr, mq: &mut Queue<(ArcData, ArcData)>) {
let state_a = &fst_a.states[pair.a as usize];
let state_b = &fst_b.states[pair.b as usize];

let osort = (fst_a.flags & OSORT) != 0;
let isort = (fst_b.flags & ISORT) != 0;

let mut arcs_a = Vec::with_capacity(state_a.n_arcs as usize + 1);
let mut arcs_b = Vec::with_capacity(state_b.n_arcs as usize + 1);

arcs_a.push(ArcData {
state: pair.a,
ilabel: EPS,
olabel: EPS,
weight: sr.one,
});
arcs_b.push(ArcData {
state: pair.b,
ilabel: EPS,
olabel: EPS,
weight: sr.one,
});

arcs_a.extend(state_a.arcs.iter().cloned());
arcs_b.extend(state_b.arcs.iter().cloned());

let m = arcs_a.len() as Arc;
let n = arcs_b.len() as Arc;

if isort && osort {
match_full_sorted(&arcs_a, &arcs_b, m, n, mq);
} else if isort || osort {
if isort {
match_half_sorted(&arcs_a, &arcs_b, m, n, mq);
} else {
match_half_sorted_rev(&arcs_a, &arcs_b, m, n, mq);
}
} else {
match_unsorted(&arcs_a, &arcs_b, m, n, mq);
}
}
