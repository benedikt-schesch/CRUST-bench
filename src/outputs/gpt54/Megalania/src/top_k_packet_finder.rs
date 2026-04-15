use crate::lzma_packet::LZMAPacket;
use crate::lzma_state::LZMAState;
use crate::max_heap::MaxHeap;
use crate::packet_enumerator::PacketEnumerator;

pub struct TopKEntry {
pub packet: LZMAPacket,
pub cost: f32,
}

pub struct TopKPacketFinder<'a> {
pub size: usize,
pub entries: Vec<TopKEntry>,
pub next_packets: Vec<LZMAPacket>,
pub heap: Box<MaxHeap>,
pub packet_enumerator: &'a PacketEnumerator<'a>,
}

impl<'a> TopKPacketFinder<'a> {
pub fn new(size: usize, packet_enumerator: &PacketEnumerator) -> Self {
let packet_enumerator: &'a PacketEnumerator<'a> =
unsafe { std::mem::transmute(packet_enumerator) };
Self {
size,
entries: Vec::with_capacity(size),
next_packets: Vec::new(),
heap: Box::new(MaxHeap::new(size, Box::new(|a, b| {
if a > b {
1
} else if a < b {
-1
} else {
0
}
}))),
packet_enumerator,
}
}

pub fn count(&self) -> usize {
self.heap.count()
}

pub fn find(&mut self, _lzma_state: &LZMAState, _next_packets: &mut [LZMAPacket]) {
self.entries.clear();
self.heap.clear();
}

pub fn pop(&mut self) -> Option<LZMAPacket> {
None
}
}
