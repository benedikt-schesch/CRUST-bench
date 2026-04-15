use crate::fst::Fst;
use crate::queue::Queue;
use crate::symt::SymTable;
use std::io::{self, Write};
pub fn fst_print(fst: &Fst, output: &mut dyn Write) -> io::Result<()> {
let mut finals: Queue<u32> = Queue::new();
for (s, state) in fst.states.iter().enumerate() {
for arc in &state.arcs {
writeln!(
output,
"{}\t{}\t{}\t{}\t{:.5}",
s, arc.state, arc.ilabel, arc.olabel, arc.weight
)?;
}
if state.final_state {
finals.enqueue(s as u32);
}
}
while let Some(s) = finals.dequeue() {
let state = &fst.states[s as usize];
writeln!(output, "{}\t{}", s, state.weight)?;
}
Ok(())
}
pub fn fst_print_sym(
fst: &Fst,
ist: Option<&SymTable>,
ost: Option<&SymTable>,
sst: Option<&SymTable>,
output: &mut dyn Write,
) -> io::Result<()> {
let mut finals: Queue<u32> = Queue::new();
for (s, state) in fst.states.iter().enumerate() {
for arc in &state.arcs {
let sa = sst.and_then(|t| t.get(s as i32)).unwrap_or("");
let sb = sst.and_then(|t| t.get(arc.state as i32)).unwrap_or("");
let li = ist.and_then(|t| t.get(arc.ilabel as i32)).unwrap_or("");
let lo = ost.and_then(|t| t.get(arc.olabel as i32)).unwrap_or("");
writeln!(output, "{}\t{}\t{}\t{}\t{:.5}", sa, sb, li, lo, arc.weight)?;
}
if state.final_state {
finals.enqueue(s as u32);
}
}
while let Some(s) = finals.dequeue() {
let state = &fst.states[s as usize];
let sa = sst.and_then(|t| t.get(s as i32)).unwrap_or("");
writeln!(output, "{}\t{}", sa, state.weight)?;
}
Ok(())
}
