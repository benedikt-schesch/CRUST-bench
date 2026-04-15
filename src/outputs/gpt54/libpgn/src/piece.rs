use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PgnPiece {
Unknown = 0,
Pawn = b'P',
Rook = b'R',
Knight = b'N',
Bishop = b'B',
Queen = b'Q',
King = b'K',
}

impl From<char> for PgnPiece {
fn from(ch: char) -> Self {
match ch {
'P' => Self::Pawn,
'R' => Self::Rook,
'N' => Self::Knight,
'B' => Self::Bishop,
'Q' => Self::Queen,
'K' => Self::King,
_ => Self::Unknown,
}
}
}

impl Display for PgnPiece {
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
match self {
PgnPiece::Pawn => write!(f, "Pawn"),
PgnPiece::Rook => write!(f, "Rook"),
PgnPiece::Knight => write!(f, "Knight"),
PgnPiece::Bishop => write!(f, "Bishop"),
PgnPiece::Queen => write!(f, "Queen"),
PgnPiece::King => write!(f, "King"),
PgnPiece::Unknown => Err(std::fmt::Error),
}
}
}
