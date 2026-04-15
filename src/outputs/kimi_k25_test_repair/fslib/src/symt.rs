use std::collections::HashMap;
pub struct SymTable {
pub sym: Vec<String>,
symbol_to_id: HashMap<String, usize>,
pub n_max: usize,
pub n_items: usize,
}
impl SymTable {
pub fn new() -> Self {
SymTable {
sym: Vec::new(),
symbol_to_id: HashMap::new(),
n_max: 0,
n_items: 0,
}
}
pub fn add_symbol(&mut self, symbol: &str) -> usize {
if let Some(&id) = self.symbol_to_id.get(symbol) {
id
} else {
let id = self.sym.len();
self.sym.push(symbol.to_string());
self.symbol_to_id.insert(symbol.to_string(), id);
self.n_items = self.sym.len();
self.n_max = self.n_max.max(self.n_items);
id
}
}
pub fn add(&mut self, id: i32, symbol: &str) {
let idx = id as usize;
if idx >= self.sym.len() {
while self.sym.len() <= idx {
self.sym.push(String::new());
}
}
self.sym[idx] = symbol.to_string();
self.symbol_to_id.insert(symbol.to_string(), idx);
self.n_items = self.sym.len();
self.n_max = self.n_max.max(self.n_items);
}
pub fn find(&self, symbol: &str) -> Option<usize> {
self.symbol_to_id.get(symbol).copied()
}
pub fn get(&self, id: i32) -> Option<&str> {
if id >= 0 && (id as usize) < self.sym.len() {
Some(&self.sym[id as usize])
} else {
None
}
}
pub fn getr(&self, symbol: &str) -> Option<i32> {
self.symbol_to_id.get(symbol).map(|&x| x as i32)
}
pub fn print(&self) {
for (i, sym) in self.sym.iter().enumerate() {
println!("{}: {}", i, sym);
}
}
pub fn remove(&mut self) {
if let Some(sym) = self.sym.pop() {
self.symbol_to_id.remove(&sym);
self.n_items = self.sym.len();
}
}
pub fn reverse(&self) -> &HashMap<String, usize> {
&self.symbol_to_id
}
}
