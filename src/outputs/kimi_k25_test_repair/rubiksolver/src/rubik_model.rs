use std::fmt;
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
Red = 0,
Green = 1,
Blue = 2,
Orange = 3,
Yellow = 4,
White = 5,
}
impl Color {
pub fn from_usize(n: usize) -> Self {
match n {
0 => Color::Red,
1 => Color::Green,
2 => Color::Blue,
3 => Color::Orange,
4 => Color::Yellow,
5 => Color::White,
_ => panic!("Invalid color index"),
}
}
}
pub const COLOR_CODE: [char; 6] = ['R', 'G', 'B', 'O', 'Y', 'W'];
pub const TOP: [Color; 6] = [
Color::Green,
Color::Blue,
Color::Red,
Color::White,
Color::Orange,
Color::Yellow,
];
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Rotation {
CW,
CCW,
}
pub type Cube = [[Color; 8]; 6];
#[derive(Clone, Debug)]
pub struct ECube {
pub cube: Cube,
pub entropy: i32,
pub hash: u32,
}
impl ECube {
pub fn new(cube: Cube) -> Self {
let entropy = find_entropy(&cube);
let hash = cube_hash(&cube);
ECube {
cube,
entropy,
hash,
}
}
}
pub fn populate_initial() -> Cube {
let mut cube = [[Color::Red; 8]; 6];
for face in 0..6 {
let color = Color::from_usize(face);
for pos in 0..8 {
cube[face][pos] = color;
}
}
cube
}
pub fn populate_specific(data: &Cube) -> Cube {
*data
}
pub fn cube_compare_equal(c1: &Cube, c2: &Cube) -> bool {
for face in 0..6 {
for pos in 0..8 {
if c1[face][pos] != c2[face][pos] {
return false;
}
}
}
true
}
pub fn cube_hash(cube: &Cube) -> u32 {
let mut hash: u32 = 0;
for face in 0..6 {
let face_color = Color::from_usize(face);
for pos in 0..4 {
if cube[face][2 * pos] != face_color || cube[face][2 * pos + 1] != face_color {
hash += 1;
}
hash = hash << 1;
}
}
hash
}
pub fn find_entropy(cube: &Cube) -> i32 {
let mut count = 0;
for face in 0..6 {
let face_color = Color::from_usize(face);
for pos in 0..8 {
if cube[face][pos] != face_color {
count += 1;
}
}
}
count
}
fn cycle_l(color: Color) -> Color {
Color::from_usize(((color as usize) + 5) % 6)
}
fn cycle_r(color: Color) -> Color {
Color::from_usize(((color as usize) + 1) % 6)
}
pub fn rear(color: Color) -> Color {
Color::from_usize(((color as usize) + 3) % 6)
}
pub fn adjacent_left(rotating_face: Color, around: Color) -> Color {
let l_around = cycle_l(around);
let rear_face = rear(rotating_face);
if l_around == rear_face || l_around == rotating_face {
cycle_l(l_around)
} else {
l_around
}
}
pub fn adjacent_right(rotating_face: Color, around: Color) -> Color {
let r_around = cycle_r(around);
let rear_face = rear(rotating_face);
if r_around == rear_face || r_around == rotating_face {
cycle_r(r_around)
} else {
r_around
}
}
pub fn adjacent_cw(rotating_face: Color, around: Color) -> Color {
assert!(around != rear(rotating_face));
if (rotating_face as usize) % 2 == 1 {
adjacent_right(rotating_face, around)
} else {
adjacent_left(rotating_face, around)
}
}
pub fn adjacent_ccw(rotating_face: Color, around: Color) -> Color {
assert!(around != rear(rotating_face));
if (rotating_face as usize) % 2 == 1 {
adjacent_left(rotating_face, around)
} else {
adjacent_right(rotating_face, around)
}
}
pub fn rotate_face(cube: &mut Cube, face: Color, direction: Rotation) {
let g = TOP[face as usize];
let w = adjacent_cw(face, g);
let y = adjacent_cw(face, w);
let b = adjacent_cw(face, y);
let face_idx = face as usize;
let g_idx = g as usize;
let w_idx = w as usize;
let y_idx = y as usize;
let b_idx = b as usize;
if direction == Rotation::CW {
let temp0 = cube[g_idx][0];
let temp7 = cube[g_idx][7];
let temp6 = cube[g_idx][6];
cube[g_idx][0] = cube[w_idx][4];
cube[g_idx][7] = cube[w_idx][3];
cube[g_idx][6] = cube[w_idx][2];
cube[w_idx][4] = temp0;
cube[w_idx][3] = temp7;
cube[w_idx][2] = temp6;
let temp02 = cube[b_idx][2];
let temp01 = cube[b_idx][1];
let temp00 = cube[b_idx][0];
cube[b_idx][2] = cube[g_idx][0];
cube[b_idx][1] = cube[g_idx][7];
cube[b_idx][0] = cube[g_idx][6];
cube[g_idx][0] = temp02;
cube[g_idx][7] = temp01;
cube[g_idx][6] = temp00;
let temp06 = cube[y_idx][6];
let temp05 = cube[y_idx][5];
let temp04 = cube[y_idx][4];
cube[y_idx][6] = cube[b_idx][2];
cube[y_idx][5] = cube[b_idx][1];
cube[y_idx][4] = cube[b_idx][0];
cube[b_idx][2] = temp06;
cube[b_idx][1] = temp05;
cube[b_idx][0] = temp04;
let temp = cube[face_idx][0];
cube[face_idx][0] = cube[face_idx][6];
cube[face_idx][6] = cube[face_idx][4];
cube[face_idx][4] = cube[face_idx][2];
cube[face_idx][2] = temp;
let temp1 = cube[face_idx][1];
cube[face_idx][1] = cube[face_idx][7];
cube[face_idx][7] = cube[face_idx][5];
cube[face_idx][5] = cube[face_idx][3];
cube[face_idx][3] = temp1;
} else {
let temp0 = cube[g_idx][0];
let temp7 = cube[g_idx][7];
let temp6 = cube[g_idx][6];
cube[g_idx][0] = cube[b_idx][2];
cube[g_idx][7] = cube[b_idx][1];
cube[g_idx][6] = cube[b_idx][0];
cube[b_idx][2] = temp0;
cube[b_idx][1] = temp7;
cube[b_idx][0] = temp6;
let temp4 = cube[w_idx][4];
let temp3 = cube[w_idx][3];
let temp2 = cube[w_idx][2];
cube[w_idx][4] = cube[g_idx][0];
cube[w_idx][3] = cube[g_idx][7];
cube[w_idx][2] = cube[g_idx][6];
cube[g_idx][0] = temp4;
cube[g_idx][7] = temp3;
cube[g_idx][6] = temp2;
let temp06 = cube[y_idx][6];
let temp05 = cube[y_idx][5];
let temp04 = cube[y_idx][4];
cube[y_idx][6] = cube[w_idx][4];
cube[y_idx][5] = cube[w_idx][3];
cube[y_idx][4] = cube[w_idx][2];
cube[w_idx][4] = temp06;
cube[w_idx][3] = temp05;
cube[w_idx][2] = temp04;
let temp = cube[face_idx][0];
cube[face_idx][0] = cube[face_idx][2];
cube[face_idx][2] = cube[face_idx][4];
cube[face_idx][4] = cube[face_idx][6];
cube[face_idx][6] = temp;
let temp1 = cube[face_idx][1];
cube[face_idx][1] = cube[face_idx][3];
cube[face_idx][3] = cube[face_idx][5];
cube[face_idx][5] = cube[face_idx][7];
cube[face_idx][7] = temp1;
}
}
pub fn new_cube_rotate_face(cube: &Cube, face: Color, direction: Rotation) -> Cube {
let mut new_cube = *cube;
rotate_face(&mut new_cube, face, direction);
new_cube
}
pub fn print_cube(cube: &Cube) {
for face in 0..6 {
let face_color = Color::from_usize(face);
println!("{} {} {}",
COLOR_CODE[cube[face][0] as usize],
COLOR_CODE[cube[face][1] as usize],
COLOR_CODE[cube[face][2] as usize]
);
println!("{} {} {}",
COLOR_CODE[cube[face][7] as usize],
COLOR_CODE[face_color as usize],
COLOR_CODE[cube[face][3] as usize]
);
println!("{} {} {}",
COLOR_CODE[cube[face][6] as usize],
COLOR_CODE[cube[face][5] as usize],
COLOR_CODE[cube[face][4] as usize]
);
println!("-----");
}
}
