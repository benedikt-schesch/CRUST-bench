//! Packet slab allocator for efficient memory management.

use crate::lzma_packet::LZMAPacket;

/// A slab allocator for managing LZMA packets efficiently.
pub struct PacketSlab {
packets: Vec<LZMAPacket>,
capacity: usize,
}

impl PacketSlab {
/// Creates a new packet slab with the specified capacity.
pub fn new(capacity: usize) -> Self {
Self {
packets: Vec::with_capacity(capacity),
capacity,
}
}

/// Creates a new packet slab with the specified capacity.
pub fn with_capacity(capacity: usize) -> Self {
Self::new(capacity)
}

/// Returns the capacity of the slab.
pub fn capacity(&self) -> usize {
self.capacity
}

/// Returns the number of packets in the slab.
pub fn len(&self) -> usize {
self.packets.len()
}

/// Returns true if the slab is empty.
pub fn is_empty(&self) -> bool {
self.packets.is_empty()
}

/// Clears the slab.
pub fn clear(&mut self) {
self.packets.clear();
}

/// Returns a slice of the packets in the slab.
pub fn packets(&self) -> &[LZMAPacket] {
&self.packets
}

/// Returns a mutable slice of the packets in the slab.
pub fn packets_mut(&mut self) -> &mut [LZMAPacket] {
&mut self.packets
}

/// Pushes a packet into the slab.
pub fn push(&mut self, packet: LZMAPacket) {
self.packets.push(packet);
}

/// Removes and returns the last packet.
pub fn pop(&mut self) -> Option<LZMAPacket> {
self.packets.pop()
}

/// Removes a packet at the specified index.
pub fn remove(&mut self, index: usize) -> LZMAPacket {
self.packets.remove(index)
}

/// Inserts a packet at the specified index.
pub fn insert(&mut self, index: usize, packet: LZMAPacket) {
self.packets.insert(index, packet);
}
}

impl Default for PacketSlab {
fn default() -> Self {
Self::new(1024)
}
}
