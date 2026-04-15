#[derive(Debug, Clone, PartialEq)]
pub enum PgnAnnotation {
None,
GoodMove,
Mistake,
InterestingMove,
DubiousMove,
BrilliantMove,
Blunder,
Unknown,
}

impl From<i32> for PgnAnnotation {
fn from(n: i32) -> Self {
match n {
1 => PgnAnnotation::GoodMove,
2 => PgnAnnotation::Mistake,
3 => PgnAnnotation::InterestingMove,
4 => PgnAnnotation::Blunder,
5 => PgnAnnotation::InterestingMove,
6 => PgnAnnotation::DubiousMove,
7 => PgnAnnotation::GoodMove,
8 => PgnAnnotation::BrilliantMove,
9 => PgnAnnotation::Mistake,
10 => PgnAnnotation::Blunder,
_ => PgnAnnotation::Unknown,
}
}
}
