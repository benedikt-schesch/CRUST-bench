
use crate::lzma_packet::LZMAPacket;
use crate::packet_slab::PacketSlab;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PacketSlabUndo {
pub index: usize,
pub operation: UndoOperation,
pub position: usize,
pub old_packet: Option<LZMAPacket>,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UndoOperation {
Insert,
Delete,
Update,
}
pub struct PacketSlabUndoStack {
stack: Vec<PacketSlabUndo>,
}
impl PacketSlabUndoStack {
pub fn new() -> Self {
Self { stack: Vec::new() }
}
pub fn with_capacity(capacity: usize) -> Self {
Self {
stack: Vec::with_capacity(capacity),
}
}
pub fn push(&mut self, undo: PacketSlabUndo) {
self.stack.push(undo);
}
pub fn insert(&mut self, undo: PacketSlabUndo) {
self.push(undo);
}
pub fn pop(&mut self) -> Option<PacketSlabUndo> {
self.stack.pop()
}
pub fn is_empty(&self) -> bool {
self.stack.is_empty()
}
pub fn len(&self) -> usize {
self.stack.len()
}
pub fn count(&self) -> usize {
self.len()
}
pub fn clear(&mut self) {
self.stack.clear();
}
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
if undo.index < slab.len() {
slab.remove(undo.index);
}
}
UndoOperation::Delete => {
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
