use std::fmt::Display;

use crate::utils::cursor::{pgn_cursor_revisit_whitespace, pgn_cursor_skip_whitespace};

#[repr(i8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PgnAnnotation {
Unknown = -1,
Null = 0,
GoodMove,
Mistake,
BrilliantMove,
Blunder,
InterestingMove,
DubiousMove,
}

impl From<i8> for PgnAnnotation {
fn from(value: i8) -> Self {
match value {
-1 => Self::Unknown,
0 => Self::Null,
1 => Self::GoodMove,
2 => Self::Mistake,
3 => Self::BrilliantMove,
4 => Self::Blunder,
5 => Self::InterestingMove,
6 => Self::DubiousMove,
_ => Self::Unknown,
}
}
}

impl PgnAnnotation {
pub fn pgn_annotation_nag_from_string(str: &str, consumed: &mut usize) -> Self {
let bytes = str.as_bytes();
let mut annotation = PgnAnnotation::Unknown;
let mut cursor = 0usize;

if bytes.get(cursor).copied() != Some(b'$') {
return annotation;
}

while bytes.get(cursor).copied() == Some(b'$') {
cursor += 1;
if !bytes
.get(cursor)
.map(|b| b.is_ascii_digit())
.unwrap_or(false)
{
break;
}

let start = cursor;
while bytes
.get(cursor)
.map(|b| b.is_ascii_digit())
.unwrap_or(false)
{
cursor += 1;
}

let num = str[start..cursor].parse::<i16>().unwrap_or(-1) as i8;
annotation = PgnAnnotation::from(num);
pgn_cursor_skip_whitespace(str, &mut cursor);
}

pgn_cursor_revisit_whitespace(str, &mut cursor);
*consumed += cursor;
annotation
}

pub fn pgn_annotation_from_string(str: &str, consumed: &mut usize) -> Self {
let bytes = str.as_bytes();
let mut annotation = PgnAnnotation::Unknown;

if bytes.is_empty() {
return annotation;
}

if bytes.len() >= 2 {
match (bytes[0], bytes[1]) {
(b'!', b'!') => {
*consumed += 2;
return PgnAnnotation::BrilliantMove;
}
(b'!', b'?') => {
*consumed += 2;
return PgnAnnotation::InterestingMove;
}
(b'?', b'!') => {
*consumed += 2;
return PgnAnnotation::DubiousMove;
}
(b'?', b'?') => {
*consumed += 2;
return PgnAnnotation::Blunder;
}
_ => {}
}
}

if bytes[0] == b'!' {
*consumed += 1;
annotation = PgnAnnotation::GoodMove;
} else if bytes[0] == b'?' {
*consumed += 1;
annotation = PgnAnnotation::Mistake;
}

annotation
}
}

impl Display for PgnAnnotation {
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
match self {
PgnAnnotation::Unknown => write!(f, ""),
PgnAnnotation::Null => write!(f, "$0"),
PgnAnnotation::GoodMove => write!(f, "!"),
PgnAnnotation::Mistake => write!(f, "?"),
PgnAnnotation::BrilliantMove => write!(f, "!!"),
PgnAnnotation::Blunder => write!(f, "??"),
PgnAnnotation::InterestingMove => write!(f, "!?"),
PgnAnnotation::DubiousMove => write!(f, "?!"),
}
}
}

impl From<&str> for PgnAnnotation {
fn from(s: &str) -> Self {
let mut consumed = 0;
Self::pgn_annotation_from_string(s, &mut consumed)
}
}
