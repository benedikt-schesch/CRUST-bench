//! Undo stack for packet slab operations to support rollback functionality.

use crate::lzma_packet::LZMAPacket;
use crate::packet_slab::PacketSlab;

/// Represents a single undo operation for the packet slab.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PacketSlabUndo {
pub index: usize,
pub operation: UndoOperation,
pub position: usize,
pub old_packet: Option<LZMAPacket>,
}

/// Types of undo operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UndoOperation {
Insert,
Delete,
Update,
}

/// A stack for managing packet slab undo operations.
pub struct PacketSlabUndoStack {
stack: Vec<PacketSlabUndo>,
}

impl PacketSlabUndoStack {
/// Creates a new empty undo stack.
pub fn new() -> Self {
Self { stack: Vec::new() }
}

/// Creates a new undo stack with the specified capacity.
pub fn with_capacity(capacity: usize) -> Self {
Self {
stack: Vec::with_capacity(capacity),
}
}

/// Pushes an undo operation onto the stack.
pub fn push(&mut self, undo: PacketSlabUndo) {
self.stack.push(undo);
}

/// Inserts an undo operation onto the stack (alias for push).
pub fn insert(&mut self, undo: PacketSlabUndo) {
self.push(undo);
}

/// Pops an undo operation from the stack.
pub fn pop(&mut self) -> Option<PacketSlabUndo> {
self.stack.pop()
}

/// Returns true if the undo stack is empty.
pub fn is_empty(&self) -> bool {
self.stack.is_empty()
}

/// Returns the number of undo operations in the stack.
pub fn len(&self) -> usize {
self.stack.len()
}

/// Returns the number of undo operations in the stack (alias for len).
pub fn count(&self) -> usize {
self.len()
}

/// Clears all undo operations from the stack.
pub fn clear(&mut self) {
self.stack.clear();
}

/// Applies the undo operations to the slab.
pub fn apply(&mut self, slab: &mut PacketSlab) {
while let Some(undo) = self.pop() {
match undo.operation {
UndoOperation::Update => {
if undo.index < slab.len() {
if let Some(packet) = undo.old_packet {
slab.packets_mut()[undo.index] = packet;
}
}
}
UndoOperation::Insert => {
// Remove the inserted element at index
if undo.index < slab.len() {
slab.remove(undo.index);
}
}
UndoOperation::Delete => {
// Restore deleted packet at index
if let Some(packet) = undo.old_packet {
if undo.index <= slab.len() {
slab.insert(undo.index, packet);
}
}
}
}
}
}
}

impl Default for PacketSlabUndoStack {
fn default() -> Self {
Self::new()
}
}
