use crate::metadata::PgnMetadata;
use crate::moves::PgnMoves;
use crate::score::PgnScore;

#[derive(Debug, Clone, PartialEq)]
pub struct Pgn {
pub metadata: Option<PgnMetadata>,
pub moves: Option<PgnMoves>,
pub score: PgnScore,
}

impl Pgn {
pub fn new() -> Self {
Self {
metadata: Some(PgnMetadata::new()),
moves: Some(PgnMoves::new()),
score: PgnScore::Unknown,
}
}

pub fn parse(&mut self, s: &str) {
*self = Self::parse_internal(s);
}

fn parse_internal(s: &str) -> Self {
let mut pgn = Pgn::new();

// Split into metadata and moves
let parts: Vec<&str> = s.split("\n\n").collect();
if !parts.is_empty() {
pgn.metadata = Some(Pgn::parse_metadata(parts[0]));

if parts.len() > 1 {
let move_text = parts[1..].join("\n\n");
pgn.moves = Some(PgnMoves::from(move_text.as_str()));

// Parse score from end of move text
if move_text.contains("1-0") {
pgn.score = PgnScore::WhiteWon;
} else if move_text.contains("0-1") {
pgn.score = PgnScore::BlackWon;
} else if move_text.contains("1/2-1/2") {
pgn.score = PgnScore::Draw;
}
}
}

pgn
}

pub fn parse_metadata(s: &str) -> PgnMetadata {
let mut metadata = PgnMetadata::new();

for line in s.lines() {
let line = line.trim();
if line.starts_with('[') && line.ends_with(']') {
let content = &line[1..line.len()-1];
let parts: Vec<&str> = content.splitn(2, '"').collect();
if parts.len() == 2 {
let key = parts[0].trim();
let value = parts[1].trim_end_matches('"');
metadata.insert(key.to_string(), value.to_string());
}
}
}

metadata
}
}

impl Default for Pgn {
fn default() -> Self {
Self::new()
}
}
