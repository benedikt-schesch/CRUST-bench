use crate::fst::Fst;

pub struct FstIter<'a> {
fst: &'a Fst,
current_state: usize,
pub state: usize,
visited_states: Vec<bool>,
}

impl<'a> FstIter<'a> {
pub fn new(fst: &'a Fst) -> Self {
FstIter {
fst,
current_state: 0,
state: 0,
visited_states: vec![false; fst.states.len()],
}
}

pub fn visited(&self, state: usize) -> bool {
self.visited_states.get(state).copied().unwrap_or(false)
}
}

impl<'a> Iterator for FstIter<'a> {
type Item = usize;

fn next(&mut self) -> Option<Self::Item> {
if self.current_state < self.fst.states.len() {
let state = self.current_state;
self.current_state += 1;
self.state = state;
if state < self.visited_states.len() {
self.visited_states[state] = true;
}
Some(state)
} else {
None
}
}
}
