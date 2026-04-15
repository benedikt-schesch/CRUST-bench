
#[derive(Debug, Clone)]
pub struct QuadtreePoint {
pub x: f64,
pub y: f64,
}
impl QuadtreePoint {
pub fn quadtree_point_new(x: f64, y: f64) -> Self {
Self { x, y }
}
pub fn quadtree_point_free(self) {
}
}
#[derive(Debug, Clone)]
pub struct QuadtreeBounds {
pub min_x: f64,
pub min_y: f64,
pub max_x: f64,
pub max_y: f64,
pub nw: Option<QuadtreePoint>, 
pub se: Option<QuadtreePoint>, 
pub width: f64,
pub height: f64,
}
impl QuadtreeBounds {
pub fn quadtree_bounds_new() -> Self {
Self {
min_x: std::f64::INFINITY,
min_y: std::f64::INFINITY,
max_x: -std::f64::INFINITY,
max_y: -std::f64::INFINITY,
nw: Some(QuadtreePoint {
x: std::f64::INFINITY,
y: std::f64::INFINITY,
}),
se: Some(QuadtreePoint {
x: -std::f64::INFINITY,
y: -std::f64::INFINITY,
}),
width: 0.0,
height: 0.0,
}
}
pub fn quadtree_bounds_extend(&mut self, x: f64, y: f64) {
if x < self.min_x {
self.min_x = x;
}
if x > self.max_x {
self.max_x = x;
}
if y < self.min_y {
self.min_y = y;
}
if y > self.max_y {
self.max_y = y;
}
if let Some(ref mut nw) = self.nw {
if x < nw.x {
nw.x = x;
}
if y > nw.y {
nw.y = y;
}
} else {
self.nw = Some(QuadtreePoint::quadtree_point_new(x, y));
}
if let Some(ref mut se) = self.se {
if x > se.x {
se.x = x;
}
if y < se.y {
se.y = y;
}
} else {
self.se = Some(QuadtreePoint::quadtree_point_new(x, y));
}
self.width = self.max_x - self.min_x;
self.height = self.max_y - self.min_y;
}
pub fn quadtree_bounds_free(&mut self) {
*self = Self::quadtree_bounds_new();
}
}
pub struct QuadtreeNode<T> {
pub bounds: Option<QuadtreeBounds>,
pub point: Option<(QuadtreePoint, T)>,
pub children: [Option<Box<QuadtreeNode<T>>>; 4],
}
impl<T> QuadtreeNode<T> {
pub fn quadtree_node_new() -> Self {
Self {
bounds: None,
point: None,
children: [None, None, None, None],
}
}
pub fn quadtree_node_isleaf(&self) -> bool {
self.children.iter().all(|c| c.is_none())
}
pub fn quadtree_node_isempty(&self) -> bool {
self.point.is_none()
}
pub fn quadtree_node_ispointer(&self) -> bool {
!self.children.iter().all(|c| c.is_none())
}
}
pub struct Quadtree<T> {
pub root: Option<Box<QuadtreeNode<T>>>,
pub bounds: QuadtreeBounds,
pub length: usize,
}
impl<T: Clone> Quadtree<T> {
pub fn quadtree_new(min_x: f64, min_y: f64, max_x: f64, max_y: f64) -> Self {
let bounds = QuadtreeBounds {
min_x,
min_y,
max_x,
max_y,
nw: None,
se: None,
width: max_x - min_x,
height: max_y - min_y,
};
let mut root = Box::new(QuadtreeNode::quadtree_node_new());
root.bounds = Some(bounds.clone());
Self {
root: Some(root),
bounds,
length: 0,
}
}
pub fn quadtree_insert(&mut self, x: f64, y: f64, value: T) -> bool {
if x < self.bounds.min_x || x > self.bounds.max_x ||
y < self.bounds.min_y || y > self.bounds.max_y {
return false;
}
self.length += 1;
if let Some(ref mut root) = self.root {
Self::insert_recursive(root, x, y, value);
}
true
}
fn insert_recursive(node: &mut QuadtreeNode<T>, x: f64, y: f64, value: T) {
if node.quadtree_node_isleaf() {
if node.quadtree_node_isempty() {
node.point = Some((QuadtreePoint::quadtree_point_new(x, y), value));
} else {
let (old_point, old_value) = node.point.take().unwrap();
let bounds = node.bounds.as_ref().unwrap();
let min_x = bounds.min_x;
let min_y = bounds.min_y;
let max_x = bounds.max_x;
let max_y = bounds.max_y;
let mid_x = (min_x + max_x) / 2.0;
let mid_y = (min_y + max_y) / 2.0;
let mut nw = Box::new(QuadtreeNode::quadtree_node_new());
nw.bounds = Some(QuadtreeBounds {
min_x,
min_y: mid_y,
max_x: mid_x,
max_y,
nw: None,
se: None,
width: mid_x - min_x,
height: max_y - mid_y,
});
let mut ne = Box::new(QuadtreeNode::quadtree_node_new());
ne.bounds = Some(QuadtreeBounds {
min_x: mid_x,
min_y: mid_y,
max_x,
max_y,
nw: None,
se: None,
width: max_x - mid_x,
height: max_y - mid_y,
});
let mut sw = Box::new(QuadtreeNode::quadtree_node_new());
sw.bounds = Some(QuadtreeBounds {
min_x,
min_y,
max_x: mid_x,
max_y: mid_y,
nw: None,
se: None,
width: mid_x - min_x,
height: mid_y - min_y,
});
let mut se = Box::new(QuadtreeNode::quadtree_node_new());
se.bounds = Some(QuadtreeBounds {
min_x: mid_x,
min_y,
max_x,
max_y: mid_y,
nw: None,
se: None,
width: max_x - mid_x,
height: mid_y - min_y,
});
Self::insert_into_child(&mut nw, &mut ne, &mut sw, &mut se,
old_point.x, old_point.y, old_value, mid_x, mid_y);
Self::insert_into_child(&mut nw, &mut ne, &mut sw, &mut se,
x, y, value, mid_x, mid_y);
node.children[0] = Some(nw);
node.children[1] = Some(ne);
node.children[2] = Some(sw);
node.children[3] = Some(se);
}
} else {
let bounds = node.bounds.as_ref().unwrap();
let mid_x = (bounds.min_x + bounds.max_x) / 2.0;
let mid_y = (bounds.min_y + bounds.max_y) / 2.0;
let idx = if x < mid_x {
if y > mid_y { 0 } else { 2 } 
} else {
if y > mid_y { 1 } else { 3 } 
};
if let Some(ref mut child) = node.children[idx] {
Self::insert_recursive(child, x, y, value);
}
}
}
fn insert_into_child(nw: &mut QuadtreeNode<T>, ne: &mut QuadtreeNode<T>,
sw: &mut QuadtreeNode<T>, se: &mut QuadtreeNode<T>,
x: f64, y: f64, value: T, mid_x: f64, mid_y: f64) {
if x < mid_x {
if y > mid_y {
nw.point = Some((QuadtreePoint::quadtree_point_new(x, y), value));
} else {
sw.point = Some((QuadtreePoint::quadtree_point_new(x, y), value));
}
} else {
if y > mid_y {
ne.point = Some((QuadtreePoint::quadtree_point_new(x, y), value));
} else {
se.point = Some((QuadtreePoint::quadtree_point_new(x, y), value));
}
}
}
pub fn quadtree_search(&self, x: f64, y: f64) -> Option<QuadtreePoint> {
if let Some(ref root) = self.root {
Self::search_recursive(root, x, y)
} else {
None
}
}
fn search_recursive(node: &QuadtreeNode<T>, x: f64, y: f64) -> Option<QuadtreePoint> {
if let Some((ref point, _)) = node.point {
if (point.x - x).abs() < f64::EPSILON && (point.y - y).abs() < f64::EPSILON {
return Some(point.clone());
}
}
if node.quadtree_node_isleaf() {
return None;
}
let bounds = node.bounds.as_ref()?;
let mid_x = (bounds.min_x + bounds.max_x) / 2.0;
let mid_y = (bounds.min_y + bounds.max_y) / 2.0;
let idx = if x < mid_x {
if y > mid_y { 0 } else { 2 }
} else {
if y > mid_y { 1 } else { 3 }
};
if let Some(ref child) = node.children[idx] {
Self::search_recursive(child, x, y)
} else {
None
}
}
pub fn quadtree_walk<F1, F2>(&self, ascent: F1, descent: F2)
where
F1: Fn(&QuadtreeNode<T>),
F2: Fn(&QuadtreeNode<T>),
{
if let Some(ref root) = self.root {
Self::walk_recursive(root, &ascent, &descent);
}
}
fn walk_recursive<F1, F2>(node: &QuadtreeNode<T>, ascent: &F1, descent: &F2)
where
F1: Fn(&QuadtreeNode<T>),
F2: Fn(&QuadtreeNode<T>),
{
descent(node);
for child in &node.children {
if let Some(ref c) = child {
Self::walk_recursive(c, ascent, descent);
}
}
ascent(node);
}
pub fn quadtree_free(&mut self) {
self.root = None;
self.length = 0;
}
}
pub mod quadtree {
pub use super::*;
}
