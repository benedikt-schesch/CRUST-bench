pub mod dht {
pub struct HASHTABLE<T> {
pub hash_table: Vec<Option<T>>,
pub lower_bound: i32,
pub higher_bound: i32,
}
impl<T> HASHTABLE<T> {
pub fn dht_init() -> HASHTABLE<T> {
HASHTABLE {
hash_table: Vec::new(),
lower_bound: 0,
higher_bound: 0,
}
}
pub fn dht_is_initialised(&self) -> bool {
self.higher_bound > self.lower_bound
}
pub fn dht_init_table(&mut self, new_lower_bound: u32, new_upper_bound: u32, migrate: bool) -> bool {
assert!(new_upper_bound > new_lower_bound, "new_upper_bound must be greater than new_lower_bound");
let new_hash_table_size = (new_upper_bound - new_lower_bound) as usize;
println!("dht_init_table called (new_lower_bound={}, new_upper_bound={}, new_hash_table_size={})",
new_lower_bound, new_upper_bound, new_hash_table_size);
let mut new_hash_table: Vec<Option<T>> = Vec::with_capacity(new_hash_table_size);
for _ in 0..new_hash_table_size {
new_hash_table.push(None);
}
if migrate {
println!("Migrating data during reinitialisation");
}
if self.dht_is_initialised() {
let old_lower_bound = self.lower_bound as u32;
let old_table = std::mem::replace(&mut self.hash_table, Vec::new());
for (i, item) in old_table.into_iter().enumerate() {
if let Some(data) = item {
let absolute_position = i as u32 + old_lower_bound;
if migrate && absolute_position >= new_lower_bound && absolute_position < new_upper_bound {
let new_index = (absolute_position - new_lower_bound) as usize;
new_hash_table[new_index] = Some(data);
} else {
println!("forgotten... todo remap send to peer");
}
}
}
}
self.hash_table = new_hash_table;
self.lower_bound = new_lower_bound as i32;
self.higher_bound = new_upper_bound as i32;
println!("dht_init_table complete");
true
}
pub fn dht_get_lower_bound(&self) -> u32 {
assert!(self.dht_is_initialised(), "DHT not initialized");
self.lower_bound as u32
}
pub fn dht_get_upper_bound(&self) -> u32 {
assert!(self.dht_is_initialised(), "DHT not initialized");
self.higher_bound as u32
}
pub fn dht_get_size(&self) -> u32 {
assert!(self.dht_is_initialised(), "DHT not initialized");
(self.higher_bound - self.lower_bound) as u32
}
pub fn dht_read(&self, location: u32) -> &Option<T> {
assert!(self.dht_is_initialised(), "DHT not initialized");
assert!(location >= self.lower_bound as u32, "location below lower bound");
assert!(location < self.higher_bound as u32, "location above upper bound");
let absolute_position = (location - self.lower_bound as u32) as usize;
&self.hash_table[absolute_position]
}
pub fn dht_write(&mut self, location: u32, data: T) {
assert!(self.dht_is_initialised(), "DHT not initialized");
assert!(location >= self.lower_bound as u32, "location below lower bound");
assert!(location < self.higher_bound as u32, "location above upper bound");
let absolute_position = (location - self.lower_bound as u32) as usize;
println!("dht_write into {}", absolute_position);
self.hash_table[absolute_position] = Some(data);
}
}
}
