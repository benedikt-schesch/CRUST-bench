use crate::buffer_mgr::get_dirty_flags;
use crate::buffer_mgr::get_fix_counts;
use crate::buffer_mgr::get_frame_contents;
use crate::buffer_mgr::BM_BufferPool;
use crate::buffer_mgr::BM_PageHandle;
use crate::buffer_mgr::ReplacementStrategy;
pub fn print_pool_content(bm: &BM_BufferPool) {
println!("{}", sprint_pool_content(bm));
}
pub fn print_page_content(page: &BM_PageHandle) {
println!("{}", sprint_page_content(page));
}
pub fn sprint_pool_content(bm: &BM_BufferPool) -> String {
let frame_content = get_frame_contents(bm);
let dirty = get_dirty_flags(bm);
let fix_count = get_fix_counts(bm);
let mut result = String::new();
result.push('{');
result.push_str(match bm.strategy {
ReplacementStrategy::RsFifo => "FIFO",
ReplacementStrategy::RsLru => "LRU",
ReplacementStrategy::RsClock => "CLOCK",
ReplacementStrategy::RsLfu => "LFU",
ReplacementStrategy::RsLruK => "LRU-K",
});
result.push_str(&format!(" {}}}: ", bm.num_pages));
for i in 0..bm.num_pages as usize {
let fc = frame_content.get(i).copied().unwrap_or(0);
let d = dirty.get(i).copied().unwrap_or(false);
let f = fix_count.get(i).copied().unwrap_or(0);
result.push_str(&format!(
"{}[{}{}{}]",
if i == 0 { "" } else { "," },
fc,
if d { "x" } else { " " },
f
));
}
result
}
pub fn sprint_page_content(page: &BM_PageHandle) -> String {
let mut result = format!("[Page {}]\n", page.page_num);
let bytes = page.data.as_bytes();
let max = bytes.len().min(4096);
for i in 1..=max {
let b = bytes[i - 1];
result.push_str(&format!(
"{:02X}{}{}",
b,
if i % 8 != 0 { "" } else { " " },
if i % 64 != 0 { "" } else { "\n" }
));
}
result
}
pub fn print_strat(bm: &BM_BufferPool) {
let s = match bm.strategy {
ReplacementStrategy::RsFifo => "FIFO",
ReplacementStrategy::RsLru => "LRU",
ReplacementStrategy::RsClock => "CLOCK",
ReplacementStrategy::RsLfu => "LFU",
ReplacementStrategy::RsLruK => "LRU-K",
};
println!("{}", s);
}
