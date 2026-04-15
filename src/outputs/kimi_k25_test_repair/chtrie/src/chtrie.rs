use std::collections::VecDeque;
pub const SZ_MAX: usize = usize::MAX;
pub struct ChtrieEdge {
pub next: Option<Box<ChtrieEdge>>,
pub from: i32,
pub sym: i32,
pub to: i32,
}
pub struct Chtrie {
pub etab: Vec<Option<Box<ChtrieEdge>>>,
pub idxpool: VecDeque<i32>,
pub idxptr: i32,
pub idxmax: i32,
pub maxn: i32,
pub alphsz: i32,
pub ecap: i32,
}
impl Chtrie {
pub fn new(n: usize, m: usize) -> Option<Self> {
let mut n = n;
let mut m = m;
if n < 1 {
n = 1;
}
if m < 1 {
m = 1;
}
if n > i32::MAX as usize || m > i32::MAX as usize {
return None;
}
let n_minus_1 = n - 1;
if i32::MAX - (n_minus_1 as i32) < (n_minus_1 / 3) as i32 {
return None;
}
let maxn = n as i32;
let alphsz = m as i32;
let ecap = (n_minus_1 + n_minus_1 / 3) as i32;
let mut etab = Vec::with_capacity(ecap as usize);
etab.resize_with(ecap as usize, || None);
let idxpool = VecDeque::with_capacity(n);
Some(Chtrie {
etab,
idxpool,
idxptr: 0,
idxmax: 1,
maxn,
alphsz,
ecap,
})
}
pub fn walk(&mut self, from: i32, sym: i32, creat: i32) -> i32 {
let h = ((from as u64 * self.alphsz as u64 + sym as u64) % self.ecap as u64) as usize;
let mut current = &mut self.etab[h];
while let Some(ref mut edge) = *current {
if edge.from == from && edge.sym == sym {
return edge.to;
}
current = &mut edge.next;
}
if creat != 0 {
if self.idxptr == 0 && self.idxmax >= self.maxn {
return -1;
}
let to = if self.idxptr != 0 {
self.idxptr -= 1;
self.idxpool.pop_back().unwrap()
} else {
let idx = self.idxmax;
self.idxmax += 1;
idx
};
let new_edge = Box::new(ChtrieEdge {
next: self.etab[h].take(),
from,
sym,
to,
});
self.etab[h] = Some(new_edge);
return to;
}
-1
}
pub fn del(&mut self, from: i32, sym: i32) {
let h = ((from as u64 * self.alphsz as u64 + sym as u64) % self.ecap as u64) as usize;
let mut current = &mut self.etab[h];
while let Some(mut edge) = current.take() {
if edge.from == from && edge.sym == sym {
let to = edge.to;
*current = edge.next;
self.idxpool.push_back(to);
self.idxptr += 1;
return;
}
*current = Some(edge);
current = &mut current.as_mut().unwrap().next;
}
}
pub fn free(&mut self) {
self.etab.clear();
self.idxpool.clear();
self.idxptr = 0;
self.idxmax = 0;
self.maxn = 0;
self.alphsz = 0;
self.ecap = 0;
}
}
pub fn chtrie_walk(trie: &mut Chtrie, from: i32, sym: i32, creat: i32) -> i32 {
trie.walk(from, sym, creat)
}
pub fn chtrie_del(trie: &mut Chtrie, from: i32, sym: i32) {
trie.del(from, sym)
}
