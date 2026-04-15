pub fn skp_(haystack: &[u8], needle: &[u8]) -> Option<usize> {
if needle.is_empty() {
return Some(0);
}
haystack.windows(needle.len()).position(|window| window == needle)
}
