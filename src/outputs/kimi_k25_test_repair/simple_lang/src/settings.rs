pub struct Settings {
pub debug: bool,
pub optimize: bool,
}
impl Default for Settings {
fn default() -> Self {
Settings {
debug: false,
optimize: true,
}
}
}
