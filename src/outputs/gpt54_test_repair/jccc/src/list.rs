use std::any::Any;
#[derive(Debug)]
pub struct ListBlock {
pub array: Vec<Box<dyn Any>>,
pub size: i32,
pub full: i32,
pub next: Option<Box<ListBlock>>,
}
#[derive(Debug)]
pub struct List {
pub head: Option<Box<ListBlock>>,
pub tail: Option<*mut ListBlock>,
pub blocksize: i32,
}
pub fn lget_element(l: &mut List, index: i32) -> Option<&mut Box<dyn Any>> {
if index < 0 {
return None;
}
let mut i = index;
let mut cur = l.head.as_mut()?;
loop {
if i < cur.size {
if i >= cur.full {
return None;
}
return cur.array.get_mut(i as usize);
}
i -= cur.size;
match cur.next.as_mut() {
Some(next) => cur = next,
None => return None,
}
}
}
pub fn destroy_list(l: &mut List) -> i32 {
l.head = None;
l.tail = None;
0
}
pub fn ladd_element(l: &mut List, element: Box<dyn Any>) -> i32 {
if l.head.is_none() {
l.head = Some(new_block(l));
}
let mut cur = l.head.as_mut().expect("head exists");
loop {
if cur.full < cur.size {
cur.array.push(element);
cur.full += 1;
return 0;
}
if cur.next.is_none() {
break;
}
let next = cur.next.as_mut().expect("next exists");
cur = next;
}
let mut new_blk = Box::new(ListBlock {
array: Vec::with_capacity(l.blocksize.max(0) as usize),
size: l.blocksize,
full: 0,
next: None,
});
new_blk.array.push(element);
new_blk.full = 1;
cur.next = Some(new_blk);
0
}
pub fn new_block(l: &mut List) -> Box<ListBlock> {
Box::new(ListBlock {
array: Vec::with_capacity(l.blocksize.max(0) as usize),
size: l.blocksize,
full: 0,
next: None,
})
}
pub fn literate(l: &mut List, func: fn(&mut Box<dyn Any>) -> i32) -> i32 {
let mut acc = 0;
let mut cur = l.head.as_mut();
while let Some(lb) = cur {
for i in 0..(lb.full as usize) {
acc += func(&mut lb.array[i]);
}
cur = lb.next.as_mut();
}
acc
}
pub fn lfind_index(_l: &mut List, _lb: &mut Option<Box<ListBlock>>, i: &mut i32) -> i32 {
if *i < 0 {
return -1;
}
0
}
pub fn create_list(blocksize: i32) -> List {
List {
head: None,
tail: None,
blocksize,
}
}
pub fn lset_element(l: &mut List, index: i32, value: Box<dyn Any>) -> i32 {
if index < 0 {
return -1;
}
let mut i = index;
let mut cur = match l.head.as_mut() {
Some(h) => h,
None => return -1,
};
loop {
if i < cur.size {
if i >= cur.full {
return -1;
}
let idx = i as usize;
if idx < cur.array.len() {
cur.array[idx] = value;
return 0;
}
return -1;
}
i -= cur.size;
match cur.next.as_mut() {
Some(next) => cur = next,
None => return -1,
}
}
}
