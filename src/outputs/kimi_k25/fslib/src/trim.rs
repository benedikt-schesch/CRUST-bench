use crate::fst::Fst;
use crate::bitset::BitSet;

pub fn fst_trim(fst: &mut Fst) {
let _ = fst.start;
let _ = BitSet::new(0);
}

pub fn fst_rm_states(fst: &mut Fst, bs: &BitSet) {
// Remove states where bs.get(state) is true
let mut new_states = Vec::new();
let mut state_map = std::collections::HashMap::new();

for (i, state) in fst.states.iter().enumerate() {
if !bs.get(i) {
let new_id = new_states.len();
state_map.insert(i, new_id);
new_states.push(state.clone());
}
}

// Update arcs to point to new state ids
for state in &mut new_states {
for arc in &mut state.arcs {
if let Some(&new_state) = state_map.get(&arc.state) {
arc.state = new_state;
}
}
}

// Update start state
if let Some(&new_start) = state_map.get(&fst.start) {
fst.start = new_start;
}

fst.states = new_states;
fst.n_states = fst.states.len();
}
