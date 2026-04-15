// Generated Rust Code
use crate::libutf_utf::*;

#[derive(Clone, Debug)]
pub struct Utf8String<'a> {
pub data: &'a [Utf8],
pub len: usize,
pub cap: usize,
}

#[derive(Clone, Copy, Debug)]
pub struct Utf8StringView<'a> {
pub data: &'a [Utf8],
pub len: usize,
}

impl Utf8String<'static> {
pub fn new() -> Self {
Utf8String {
data: &[],
len: 0,
cap: 0,
}
}

pub fn init(&mut self) {
self.data = &[];
self.len = 0;
self.cap = 0;
}

pub fn destroy(&mut self) {
self.init();
}

pub fn reserve(&mut self, len: usize) -> Result<(), ()> {
if len > self.cap {
self.cap = len;
}
Ok(())
}

pub fn shrink_to_fit(&mut self) -> Result<(), ()> {
self.cap = self.len;
Ok(())
}

pub fn clear(&mut self) {
self.len = 0;
self.data = &[];
}

pub fn is_empty(&self) -> bool {
self.len == 0
}

pub fn append(&mut self, other: &Utf8String) -> Result<(), ()> {
self.append_literal(&other.data[..other.len.min(other.data.len())])
}

pub fn append_view(&mut self, view: &Utf8StringView) -> Result<(), ()> {
self.append_literal(&view.data[..view.len.min(view.data.len())])
}

pub fn append_character(&mut self, c: Utf8) -> Result<(), ()> {
self.append_literal(&[c])
}

pub fn append_literal(&mut self, literal: &[Utf8]) -> Result<(), ()> {
let current = &self.data[..self.len.min(self.data.len())];
let mut v = Vec::with_capacity(current.len() + literal.len());
v.extend_from_slice(current);
v.extend_from_slice(literal);
let leaked: &'static [Utf8] = if v.is_empty() {
&[]
} else {
Box::leak(v.into_boxed_slice())
};
self.data = leaked;
self.len = leaked.len();
if self.cap < self.len {
self.cap = self.len;
}
Ok(())
}

pub fn prepend(&mut self, other: &Utf8String) -> Result<(), ()> {
self.prepend_literal(&other.data[..other.len.min(other.data.len())])
}

pub fn prepend_view(&mut self, view: &Utf8StringView) -> Result<(), ()> {
self.prepend_literal(&view.data[..view.len.min(view.data.len())])
}

pub fn prepend_character(&mut self, c: Utf8) -> Result<(), ()> {
self.prepend_literal(&[c])
}

pub fn prepend_literal(&mut self, literal: &[Utf8]) -> Result<(), ()> {
let current = &self.data[..self.len.min(self.data.len())];
let mut v = Vec::with_capacity(current.len() + literal.len());
v.extend_from_slice(literal);
v.extend_from_slice(current);
let leaked: &'static [Utf8] = if v.is_empty() {
&[]
} else {
Box::leak(v.into_boxed_slice())
};
self.data = leaked;
self.len = leaked.len();
if self.cap < self.len {
self.cap = self.len;
}
Ok(())
}

pub fn insert(&mut self, pos: usize, other: &Utf8String) -> Result<(), ()> {
self.insert_literal(pos, &other.data[..other.len.min(other.data.len())])
}

pub fn insert_view(&mut self, pos: usize, view: &Utf8StringView) -> Result<(), ()> {
self.insert_literal(pos, &view.data[..view.len.min(view.data.len())])
}

pub fn insert_character(&mut self, pos: usize, c: Utf8) -> Result<(), ()> {
self.insert_literal(pos, &[c])
}

pub fn insert_literal(&mut self, pos: usize, literal: &[Utf8]) -> Result<(), ()> {
if pos > self.len {
return Err(());
}
let current = &self.data[..self.len.min(self.data.len())];
let mut v = Vec::with_capacity(current.len() + literal.len());
v.extend_from_slice(&current[..pos]);
v.extend_from_slice(literal);
v.extend_from_slice(&current[pos..]);
let leaked: &'static [Utf8] = if v.is_empty() {
&[]
} else {
Box::leak(v.into_boxed_slice())
};
self.data = leaked;
self.len = leaked.len();
if self.cap < self.len {
self.cap = self.len;
}
Ok(())
}

pub fn replace(&mut self, pos: usize, len: usize, replacement: &Utf8String) -> Result<(), ()> {
self.replace_literal(
pos,
len,
&replacement.data[..replacement.len.min(replacement.data.len())],
)
}

pub fn replace_view(
&mut self,
pos: usize,
len: usize,
replacement: &Utf8StringView,
) -> Result<(), ()> {
self.replace_literal(
pos,
len,
&replacement.data[..replacement.len.min(replacement.data.len())],
)
}

pub fn replace_character(&mut self, pos: usize, len: usize, c: Utf8) -> Result<(), ()> {
self.replace_literal(pos, len, &[c])
}

pub fn replace_literal(&mut self, pos: usize, len: usize, literal: &[Utf8]) -> Result<(), ()> {
if pos > self.len {
return Err(());
}
let current = &self.data[..self.len.min(self.data.len())];
let end = pos.saturating_add(len).min(current.len());
let mut v = Vec::with_capacity(current.len() + literal.len().saturating_sub(end - pos));
v.extend_from_slice(&current[..pos]);
v.extend_from_slice(literal);
v.extend_from_slice(&current[end..]);
let leaked: &'static [Utf8] = if v.is_empty() {
&[]
} else {
Box::leak(v.into_boxed_slice())
};
self.data = leaked;
self.len = leaked.len();
if self.cap < self.len {
self.cap = self.len;
}
Ok(())
}

