#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PgnScore {
Unknown = 0,
Ongoing,
Draw,
WhiteWon,
BlackWon,
Forfeit,
WhiteForfeit,
BlackForfeit,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PgnScoreSingle {
Zero = 0,
One,
Half,
}
impl PgnScore {
fn parse_single(s: &str, consumed: &mut usize) -> Option<PgnScoreSingle> {
if s.len() >= 3 && &s[..3] == "1/2" {
*consumed += 3;
Some(PgnScoreSingle::Half)
} else if s.starts_with('0') {
*consumed += 1;
Some(PgnScoreSingle::Zero)
} else if s.starts_with('1') {
*consumed += 1;
Some(PgnScoreSingle::One)
} else {
None
}
}
fn from_string_with_consumption(s: &str, consumed: &mut usize) -> Self {
let bytes = s.as_bytes();
let mut cursor = 0usize;
if bytes.get(cursor).copied() == Some(b'*') {
cursor += 1;
*consumed += cursor;
return Self::Ongoing;
}
let white = match Self::parse_single(&s[cursor..], &mut cursor) {
Some(v) => v,
None => return Self::Unknown,
};
if bytes.get(cursor).copied() != Some(b'-') {
return Self::Unknown;
}
cursor += 1;
let black = match Self::parse_single(&s[cursor..], &mut cursor) {
Some(v) => v,
None => return Self::Unknown,
};
*consumed += cursor;
match (white, black) {
(PgnScoreSingle::Half, PgnScoreSingle::Half) => Self::Draw,
(PgnScoreSingle::One, PgnScoreSingle::Zero) => Self::WhiteWon,
(PgnScoreSingle::Zero, PgnScoreSingle::One) => Self::BlackWon,
(PgnScoreSingle::Zero, PgnScoreSingle::Zero) => Self::Forfeit,
(PgnScoreSingle::Zero, PgnScoreSingle::Half) => Self::WhiteForfeit,
(PgnScoreSingle::Half, PgnScoreSingle::Zero) => Self::BlackForfeit,
_ => Self::Unknown,
}
}
}
impl From<&str> for PgnScore {
fn from(s: &str) -> Self {
let mut consumed = 0;
Self::from_string_with_consumption(s, &mut consumed)
}
}
impl std::fmt::Display for PgnScore {
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
match self {
PgnScore::Unknown => write!(f, ""),
PgnScore::Ongoing => write!(f, "*"),
PgnScore::Draw => write!(f, "1/2-1/2"),
PgnScore::WhiteWon => write!(f, "1-0"),
PgnScore::BlackWon => write!(f, "0-1"),
PgnScore::Forfeit => write!(f, "0-0"),
PgnScore::WhiteForfeit => write!(f, "0-1/2"),
PgnScore::BlackForfeit => write!(f, "1/2-0"),
}
}
}
