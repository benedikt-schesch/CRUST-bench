
#[derive(Clone, Debug)]
pub struct Point {
pub x: f64,
pub y: f64,
pub z: f64,
}
pub struct KDTree {
points: Vec<Point>,
}
impl KDTree {
pub fn new() -> Self {
KDTree {
points: Vec::new(),
}
}
pub fn build(&mut self, x: &mut [f64], y: &mut [f64], z: &mut [f64], n: usize) {
self.points.clear();
for i in 0..n {
self.points.push(Point {
x: x[i],
y: y[i],
z: z[i],
});
}
}
pub fn search(&self, iter: &mut KDTreeIterator, x: f64, y: f64, z: f64, radius: f64) {
iter.reset();
let radius_sq = radius * radius;
for point in &self.points {
let dx = point.x - x;
let dy = point.y - y;
let dz = point.z - z;
let dist_sq = dx * dx + dy * dy + dz * dz;
if dist_sq <= radius_sq {
iter.push(point.clone());
}
}
}
pub fn search_space(&self, xmin: f64, xmax: f64, ymin: f64, ymax: f64, zmin: f64, zmax: f64) -> KDTreeIterator {
let mut iter = KDTreeIterator::new();
for point in &self.points {
if point.x >= xmin && point.x <= xmax &&
point.y >= ymin && point.y <= ymax &&
point.z >= zmin && point.z <= zmax {
iter.push(point.clone());
}
}
iter
}
}
pub struct KDTreeIterator {
points: Vec<Point>,
index: usize,
pub size: usize,
}
impl KDTreeIterator {
pub fn new() -> Self {
KDTreeIterator {
points: Vec::new(),
index: 0,
size: 0,
}
}
fn reset(&mut self) {
self.points.clear();
self.index = 0;
self.size = 0;
}
fn push(&mut self, point: Point) {
self.points.push(point);
self.size = self.points.len();
}
pub fn get_next(&mut self) -> Option<Point> {
if self.index < self.points.len() {
let point = self.points[self.index].clone();
self.index += 1;
Some(point)
} else {
None
}
}
}
