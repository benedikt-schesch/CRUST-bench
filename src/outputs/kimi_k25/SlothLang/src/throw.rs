pub fn math_err(msg: &str) {
panic!("[ERROR] {}", msg);
}

pub fn op_err(type_: &str, code: u8) {
panic!("[ERROR] invalid {} code: 0x{:02x}", type_, code);
}
