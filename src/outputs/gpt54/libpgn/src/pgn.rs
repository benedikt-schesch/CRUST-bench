use crate::metadata::PgnMetadata;
use crate::moves::{PgnMove, PgnMoves};
use crate::score::PgnScore;
use crate::utils::cursor::pgn_cursor_skip_whitespace;

#[derive(Debug)]
pub struct Pgn {
pub metadata: Option<Box<PgnMetadata>>,
pub moves: Option<Box<PgnMoves>>,
pub score: PgnScore,
}

impl Pgn {
pub fn new() -> Self {
Self {
metadata: None,
moves: None,
score: PgnScore::Unknown,
}
}

pub fn parse_metadata(s: &str) -> PgnMetadata {
PgnMetadata::from_string(s)
}

pub fn parse_move(s: &str) -> PgnMove {
PgnMove::from(s)
}

pub fn parse_moves(s: &str) -> PgnMoves {
PgnMoves::from(s)
}

pub fn parse_score(s: &str) -> PgnScore {
PgnScore::from(s)
}

pub fn parse(&mut self, s: &str) -> usize {
let mut cursor = 0usize;

let mut metadata_consumed = 0usize;
let metadata = PgnMetadata::from_string_with_consumption(&s[cursor..], &mut metadata_consumed);
cursor += metadata_consumed;
self.metadata = Some(Box::new(metadata));

pgn_cursor_skip_whitespace(s, &mut cursor);

let mut moves_consumed = 0usize;
let moves = PgnMoves::from_string_with_consumption(&s[cursor..], &mut moves_consumed);
cursor += moves_consumed;
self.moves = Some(Box::new(moves));

let mut score_consumed = 0usize;
self.score = {
let score = PgnScore::from(&s[cursor..]);
if score != PgnScore::Unknown {
score_consumed = score.to_string().len();
}
score
};
cursor += score_consumed;

cursor
}
}
