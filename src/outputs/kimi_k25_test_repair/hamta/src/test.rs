use crate::hamta::{Hamt, KeyValue, hamt_int_hash, hamt_int_equals, hamt_str_hash, hamt_str_equals};
fn hamt_string_hash(s: &String) -> u32 {
hamt_str_hash(s)
}
fn hamt_string_equals(a: &String, b: &String) -> bool {
hamt_str_equals(a, b)
}
fn main() {
test_int_hamt();
test_string_hamt_basic();
test_string_hamt_multiple();
test_string_hamt_many();
}
fn test_int_hamt() {
let mut h: Hamt<i32, i32> = Hamt::new_hamt(hamt_int_hash, hamt_int_equals);
let key = 1;
let value = 100;
let mut conflict_kv: Option<KeyValue<i32, i32>> = None;
let _conflict = h.hamt_set(key, value, &mut conflict_kv);
for i in 0..10 {
let key = i;
let value = i * 10;
let mut conflict_kv: Option<KeyValue<i32, i32>> = None;
let _conflict = h.hamt_set(key, value, &mut conflict_kv);
}
let i = 5;
let mut removed_kv: Option<KeyValue<i32, i32>> = None;
let _removed = h.hamt_remove(&i, &mut removed_kv);
}
fn test_string_hamt_basic() {
let mut h: Hamt<String, String> = Hamt::new_hamt(hamt_string_hash, hamt_string_equals);
let mut conflict_kv: Option<KeyValue<String, String>> = None;
let x = "x".to_string();
let x1 = "x1".to_string();
let y = "y".to_string();
let y1 = "y1".to_string();
h.hamt_set(x.clone(), x1.clone(), &mut conflict_kv);
h.hamt_set(y.clone(), y1.clone(), &mut conflict_kv);
h.hamt_set(x.clone(), y.clone(), &mut conflict_kv);
h.hamt_set(y.clone(), x.clone(), &mut conflict_kv);
let mut conflict_kv2: Option<KeyValue<String, String>> = None;
let _removed = h.hamt_remove(&x, &mut conflict_kv2);
h.hamt_destroy(|_| {}, |_| {});
}
fn test_string_hamt_multiple() {
let mut h: Hamt<String, String> = Hamt::new_hamt(hamt_string_hash, hamt_string_equals);
let strings = vec!["a", "b", "c"];
let num_elements = strings.len() as i32;
for s in &strings {
let mut conflict_kv: Option<KeyValue<String, String>> = None;
h.hamt_set(s.to_string(), s.to_string(), &mut conflict_kv);
}
assert_eq!(h.hamt_size(), num_elements, "error, hamt size doesn't match 1");
for s in &strings {
let _found = h.hamt_search(&s.to_string());
}
assert_eq!(h.hamt_size(), num_elements, "error, hamt size doesn't match");
for s in &strings {
let mut removed_kv: Option<KeyValue<String, String>> = None;
let _removed = h.hamt_remove(&s.to_string(), &mut removed_kv);
assert_eq!(h.hamt_size(), num_elements - 1, "error, hamt size doesn't match after removal");
}
}
fn test_string_hamt_many() {
let mut h: Hamt<String, String> = Hamt::new_hamt(hamt_string_hash, hamt_string_equals);
let strings = vec!["foo", "bar", "baz"];
for s in &strings {
let mut conflict_kv: Option<KeyValue<String, String>> = None;
h.hamt_set(s.to_string(), s.to_string(), &mut conflict_kv);
}
assert_eq!(h.hamt_size(), strings.len() as i32, "error, hamt size doesn't match");
}
