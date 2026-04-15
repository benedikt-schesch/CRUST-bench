pub mod quadtree {
use std::cell::RefCell;

#[derive(Default, Clone)]
pub struct QuadtreePoint {
pub x: f64,
pub y: f64,
}

impl QuadtreePoint {
pub fn quadtree_point_new(x: f64, y: f64) -> QuadtreePoint {
QuadtreePoint { x, y }
}

pub fn quadtree_point_free(&self) {}
}

#[derive(Default, Clone)]
pub struct QuadtreeBounds {
pub nw: Option<Box<QuadtreePoint>>,
pub se: Option<Box<QuadtreePoint>>,
pub width: f64,
pub height: f64,
}

impl QuadtreeBounds {
pub fn quadtree_bounds_new() -> QuadtreeBounds {
QuadtreeBounds {
nw: Some(Box::new(QuadtreePoint::quadtree_point_new(
f64::INFINITY,
f64::NEG_INFINITY,
))),
se: Some(Box::new(QuadtreePoint::quadtree_point_new(
f64::NEG_INFINITY,
f64::INFINITY,
))),
width: 0.0,
height: 0.0,
}
}

pub fn quadtree_bounds_extend(&mut self, x: f64, y: f64) {
let mut min_x = x;
let mut max_x = x;
let mut min_y = y;
let mut max_y = y;

if let Some(nw) = &self.nw {
if nw.x.is_finite() {
min_x = min_x.min(nw.x);
}
if nw.y.is_finite() {
max_y = max_y.max(nw.y);
}
}

if let Some(se) = &self.se {
if se.x.is_finite() {
max_x = max_x.max(se.x);
}
if se.y.is_finite() {
min_y = min_y.min(se.y);
}
}

self.nw = Some(Box::new(QuadtreePoint::quadtree_point_new(min_x, max_y)));
self.se = Some(Box::new(QuadtreePoint::quadtree_point_new(max_x, min_y)));
self.width = max_x - min_x;
self.height = max_y - min_y;
}

pub fn quadtree_bounds_free(&self) {}
}

#[derive(Default, Clone)]
pub struct QuadtreeNode<T> {
pub ne: Option<Box<QuadtreeNode<T>>>,
pub nw: Option<Box<QuadtreeNode<T>>>,
pub se: Option<Box<QuadtreeNode<T>>>,
pub sw: Option<Box<QuadtreeNode<T>>>,
pub bounds: Option<Box<QuadtreeBounds>>,
pub point: Option<Box<QuadtreePoint>>,
pub key: Option<T>,
}

impl<T> QuadtreeNode<T> {
pub fn node_contains_(&mut self, point: Option<Box<QuadtreePoint>>) {
let _ = point;
}

pub fn get_quadrant_(&mut self, point: Option<Box<QuadtreePoint>>) {
let _ = point;
}

pub fn find_(&mut self, x: f64, y: f64) {
let _ = (x, y);
}

pub fn quadtree_node_new() -> QuadtreeNode<T> {
QuadtreeNode {
ne: None,
nw: None,
se: None,
sw: None,
bounds: None,
point: Some(Box::new(QuadtreePoint::default())),
key: None,
}
}

pub fn quadtree_node_free(&self, value_free: Option<fn(Option<T>)>) {
let _ = value_free;
}

pub fn quadtree_node_ispointer(&self) -> bool {
self.point.is_some()
|| self.nw.is_some()
|| self.ne.is_some()
|| self.sw.is_some()
|| self.se.is_some()
}

pub fn quadtree_node_isempty(&self) -> bool {
self.nw.is_none() && self.ne.is_none() && self.sw.is_none() && self.se.is_none()
}

pub fn quadtree_node_isleaf(&self) -> bool {
self.point.is_some()
}

pub fn quadtree_node_reset(&self, value_free: Option<fn(Option<T>)>) {
let _ = value_free;
}

pub fn quadtree_node_with_bounds(
minx: f64,
miny: f64,
maxx: f64,
maxy: f64,
) -> QuadtreeNode<T> {
let min_x = minx.min(maxx);
let max_x = minx.max(maxx);
let min_y = miny.min(maxy);
let max_y = miny.max(maxy);

QuadtreeNode {
ne: None,
nw: None,
se: None,
sw: None,
bounds: Some(Box::new(QuadtreeBounds {
nw: Some(Box::new(QuadtreePoint::quadtree_point_new(min_x, max_y))),
se: Some(Box::new(QuadtreePoint::quadtree_point_new(max_x, min_y))),
width: (max_x - min_x).abs(),
height: (max_y - min_y).abs(),
})),
point: None,
key: None,
}
}
}

#[derive(Default)]
pub struct Quadtree<T> {
pub root: Option<Box<QuadtreeNode<T>>>,
pub key_free: Option<fn(Option<T>)>,
pub length: u32,
}

impl<T> Quadtree<T> {
pub fn split_node_(&mut self, node: Option<Box<QuadtreeNode<T>>>) {
let _ = node;
}

pub fn insert_(
&mut self,
tree: Option<Box<QuadtreeNode<T>>>,
point: Option<Box<QuadtreePoint>>,
key: Option<T>,
) {
let _ = (tree, point, key);
}

pub fn quadtree_new(minx: f64, miny: f64, maxx: f64, maxy: f64) -> Quadtree<T> {
Quadtree {
root: Some(Box::new(QuadtreeNode::quadtree_node_with_bounds(
minx, miny, maxx, maxy,
))),
key_free: None,
length: 0,
}
}

pub fn quadtree_free(&mut self) {
self.root = None;
self.length = 0;
}

pub fn quadtree_search(&self, x: f64, y: f64) -> &mut Option<Box<QuadtreePoint>> {
let found = self.find_ref(x, y).cloned().map(Box::new);
Box::leak(Box::new(found))
}

pub fn quadtree_insert(&mut self, x: f64, y: f64, key: Option<T>) -> bool
where
T: Clone,
{
self.quadtree_insert_mut(x, y, key)
}

pub fn quadtree_walk(
&self,
descent: fn(&mut Option<Box<QuadtreeNode<T>>>),
ascent: fn(&mut Option<Box<QuadtreeNode<T>>>),
) where
T: Clone,
{
fn walk_node<T>(
node: &Option<Box<QuadtreeNode<T>>>,
descent: fn(&mut Option<Box<QuadtreeNode<T>>>),
ascent: fn(&mut Option<Box<QuadtreeNode<T>>>),
) where
T: Clone,
{
if let Some(n) = node {
let mut current = Some(Box::new((**n).clone()));
descent(&mut current);
walk_node(&n.nw, descent, ascent);
walk_node(&n.ne, descent, ascent);
walk_node(&n.sw, descent, ascent);
walk_node(&n.se, descent, ascent);
ascent(&mut current);
}
}

walk_node(&self.root, descent, ascent);
}

fn node_contains(node: &QuadtreeNode<T>, p: &QuadtreePoint) -> bool {
if let Some(bounds) = &node.bounds {
if let (Some(nw), Some(se)) = (&bounds.nw, &bounds.se) {
return nw.x <= p.x && nw.y >= p.y && se.x >= p.x && se.y <= p.y;
}
}
false
}

fn get_quadrant_ref<'a>(
node: &'a QuadtreeNode<T>,
p: &QuadtreePoint,
) -> Option<&'a QuadtreeNode<T>> {
if let Some(nw) = &node.nw {
if Self::node_contains(nw, p) {
return Some(nw);
}
}
if let Some(ne) = &node.ne {
if Self::node_contains(ne, p) {
return Some(ne);
}
}
if let Some(sw) = &node.sw {
if Self::node_contains(sw, p) {
return Some(sw);
}
}
if let Some(se) = &node.se {
if Self::node_contains(se, p) {
return Some(se);
}
}
None
}