pub fn erase(&mut self, pos: usize, len: usize) -> Result<(), ()> {
if pos > self.len {
return Err(());
}
let current = &self.data[..self.len.min(self.data.len())];
let end = pos.saturating_add(len).min(current.len());
let mut v = Vec::with_capacity(current.len() - (end - pos));
v.extend_from_slice(&current[..pos]);
v.extend_from_slice(&current[end..]);
let leaked: &'static [Utf8] = if v.is_empty() {
&[]
} else {
Box::leak(v.into_boxed_slice())
};
self.data = leaked;
self.len = leaked.len();
self.cap = self.cap.max(self.len);
Ok(())
}

pub fn concat(&self, other: &Utf8String) -> Result<Utf8String<'static>, ()> {
self.concat_literal(&other.data[..other.len.min(other.data.len())])
}

pub fn concat_view(&self, other: &Utf8StringView) -> Result<Utf8String<'static>, ()> {
self.concat_literal(&other.data[..other.len.min(other.data.len())])
}

pub fn concat_character(&self, c: Utf8) -> Result<Utf8String<'static>, ()> {
self.concat_literal(&[c])
}

pub fn concat_literal(&self, literal: &[Utf8]) -> Result<Utf8String<'static>, ()> {
let current = &self.data[..self.len.min(self.data.len())];
let total_len = current.len() + literal.len();
let leaked: &'static [Utf8] = if total_len == 0 {
&[]
} else {
let mut v = Vec::with_capacity(total_len);
v.extend_from_slice(current);
v.extend_from_slice(literal);
Box::leak(v.into_boxed_slice())
};

Ok(Utf8String {
data: leaked,
len: total_len,
cap: total_len,
})
}

pub fn compare(&self, other: &Utf8String) -> i32 {
compare_slices(
&self.data[..self.len.min(self.data.len())],
&other.data[..other.len.min(other.data.len())],
)
}

pub fn compare_literal(&self, literal: &[Utf8]) -> i32 {
compare_slices(&self.data[..self.len.min(self.data.len())], literal)
}

pub fn substring(&self, start: usize, end: usize) -> Utf8StringView<'_> {
let current = &self.data[..self.len.min(self.data.len())];
let mut actual_end = end;
if actual_end == usize::MAX || actual_end > current.len() {
actual_end = current.len();
}
let actual_start = if start > actual_end { actual_end } else { start };
Utf8StringView {
data: &current[actual_start..actual_end],
len: actual_end - actual_start,
}
}

pub fn substring_copy(&self, start: usize, end: usize) -> Result<Utf8String<'static>, ()> {
let view = self.substring(start, end);
let leaked: &'static [Utf8] = if view.len == 0 {
&[]
} else {
Box::leak(view.data.to_vec().into_boxed_slice())
};
Ok(Utf8String {
data: leaked,
len: view.len,
cap: view.len,
})
}

pub fn index_of_character(&self, pos: usize, c: Utf8) -> Option<usize> {
let n = self.len.min(self.data.len());
(pos..n).find(|&i| self.data[i] == c)
}

pub fn last_index_of_character(&self, pos: usize, c: Utf8) -> Option<usize> {
if self.len == 0 {
return None;
}
let n = self.len.min(self.data.len());
if n == 0 {
return None;
}
let mut i = if pos == usize::MAX {
n - 1
} else if pos >= n {
return None;
} else {
pos
};
loop {
if self.data[i] == c {
return Some(i);
}
if i == 0 {
break;
}
i -= 1;
}
None
}
}

impl<'a> Utf8StringView<'a> {
pub fn is_empty(&self) -> bool {
self.len == 0
}

pub fn compare(&self, other: &Utf8StringView) -> i32 {
compare_slices(
&self.data[..self.len.min(self.data.len())],
&other.data[..other.len.min(other.data.len())],
)
}

pub fn compare_literal(&self, literal: &[Utf8]) -> i32 {
compare_slices(&self.data[..self.len.min(self.data.len())], literal)
}

pub fn substring(&self, start: usize, end: usize) -> Utf8StringView<'a> {
let current = &self.data[..self.len.min(self.data.len())];
let mut actual_end = end;
if actual_end == usize::MAX || actual_end > current.len() {
actual_end = current.len();
}
let actual_start = if start > actual_end { actual_end } else { start };
Utf8StringView {
data: &current[actual_start..actual_end],
len: actual_end - actual_start,
}
}

pub fn substring_copy(&self, start: usize, end: usize) -> Result<Utf8String<'static>, ()> {
let view = self.substring(start, end);
let leaked: &'static [Utf8] = if view.len == 0 {
&[]
} else {
Box::leak(view.data.to_vec().into_boxed_slice())
};
Ok(Utf8String {
data: leaked,
len: view.len,
cap: view.len,
})
}

pub fn index_of_character(&self, pos: usize, c: Utf8) -> Option<usize> {
let n = self.len.min(self.data.len());
(pos..n).find(|&i| self.data[i] == c)
}

pub fn last_index_of_character(&self, pos: usize, c: Utf8) -> Option<usize> {
if self.len == 0 {
return None;
}
let n = self.len.min(self.data.len());
if n == 0 {
return None;
}
let mut i = if pos == usize::MAX {
n - 1
} else if pos >= n {
return None;
} else {
pos
};
loop {
if self.data[i] == c {
return Some(i);
}
if i == 0 {
break;
}
i -= 1;
}
None
}
}

fn compare_slices(a: &[Utf8], b: &[Utf8]) -> i32 {
let min_len = a.len().min(b.len());
for i in 0..min_len {
if a[i] != b[i] {
return (a[i] as i32) - (b[i] as i32);
}
}
if a.len() < b.len() {
-1
} else if a.len() > b.len() {
1
} else {
0
}
}
