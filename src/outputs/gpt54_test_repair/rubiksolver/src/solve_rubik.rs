use crate::hash::Hash;
use crate::heap::Heap;
use crate::rubik_model::cube_compare_equal;
use crate::rubik_model::new_cube_rotate_face;
use crate::rubik_model::populate_specific;
use crate::rubik_model::Color;
use crate::rubik_model::ECube;
use crate::rubik_model::Rotation;
fn ecube_compare_equal(a: &ECube, b: &ECube) -> bool {
if a.entropy != b.entropy {
return false;
}
cube_compare_equal(&a.cube, &b.cube)
}
fn main() {
let cube_data = [
[
Color::Red,
Color::Red,
Color::Red,
Color::Red,
Color::Red,
Color::Red,
Color::Red,
Color::Red,
],
[
Color::Green,
Color::Yellow,
Color::Blue,
Color::Orange,
Color::Orange,
Color::Yellow,
Color::Green,
Color::Green,
],
[
Color::Blue,
Color::Blue,
Color::Blue,
Color::Orange,
Color::Orange,
Color::Green,
Color::Orange,
Color::Blue,
],
[
Color::White,
Color::White,
Color::White,
Color::Green,
Color::Green,
Color::White,
Color::Yellow,
Color::Blue,
],
[
Color::Orange,
Color::Green,
Color::Blue,
Color::Yellow,
Color::Yellow,
Color::Yellow,
Color::Yellow,
Color::Blue,
],
[
Color::Yellow,
Color::Orange,
Color::White,
Color::White,
Color::White,
Color::White,
Color::Green,
Color::Orange,
],
];
let mut count_explored = 0i32;
let mut count_unexplored = 1i32;
let cube = populate_specific(&cube_data);
let ecube = ECube::new(cube);
let mut unexplored_hash = Hash::new(33_554_432, |e: &ECube| e.hash);
let mut unexplored = Heap::new(5000, |a: &ECube, b: &ECube| a.entropy < b.entropy);
unexplored.insert(ecube.clone());
unexplored_hash.insert(ecube, ecube_compare_equal);
let mut explored_hash = Hash::new(33_554_432, |e: &ECube| e.hash);
while !unexplored.is_empty() {
println!("Unexplored nodes = {:6}", count_unexplored);
let x = unexplored.delete_min().expect("heap unexpectedly empty");
unexplored_hash.delete(&x, ecube_compare_equal);
count_unexplored -= 1;
println!("Entropy of x: {}", x.entropy);
if x.entropy == 0 {
println!("Found goal.");
return;
}
explored_hash.insert(x.clone(), ecube_compare_equal);
count_explored += 1;
println!("Explored nodes = {:6}", count_explored);
for face in 0..=5 {
for dir in 0..=1 {
let face_color = Color::from_usize(face);
let rotation = if dir == 0 {
Rotation::CW
} else {
Rotation::CCW
};
let new_cube = new_cube_rotate_face(&x.cube, face_color, rotation);
let y = ECube::new(new_cube);
if explored_hash.element_exists(&y, ecube_compare_equal) {
continue;
}
if !unexplored_hash.element_exists(&y, ecube_compare_equal) {
unexplored.insert(y.clone());
unexplored_hash.insert(y, ecube_compare_equal);
count_unexplored += 1;
}
}
}
}
}
