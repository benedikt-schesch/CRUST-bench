use std::any::Any;
use std::fmt;
#[derive(Debug)]
pub struct BucketNode {
pub key: String,
pub value: Box<dyn Any>,
pub next: Option<Box<BucketNode>>,
}
#[derive(Debug)]
pub struct Hashmap {
pub buckets: Vec<Option<Box<BucketNode>>>,
pub size: i32,
pub cap: i32,
pub hash: fn(&str) -> u32,
pub equals: fn(&str, &str) -> bool,
}
pub fn create_hashmap(cap: i32) -> Hashmap {
let capacity = if cap < 0 { 0 } else { cap as usize };
Hashmap {
buckets: std::iter::repeat_with(|| None).take(capacity).collect(),
size: 0,
cap,
hash: fnva1,
equals: equal_key,
}
}
pub fn hm_get<'a>(h: &'a Hashmap, key: &'a str) -> Option<&'a BucketNode> {
if h.cap <= 0 || h.buckets.is_empty() {
return None;
}
let a = ((h.hash)(key) % (h.cap as u32)) as usize;
let b = h.buckets.get(a)?.as_deref()?;
if (h.equals)(&b.key, key) {
Some(b)
} else {
None
}
}
pub fn hm_set(h: &mut Hashmap, key: &str, value: Box<dyn Any>) -> i32 {
if h.cap <= 0 {
return -1;
}
let a = ((h.hash)(key) % (h.cap as u32)) as usize;
let is_empty = h.buckets.get(a).map(|x| x.is_none()).unwrap_or(true);
if is_empty {
if h.size == h.cap {
double_cap(h);
}
let a2 = ((h.hash)(key) % (h.cap as u32)) as usize;
h.size += 1;
h.buckets[a2] = Some(Box::new(BucketNode {
key: key.to_string(),
value,
next: None,
}));
0
} else {
-1
}
}
pub fn test_hash_set_and_get() -> i32 {
let mut h = create_hashmap(100);
let ret = hm_set(&mut h, "test", Box::new(String::from("jake")));
assert!(ret != -1);
let got = hm_get(&h, "test").expect("expected hashmap value");
let s = got
.value
.downcast_ref::<String>()
.expect("expected String value");
assert!(s == "jake");
0
}
pub fn double_cap(h: &mut Hashmap) {
let new_cap = if h.cap <= 0 { 0 } else { h.cap * 2 };
let mut new_buckets: Vec<Option<Box<BucketNode>>> =
std::iter::repeat_with(|| None).take(new_cap as usize).collect();
for bucket in h.buckets.drain(..) {
if let Some(node) = bucket {
let a = if h.cap > 0 {
((h.hash)(&node.key) % (h.cap as u32)) as usize
} else {
0
};
if a < new_buckets.len() {
new_buckets[a] = Some(node);
}
}
}
h.buckets = new_buckets;
h.cap = new_cap;
}
pub fn fnva1(value: &str) -> u32 {
let mut h: u64 = 16777619;
let prime: u64 = 2166136261;
for b in value.bytes() {
h ^= b as u64;
h = h.wrapping_mul(prime);
}
h as u32
}
pub fn test_hash_init() -> i32 {
let h = create_hashmap(100);
assert!(h.size == 0);
assert!(h.cap == 100);
0
}
pub fn destroy_hashmap(h: &mut Hashmap) {
h.buckets.clear();
h.size = 0;
h.cap = 0;
}
pub fn test_hash_init_and_store() -> i32 {
let mut h = create_hashmap(100);
assert!(h.size == 0);
assert!(h.cap == 100);
let ret = hm_set(&mut h, "test", Box::new(String::from("jake")));
assert!(ret != -1);
let ind = ((h.hash)("test") % (h.cap as u32)) as usize;
let b = h.buckets[ind].as_ref().expect("bucket missing");
assert!(b.key == "test");
assert!(h.size == 1);
assert!(h.cap == 100);
0
}
pub fn test_hash_set_and_double_get() -> i32 {
let mut h = create_hashmap(100);
let ret = hm_set(&mut h, "test", Box::new(String::from("jake")));
assert!(ret != -1);
double_cap(&mut h);
let got = hm_get(&h, "test").expect("expected hashmap value");
let s = got
.value
.downcast_ref::<String>()
.expect("expected String value");
assert!(s == "jake");
0
}
pub fn equal_key(a: &str, b: &str) -> bool {
a == b
}
