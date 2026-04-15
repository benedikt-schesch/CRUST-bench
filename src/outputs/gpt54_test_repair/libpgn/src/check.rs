#[repr(i8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PgnCheck {
Mate = -1,
None = 0,
Single,
Double,
}
impl PgnCheck {
pub fn __pgn_check_from_string(str: &str, consumed: &mut usize) -> Self {
let bytes = str.as_bytes();
let mut cursor = 0usize;
let mut check = PgnCheck::None;
match bytes.get(cursor).copied() {
Some(b'+') => {
let mut pluses = 0usize;
while bytes.get(cursor).copied() == Some(b'+') {
pluses += 1;
cursor += 1;
}
assert!(pluses <= 2);
check = match pluses {
0 => PgnCheck::None,
1 => PgnCheck::Single,
2 => PgnCheck::Double,
_ => unreachable!(),
};
}
Some(b'#') => {
check = PgnCheck::Mate;
cursor += 1;
}
_ => {}
}
*consumed += cursor;
check
}
}
impl From<&str> for PgnCheck {
fn from(s: &str) -> Self {
let mut consumed = 0;
Self::__pgn_check_from_string(s, &mut consumed)
}
}
