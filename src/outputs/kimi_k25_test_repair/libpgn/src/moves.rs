use crate::annotation::PgnAnnotation;
use crate::check::PgnCheck;
use crate::coordinate::PgnCoordinate;
use crate::piece::PgnPiece;
use crate::comments::PgnCommentPosition;

pub const PGN_CASTLING_KINGSIDE: &str = "O-O";
pub const PGN_CASTLING_QUEENSIDE: &str = "O-O-O";

#[derive(Debug, Clone, PartialEq)]
pub struct PgnComments {
pub values: Vec<PgnComment>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PgnAlternatives {
pub values: Vec<PgnMoves>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PgnComment {
pub value: String,
pub position: PgnCommentPosition,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PgnMove {
pub from: PgnCoordinate,
pub to: PgnCoordinate,
pub dest: PgnCoordinate,
pub piece: PgnPiece,
pub promotion: Option<PgnPiece>,
pub promoted_to: PgnPiece,
pub check: PgnCheck,
pub annotation: PgnAnnotation,
pub notation: String,
pub captures: bool,
pub castles: &'static str,
pub en_passant: bool,
pub comments: Option<PgnComments>,
pub alternatives: Option<PgnAlternatives>,
}

impl PgnMove {
pub fn new() -> Self {
Self {
from: PgnCoordinate { file: None, rank: None },
to: PgnCoordinate { file: None, rank: None },
dest: PgnCoordinate { file: None, rank: None },
piece: PgnPiece::None,
promotion: None,
promoted_to: PgnPiece::None,
check: PgnCheck::None,
annotation: PgnAnnotation::None,
notation: String::new(),
captures: false,
castles: "",
en_passant: false,
comments: None,
alternatives: None,
}
}
}

impl Default for PgnMove {
fn default() -> Self {
Self::new()
}
}

impl std::fmt::Display for PgnMove {
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
write!(f, "{}", self.notation)
}
}

impl From<&str> for PgnMove {
fn from(s: &str) -> Self {
let mut mv = PgnMove::new();
mv.notation = s.to_string();
let s = s.trim();
if s.is_empty() {
return mv;
}

if s.starts_with("O-O-O") {
mv.castles = PGN_CASTLING_QUEENSIDE;
mv.piece = PgnPiece::King;
mv.dest = PgnCoordinate { file: Some('c'), rank: Some(1) };
let rest = &s[5..];
parse_check_and_annotation(&mut mv, rest);
return mv;
} else if s.starts_with("O-O") {
mv.castles = PGN_CASTLING_KINGSIDE;
mv.piece = PgnPiece::King;
mv.dest = PgnCoordinate { file: Some('g'), rank: Some(1) };
let rest = &s[3..];
parse_check_and_annotation(&mut mv, rest);
return mv;
}

let mut chars = s.chars().peekable();
let first = *chars.peek().unwrap();
match first {
'K' => { mv.piece = PgnPiece::King; chars.next(); },
'Q' => { mv.piece = PgnPiece::Queen; chars.next(); },
'R' => { mv.piece = PgnPiece::Rook; chars.next(); },
'B' => { mv.piece = PgnPiece::Bishop; chars.next(); },
'N' => { mv.piece = PgnPiece::Knight; chars.next(); },
_ => { mv.piece = PgnPiece::Pawn; }
}

let mut disambiguation = String::new();
let mut found_capture = false;
while let Some(&ch) = chars.peek() {
if ch == 'x' {
found_capture = true;
chars.next();
break;
} else if ch.is_ascii_digit() || (ch >= 'a' && ch <= 'h') {
disambiguation.push(ch);
chars.next();
} else {
break;
}
}

if disambiguation.len() == 1 {
if disambiguation.chars().next().unwrap().is_ascii_alphabetic() {
mv.from.file = Some(disambiguation.chars().next().unwrap());
} else {
mv.from.rank = Some(disambiguation.parse().unwrap());
}
} else if disambiguation.len() == 2 {
mv.from.file = Some(disambiguation.chars().next().unwrap());
mv.from.rank = Some(disambiguation.chars().nth(1).unwrap().to_digit(10).unwrap() as i32);
}

mv.captures = found_capture || s.contains('x');

let mut dest_file = None;
let mut dest_rank = None;
while let Some(&ch) = chars.peek() {
if ch >= 'a' && ch <= 'h' {
dest_file = Some(ch);
chars.next();
} else if ch.is_ascii_digit() {
dest_rank = Some(ch.to_digit(10).unwrap() as i32);
chars.next();
} else {
break;
}
}

mv.dest.file = dest_file;
mv.dest.rank = dest_rank;
mv.to = mv.dest.clone();

if let Some(&ch) = chars.peek() {
if ch == '=' {
chars.next();
if let Some(&p) = chars.peek() {
match p {
'Q' => { mv.promoted_to = PgnPiece::Queen; mv.promotion = Some(PgnPiece::Queen); chars.next(); },
'R' => { mv.promoted_to = PgnPiece::Rook; mv.promotion = Some(PgnPiece::Rook); chars.next(); },
'B' => { mv.promoted_to = PgnPiece::Bishop; mv.promotion = Some(PgnPiece::Bishop); chars.next(); },
'N' => { mv.promoted_to = PgnPiece::Knight; mv.promotion = Some(PgnPiece::Knight); chars.next(); },
_ => {}
}
}
}
}

if s.contains("e.p.") || s.contains("e.p") {
mv.en_passant = true;
}

let remaining: String = chars.collect();
parse_check_and_annotation(&mut mv, &remaining);

mv
}
}

fn parse_check_and_annotation(mv: &mut PgnMove, s: &str) {
let s = s.trim();
if s.contains('#') {
mv.check = PgnCheck::Mate;
} else if s.contains('+') {
if s.contains("++") {
mv.check = PgnCheck::Double;
} else {
mv.check = PgnCheck::Single;
}
}

if s.contains("!!") {
mv.annotation = PgnAnnotation::BrilliantMove;
} else if s.contains("??") {
mv.annotation = PgnAnnotation::Blunder;
} else if s.contains("!?") {
mv.annotation = PgnAnnotation::InterestingMove;
} else if s.contains("?!") {
mv.annotation = PgnAnnotation::DubiousMove;
} else if s.contains('!') {
mv.annotation = PgnAnnotation::GoodMove;
} else if s.contains('?') {
mv.annotation = PgnAnnotation::Mistake;
}

if let Some(pos) = s.find('$') {
let num_str: String = s[pos+1..].chars().take_while(|c| c.is_ascii_digit()).collect();
if let Ok(n) = num_str.parse::<i32>() {
mv.annotation = PgnAnnotation::from(n);
}
}
}

#[derive(Debug, Clone, PartialEq)]
pub struct PgnMovePair {
pub white: PgnMove,
pub black: PgnMove,
pub alternatives: Option<PgnAlternatives>,
pub comments: Option<PgnComments>,
}

impl PgnMovePair {
pub fn new() -> Self {
Self {
white: PgnMove::new(),
black: PgnMove::new(),
alternatives: None,
comments: None,
}
}
}

impl Default for PgnMovePair {
fn default() -> Self {
Self::new()
}
}

#[derive(Debug, Clone, PartialEq)]
pub struct PgnMoves {
pub moves: Vec<PgnMove>,
pub values: Vec<PgnMovePair>,
}

impl PgnMoves {
pub fn new() -> Self {
Self {
moves: Vec::new(),
values: Vec::new(),
}
}
}

impl Default for PgnMoves {
fn default() -> Self {
Self::new()
}
}

impl From<&str> for PgnMoves {
fn from(s: &str) -> Self {
let mut moves = PgnMoves::new();
let mut current_pair = PgnMovePair::new();
let mut move_number = 0;
let mut is_white = true;

let tokens = tokenize(s);
let mut i = 0;
while i < tokens.len() {
let token = &tokens[i];

if is_move_number(token) {
i += 1;
continue;
}

if token.starts_with('{') {
i += 1;
continue;
}

if token == "(" {
let mut depth = 1;
let mut j = i + 1;
while j < tokens.len() && depth > 0 {
if tokens[j] == "(" { depth += 1; }
else if tokens[j] == ")" { depth -= 1; }
j += 1;
}
let variation = tokens[i+1..j-1].join(" ");
let var_moves = PgnMoves::from(variation.as_str());
if is_white {
current_pair.alternatives = Some(PgnAlternatives { values: vec![var_moves] });
} else {
current_pair.alternatives = Some(PgnAlternatives { values: vec![var_moves] });
}
i = j;
continue;
}

if token == ")" {
i += 1;
continue;
}

if !token.is_empty() && !token.starts_with('$') {
let move_str = if token.contains('$') {
let parts: Vec<&str> = token.split('$').collect();
parts[0].to_string()
} else {
token.to_string()
};
let mv = PgnMove::from(move_str.as_str());
if is_white {
current_pair.white = mv;
is_white = false;
} else {
current_pair.black = mv;
moves.values.push(current_pair);
current_pair = PgnMovePair::new();
is_white = true;
move_number += 1;
}
}

if token.starts_with('$') {
let num: i32 = token[1..].parse().unwrap_or(0);
let annotation = PgnAnnotation::from(num);
if is_white {
// We are waiting for white's move, meaning we just completed a black move
// Apply annotation to the last pair's black move
if let Some(last_pair) = moves.values.last_mut() {
last_pair.black.annotation = annotation;
last_pair.black.notation.push(' ');
last_pair.black.notation.push_str(token);
}
} else {
// We are waiting for black's move, meaning we just completed a white move
current_pair.white.annotation = annotation;
current_pair.white.notation.push(' ');
current_pair.white.notation.push_str(token);
}
}

i += 1;
}

if !is_white {
moves.values.push(current_pair);
}

moves
}
}

fn tokenize(s: &str) -> Vec<String> {
let mut tokens = Vec::new();
let mut current = String::new();
let mut in_comment = false;
let mut in_variation = false;

for ch in s.chars() {
if ch == '{' {
if !current.is_empty() {
tokens.push(current.trim().to_string());
current.clear();
}
in_comment = true;
current.push(ch);
} else if ch == '}' {
current.push(ch);
tokens.push(current.trim().to_string());
current.clear();
in_comment = false;
} else if in_comment {
current.push(ch);
} else if ch == '(' {
if !current.is_empty() {
tokens.push(current.trim().to_string());
current.clear();
}
tokens.push("(".to_string());
} else if ch == ')' {
if !current.is_empty() {
tokens.push(current.trim().to_string());
current.clear();
}
tokens.push(")".to_string());
} else if ch.is_whitespace() {
if !current.is_empty() {
tokens.push(current.trim().to_string());
current.clear();
}
} else {
current.push(ch);
}
}

if !current.is_empty() {
tokens.push(current.trim().to_string());
}

tokens
}

fn is_move_number(s: &str) -> bool {
s.chars().all(|c| c.is_ascii_digit() || c == '.')
}
