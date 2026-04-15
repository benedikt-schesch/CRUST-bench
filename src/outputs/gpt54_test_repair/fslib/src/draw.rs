use std::fs::File;
use std::io::{self, Write};
use crate::fst::Fst;
use crate::symt::SymTable;
const HEADER: &str = "digraph T {\n\trankdir = LR;\n\torientation = Landscape;\n";
const FOOTER: &str = "}\n";
pub fn fst_draw(fst: &Fst, fout: &mut File) -> io::Result<()> {
fout.write_all(HEADER.as_bytes())?;
for (s, state) in fst.states.iter().enumerate() {
if !state.final_state {
writeln!(
fout,
"\t{} [label = \"{}\", shape = circle, style = {} ];",
s,
s,
if s as u32 == fst.start { "filled" } else { "solid" }
)?;
} else {
writeln!(
fout,
"\t{} [label = \"{}\", shape = doublecircle, style = filled ];",
s, s
)?;
}
for arc in &state.arcs {
writeln!(
fout,
"\t\t{} -> {} [ label = \"{}:{}/{}\" ];",
s, arc.state, arc.ilabel, arc.olabel, arc.weight
)?;
}
}
fout.write_all(FOOTER.as_bytes())?;
Ok(())
}
pub fn fst_draw_sym(
fst: &Fst,
fout: &mut File,
ist: Option<&SymTable>,
ost: Option<&SymTable>,
sst: Option<&SymTable>,
) -> io::Result<()> {
fout.write_all(HEADER.as_bytes())?;
for (s, state) in fst.states.iter().enumerate() {
let sa = sst.and_then(|t| t.get(s as i32)).unwrap_or("");
if !state.final_state {
writeln!(
fout,
"\t{} [label = \"{}\", shape = circle, style = {} ];",
sa,
sa,
if s as u32 == fst.start { "filled" } else { "solid" }
)?;
} else {
writeln!(
fout,
"\t{} [label = \"{}\", shape = doublecircle, style = filled ];",
sa, sa
)?;
}
for arc in &state.arcs {
let sb = sst.and_then(|t| t.get(arc.state as i32)).unwrap_or("");
let li = ist.and_then(|t| t.get(arc.ilabel as i32)).unwrap_or("");
let lo = ost.and_then(|t| t.get(arc.olabel as i32)).unwrap_or("");
writeln!(
fout,
"\t\t{} -> {} [ label = \"{}:{}/{}\" ];",
sa, sb, li, lo, arc.weight
)?;
}
}
fout.write_all(FOOTER.as_bytes())?;
Ok(())
}
fn trn(_st: &mut SymTable, id: usize, _token: &str) -> String {
id.to_string()
}
fn trt(st: &mut SymTable, id: usize, _token: &str) -> String {
st.get(id as i32).unwrap_or("").to_string()
}
