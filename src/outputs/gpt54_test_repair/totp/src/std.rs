
pub const SIZE_MAX: usize = !0;
pub fn memset(s: &mut [u8], c: i32, n: usize) -> &mut [u8] {
let limit = n.min(s.len());
let byte = c as u8;
let mut i = 0usize;
while i < limit {
s[i] = byte;
i += 1;
}
s
}
pub fn memcpy<'a>(dst: &'a mut [u8], src: &'a [u8], n: usize) -> &'a mut [u8] {
let limit = n.min(dst.len()).min(src.len());
let mut i = 0usize;
while i < limit {
dst[i] = src[i];
i += 1;
}
dst
}
pub fn strlen(s: &str) -> usize {
s.len()
}
