#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Morton {
pub lo: u32,
pub hi: u32,
}

pub fn unmortoner(x: u64) -> u32 {
let mut x = x;
x &= 0x5555_5555_5555_5555;
x = (x | (x >> 0o1)) & 0x3333_3333_3333_3333;
x = (x | (x >> 0o2)) & 0x0F0F_0F0F_0F0F_0F0F;
x = (x | (x >> 0o4)) & 0x00FF_00FF_00FF_00FF;
x = (x | (x >> 0o10)) & 0x0000_FFFF_0000_FFFF;
x = (x | (x >> 0o20)) & 0x0000_0000_FFFF_FFFF;
x as u32
}

pub fn morton(hi: u32, lo: u32) -> u64 {
let mut xu = lo as u64;
let mut yu = hi as u64;

xu = (xu | (xu << 0o20)) & 0x0000_FFFF_0000_FFFF;
xu = (xu | (xu << 0o10)) & 0x00FF_00FF_00FF_00FF;
xu = (xu | (xu << 0o4)) & 0x0F0F_0F0F_0F0F_0F0F;
xu = (xu | (xu << 0o2)) & 0x3333_3333_3333_3333;
xu = (xu | (xu << 0o1)) & 0x5555_5555_5555_5555;

yu = (yu | (yu << 0o20)) & 0x0000_FFFF_0000_FFFF;
yu = (yu | (yu << 0o10)) & 0x00FF_00FF_00FF_00FF;
yu = (yu | (yu << 0o4)) & 0x0F0F_0F0F_0F0F_0F0F;
yu = (yu | (yu << 0o2)) & 0x3333_3333_3333_3333;
yu = (yu | (yu << 0o1)) & 0x5555_5555_5555_5555;

xu | (yu << 1)
}
