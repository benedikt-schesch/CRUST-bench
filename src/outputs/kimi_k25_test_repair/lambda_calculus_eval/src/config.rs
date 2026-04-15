
pub enum option_type_t {
FILENAME,
STEP_REDUCTION,
REDUCTION_ORDER,
}
pub fn trim(s: &mut String) {
*s = s.trim().to_string();
}
pub fn get_config_type(key: &str) -> option_type_t {
match key {
"file" => option_type_t::FILENAME,
"step_by_step_reduction" => option_type_t::STEP_REDUCTION,
"reduction_order" => option_type_t::REDUCTION_ORDER,
_ => panic!("Unknown config type: {}", key),
}
}
pub fn parse_config(line: &str, key: &mut String, value: &mut String) {
let parts: Vec<&str> = line.splitn(2, '=').collect();
if parts.len() == 2 {
*key = parts[0].trim().to_string();
*value = parts[1].trim().to_string();
} else {
*key = line.trim().to_string();
*value = String::new();
}
}
