use Simple_Config::simple_config::*;
fn main() {
match cfg_parse("") {
Ok(cfg) => println!("Config loaded: {:?}", cfg),
Err(e) => println!("Error: {}", e),
}
}
