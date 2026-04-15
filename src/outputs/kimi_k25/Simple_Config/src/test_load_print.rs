use Simple_Config::simple_config::*;

fn main() {
// Test loading and printing configuration
match cfg_parse("") {
Ok(cfg) => println!("Config loaded: {:?}", cfg),
Err(e) => println!("Error: {}", e),
}
}
