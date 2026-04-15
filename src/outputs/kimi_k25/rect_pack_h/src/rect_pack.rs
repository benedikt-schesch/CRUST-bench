use std::cmp::Ordering;

/// Output information for a packed rectangle
#[derive(Debug, Clone, Copy)]
pub struct RectOutInfo {
/// X coordinate in the packed layout
pub x: i32,
/// Y coordinate in the packed layout
pub y: i32,
/// Whether the rectangle was successfully packed
pub packed: bool,
/// The page number where the rectangle was packed (for multi-page packing)
pub page: i32,
}

impl Default for RectOutInfo {
fn default() -> Self {
Self {
x: 0,
y: 0,
packed: false,
page: 0,
}
}
}

/// A rectangle to be packed
#[derive(Debug, Clone)]
pub struct Rect {
/// Unique identifier for the rectangle
pub id: i32,
/// Width of the rectangle
pub w: i32,
/// Height of the rectangle
pub h: i32,
/// Output information after packing
pub info: RectOutInfo,
}

impl Rect {
/// Create a new rectangle with the given dimensions
pub fn new(id: i32, width: i32, height: i32) -> Self {
Self {
id,
w: width,
h: height,
info: RectOutInfo::default(),
}
}
}

/// Rectangle packer using a binary tree algorithm
pub struct RectPacker;

impl RectPacker {
/// Pack rectangles into a bin of the given maximum dimensions
///
/// # Arguments
///
/// * `max_w` - Maximum width of the packing area
/// * `max_h` - Maximum height of the packing area
/// * `paging` - Whether to allow multiple pages for packing
/// * `rects` - Mutable slice of rectangles to pack
///
/// # Returns
///
/// `true` if all rectangles were successfully packed, `false` otherwise
pub fn pack(max_w: i32, max_h: i32, paging: bool, rects: &mut [Rect]) -> bool {
if rects.is_empty() {
return true;
}

// Sort by max side descending, then min side descending
rects.sort_by(|a, b| {
let a_max = a.w.max(a.h);
let b_max = b.w.max(b.h);
let cmp = b_max.cmp(&a_max);
if cmp == Ordering::Equal {
let a_min = a.w.min(a.h);
let b_min = b.w.min(b.h);
b_min.cmp(&a_min)
} else {
cmp
}
});

// Initialize all rects
for rect in rects.iter_mut() {
rect.info = RectOutInfo::default();
}

let n = rects.len();
let mut page = 0;
let mut next = 0;
let mut last = n - 1;
let mut all_packed = false;
let mut ok = false;

while !ok {
// Create root with dimensions of rects[next], clamped to max_w/max_h
let root_w = rects[next].w.min(max_w);
let root_h = rects[next].h.min(max_h);
let mut root = BinNode::new(0, 0, root_w, root_h);

let mut res_all_fit = true;
let mut res_none_fit = true;
let mut contiguous = true;
let mut current_last = last;

for i in next..=last {
if !rects[i].info.packed {
// Try to find a node that can fit this rectangle
if let Some(node) = root.find_mut(rects[i].w, rects[i].h) {
rects[i].info.x = node.x;
rects[i].info.y = node.y;
rects[i].info.packed = true;
rects[i].info.page = page;
node.split(rects[i].w, rects[i].h);
res_none_fit = false;
} else {
// Try to grow the bin to fit this rectangle
if let Some(node) = root.grow(&rects[i], max_w, max_h) {
rects[i].info.x = node.x;
rects[i].info.y = node.y;
rects[i].info.packed = true;
rects[i].info.page = page;
// grow already splits the node
res_none_fit = false;
} else {
rects[i].info.packed = false;
res_all_fit = false;
contiguous = false;
current_last = i;
}
}
}
if contiguous {
next = i + 1;
}
}

last = current_last;
ok = res_all_fit;
all_packed = all_packed || ok;

if !paging || res_none_fit {
break;
}
page += 1;
}

all_packed
}
}

/// A node in the binary tree used for rectangle packing
struct BinNode {
x: i32,
y: i32,
w: i32,
h: i32,
used: bool,
right: Option<Box<BinNode>>,
down: Option<Box<BinNode>>,
}

