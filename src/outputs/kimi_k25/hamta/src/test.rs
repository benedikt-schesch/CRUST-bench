use crate::hamta::{Hamt, KeyValue, hamt_int_hash, hamt_int_equals, hamt_str_hash, hamt_str_equals};

// Wrapper functions to adapt &String to &str for the hash and equals functions
// This is necessary because function pointer types do not support automatic coercion
fn hamt_string_hash(s: &String) -> u32 {
hamt_str_hash(s)
}

fn hamt_string_equals(a: &String, b: &String) -> bool {
hamt_str_equals(a, b)
}

fn main() {
// Test integer Hamt (corresponds to lines 5-54 in original errors)
test_int_hamt();

// Test string Hamt basic operations (corresponds to lines 63-80)
test_string_hamt_basic();

// Test string Hamt with multiple elements (corresponds to lines 94-112)
test_string_hamt_multiple();

// Test string Hamt with many elements (corresponds to lines 117-132)
test_string_hamt_many();
}

fn test_int_hamt() {
// Line 5: Create Hamt without turbofish on function pointers
let mut h: Hamt<i32, i32> = Hamt::new_hamt(hamt_int_hash, hamt_int_equals);

// Line 20: Insert with proper types (pass by value, not &mut Box)
let key = 1;
let value = 100;
let mut conflict_kv: Option<KeyValue<i32, i32>> = None;
let _conflict = h.hamt_set(key, value, &mut conflict_kv);

// Lines 41-ish: Insert in a loop with proper types
for i in 0..10 {
let key = i;
let value = i * 10;
let mut conflict_kv: Option<KeyValue<i32, i32>> = None;
let _conflict = h.hamt_set(key, value, &mut conflict_kv);
}

// Line 54: Remove with proper reference (&i, not &mut Box::new(i))
let i = 5;
let mut removed_kv: Option<KeyValue<i32, i32>> = None;
let _removed = h.hamt_remove(&i, &mut removed_kv);
}

fn test_string_hamt_basic() {
// Line 63: Use String instead of str to satisfy Sized bound
let mut h: Hamt<String, String> = Hamt::new_hamt(hamt_string_hash, hamt_string_equals);
let mut conflict_kv: Option<KeyValue<String, String>> = None;

// Lines 72-75: Use String values
let x = "x".to_string();
let x1 = "x1".to_string();
let y = "y".to_string();
let y1 = "y1".to_string();

h.hamt_set(x.clone(), x1.clone(), &mut conflict_kv);
h.hamt_set(y.clone(), y1.clone(), &mut conflict_kv);
h.hamt_set(x.clone(), y.clone(), &mut conflict_kv);
h.hamt_set(y.clone(), x.clone(), &mut conflict_kv);

// Line 76: Remove with proper reference
let mut conflict_kv2: Option<KeyValue<String, String>> = None;
let _removed = h.hamt_remove(&x, &mut conflict_kv2);

// Line 80: Destroy with destructors
h.hamt_destroy(|_| {}, |_| {});
}

fn test_string_hamt_multiple() {
// Lines 94-112
let mut h: Hamt<String, String> = Hamt::new_hamt(hamt_string_hash, hamt_string_equals);
let strings = vec!["a", "b", "c"];
let num_elements = strings.len() as i32;

// Line 94: Pass String values, not &mut Box::new(...)
for s in &strings {
let mut conflict_kv: Option<KeyValue<String, String>> = None;
h.hamt_set(s.to_string(), s.to_string(), &mut conflict_kv);
}

// Line 97
assert_eq!(h.hamt_size(), num_elements, "error, hamt size doesn't match 1");

// Lines 99-102: Search with proper reference
for s in &strings {
let _found = h.hamt_search(&s.to_string());
}

// Line 103
assert_eq!(h.hamt_size(), num_elements, "error, hamt size doesn't match");

// Lines 109-112: Remove with proper reference
for s in &strings {
let mut removed_kv: Option<KeyValue<String, String>> = None;
let _removed = h.hamt_remove(&s.to_string(), &mut removed_kv);
// Line 112
assert_eq!(h.hamt_size(), num_elements - 1, "error, hamt size doesn't match after removal");
}
}

fn test_string_hamt_many() {
// Lines 117-132
let mut h: Hamt<String, String> = Hamt::new_hamt(hamt_string_hash, hamt_string_equals);
let strings = vec!["foo", "bar", "baz"];

// Line 128: Insert strings
for s in &strings {
let mut conflict_kv: Option<KeyValue<String, String>> = None;
h.hamt_set(s.to_string(), s.to_string(), &mut conflict_kv);
}

// Line 132
assert_eq!(h.hamt_size(), strings.len() as i32, "error, hamt size doesn't match");
}
