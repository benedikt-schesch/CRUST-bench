use Simple_Config::simple_config::{Cfg, CfgColor, CfgEntry, cfg_parse, CfgVal};
fn main() {
let input = "test = value";
match cfg_parse(input) {
Ok(cfg) => {
println!("Parsed successfully: {:?}", cfg);
}
Err(e) => {
println!("Parse error: {}", e);
}
}
}