fn find_in_node<'a>(node: &'a QuadtreeNode<T>, x: f64, y: f64) -> Option<&'a QuadtreePoint> {
if node.quadtree_node_isleaf() {
if let Some(point) = &node.point {
if point.x == x && point.y == y {
return Some(point);
}
}
}

if let Some(next) = Self::get_quadrant_ref(node, &QuadtreePoint { x, y }) {
return Self::find_in_node(next, x, y);
}

None
}

fn find_ref(&self, x: f64, y: f64) -> Option<&QuadtreePoint> {
self.root
.as_ref()
.and_then(|root| Self::find_in_node(root, x, y))
}
}

impl<T: Clone> Quadtree<T> {
fn split_node_internal(node: &mut QuadtreeNode<T>) -> bool {
let bounds = match &node.bounds {
Some(b) => b,
None => return false,
};
let nw_point = match &bounds.nw {
Some(p) => p,
None => return false,
};

let x = nw_point.x;
let y = nw_point.y;
let hw = bounds.width / 2.0;
let hh = bounds.height / 2.0;

let nw = QuadtreeNode::quadtree_node_with_bounds(x, y - hh, x + hw, y);
let ne = QuadtreeNode::quadtree_node_with_bounds(x + hw, y - hh, x + hw * 2.0, y);
let sw = QuadtreeNode::quadtree_node_with_bounds(x, y - hh * 2.0, x + hw, y - hh);
let se =
QuadtreeNode::quadtree_node_with_bounds(x + hw, y - hh * 2.0, x + hw * 2.0, y - hh);

node.nw = Some(Box::new(nw));
node.ne = Some(Box::new(ne));
node.sw = Some(Box::new(sw));
node.se = Some(Box::new(se));
node.point = None;
node.key = None;
true
}

fn insert_internal(
root: &mut QuadtreeNode<T>,
point: Option<Box<QuadtreePoint>>,
key: Option<T>,
) -> i32 {
if root.quadtree_node_isleaf() {
if let (Some(root_p), Some(new_p)) = (&root.point, &point) {
if root_p.x == new_p.x && root_p.y == new_p.y {
root.point = point;
root.key = key;
2
} else {
let old_point = root.point.take();
let old_key = root.key.take();

if !Self::split_node_internal(root) {
root.point = old_point;
root.key = old_key;
return 0;
}

let reinsert_status = Self::insert_internal(root, old_point, old_key);
if reinsert_status == 0 {
return 0;
}

Self::insert_internal(root, point, key)
}
} else {
0
}
} else if root.quadtree_node_ispointer() {
let p = match &point {
Some(p) => p,
None => return 0,
};

if let Some(nw) = root.nw.as_mut() {
if Quadtree::<T>::node_contains(nw, p) {
return Self::insert_internal(nw, point, key);
}
}
if let Some(ne) = root.ne.as_mut() {
if Quadtree::<T>::node_contains(ne, p) {
return Self::insert_internal(ne, point, key);
}
}
if let Some(sw) = root.sw.as_mut() {
if Quadtree::<T>::node_contains(sw, p) {
return Self::insert_internal(sw, point, key);
}
}
if let Some(se) = root.se.as_mut() {
if Quadtree::<T>::node_contains(se, p) {
return Self::insert_internal(se, point, key);
}
}
0
} else {
root.point = point;
root.key = key;
1
}
}

pub fn quadtree_insert_mut(&mut self, x: f64, y: f64, key: Option<T>) -> bool {
let point = Box::new(QuadtreePoint::quadtree_point_new(x, y));

let contains = match &self.root {
Some(root) => Self::node_contains(root, &point),
None => false,
};
if !contains {
return false;
}

let status = match self.root.as_mut() {
Some(mut_root) => Self::insert_internal(mut_root, Some(point), key),
None => 0,
};

if status == 1 {
self.length += 1;
}

status != 0
}
}

thread_local! {
static ELISION_STATE: RefCell<()> = const { RefCell::new(()) };
}
}

pub fn elision_<T>(key: Option<Box<T>>) {
let _ = key;
}
