use std::cmp::Ordering;
#[derive(Debug, Clone, Copy)]
pub struct RectOutInfo {
pub x: i32,
pub y: i32,
pub packed: bool,
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
#[derive(Debug, Clone)]
pub struct Rect {
pub id: i32,
pub w: i32,
pub h: i32,
pub info: RectOutInfo,
}
impl Rect {
pub fn new(id: i32, width: i32, height: i32) -> Self {
Self {
id,
w: width,
h: height,
info: RectOutInfo::default(),
}
}
}
pub struct RectPacker;
#[derive(Debug, Clone, Copy)]
struct PackRes {
all_fit: bool,
none_fit: bool,
}
struct PackCtx<'a> {
r: &'a mut [Rect],
n: i32,
max_w: i32,
max_h: i32,
page: i32,
next: i32,
last: i32,
}
impl RectPacker {
fn compare_rect_max_side(r1: &Rect, r2: &Rect) -> Ordering {
let max2 = r2.w.max(r2.h);
let max1 = r1.w.max(r1.h);
let diff = max2 - max1;
if diff == 0 {
let min2 = r2.w.min(r2.h);
let min1 = r1.w.min(r1.h);
let diff2 = min2 - min1;
diff2.cmp(&0)
} else {
diff.cmp(&0)
}
}
fn pack_bin_tree(ctx: &mut PackCtx<'_>) -> PackRes {
let mut res = PackRes {
all_fit: true,
none_fit: true,
};
let next_idx = ctx.next as usize;
let root_w = ctx.r[next_idx].w;
let root_h = ctx.r[next_idx].h;
let mut root = BinNode::new(0, 0, root_w.min(ctx.max_w), root_h.min(ctx.max_h));
let mut contiguous = true;
let mut last = ctx.last;
let start = ctx.next;
let end = ctx.last;
for i in start..=end {
let idx = i as usize;
if !ctx.r[idx].info.packed {
let rw = ctx.r[idx].w;
let rh = ctx.r[idx].h;
if let Some((x, y)) = root.find(rw, rh).map(|n| (n.x, n.y)) {
ctx.r[idx].info.x = x;
ctx.r[idx].info.y = y;
ctx.r[idx].info.packed = true;
ctx.r[idx].info.page = ctx.page;
let _ = root.split_at(x, y, rw, rh);
res.none_fit = false;
} else {
let rect_clone = ctx.r[idx].clone();
if let Some((x, y)) = root.grow(&rect_clone, ctx.max_w, ctx.max_h).map(|n| (n.x, n.y)) {
ctx.r[idx].info.x = x;
ctx.r[idx].info.y = y;
ctx.r[idx].info.packed = true;
ctx.r[idx].info.page = ctx.page;
res.none_fit = false;
} else {
ctx.r[idx].info.packed = false;
res.all_fit = false;
contiguous = false;
last = i;
}
}
}
if contiguous {
ctx.next = i + 1;
}
}
ctx.last = last;
res
}
pub fn pack(max_w: i32, max_h: i32, paging: bool, rects: &mut [Rect]) -> bool {
if rects.is_empty() {
return true;
}
rects.sort_by(Self::compare_rect_max_side);
let n = rects.len() as i32;
for rect in rects.iter_mut() {
rect.info = RectOutInfo::default();
}
let mut ctx = PackCtx {
r: rects,
n,
max_w,
max_h,
page: 0,
next: 0,
last: n - 1,
};
let mut ok = false;
let mut all_packed = false;
while !ok {
let res = Self::pack_bin_tree(&mut ctx);
ok = res.all_fit;
all_packed = all_packed || ok;
if !paging || res.none_fit {
break;
}
ctx.page += 1;
}
all_packed
}
}
#[derive(Clone)]
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
fn find(&self, w: i32, h: i32) -> Option<&BinNode> {
if self.used {
if let Some(ref right) = self.right {
if let Some(node) = right.find(w, h) {
return Some(node);
}
}
if let Some(ref down) = self.down {
return down.find(w, h);
}
None
} else if w <= self.w && h <= self.h {
Some(self)
} else {
None
}
}
fn find_mut_at(&mut self, x: i32, y: i32) -> Option<&mut BinNode> {
if self.x == x && self.y == y {
return Some(self);
}
if let Some(ref mut right) = self.right {
if let Some(node) = right.find_mut_at(x, y) {
return Some(node);
}
}
if let Some(ref mut down) = self.down {
if let Some(node) = down.find_mut_at(x, y) {
return Some(node);
}
}
None
}
fn split_at(&mut self, x: i32, y: i32, w: i32, h: i32) -> Option<&Self> {
let node = self.find_mut_at(x, y)?;
Some(node.split(w, h))
}
fn split(&mut self, w: i32, h: i32) -> &Self {
self.used = true;
self.down = Some(Box::new(BinNode::new(
self.x,
self.y + h,
self.w,
self.h - h,
)));
self.right = Some(Box::new(BinNode::new(
self.x + w,
self.y,
self.w - w,
h,
)));
self
}
fn grow_right(&mut self, rect: &Rect, _max_w: i32, _max_h: i32) -> Option<&BinNode> {
let old = self.clone();
self.used = true;
self.x = 0;
self.y = 0;
self.w = old.w + rect.w;
self.h = old.h;
self.down = Some(Box::new(old));
self.right = Some(Box::new(BinNode::new(self.down.as_ref()?.w, 0, rect.w, self.h)));
let (x, y) = self.find(rect.w, rect.h).map(|n| (n.x, n.y))?;
self.split_at(x, y, rect.w, rect.h)
}
fn grow_down(&mut self, rect: &Rect, _max_w: i32, _max_h: i32) -> Option<&BinNode> {
let old = self.clone();
self.used = true;
self.x = 0;
self.y = 0;
self.w = old.w;
self.h = old.h + rect.h;
self.down = Some(Box::new(BinNode::new(0, old.h, old.w, rect.h)));
self.right = Some(Box::new(old));
let (x, y) = self.find(rect.w, rect.h).map(|n| (n.x, n.y))?;
self.split_at(x, y, rect.w, rect.h)
}
fn grow(&mut self, rect: &Rect, max_w: i32, max_h: i32) -> Option<&BinNode> {
let can_grow_down = rect.w <= self.w && (rect.h + self.h) <= max_h;
let can_grow_right = rect.h <= self.h && (rect.w + self.w) <= max_w;
let should_grow_right = can_grow_right && self.h >= (self.w + rect.w);
let should_grow_down = can_grow_down && self.w >= (self.h + rect.h);
if should_grow_right {
return self.grow_right(rect, max_w, max_h);
} else if should_grow_down {
return self.grow_down(rect, max_w, max_h);
}
if can_grow_right {
self.grow_right(rect, max_w, max_h)
} else if can_grow_down {
self.grow_down(rect, max_w, max_h)
} else {
None
}
}
}
