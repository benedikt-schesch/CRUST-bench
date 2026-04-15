
pub struct Hash<T> {
buckets: Vec<Vec<T>>,
hash_function: Box<dyn Fn(&T) -> u32>,
max_key: u32,
}
impl<T> Hash<T> {
pub fn new(max_key: u32, hash_function: impl Fn(&T) -> u32 + 'static) -> Self {
let mut buckets = Vec::with_capacity((max_key + 1) as usize);
for _ in 0..=max_key {
buckets.push(Vec::new());
}
Self {
buckets,
hash_function: Box::new(hash_function),
max_key,
}
}
pub fn insert(&mut self, element: T, compare_equal: impl Fn(&T, &T) -> bool) -> bool {
let ix = (self.hash_function)(&element) as usize;
assert!(ix <= self.max_key as usize);
let bucket = &mut self.buckets[ix];
for existing in bucket.iter() {
if compare_equal(existing, &element) {
return false;
}
}
bucket.push(element);
true
}
pub fn element_exists(&self, element: &T, compare_equal: impl Fn(&T, &T) -> bool) -> bool {
let ix = (self.hash_function)(element) as usize;
assert!(ix <= self.max_key as usize);
let bucket = &self.buckets[ix];
for existing in bucket.iter() {
if compare_equal(existing, element) {
return true;
}
}
false
}
pub fn delete(&mut self, element: &T, compare_equal: impl Fn(&T, &T) -> bool) -> bool {
let ix = (self.hash_function)(element) as usize;
assert!(ix <= self.max_key as usize);
let bucket = &mut self.buckets[ix];
for i in 0..bucket.len() {
if compare_equal(&bucket[i], element) {
bucket.remove(i);
return true;
}
}
false
}
}
