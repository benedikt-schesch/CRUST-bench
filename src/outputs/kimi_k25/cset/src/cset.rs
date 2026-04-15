use std::collections::HashSet;
use std::hash::Hash;

/// A generic set implementation that stores unique elements.
pub struct Cset<T: Hash + Eq> {
inner: HashSet<T>,
}

impl<T: Hash + Eq> Cset<T> {
/// Creates a new empty set.
pub fn new() -> Self {
Cset {
inner: HashSet::new(),
}
}

/// Adds a value to the set.
/// Returns true if the value was not already present in the set.
pub fn insert(&mut self, value: T) -> bool {
self.inner.insert(value)
}

/// Alias for insert.
pub fn add(&mut self, value: T) -> bool {
self.insert(value)
}

/// Returns true if the set contains the specified value.
pub fn contains(&self, value: &T) -> bool {
self.inner.contains(value)
}

/// Removes a value from the set.
/// Returns true if the value was present in the set.
pub fn remove(&mut self, value: T) -> bool {
self.inner.remove(&value)
}

/// Returns the number of elements in the set.
pub fn len(&self) -> usize {
self.inner.len()
}

/// Alias for len.
pub fn size(&self) -> usize {
self.len()
}

/// Returns the number of elements the set can hold without reallocating.
pub fn capacity(&self) -> usize {
self.inner.capacity()
}

/// Returns true if the set contains no elements.
pub fn is_empty(&self) -> bool {
self.inner.is_empty()
}

/// Clears the set, removing all values.
pub fn clear(&mut self) {
self.inner.clear()
}

/// Computes the union of sets a and b, storing the result in self.
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

/// Computes the intersection of sets a and b, storing the result in self.
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

/// Computes the difference of sets a and b (elements in a but not in b), storing the result in self.
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

/// Returns a new set containing elements in either set but not both (symmetric difference).
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

/// Returns true if all elements of self are contained in other (subset).
pub fn is_subset(&self, other: &Self) -> bool {
self.inner.is_subset(&other.inner)
}

/// Returns true if all elements of other are contained in self (superset).
pub fn is_superset(&self, other: &Self) -> bool {
self.inner.is_superset(&other.inner)
}

/// Returns true if the set has no elements in common with other.
pub fn is_disjoint(&self, other: &Self) -> bool {
self.inner.is_disjoint(&other.inner)
}

/// Returns an iterator over the set.
pub fn iter(&self) -> impl Iterator<Item = &T> {
self.inner.iter()
}

/// Sets a custom comparator (not supported with HashSet backend, does nothing).
pub fn set_comparator<F>(&mut self, _cmp: F)
where
F: Fn(&T, &T) -> bool,
{
// HashSet does not support custom runtime comparators.
// This method is provided for API compatibility but has no effect.
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
