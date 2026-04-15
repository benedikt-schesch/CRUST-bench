use std::sync::Mutex;
const BUFFER_SZ: usize = 8192;
const DUMMY_BLK_SZ: usize = 12;
const ADDR_2G: usize = 0x80000000;
struct LjmmState {
page_size: usize,
page_mask: usize,
addr_upbound: usize,
addr_lowbound: usize,
dummy_blk: Option<Vec<u8>>,
map_file: String,
buffer: Option<Vec<u8>>,
buf_len: usize,
os_take_care_1g_2g: i8,
init_succ: i8,
}
impl LjmmState {
const fn new() -> Self {
Self {
page_size: 0,
page_mask: 0,
addr_upbound: 0,
addr_lowbound: 0,
dummy_blk: None,
map_file: String::new(),
buffer: None,
buf_len: 0,
os_take_care_1g_2g: 0,
init_succ: 0,
}
}
}
static LJMM: Mutex<LjmmState> = Mutex::new(LjmmState::new());
pub fn ljmm_init() -> i32 {
let mut ljmm = LJMM.lock().unwrap();
#[cfg(stress_test)]
let os_take_care = 0i8;
#[cfg(not(stress_test))]
let os_take_care = 1i8;
ljmm.os_take_care_1g_2g = os_take_care;
ljmm.addr_lowbound = 0;
ljmm.addr_upbound = ADDR_2G;
ljmm.page_size = 4096;
ljmm.page_mask = ljmm.page_size - 1;
ljmm.dummy_blk = Some(vec![0u8; DUMMY_BLK_SZ]);
ljmm.map_file = "/proc/self/maps".to_string();
ljmm.buffer = Some(vec![0u8; BUFFER_SZ]);
ljmm.init_succ = 1;
ljmm.init_succ as i32
}
pub fn ljmm_let_os_take_care_1g_2g(turn_on: i32) {
let mut ljmm = LJMM.lock().unwrap();
ljmm.os_take_care_1g_2g = turn_on as i8;
}
pub fn ljmm_test_set_test_param(map_file: &str, sbrk0: usize, page_size: i32) {
let mut ljmm = LJMM.lock().unwrap();
ljmm.map_file = map_file.to_string();
ljmm.addr_lowbound = sbrk0;
assert!(page_size != 0 && (((page_size - 1) & page_size) == 0),
"page_size must be non-zero and a power of 2");
let ps = page_size as usize;
ljmm.page_size = ps;
ljmm.page_mask = ps - 1;
}
