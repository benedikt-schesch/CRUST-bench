
pub struct Heap<T> {
data: Vec<T>,
comparator: Box<dyn Fn(&T, &T) -> bool>,
}
impl<T> Heap<T> {
pub fn new(init_size: usize, comparator: impl Fn(&T, &T) -> bool + 'static) -> Self {
Self {
data: Vec::with_capacity(init_size),
comparator: Box::new(comparator),
}
}
pub fn is_empty(&self) -> bool {
self.data.is_empty()
}
pub fn find_min(&self) -> Option<&T> {
self.data.get(0)
}
pub fn insert(&mut self, element: T) {
self.data.push(element);
let mut son = self.data.len() - 1;
while son > 0 {
let parent = (son - 1) / 2;
if !(self.comparator)(&self.data[son], &self.data[parent]) {
break;
}
self.data.swap(parent, son);
son = parent;
}
}
pub fn delete_min(&mut self) -> Option<T> {
if self.data.is_empty() {
return None;
}
let min = self.data.swap_remove(0);
let mut parent = 0usize;
loop {
let son1 = 2 * parent + 1;
let son2 = 2 * parent + 2;
if son1 >= self.data.len() {
break;
}
let son = if son2 < self.data.len() {
if (self.comparator)(&self.data[son1], &self.data[son2]) {
son1
} else {
son2
}
} else {
son1
};
if (self.comparator)(&self.data[son], &self.data[parent]) {
self.data.swap(parent, son);
}
parent = son;
}
Some(min)
}
}
