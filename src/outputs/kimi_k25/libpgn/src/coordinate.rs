pub const UNKNOWN: i32 = -1;

#[derive(Debug, Clone, PartialEq)]
pub struct PgnCoordinate {
pub file: Option<char>,
pub rank: Option<i32>,
}

pub fn pgn_coordinate_file_as_index(file: char) -> i32 {
if file >= 'a' && file <= 'h' {
(file as u8 - b'a') as i32
} else {
UNKNOWN
}
}
