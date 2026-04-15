#[derive(Debug, Clone, PartialEq)]
pub enum PgnScore {
WhiteWon,
BlackWon,
Draw,
Unknown,
Forfeit,
WhiteForfeit,
BlackForfeit,
}

impl From<&str> for PgnScore {
fn from(s: &str) -> Self {
match s {
"1-0" => PgnScore::WhiteWon,
"0-1" => PgnScore::BlackWon,
"1/2-1/2" => PgnScore::Draw,
"0-0" => PgnScore::Forfeit,
"0-1/2" => PgnScore::WhiteForfeit,
"1/2-0" => PgnScore::BlackForfeit,
_ => PgnScore::Unknown,
}
}
}

impl From<&String> for PgnScore {
fn from(s: &String) -> Self {
Self::from(s.as_str())
}
}
