use crate::amp::Amp;
fn main() {
let mut msg = Amp {
version: 0,
argc: 0,
buf: String::new(),
pos: 0,
};
msg.decode("\x12argument1");
}
