
use crate::lzma_packet::LZMAPacket;
pub struct PacketSlab {
packets: Vec<LZMAPacket>,
capacity: usize,
}
impl PacketSlab {
pub fn new(capacity: usize) -> Self {
Self {
packets: Vec::with_capacity(capacity),
capacity,
}
}
pub fn with_capacity(capacity: usize) -> Self {
Self::new(capacity)
}
pub fn capacity(&self) -> usize {
self.capacity
}
pub fn len(&self) -> usize {
self.packets.len()
}
pub fn is_empty(&self) -> bool {
self.packets.is_empty()
}
pub fn clear(&mut self) {
self.packets.clear();
}
pub fn packets(&self) -> &[LZMAPacket] {
&self.packets
}
pub fn packets_mut(&mut self) -> &mut [LZMAPacket] {
&mut self.packets
}
pub fn push(&mut self, packet: LZMAPacket) {
self.packets.push(packet);
}
pub fn pop(&mut self) -> Option<LZMAPacket> {
self.packets.pop()
}
pub fn remove(&mut self, index: usize) -> LZMAPacket {
self.packets.remove(index)
}
pub fn insert(&mut self, index: usize, packet: LZMAPacket) {
self.packets.insert(index, packet);
}
}
impl Default for PacketSlab {
fn default() -> Self {
Self::new(1024)
}
}
