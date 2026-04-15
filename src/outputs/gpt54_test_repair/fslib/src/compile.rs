use crate::sr;
use crate::fst;
use crate::symt;
use std::io::{self, BufRead};
use crate::symt::SymTable;
fn trn(token: &str, _symt: &SymTable)-> usize{
match token.parse::<isize>() {
Ok(v) => v as usize,
Err(_) => usize::MAX,
}
}
fn trt(token: &str, symt: &SymTable)-> usize{
symt.getr(token).map(|v| v as usize).unwrap_or(usize::MAX)
}
fn add_arc(
fst: &mut fst::Fst,
sa: usize,
sb: usize,
li: usize,
lo: usize,
w: f32,
) {
while sa + 1 > fst.n_states as usize || sb + 1 > fst.n_states as usize {
fst.add_state();
}
fst.add_arc(sa as u32, sb as u32, li as u32, lo as u32, w);
}
fn add_final(fst: &mut fst::Fst, s: usize, w: f32) {
while s + 1 > fst.n_states as usize {
fst.add_state();
}
fst.set_final(s as u32, w);
}
fn parse_line(fst: &mut fst::Fst, buf: &mut str)->i32{
let sr = sr::sr_get(fst.sr_type);
let parts: Vec<&str> = buf.trim().split('\t').collect();
match parts.len() {
5 => {
let sa = parts[0].parse::<usize>().ok();
let sb = parts[1].parse::<usize>().ok();
let li = parts[2].parse::<usize>().ok();
let lo = parts[3].parse::<usize>().ok();
let w = parts[4].parse::<f32>().ok();
if let (Some(sa), Some(sb), Some(li), Some(lo), Some(w)) = (sa, sb, li, lo, w) {
add_arc(fst, sa, sb, li, lo, w);
0
} else {
-1
}
}
4 => {
let sa = parts[0].parse::<usize>().ok();
let sb = parts[1].parse::<usize>().ok();
let li = parts[2].parse::<usize>().ok();
let lo = parts[3].parse::<usize>().ok();
if let (Some(sa), Some(sb), Some(li), Some(lo)) = (sa, sb, li, lo) {
add_arc(fst, sa, sb, li, lo, sr.one);
0
} else {
-1
}
}
2 => {
let sf = parts[0].parse::<usize>().ok();
let w = parts[1].parse::<f32>().ok();
if let (Some(sf), Some(w)) = (sf, w) {
add_final(fst, sf, w);
0
} else {
-1
}
}
1 => {
if let Ok(sf) = parts[0].parse::<usize>() {
add_final(fst, sf, sr.one);
0
} else {
-1
}
}
_ => -1,
}
}
fn parse_line_sym(fst: &mut fst::Fst, buf: &mut str, ist: &SymTable, ost: &SymTable, sst: &SymTable)->i32{
let sr = sr::sr_get(fst.sr_type);
let parts: Vec<&str> = buf.trim().split('\t').collect();
match parts.len() {
5 => {
let sa = trt(parts[0], sst);
let sb = trt(parts[1], sst);
let li = trt(parts[2], ist);
let lo = trt(parts[3], ost);
let w = parts[4].parse::<f32>().unwrap_or(-1.0);
if sa == usize::MAX || sb == usize::MAX || li == usize::MAX || lo == usize::MAX {
-1
} else {
add_arc(fst, sa, sb, li, lo, w);
0
}
}
4 => {
let sa = trt(parts[0], sst);
let sb = trt(parts[1], sst);
let li = trt(parts[2], ist);
let lo = trt(parts[3], ost);
if sa == usize::MAX || sb == usize::MAX || li == usize::MAX || lo == usize::MAX {
-1
} else {
add_arc(fst, sa, sb, li, lo, sr.one);
0
}
}
2 => {
let sf = trt(parts[0], sst);
let w = parts[1].parse::<f32>().unwrap_or(-1.0);
if sf == usize::MAX {
-1
} else {
add_final(fst, sf, w);
0
}
}
1 => {
let sf = trt(parts[0], sst);
if sf == usize::MAX {
-1
} else {
add_final(fst, sf, sr.one);
0
}
}
_ => -1,
}
}
fn parse_line_sym_acc(fst: &mut fst::Fst, buf: &mut str, ist: &SymTable, _ost: &SymTable, sst: &SymTable)->i32{
let sr = sr::sr_get(fst.sr_type);
let parts: Vec<&str> = buf.trim().split('\t').collect();
match parts.len() {
4 => {
let sa = trt(parts[0], sst);
let sb = trt(parts[1], sst);
let li = trt(parts[2], ist);
let w = parts[3].parse::<f32>().unwrap_or(-1.0);
if sa == usize::MAX || sb == usize::MAX || li == usize::MAX {
-1
} else {
add_arc(fst, sa, sb, li, li, w);
0
}
}
3 => {
let sa = trt(parts[0], sst);
let sb = trt(parts[1], sst);
let li = trt(parts[2], ist);
if sa == usize::MAX || sb == usize::MAX || li == usize::MAX {
-1
} else {
add_arc(fst, sa, sb, li, li, sr.one);
0
}
}
2 => {
let sf = trt(parts[0], sst);
let w = parts[1].parse::<f32>().unwrap_or(-1.0);
if sf == usize::MAX {
-1
} else {
add_final(fst, sf, w);
0
}
}
1 => {
let sf = trt(parts[0], sst);
if sf == usize::MAX {
-1
} else {
add_final(fst, sf, sr.one);
0
}
}
_ => -1,
}
}
fn fst_compile(fst: &mut fst::Fst, fin: &mut dyn BufRead, ist: &SymTable, ost: &SymTable, sst: &SymTable, is_acc: bool)-> fst::Fst{
let mut line_no = 1usize;
let mut line = String::new();
loop {
line.clear();
let n = fin.read_line(&mut line).unwrap();
if n == 0 {
break;
}
line_no += 1;
let mut owned = line.clone();
let res = if !is_acc {
parse_line_sym(fst, owned.as_mut_str(), ist, ost, sst)
} else {
parse_line_sym_acc(fst, owned.as_mut_str(), ist, ost, sst)
};
if res != 0 {
panic!("Invalid input line {}: {}", line_no, line);
}
}
if let Some(start_state) = sst.getr(fst::START_STATE) {
fst.start = start_state as u32;
}
fst.clone()
}
fn fst_compile_str(fst: &mut fst::Fst, s: &str) -> fst::Fst{
let mut line = 1usize;
for tok in s.split('\n') {
if !tok.is_empty() {
let mut owned = tok.to_string();
if parse_line(fst, owned.as_mut_str()) != 0 {
panic!("Invalid input line {}: {}", line, tok);
}
}
line += 1;
}
fst.clone()
}
