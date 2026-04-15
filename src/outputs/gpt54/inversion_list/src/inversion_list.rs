use std::fmt;

#[derive(Debug)]
pub enum InversionListError {
ValueOutOfRange(u32, u32),
Generic(String),
}

#[derive(Clone, Debug)]
pub struct InversionList {
capacity: u32,
support: u32,
pub intervals: Vec<(u32, u32)>,
}

impl InversionList {
pub fn new(capacity: u32, values: &[u32]) -> Result<Self, InversionListError> {
let mut buffer = values.to_vec();
buffer.sort_unstable();

if let Some(&max) = buffer.last() {
if max >= capacity {
return Err(InversionListError::ValueOutOfRange(max, capacity));
}
}

let mut dedup = Vec::new();
for v in buffer {
if dedup.last().copied() != Some(v) {
dedup.push(v);
}
}

let support = dedup.len() as u32;
let mut intervals = Vec::new();

if !dedup.is_empty() {
let mut start = dedup[0];
let mut end = dedup[0] + 1;
for &v in dedup.iter().skip(1) {
if v == end {
end += 1;
} else if v > end {
intervals.push((start, end));
start = v;
end = v + 1;
}
}
intervals.push((start, end));
}

Ok(Self {
capacity,
support,
intervals,
})
}

pub fn capacity(&self) -> u32 {
self.capacity
}

pub fn support(&self) -> u32 {
self.support
}

pub fn contains(&self, value: u32) -> bool {
self.intervals
.iter()
.any(|&(inf, sup)| value >= inf && value < sup)
}

pub fn clone_list(&self) -> Self {
self.clone()
}

pub fn complement(&self) -> Self {
if self.intervals.is_empty() {
return Self {
capacity: self.capacity,
support: self.capacity,
intervals: if self.capacity == 0 {
Vec::new()
} else {
vec![(0, self.capacity)]
},
};
}

let mut intervals = Vec::new();
let mut current = 0u32;

for &(inf, sup) in &self.intervals {
if current < inf {
intervals.push((current, inf));
}
current = sup;
}

if current < self.capacity {
intervals.push((current, self.capacity));
}

Self {
capacity: self.capacity,
support: self.capacity.saturating_sub(self.support),
intervals,
}
}

pub fn to_str(&self) -> String {
let mut parts = Vec::new();
for &(inf, sup) in &self.intervals {
for v in inf..sup {
parts.push(v.to_string());
}
}
format!("[{}]", parts.join(", "))
}

pub fn equal(&self, other: &Self) -> bool {
self.support == other.support && self.intervals == other.intervals
}

pub fn is_strict_subset_of(&self, other: &Self) -> bool {
if self.support >= other.support {
return false;
}

for v in self {
if !other.contains(v) {
return false;
}
}
true
}

pub fn is_subset_of(&self, other: &Self) -> bool {
self.equal(other) || self.is_strict_subset_of(other)
}

pub fn is_disjoint(&self, other: &Self) -> bool {
for v in self {
if other.contains(v) {
return false;
}
}
true
}

pub fn union(&self, other: &Self) -> Self {
let cap = self.capacity.max(other.capacity);
let mut values = Vec::new();

let max_end = self
.intervals
.last()
.map(|x| x.1)
.unwrap_or(0)
.max(other.intervals.last().map(|x| x.1).unwrap_or(0));

let start = self
.intervals
.first()
.map(|x| x.0)
.unwrap_or(u32::MAX)
.min(other.intervals.first().map(|x| x.0).unwrap_or(u32::MAX));

if start == u32::MAX {
return Self {
capacity: cap,
support: 0,
intervals: Vec::new(),
};
}

let mut i = start;
while i <= max_end {
if self.contains(i) || other.contains(i) {
values.push(i);
}
if i == u32::MAX {
break;
}
i += 1;
}

Self::new(cap, &values).unwrap_or(Self {
capacity: cap,
support: 0,
intervals: Vec::new(),
})
}

pub fn intersection(&self, other: &Self) -> Self {
let cap = self.capacity.max(other.capacity);
let mut values = Vec::new();

let mut i = 0usize;
let mut j = 0usize;

while i < self.intervals.len() && j < other.intervals.len() {
let (s1_min, s1_max) = self.intervals[i];
let (s2_min, s2_max) = other.intervals[j];
let min = s1_min.max(s2_min);
let max = s1_max.min(s2_max);

if min <= max {
for k in min..=max {
if self.contains(k) && other.contains(k) {
values.push(k);
}
}
}

if s1_max < s2_max {
i += 1;
} else if s1_max > s2_max {
j += 1;
} else {
i += 1;
j += 1;
}
}

Self::new(cap, &values).unwrap_or(Self {
capacity: cap,
support: 0,
intervals: Vec::new(),
})
}

pub fn difference(&self, other: &Self) -> Self {
let cap = self.capacity.max(other.capacity);
let mut values = Vec::new();

for v in self {
if !other.contains(v) {
values.push(v);
}
}

Self::new(cap, &values).unwrap_or(Self {
capacity: cap,
support: 0,
intervals: Vec::new(),
})
}

pub fn symmetric_difference(&self, other: &Self) -> Self {
let i = self.intersection(other);
let u = self.union(other);
u.difference(&i)
}
}

impl PartialEq for InversionList {
fn eq(&self, other: &Self) -> bool {
self.equal(other)
}
}

impl Eq for InversionList {}

impl fmt::Display for InversionList {
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
write!(f, "{}", self.to_str())
}
}

pub struct InversionListIterator<'a> {
list: &'a InversionList,
interval_index: usize,
current_value: u32,
}

impl<'a> InversionListIterator<'a> {
pub fn new(list: &'a InversionList) -> Self {
if let Some(&(inf, _)) = list.intervals.first() {
Self {
list,
interval_index: 0,
current_value: inf,
}
} else {
Self {
list,
interval_index: 0,
current_value: 0,
}
}
}
}

impl<'a> Iterator for InversionListIterator<'a> {
type Item = u32;

fn next(&mut self) -> Option<Self::Item> {
while self.interval_index < self.list.intervals.len() {
let (inf, sup) = self.list.intervals[self.interval_index];
if self.current_value < inf {
self.current_value = inf;
}
if self.current_value < sup {
let value = self.current_value;
self.current_value += 1;
return Some(value);
}
self.interval_index += 1;
if self.interval_index < self.list.intervals.len() {
self.current_value = self.list.intervals[self.interval_index].0;
}
}
None
}
}

pub struct InversionListCoupleIterator<'a> {
list: &'a InversionList,
couple_index: usize,
}

impl<'a> InversionListCoupleIterator<'a> {
pub fn new(list: &'a InversionList) -> Self {
Self {
list,
couple_index: 0,
}
}
}

impl<'a> Iterator for InversionListCoupleIterator<'a> {
type Item = (u32, u32);

fn next(&mut self) -> Option<Self::Item> {
if self.couple_index < self.list.intervals.len() {
let item = self.list.intervals[self.couple_index];
self.couple_index += 1;
Some(item)
} else {
None
}
}
}

impl<'a> IntoIterator for &'a InversionList {
type Item = u32;
type IntoIter = InversionListIterator<'a>;

fn into_iter(self) -> Self::IntoIter {
InversionListIterator::new(self)
}
}
