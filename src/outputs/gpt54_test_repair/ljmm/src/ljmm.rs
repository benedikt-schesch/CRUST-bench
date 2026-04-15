use std::sync::{Mutex, OnceLock};
const BUFFER_SZ: usize = 8192;
const DUMMY_BLK_SZ: usize = 12;
const ADDR_2G: usize = 0x8000_0000;
const ADDR_1G: usize = 0x4000_0000;
#[derive(Clone, Default)]
struct Ljmm {
page_size: usize,
page_mask: usize,
addr_upbound: usize,
addr_lowbound: usize,
dummy_blk: Option<Vec<u8>>,
map_file: Option<String>,
buffer: String,
buf_len: i32,
os_take_care_1g_2g: bool,
init_succ: bool,
}
fn state() -> &'static Mutex<Ljmm> {
static STATE: OnceLock<Mutex<Ljmm>> = OnceLock::new();
STATE.get_or_init(|| Mutex::new(Ljmm::default()))
}
pub fn ljmm_init() -> i32 {
let mut ljmm = state().lock().expect("ljmm mutex poisoned");
ljmm.os_take_care_1g_2g = true;
ljmm.addr_lowbound = 0;
ljmm.addr_upbound = ADDR_2G;
ljmm.page_size = 4096;
ljmm.page_mask = ljmm.page_size - 1;
ljmm.dummy_blk = Some(vec![0u8; DUMMY_BLK_SZ]);
ljmm.map_file = Some("/proc/self/maps".to_string());
ljmm.buffer = String::with_capacity(BUFFER_SZ);
ljmm.buf_len = 0;
ljmm.init_succ = true;
1
}
pub fn ljmm_let_os_take_care_1g_2g(turn_on: i32) {
let mut ljmm = state().lock().expect("ljmm mutex poisoned");
ljmm.os_take_care_1g_2g = turn_on != 0;
}
pub fn ljmm_test_set_test_param(map_file: &str, sbrk0: usize, page_size: i32) {
assert!(page_size > 0);
let ps = page_size as usize;
assert!(((ps - 1) & ps) == 0);
let mut ljmm = state().lock().expect("ljmm mutex poisoned");
ljmm.map_file = Some(map_file.to_string());
ljmm.addr_lowbound = sbrk0;
ljmm.page_size = ps;
ljmm.page_mask = ps - 1;
}
