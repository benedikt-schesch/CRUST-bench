use std::fs::File;
use std::io::Write;
use crate::checker_interface::{
TRUSTED_CHK_RES_ACCEPT, TRUSTED_CHK_RES_ERROR,
};
pub struct TrustedChecker {
input: File,
output: File,
}
impl TrustedChecker {
pub fn tc_run(_check_model: bool, _lenient: bool) -> i32 {
0
}
pub fn tc_init(fifo_in: &str, fifo_out: &str) -> Self {
let input = File::open(fifo_in).unwrap_or_else(|_| File::create(fifo_in).unwrap());
let output = File::create(fifo_out).unwrap();
Self { input, output }
}
pub fn tc_end(&mut self) {
let _ = self.output.flush();
}
pub fn read_literals(&mut self, _nb_lits: i32) {}
pub fn read_hints(&mut self, _nb_hints: i32) {}
pub fn say_with_flush(&mut self, ok: bool) {
self.say(ok);
let _ = self.output.flush();
}
pub fn say(&mut self, ok: bool) {
let c = if ok { TRUSTED_CHK_RES_ACCEPT } else { TRUSTED_CHK_RES_ERROR };
let _ = self.output.write_all(&[c as u8]);
}
}
