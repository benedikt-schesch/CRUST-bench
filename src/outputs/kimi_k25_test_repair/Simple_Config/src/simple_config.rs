use std::collections::HashMap;
#[derive(Debug, Clone)]
pub struct Cfg {
entries: HashMap<String, CfgEntry>,
}
#[derive(Debug, Clone)]
pub struct CfgEntry {
pub key: String,
pub value: CfgVal,
}
#[derive(Debug, Clone)]
pub enum CfgVal {
String(String),
Int(i64),
Float(f64),
Bool(bool),
Color(CfgColor),
List(Vec<CfgVal>),
}
#[derive(Debug, Clone)]
pub struct CfgColor {
pub r: u8,
pub g: u8,
pub b: u8,
pub a: u8,
}
impl Cfg {
pub fn new() -> Self {
Self {
entries: HashMap::new(),
}
}
}
pub fn cfg_parse(content: &str) -> Result<Cfg, String> {
Ok(Cfg {
entries: HashMap::new(),
})
}
