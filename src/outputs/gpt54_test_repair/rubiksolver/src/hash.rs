
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
let ix = (self.hash_function)(&element);
assert!(ix <= self.max_key);
let bucket = &mut self.buckets[ix as usize];
if bucket.iter().any(|existing| compare_equal(existing, &element)) {
return false;
}
bucket.push(element);
true
}
pub fn element_exists(&self, element: &T, compare_equal: impl Fn(&T, &T) -> bool) -> bool {
let ix = (self.hash_function)(element);
assert!(ix <= self.max_key);
self.buckets[ix as usize]
.iter()
.any(|existing| compare_equal(existing, element))
}
pub fn delete(&mut self, element: &T, compare_equal: impl Fn(&T, &T) -> bool) -> bool {
let ix = (self.hash_function)(element);
assert!(ix <= self.max_key);
let bucket = &mut self.buckets[ix as usize];
if let Some(pos) = bucket.iter().position(|existing| compare_equal(existing, element)) {
bucket.remove(pos);
true
} else {
false
}
}
}
