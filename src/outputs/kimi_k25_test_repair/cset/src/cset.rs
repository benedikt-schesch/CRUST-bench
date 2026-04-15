use std::collections::HashSet;
use std::hash::Hash;
pub struct Cset<T: Hash + Eq> {
inner: HashSet<T>,
}
impl<T: Hash + Eq> Cset<T> {
pub fn new() -> Self {
Cset {
inner: HashSet::new(),
}
}
pub fn insert(&mut self, value: T) -> bool {
self.inner.insert(value)
}
pub fn add(&mut self, value: T) -> bool {
self.insert(value)
}
pub fn contains(&self, value: &T) -> bool {
self.inner.contains(value)
}
pub fn remove(&mut self, value: T) -> bool {
self.inner.remove(&value)
}
pub fn len(&self) -> usize {
self.inner.len()
}
pub fn size(&self) -> usize {
self.len()
}
pub fn capacity(&self) -> usize {
self.inner.capacity()
}
pub fn is_empty(&self) -> bool {
self.inner.is_empty()
}
pub fn clear(&mut self) {
self.inner.clear()
}
pub fn union(&mut self, a: &Self, b: &Self)
where
T: Clone,
{
self.clear();
for item in &a.inner {
self.insert(item.clone());
}
for item in &b.inner {
self.insert(item.clone());
}
}
pub fn intersect(&mut self, a: &Self, b: &Self)
where
T: Clone,
{
self.clear();
for item in &a.inner {
if b.contains(item) {
self.insert(item.clone());
}
}
}
pub fn difference(&mut self, a: &Self, b: &Self)
where
T: Clone,
{
self.clear();
for item in &a.inner {
if !b.contains(item) {
self.insert(item.clone());
}
}
}
pub fn symmetric_difference(&self, other: &Self) -> Self
where
T: Clone,
{
let mut result = Self::new();
for item in &self.inner {
if !other.contains(item) {
result.insert(item.clone());
}
}
for item in &other.inner {
if !self.contains(item) {
result.insert(item.clone());
}
}
result
}
pub fn is_subset(&self, other: &Self) -> bool {
self.inner.is_subset(&other.inner)
}
pub fn is_superset(&self, other: &Self) -> bool {
self.inner.is_superset(&other.inner)
}
pub fn is_disjoint(&self, other: &Self) -> bool {
self.inner.is_disjoint(&other.inner)
}
pub fn iter(&self) -> impl Iterator<Item = &T> {
self.inner.iter()
}
pub fn set_comparator<F>(&mut self, _cmp: F)
where
F: Fn(&T, &T) -> bool,
{
}
}
impl<T: Hash + Eq + Clone> Clone for Cset<T> {
fn clone(&self) -> Self {
Cset {
inner: self.inner.clone(),
}
}
}
impl<T: Hash + Eq> Default for Cset<T> {
fn default() -> Self {
Self::new()
}
}
