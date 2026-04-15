//! Tests for packet slab undo stack.

use crate::packet_slab_undo_stack::{PacketSlabUndo, PacketSlabUndoStack, UndoOperation};
use crate::lzma_packet::LZMAPacket;
use crate::packet_slab::PacketSlab;

fn main() {
// Setup test data
let packets = vec![
LZMAPacket::literal(),
LZMAPacket::match_packet(5, 10),
LZMAPacket::short_rep_packet(),
];

let undo = PacketSlabUndo {
index: 0,
operation: UndoOperation::Update,
position: 0,
old_packet: Some(packets[0]),
};

// Create undo stack and test operations
let mut stack = PacketSlabUndoStack::new();
stack.push(undo);

let undo2 = PacketSlabUndo {
index: 1,
operation: UndoOperation::Delete,
position: 1,
old_packet: Some(packets[1]),
};

stack.push(undo2);

// Test applying undo operations
let mut slab = PacketSlab::new(10);
slab.push(packets[0]);
slab.push(packets[1]);

stack.apply(&mut slab);
}
