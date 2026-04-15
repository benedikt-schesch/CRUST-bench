use crate::fst::{Fst, ArcData};
use crate::sr::Sr;
use crate::heap::Heap;
use std::collections::HashMap;
use std::cmp::Ordering;
pub struct ShortestPath {
sr: Sr,
weights: Vec<f32>,
backtrack: Vec<Option<ArcData>>,
}
impl ShortestPath {
pub fn new(fst: &Fst) -> Self {
Self {
sr: crate::sr::sr_get(fst.sr_type),
weights: vec![f32::MAX; fst.n_states as usize],
backtrack: vec![None; fst.n_states as usize],
}
}
fn backtrace(&self, _path: &mut Fst, _final_state: u32) {
}
pub fn find_shortest_path(_fst: &Fst, _path: &mut Fst) {
}
fn states_cmp(&self, a: &u32, b: &u32) -> Ordering {
self.weights[*b as usize]
.partial_cmp(&self.weights[*a as usize])
.unwrap_or(Ordering::Equal)
}
}
fn states_hash(a: &u32) -> u64 {
*a as u64
}
fn states_key_eq(a: &u32, b: &u32) -> bool {
a == b
}
