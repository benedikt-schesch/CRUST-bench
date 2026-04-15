//! Type definitions for M17 protocol

/// Frame type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameType {
/// Stream frame
Stream,
/// Packet frame
Packet,
/// Reserved
Reserved,
}

/// Preamble type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreambleType {
/// Standard preamble
Standard,
/// Short preamble
Short,
}

/// Link Setup Frame structure
#[derive(Debug, Clone, Copy)]
pub struct LSF {
/// Raw LSF data bytes
pub data: [u8; 30],
}

/// Symbols per frame (40ms frame at 4800 baud = 192 symbols)
pub const SYM_PER_FRA: usize = 192;

/// Symbols per payload (frame minus syncword and other overhead)
pub const SYM_PER_PLD: usize = 184;

/// Symbols per syncword (typically 8 symbols for 16-bit syncword at 2 bits/symbol)
pub const SYM_PER_SWD: usize = 8;
