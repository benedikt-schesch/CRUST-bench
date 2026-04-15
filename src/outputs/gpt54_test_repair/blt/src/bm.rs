
use std::io::{self, BufRead};
use std::sync::{Mutex, OnceLock};
use std::time::Instant;
static BM_TP: OnceLock<Mutex<Option<Instant>>> = OnceLock::new();
fn bm_state() -> &'static Mutex<Option<Instant>> {
BM_TP.get_or_init(|| Mutex::new(None))
}
pub fn bm_init() {
let mut guard = bm_state().lock().unwrap();
*guard = Some(Instant::now());
}
pub fn bm_report(msg: &str) {
let mut guard = bm_state().lock().unwrap();
let now = Instant::now();
let start = guard.unwrap_or(now);
let d = now.duration_since(start);
println!("{}: {}.{:09}s", msg, d.as_secs(), d.subsec_nanos());
*guard = Some(now);
}
pub fn bm_read_keys<F>(mut cb: F)
where
F: FnMut(&[&str], i32),
{
let stdin = io::stdin();
let mut keys: Vec<String> = Vec::new();
for line in stdin.lock().lines() {
match line {
Ok(s) => keys.push(s),
Err(_) => return,
}
}
if keys.len() > 2 {
for i in (2..keys.len()).rev() {
let j = i / 2;
keys.swap(i, j);
}
}
let refs: Vec<&str> = keys.iter().map(|s| s.as_str()).collect();
cb(&refs, refs.len() as i32);
}
