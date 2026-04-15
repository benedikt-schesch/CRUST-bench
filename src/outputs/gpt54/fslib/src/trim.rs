use crate::queue::Queue;
use crate::fst::{ArcData, Fst};
use crate::bitset::BitSet;

pub fn fst_close(_fst: &mut Fst, _finals: &mut Queue<(ArcData, ArcData)>) {
}

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
let mut idx = vec![0u32; old_n];
let mut shift = 0u32;
let mut new_states = Vec::new();

for s in 0..old_n {
if mask.get(s) {
shift += 1;
} else {
idx[s] = shift;
new_states.push(fst.states[s].clone());
}
}

fst.states = new_states;
fst.n_states = fst.states.len() as u32;

for state in &mut fst.states {
let mut new_arcs = Vec::new();
for arc in &state.arcs {
let new_state = arc.state.saturating_sub(idx[arc.state as usize]);
if new_state < fst.n_states {
let mut na = arc.clone();
na.state = new_state;
new_arcs.push(na);
}
}
state.arcs = new_arcs;
state.n_arcs = state.arcs.len() as u32;
}
}

pub fn fst_get_finals(_fst: &mut Fst, _finals: &mut Queue<(ArcData, ArcData)>) {
}

pub fn fst_trim(_fst: &mut Fst) {
}