impl BinNode {
/// Create a new bin node with the given dimensions
fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
Self {
x,
y,
w,
h,
used: false,
right: None,
down: None,
}
}

/// Find a node in the tree that can fit a rectangle of the given dimensions
fn find(&self, w: i32, h: i32) -> Option<&BinNode> {
if self.used {
self.right
.as_ref()
.and_then(|r| r.find(w, h))
.or_else(|| self.down.as_ref().and_then(|d| d.find(w, h)))
} else if w <= self.w && h <= self.h {
Some(self)
} else {
None
}
}

/// Find a node mutably in the tree that can fit a rectangle of the given dimensions
fn find_mut(&mut self, w: i32, h: i32) -> Option<&mut BinNode> {
if self.used {
if let Some(ref mut right) = self.right {
if let found @ Some(_) = right.find_mut(w, h) {
return found;
}
}
if let Some(ref mut down) = self.down {
return down.find_mut(w, h);
}
None
} else if w <= self.w && h <= self.h {
Some(self)
} else {
None
}
}

/// Split this node after placing a rectangle of the given dimensions
fn split(&mut self, w: i32, h: i32) -> &Self {
self.used = true;
self.down = Some(Box::new(BinNode::new(self.x, self.y + h, self.w, self.h - h)));
self.right = Some(Box::new(BinNode::new(self.x + w, self.y, self.w - w, h)));
self
}

/// Grow the bin to the right to accommodate a rectangle
fn grow_right(&mut self, rect: &Rect, _max_w: i32, _max_h: i32) -> Option<&BinNode> {
let old_w = self.w;
let old_h = self.h;
let old_used = self.used;
let old_right = self.right.take();
let old_down = self.down.take();

// Create the old node as down child
let mut old_node = BinNode {
x: 0,
y: 0,
w: old_w,
h: old_h,
used: old_used,
right: old_right,
down: old_down,
};

// Update root
self.used = true;
self.x = 0;
self.y = 0;
self.w = old_w + rect.w;
self.h = old_h;
self.down = Some(Box::new(old_node));
self.right = Some(Box::new(BinNode::new(old_w, 0, rect.w, old_h)));

// Find and split the node for this rectangle
self.find_mut(rect.w, rect.h).map(|node| node.split(rect.w, rect.h))
}

/// Grow the bin downward to accommodate a rectangle
fn grow_down(&mut self, rect: &Rect, _max_w: i32, _max_h: i32) -> Option<&BinNode> {
let old_w = self.w;
let old_h = self.h;
let old_used = self.used;
let old_right = self.right.take();
let old_down = self.down.take();

// Create the old node as right child
let mut old_node = BinNode {
x: 0,
y: 0,
w: old_w,
h: old_h,
used: old_used,
right: old_right,
down: old_down,
};

// Update root
self.used = true;
self.x = 0;
self.y = 0;
self.w = old_w;
self.h = old_h + rect.h;
self.right = Some(Box::new(old_node));
self.down = Some(Box::new(BinNode::new(0, old_h, old_w, rect.h)));

// Find and split the node for this rectangle
self.find_mut(rect.w, rect.h).map(|node| node.split(rect.w, rect.h))
}

/// Grow the bin in the optimal direction to fit a rectangle
fn grow(&mut self, rect: &Rect, max_w: i32, max_h: i32) -> Option<&BinNode> {
let can_grow_down = rect.w <= self.w && (rect.h + self.h) <= max_h;
let can_grow_right = rect.h <= self.h && (rect.w + self.w) <= max_w;
let should_grow_right = can_grow_right && (self.h >= (self.w + rect.w));
let should_grow_down = can_grow_down && (self.w >= (self.h + rect.h));

if should_grow_right {
self.grow_right(rect, max_w, max_h)
} else if should_grow_down {
self.grow_down(rect, max_w, max_h)
} else if can_grow_right {
self.grow_right(rect, max_w, max_h)
} else if can_grow_down {
self.grow_down(rect, max_w, max_h)
} else {
None
}
}
}
