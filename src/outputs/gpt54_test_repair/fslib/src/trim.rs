use crate::bitset::BitSet;
use crate::fst::{ArcData, Fst};
use crate::queue::Queue;

pub fn fst_close(_fst: &mut Fst, _finals: &mut Queue<(ArcData, ArcData)>) {}

pub fn fst_reverse(fst: &mut Fst) {
let orig = fst.clone();
let start_s = fst.start;

for (s, state) in fst.states.iter_mut().enumerate() {
state.n_arcs = 0;
state.arcs.clear();
if state.final_state {
fst.start = s as u32;
state.final_state = false;
}
}

if (start_s as usize) < fst.states.len() {
fst.states[start_s as usize].final_state = true;
fst.states[start_s as usize].weight = 0.0;
}

for (s, state) in orig.states.iter().enumerate() {
for arc in &state.arcs {
fst.add_arc(arc.state, s as u32, arc.ilabel, arc.olabel, arc.weight);
}
}
}

pub fn fst_rm_states(fst: &mut Fst, mask: &BitSet) {
let old_n = fst.n_states as usize;
let mut new_index: Vec<Option<u32>> = vec![None; old_n];
let mut new_states = Vec::new();

for s in 0..old_n {
if !mask.get(s) {
let idx = new_states.len() as u32;
new_index[s] = Some(idx);
new_states.push(fst.states[s].clone());
}
}

for state in &mut new_states {
let mut new_arcs = Vec::new();
for arc in &state.arcs {
let dst = arc.state as usize;
if dst < old_n {
if let Some(mapped) = new_index[dst] {
let mut na = arc.clone();
na.state = mapped;
new_arcs.push(na);
}
}
}
state.arcs = new_arcs;
state.n_arcs = state.arcs.len() as u32;
state.n_max = state.n_arcs;
}

let old_start = fst.start as usize;
fst.states = new_states;
fst.n_states = fst.states.len() as u32;
fst.n_max = fst.n_states;

if old_start < old_n {
if let Some(mapped_start) = new_index[old_start] {
fst.start = mapped_start;
} else {
fst.start = 0;
}
} else {
fst.start = 0;
}
}

pub fn fst_get_finals(_fst: &mut Fst, _finals: &mut Queue<(ArcData, ArcData)>) {}

pub fn fst_trim(fst: &mut Fst) {
let n = fst.n_states as usize;
if n == 0 {
return;
}

let mut accessible = vec![false; n];
let mut q = std::collections::VecDeque::new();
let start = fst.start as usize;
if start < n {
accessible[start] = true;
q.push_back(start);
}

while let Some(s) = q.pop_front() {
for arc in &fst.states[s].arcs {
let t = arc.state as usize;
if t < n && !accessible[t] {
accessible[t] = true;
q.push_back(t);
}
}
}

let mut rev: Vec<Vec<usize>> = vec![Vec::new(); n];
for (s, state) in fst.states.iter().enumerate() {
for arc in &state.arcs {
let t = arc.state as usize;
if t < n {
rev[t].push(s);
}
}
}

let mut coaccessible = vec![false; n];
let mut rq = std::collections::VecDeque::new();
for (s, state) in fst.states.iter().enumerate() {
if state.final_state {
coaccessible[s] = true;
rq.push_back(s);
}
}

while let Some(s) = rq.pop_front() {
for &p in &rev[s] {
if !coaccessible[p] {
coaccessible[p] = true;
rq.push_back(p);
}
}
}

let mut mask = BitSet::new(n);
for s in 0..n {
if !(accessible[s] && coaccessible[s]) {
mask.set(s);
}
}

fst_rm_states(fst, &mask);
}
